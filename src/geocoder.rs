use std::cmp;

use rayon::prelude::*;

/// 純粋な位置情報を表す構造体。
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Location {
    /// 緯度。
    pub latitude: f32,

    /// 経度。
    pub longitude: f32,
}

impl Location {
    const R: f32 = 6371.01_e3;

    /// 他の`Location`との距離をメートル単位で返す。
    pub fn distance(&self, other: &Location) -> f32 {
        let lat1 = self.latitude.to_radians();
        let lon1 = self.longitude.to_radians();
        let lat2 = other.latitude.to_radians();
        let lon2 = other.longitude.to_radians();

        let d_lat = lat2 - lat1;
        let d_lon = lon2 - lon1;

        let a = (d_lat / 2.).sin() * (d_lat / 2.).sin()
            + lat1.cos() * lat2.cos() * (d_lon / 2.).sin() * (d_lon / 2.).sin();

        let c = 2. * a.sqrt().atan2((1. - a).sqrt());
        Location::R * c
    }
}

/// 測位した位置情報を表す構造体。
#[derive(Debug, Clone)]
pub struct UserLocation {
    /// ユーザーの位置。
    pub location: Location,

    /// 位置の精度。
    pub accuracy: f32,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Address {
    /// 緯度・経度。
    pub location: Location,

    /// 都道府県。
    pub prefecture: String,

    /// 市区町村。
    pub municipality: String,

    /// 町字・大字。
    pub town: String,
}

#[derive(Debug, Clone)]
pub struct Config {
    /// 検索時の最大入力精度。これを上回る精度の入力は、`max_accuracy`に切り詰められる。
    pub max_accuracy: f32,

    /// 検索する最大の距離。これを上回る距離の住所は検索対象にしない。
    pub max_distance: f32,
}

#[derive(Debug, Clone)]
pub struct ReverseGeocoder {
    config: Config,
    addresses: Vec<Address>,
}

impl ReverseGeocoder {
    pub fn new(config: Config) -> ReverseGeocoder {
        ReverseGeocoder {
            config,
            addresses: Vec::new(),
        }
    }

    pub fn add(&mut self, address: Address) {
        self.addresses.push(address);
    }

    pub fn reverse(&self, location: &UserLocation) -> Option<Address> {
        let location = UserLocation {
            location: location.location.clone(),
            accuracy: if location.accuracy < self.config.max_accuracy {
                location.accuracy
            } else {
                self.config.max_accuracy
            },
        };

        self.addresses
            .par_iter()
            .filter_map(|addr| {
                let dist = addr.location.distance(&location.location);
                // dist > max_distance or is NaN
                if !(dist < self.config.max_distance) {
                    None
                } else {
                    Some((addr, dist))
                }
            })
            .min_by_key(|&(_, dist)| UncheckedOrd(dist))
            .map(|(addr, _)| addr.clone())
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
struct UncheckedOrd<T: PartialOrd>(T);

impl<T: PartialOrd> Eq for UncheckedOrd<T> {}

impl<T: PartialOrd> Ord for UncheckedOrd<T> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        match self.partial_cmp(other) {
            Some(ord) => ord,
            None => unsafe { std::hint::unreachable_unchecked() },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_location_dist() {
        let tokyo = Location {
            latitude: 35.681236,
            longitude: 139.767125,
        };
        let shinagawa = Location {
            latitude: 35.628471,
            longitude: 139.738760,
        };

        assert!((tokyo.distance(&shinagawa) - 6402.453).abs() < f32::EPSILON);
    }
}
