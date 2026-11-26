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

## Releasing

`.github/workflows/publish.yml` publishes to crates.io as `opencc-rs-tw2`, using crates.io Trusted Publishing (no API token is stored in the repository).

On every push to `main` it reads the version from `Cargo.toml`:

- No `v<version>` tag yet: run `cargo test --locked`, publish to crates.io, then create the tag and a GitHub Release whose notes list the commits since the previous tag.
- The tag already exists: skip.

To release, bump `version` in `Cargo.toml` (and `Cargo.lock` via `cargo build`), commit, and push to `main`.

Pushing a `v*` tag manually, or running the workflow with `workflow_dispatch`, also publishes; the GitHub Release is created if it does not exist. The crates.io Trusted Publisher is bound to the file name `publish.yml`, so renaming the workflow requires updating the crate settings on crates.io.


## 本機與 CI 驗證分工

- 啟用本機 pre-commit hook（每個 clone 設定一次）：`git config core.hooksPath .githooks`
- 分工原則：本機負責 format 與快速回歸；CI 負責完整測試矩陣與「乾淨環境」驗證。hook 只放快檢查，慢測試一律交給 CI，避免被 `--no-verify` 繞過。
- 本機 hook：`cargo fmt --check`；6 repo 並排時加跑產生器 `generate_taiwan_quality.py --check`（單獨 clone 自動略過）。
- CI：`ci.yml` 跑 cargo test 與產生器政策 unittest；家族一致性的阻擋式防線是每週排程 `family-check.yml`，PR 內的 family-check job 為非阻擋警告（聯動發版期間紅燈屬預期）。（`ci.yml` 與 `family-check.yml` 由 CI 強化批次 PR #2 一併導入，合併後生效。）
