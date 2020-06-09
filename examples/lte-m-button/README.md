# SORACOM Orbit SDK for Rust - Sample for SORACOM LTE-M Button Series

## Overview

This is a sample code for processing data sent from the SORACOM LTE-M Button series ([SORACOM LTE-M Button Plus](https://soracom.jp/store/5207/) and [SORACOM LTE-M Button for Enterprise](https://soracom.jp/store/5206/)) using [SORACOM Orbit](https://soracom.jp/services/orbit/).

Normally, the following JSON is sent from the SORACOM LTE-M Button series:

```json
{
    "clickType":1,
    "clickTypeName":"SINGLE",
    "batteryLevel":1,
    "binaryParserEnabled":true
}
```

This sample code adds IMSI and name for identifying the SIM, and location information obtained through the [Simple Positioning Feature](https://users.soracom.io/ja-jp/docs/air/get-location-info/#%e3%83%87%e3%83%90%e3%82%a4%e3%82%b9%e3%81%ae%e7%b0%a1%e6%98%93%e7%9a%84%e3%81%aa%e4%bd%8d%e7%bd%ae%e6%83%85%e5%a0%b1-%e7%b0%a1%e6%98%93%e4%bd%8d%e7%bd%ae%e6%b8%ac%e4%bd%8d%e6%a9%9f%e8%83%bd-%e3%82%92%e9%80%81%e4%bf%a1%e3%81%99%e3%82%8b-plan-km1-%e3%81%ae%e3%81%bf) to transform it into the following JSON:

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

## How to Run

1. Compile the source code.
   ```
   cargo build --release
   ```

2. Create a Soralet to be a container for the uploaded code (only for the first time).
   ```
   make create-soralet
   ```
   You need to create a SORACOM account and install [soracom-cli](https://github.com/soracom/soracom-cli) in advance.
   If you want to use a profile other than the default, specify it as an argument:
   ```
   make create-soralet soracom_profile=${profile_name}
   ```
   By default, a Soralet with the ID `lte-m-button` is created. If you want to use a different ID, specify it like this:
   ```
   make create-soralet soralet_id=${soralet_id}
   ```
   You can specify both `soralet_id` and `soracom_profile` at the same time.

3. Upload the compiled code.
   ```
   make upload
   ```
   To override the profile or Soralet ID, follow the same method as in step 3.

4. Test the uploaded code.
   ```
   make test
   ```
   If you see the following result, it's successful:
   ```json
   {
        "body": "{\"clickType\":1,\"clickTypeName\":\"SINGLE\",\"batteryLevel\":1,\"imsi\":\"001019999999999\",\"name\":\"test name\",\"location\":{\"lat\":43.12345,\"lon\":138.112233},\"timestamp\":1702958495084,\"userdata\":\"\"}",
        "contentType": "application/json",
        "encodingType": "plain",
        "resultCode": 0
   }
   ```

5. If it doesn’t work, check the logs.
   ```
   make log
   ```
   The log will be displayed like the following. You can check how far it has been executed or the values stored in variables by comparing it with the messages output by the `log()` function in the sample code.
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
   ]
   ```

6. Once you have confirmed the operation, link the uploaded Soralet to the LTE-M button group settings, and try pressing the button in reality. For details, please refer to the [User Documentation](https://developers.soracom.io/en/docs/orbit/configuration/).