use crate::csv::CsvFieldParser;
use crate::geocoder::{Address, Location};

/// [Geolonia 住所データ]のCSVをパースし、`Address`に変換する。
///
/// [Geolonia 住所データ]: https://geolonia.github.io/japanese-addresses/
pub fn parse_geolonia(s: &str) -> Result<Address, ParseAddressError> {
    let mut fields =
        CsvFieldParser::new(s).map(|v| v.map_err(|_| ParseAddressError::InvaleFieldFormat));

    // 都道府県コード
    let _pref_code = fields.next().ok_or(ParseAddressError::LackingField)??;
    // 都道府県名
    let prefecture = fields.next().ok_or(ParseAddressError::LackingField)??;
    // 都道府県名カナ
    let _pref_kana = fields.next().ok_or(ParseAddressError::LackingField)??;
    // 都道府県名ローマ字
    let _pref_roma = fields.next().ok_or(ParseAddressError::LackingField)??;
    // 市区町村コード
    let _muni_code = fields.next().ok_or(ParseAddressError::LackingField)??;
    // 市区町村名
    let municipality = fields.next().ok_or(ParseAddressError::LackingField)??;
    // 市区町村名カナ
    let _muni_kana = fields.next().ok_or(ParseAddressError::LackingField)??;
    // 市区町村名ローマ字
    let _muni_roma = fields.next().ok_or(ParseAddressError::LackingField)??;
    // 大字町丁目コード
    let _town_code = fields.next().ok_or(ParseAddressError::LackingField)??;
    // 大字町丁目名
    let town = fields.next().ok_or(ParseAddressError::LackingField)??;
    // 緯度
    let latitude = fields
        .next()
        .ok_or(ParseAddressError::LackingField)??
        .parse()
        .map_err(|_| ParseAddressError::InvalidFloat)?;
    // 経度
    let longitude = fields
        .next()
        .ok_or(ParseAddressError::LackingField)??
        .parse()
        .map_err(|_| ParseAddressError::InvalidFloat)?;

    if fields.next().is_some() {
        return Err(ParseAddressError::TooManyField);
    }

    Ok(Address {
        location: Location {
            latitude,
            longitude,
        },
        prefecture,
        municipality,
        town,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseAddressError {
    /// フィールドの不足
    LackingField,

    /// フィールドが多過ぎる
    TooManyField,

    /// フィールドの形式が不正
    InvaleFieldFormat,

    /// 小数値が不正
    InvalidFloat,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_address() {
        let line = concat!(
            r#""01","北海道","ホッカイドウ","HOKKAIDO","#,
            r#""01101","札幌市中央区","サッポロシチュウオウク","SAPPORO SHI CHUO KU","#,
            r#""011010001001","旭ケ丘一丁目","43.042230","141.319722""#
        );
        let address = parse_geolonia(line);

        assert_eq!(
            address,
            Ok(Address {
                location: Location {
                    latitude: 43.042230,
                    longitude: 141.319722,
                },
                prefecture: "北海道".to_string(),
                municipality: "札幌市中央区".to_string(),
                town: "旭ケ丘一丁目".to_string(),
            })
        );
    }
}
