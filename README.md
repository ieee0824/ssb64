# ssb64

macOSでスクリーンショットを撮影し、base64エンコードしてクリップボードにコピーするCLIツール。

## インストール

### Homebrew

```sh
brew install ieee0824/tap/ssb64
```

### ソースから

```sh
cargo install --git https://github.com/ieee0824/ssb64.git
```

## 使い方

```sh
ssb64
```

実行するとマウスカーソルが範囲選択モードになるので、ドラッグでキャプチャ範囲を選択してください。選択した領域のスクリーンショットがbase64エンコードされてクリップボードにコピーされます。

## 動作環境

- macOS (Apple Silicon)
