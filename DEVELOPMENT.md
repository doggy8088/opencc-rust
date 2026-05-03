# Development Guide

## Architecture

This package ports the local C# OpenCC implementation directly:

1. `DictData` constants are generated from `OpenCC/src/OpenCC/Internal/DictData.cs`.
2. `LocaleData` maps are represented by `locale::from::*`, `locale::to::*`, `locale::from_map()`, and `locale::to_map()`.
3. Presets are represented by `presets::full`, `presets::cn2t`, and `presets::t2cn`.
4. `Converter` owns one `Trie` per dictionary group and applies them sequentially.
5. `Trie` walks Unicode scalar values (`char`) and uses longest-match wins.
6. `HtmlConverter` uses `xmltree` for XML-compatible HTML and keeps an original DOM clone for `restore()`.

## Dictionary format

Raw dictionaries follow the original compact format:

```text
來源 目標|來源2 目標2
```

Malformed entries with fewer than two space-separated fields are ignored, matching the C# implementation.

## Validation checklist

Run this before publishing changes:

```bash
cargo fmt --check
cargo test
cargo run --example basic
cargo run --example custom
cargo run --example html
```

## Regenerating embedded dictionaries

If the upstream C# dictionary changes, regenerate `src/dict_data.rs` from `OpenCC/src/OpenCC/Internal/DictData.cs` and rerun the full test suite. Keep the generated file as a simple constant table; conversion logic belongs in `src/lib.rs`.
