# transform_specific_chars.anm2

[English](README.en.md) | [日本語](README.md)

[![AviUtl2 Catalog](https://aviutl2-catalog-badge.sevenc7c.workers.dev/badge/v/sevenc-nanashi.transform_specific_chars-anm2)](https://aviutl2-catalog-badge.sevenc7c.workers.dev/package/sevenc-nanashi.transform_specific_chars-anm2)

指定した文字のみに加工を適用する AviUtl2 スクリプト。

## インストール

[Releases](https://github.com/sevenc-nanashi/transform_specific_chars.anm2/releases/latest) から `sevenc-nanashi.transform_specific_chars-anm2-v{{version}}.au2pkg.zip` をダウンロードし、AviUtl2 のプレビューにドラッグ＆ドロップしてください。

## 使い方

1. テキストオブジェクトで「文字毎に個別オブジェクト」を有効にします。
2. このスクリプトを追加します。
3. `対象文字` に加工対象を入力します。
4. 位置・回転・拡大率・透明度・文字色などを調整します。

## 対象文字の指定

`正規表現` がオフの場合、`対象文字` は次のルールで解釈されます。

- 通常の文字: その文字に一致
- `[[A-Z]]`: 文字範囲に一致
- `{{Hiragana}}`: Unicode プロパティに一致（[regex](https://docs.rs/regex) の `\p{Hiragana}` に対応）

`正規表現` がオンの場合、`対象文字` はそのまま Rust の [regex](https://docs.rs/regex) として評価され、マッチした範囲に含まれる文字へ加工が適用されます。

## 注意点

- 表示クリア記法 `<c...>`、スクリプト記法（`<?...?>`) を含むテキストはエラーになります。

## ライセンス

MIT License で公開しています。詳細は [LICENSE](LICENSE) を参照してください。
