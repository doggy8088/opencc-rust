# opencc-rust 範例

這個資料夾同時包含 Cargo 原本的單檔範例，以及給使用者閱讀的資料夾式範例。資料夾式範例各自附有 README，說明完整用途與細節。

## 範例列表

| 範例 | 用途 |
| --- | --- |
| [basic-conversion](basic-conversion/) | 基本簡體轉臺灣繁體。 |
| [no-phrase-conversion](no-phrase-conversion/) | 只做字形轉換，不套用地區詞彙。 |
| [locale-differences](locale-differences/) | 比較不同簡繁詞庫輸出。 |
| [custom-dictionary](custom-dictionary/) | 使用自訂詞典與多階段轉換。 |
| [html-conversion](html-conversion/) | 轉換 XML/HTML 片段並還原。 |

## 執行方式

在 `opencc-rust/` 目錄下執行：

```bash
cargo run --example basic-conversion
cargo run --example no-phrase-conversion
cargo run --example locale-differences
cargo run --example custom-dictionary
cargo run --example html-conversion
```
