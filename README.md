# opencc-rust

`opencc-rust` 是依照本資料夾上層 `OpenCC/` C# 實作移植的純 Rust OpenCC 函式庫。核心行為保留原實作的內嵌字典、locale preset、Trie 最長匹配與多階段轉換流程。

## 功能

- 內建 `cn`、`hk`、`tw`、`tw2`、`twp`、`jp` locale。
- 支援 `full`、`cn2t`、`t2cn` preset。
- 支援自訂字典與多個字典群組串接。
- Unicode scalar/code point 層級的 Trie 最長匹配。
- XML-compatible HTML 轉換與還原。
- MIT 授權，與原 C# OpenCC 專案相同。

## 新手上路

### 開發環境需求

- Rust 1.95+（專案使用 edition 2024）
- Cargo
- Git

安裝 Rust：

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustc --version
cargo --version
```

### 加入專案

在 `Cargo.toml` 加入：

```toml
[dependencies]
opencc-rust = { git = "https://github.com/doggy8088/opencc-rust" }
```

### 第一個程式

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let converter = opencc_rust::converter("cn", "tw2")?;
    println!("{}", converter.convert("汉语")); // 漢語
    Ok(())
}
```

## Locale 與 preset

預設 `opencc_rust::converter(from, to)` 使用 full preset：

| from/to | 說明 |
| --- | --- |
| `cn` | 中國大陸簡體 |
| `hk` | 香港繁體異體字 |
| `tw` | 台灣繁體異體字 |
| `tw2` | 台灣繁體常用詞 |
| `twp` | 台灣繁體含 IT、姓名與其他詞彙 |
| `jp` | 日本新字體/異體字 |
| `t` | passthrough，不載入該階段字典 |

方向限定 preset：

```rust
let cn_to_tw = opencc_rust::presets::cn2t::converter("cn", "tw2")?;
let tw_to_cn = opencc_rust::presets::t2cn::converter("tw", "cn")?;
```

## 自訂字典

字串格式與 C# 實作相同：每筆 `來源 目標`，筆與筆之間以 `|` 分隔。

```rust
let converter = opencc_rust::custom_converter_from_string("香蕉 banana|蘋果 apple|梨 pear");
assert_eq!(converter.convert("香蕉 蘋果 梨"), "banana apple pear");
```

也可以使用 `DictEntry`：

```rust
use opencc_rust::{custom_converter_from_entries, DictEntry};

let converter = custom_converter_from_entries([
    DictEntry::new("“", "「"),
    DictEntry::new("”", "」"),
]);
```

## 進階組合

`converter_factory` 會依序套用每個 `DictGroup`，等同 C# 版本先處理 `from` 群組，再處理 `to` 群組。

```rust
use opencc_rust::{converter_factory, DictEntry, DictGroup};

let first = DictGroup::from_entry_sets([vec![DictEntry::new("a", "b")]]);
let second = DictGroup::from_entry_sets([vec![DictEntry::new("b", "c")]]);
let converter = converter_factory([first, second]);
assert_eq!(converter.convert("a"), "c");
```

## XML-compatible HTML 轉換

與 C# 版本一樣，這裡處理的是可被 XML parser 解析的 HTML/XML。會轉換 lang 範圍內的文字節點、`meta[name=description|keywords]` 的 `content`、`img alt`、`input[type=button] value`，略過 `script`、`style` 與 `ignore-opencc` class。

```rust
let converter = opencc_rust::converter("hk", "cn")?;
let xml = "<html lang='zh-HK'><body><p lang='zh-HK'>漢語</p></body></html>";
let mut html = opencc_rust::HtmlConverter::from_xml_str(converter, xml, "zh-HK", "zh-CN")?;
html.convert();
println!("{}", html.to_xml_string()?);
html.restore();
```

## 開發

```bash
cargo fmt --check
cargo test
cargo run --example basic
cargo run --example custom
cargo run --example html
```

更多移植細節請見 `DEVELOPMENT.md`。

## License

MIT
