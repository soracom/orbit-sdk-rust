# SORACOM Orbit SDK for Rust

SORACOM Orbit のための Rust 言語の SDK です。

SORACOM Orbit でデータ変換処理を行うための WASM モジュールを Rust 言語のソースコードから生成する際にご利用ください。

## 前提条件

SDK 自体のコンパイル、および SDK を使ったプログラムのコンパイルには [Rust](https://www.rust-lang.org/) を利用します。事前にインストールしておいてください。

Rust をインストールした後、以下のように WASM ターゲットをインストールしてください。

```console
rustup target add wasm32-unknown-unknown
```

## 利用方法

このリポジトリの `src` ディレクトリに SDK のソースコードが含まれています。具体的な利用方法は `examples` ディレクトリの各サンプルをご参照ください。

## 利用例

このリポジトリの `examples` ディレクトリには以下のサンプルが含まれています。

- [`lte-m-button`](./examples/lte-m-button/)

  SORACOM LTE-M Button シリーズ ([SORACOM LTE-M Button Plus](https://soracom.jp/store/5207/) および [SORACOM LTE-M Button for Enterprise](https://soracom.jp/store/5206/)) から送られてくるデータに補助的な情報を追加して送信するサンプルです。

- [`sensit`](./examples/sensit/)

  [Sens'it](https://soracom.jp/store/5214/) から送られてくるデータを取り扱うサンプルです。

## ライセンス

この SDK は MIT ライセンスの下で公開されています。詳細については [LICENSE](./LICENSE) ファイルをご覧ください。
