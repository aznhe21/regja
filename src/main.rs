mod csv;
mod geocoder;
pub mod geolonia;

use std::env;
use std::fs::File;
use std::{path::Path, io::{BufRead, BufReader}};

use crate::geocoder::{Config, Location, UserLocation, ReverseGeocoder};

fn load_geolonia<P: AsRef<Path>>(gc: &mut ReverseGeocoder, path: P) -> std::io::Result<()> {
    let f = BufReader::new(File::open(path)?);
    let mut lines = f.lines();
    lines.next().expect("ヘッダがありません")?;
    for line in lines {
        let line = line?;
        let addr = geolonia::parse_geolonia(&line).unwrap();
        gc.add(addr);
    }

    Ok(())
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if !(3..=4).contains(&args.len()) {
        println!("{} latitude longitude [accuracy]", env!("CARGO_PKG_NAME"));
        std::process::exit(1);
    }

    let location = UserLocation {
        location: Location {
            latitude: args[1].parse().expect("緯度の値が不正です"),
            longitude: args[2].parse().expect("緯度の値が不正です"),
        },
        accuracy: args.get(3).map(|a| a.parse()).transpose().expect("精度の値が不正です").unwrap_or(1.),
    };

    let mut gc = ReverseGeocoder::new(Config {
        // 100mより悪い精度の入力は100mの精度として扱う
        max_accuracy: 100.,
        // 1km以内を検索する
        max_distance: 1000.,
    });

    load_geolonia(&mut gc, "./latest.csv").unwrap();

    if let Some(addr) = gc.reverse(&location) {
        println!("最寄りの住所");
        println!("{}{}{}", addr.prefecture, addr.municipality, addr.town);
        println!("{} / {}", addr.location.latitude, addr.location.longitude);
    } else {
        println!("最寄りの住所が見つかりませんでした");
        std::process::exit(1);
    }
}
