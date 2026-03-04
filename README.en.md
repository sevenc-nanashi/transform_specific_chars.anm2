# transform_specific_chars.anm2

[English](README.en.md) | [日本語](README.md)

[![AviUtl2 Catalog](https://aviutl2-catalog-badge.sevenc7c.workers.dev/badge/v/sevenc-nanashi.transform_specific_chars-anm2)](https://aviutl2-catalog-badge.sevenc7c.workers.dev/package/sevenc-nanashi.transform_specific_chars-anm2)

A script for AviUtl2 that applies transforms only to specified characters in a text object.
This script requires the text option "per-character individual object" to be enabled.

## Installation

Download `sevenc-nanashi.transform_specific_chars-anm2-v{{version}}.au2pkg.zip` from [Releases](https://github.com/sevenc-nanashi/transform_specific_chars.anm2/releases/latest), then drag and drop it into the AviUtl2 preview.

## Usage

1. Enable "per-character individual object" in your text object.
2. Add this script.
3. Set `Target Chars`.
4. Toggle `Regex` if needed.
5. Adjust position, rotation, scale, transparency, and color parameters.

## Target Pattern Syntax

When `Regex` is off, `Target Chars` is parsed with the following rules:

- Plain characters: match the character itself.
- `[[A-Z]]`: match a character range.
- `{{Hiragana}}`: match a Unicode property (Corresponds to `\p{Hiragana}` in [regex](https://docs.rs/regex)).

When `Regex` is on, `Target Chars` is evaluated as a Rust [regex](https://docs.rs/regex), and the transform is applied to characters within the matched range.

## Notes

- Text containing the clear notation `<c...>` and script notation (`<?...?>`) will cause an error.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
