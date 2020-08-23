# REGJA - REverse Geocoder for JApan

[Geoloniaの住所データ]を使って、指定された緯度・経度の住所を逆ジオコーディングするためのRust製ライブラリ・プログラムです。

[Geoloniaの住所データ]: https://geolonia.github.io/japanese-addresses/

## How to use

```sh
$ curl -LO https://raw.githubusercontent.com/geolonia/japanese-addresses/master/data/latest.csv
$ cargo run --release 35.681236 139.767125
    Finished release [optimized] target(s) in 0.02s
     Running `target/release/regja 35.681236 139.767125`
最寄りの住所
東京都千代田区丸の内一丁目
35.68156 / 139.7672
```
