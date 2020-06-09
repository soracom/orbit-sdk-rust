# SORACOM Orbit SDK for Rust - SORACOM LTE-M Button シリーズ向けのサンプル

## 概要

SORACOM LTE-M Button シリーズ ([SORACOM LTE-M Button Plus](https://soracom.jp/store/5207/) および [SORACOM LTE-M Button for Enterprise](https://soracom.jp/store/5206/)) から送られてくるデータを [SORACOM Orbit](https://soracom.jp/services/orbit/) を使って加工するためのサンプルコードです。

SORACOM LTE-M Button シリーズからは通常以下のような JSON が送られてきます。

```json
{
    "clickType":1,
    "clickTypeName":"SINGLE",
    "batteryLevel":1,
    "binaryParserEnabled":true
}
```

ここに SIM を特定するための IMSI や名前を追加したり、[簡易位置即位機能](https://users.soracom.io/ja-jp/docs/air/get-location-info/#%e3%83%87%e3%83%90%e3%82%a4%e3%82%b9%e3%81%ae%e7%b0%a1%e6%98%93%e7%9a%84%e3%81%aa%e4%bd%8d%e7%bd%ae%e6%83%85%e5%a0%b1-%e7%b0%a1%e6%98%93%e4%bd%8d%e7%bd%ae%e6%b8%ac%e4%bd%8d%e6%a9%9f%e8%83%bd-%e3%82%92%e9%80%81%e4%bf%a1%e3%81%99%e3%82%8b-plan-km1-%e3%81%ae%e3%81%bf) により取得した位置情報を追加して、以下のような JSON に変換します。

```json
{
    "clickType":1,
    "clickTypeName":"SINGLE",
    "batteryLevel":1,
    "binaryParserEnabled":true,
    "imsi":"001019999999999",
    "name":"test name",
    "location":{
        "lat":43.12345,
        "lon":138.112233,
    },
    "time": 1702958495084,
    "userdata": ""
}
```

## 実行方法

1. ソースコードをコンパイルします。
   ```
   cargo build --release
   ```

2. コンパイルしたコードをアップロードするための受け皿となる Soralet を作成します。（最初の一度のみ）
   ```
   make create-soralet
   ```
   事前に SORACOM のアカウントを作成し、[soracom-cli](https://github.com/soracom/soracom-cli) をインストールしておく必要があります。
   default 以外のプロファイルを使用したい場合は、
   ```
   make create-soralet soracom_profile=${profile_name}
   ```
   のように引数に指定します。

   デフォルトでは `lte-m-button` という ID の Soralet を作成します。異なる ID を使用したい場合は
   ```
   make create-soralet soralet_id=${soralet_id}
   ```
   のように指定します。
   `soralet_id` と `soracom_profile` は両方とも指定することもできます。

3. コンパイルしたコードをアップロードします。
   ```
   make upload
   ```
   プロファイルや Soralet ID をオーバーライドしたい場合は 3. と同様です。

4. アップロードしたコードをテストします。
   ```
   make test
   ```
   以下のような結果が表示されたら成功です。
   ```json
   {
        "body": "{\"clickType\":1,\"clickTypeName\":\"SINGLE\",\"batteryLevel\":1,\"imsi\":\"001019999999999\",\"name\":\"test name\",\"location\":{\"lat\":43.12345,\"lon\":138.112233},\"timestamp\":1702958495084,\"userdata\":\"\"}",
        "contentType": "application/json",
        "encodingType": "plain",
        "resultCode": 0
   }
   ```

5. うまく行かない場合はログを確認します。
   ```
   make log
   ```
   以下のようにログが表示されます。サンプルコードの中の `log()` 関数で出力しているメッセージと見比べて、どこまで実行されているかを調べたり、変数の中に格納されている値を確認することができます。
   ```json
   [
        {
            "createdTime": 1702958495320,
            "message": "hello, orbit!",
            "operatorId": "OP00xxxxxxxx",
            "soraletId": "lte-m-button",
            "version": 2
        },
   ...
   ```

6．動作確認ができたら、LTE-M ボタンのグループ設定から今アップロードした Soralet を関連付け、実際にボタンを押してみましょう。詳細は [ユーザードキュメント](https://users.soracom.io/ja-jp/docs/orbit/running/) を参照してください。