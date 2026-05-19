# 基本簡繁轉換

這個範例示範如何使用 `opencc_rust::converter("cn", "tw2")`。

## 重點

- `converter()` 會回傳 `Result<Converter, OpenCCError>`。
- `Converter::convert()` 可重複處理多段文字。

## 執行

```bash
cargo run --example basic-conversion
```
