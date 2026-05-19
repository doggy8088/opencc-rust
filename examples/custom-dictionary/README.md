# 自訂詞典

這個範例示範 `custom_converter_from_entries()`、`DictGroup` 與 `converter_factory()`。

## 重點

- 自訂詞典可用 `DictEntry` 或字串表示。
- `converter_factory()` 會依序套用多個詞庫群組。
- 較長詞條會優先匹配。

## 執行

```bash
cargo run --example custom-dictionary
```
