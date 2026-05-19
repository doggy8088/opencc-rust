# 不套用詞彙轉換

這個範例示範 `cn -> t` 與 `cn -> tw2` 的差異。

## 重點

- `converter("cn", "t")` 只轉成 OpenCC 標準繁體。
- `converter("cn", "tw2")` 會套用臺灣常用詞彙。

## 執行

```bash
cargo run --example no-phrase-conversion
```
