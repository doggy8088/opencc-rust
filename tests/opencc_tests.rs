use opencc_rust::{
    DictEntry, DictGroup, OpenCCError, Trie, converter, custom_converter_from_entries,
    custom_converter_from_string, presets,
};

#[test]
fn trie_uses_longest_match() {
    let mut trie = Trie::new();
    trie.add_word("ab", "X");
    trie.add_word("a", "Y");

    assert_eq!(trie.convert("abca"), "XcY");
}

#[test]
fn trie_loads_pipe_separated_dictionary_and_skips_malformed_lines() {
    let mut trie = Trie::new();
    trie.load_dict_str("a b|invalid|c d");

    assert_eq!(trie.convert("a"), "b");
    assert_eq!(trie.convert("c"), "d");
    assert_eq!(trie.convert("x"), "x");
}

#[test]
fn converter_factory_converts_sequentially() {
    let group1 = DictGroup::from_entry_sets([vec![DictEntry::new("a", "b")]]);
    let group2 = DictGroup::from_entry_sets([vec![DictEntry::new("b", "c")]]);

    let converter = opencc_rust::converter_factory([group1, group2]);

    assert_eq!(converter.convert("a"), "c");
}

#[test]
fn built_in_converter_converts_cn_to_tw2() {
    let converter = converter("cn", "tw2").unwrap();

    assert_eq!(converter.convert("汉语"), "漢語");
}

#[test]
fn built_in_converter_converts_tw_to_cn() {
    let converter = converter("tw", "cn").unwrap();

    assert_eq!(converter.convert("漢語"), "汉语");
}

#[test]
fn custom_converters_convert_entries_and_raw_dictionary() {
    let raw = custom_converter_from_string("香蕉 banana|蘋果 apple|梨 pear");
    let entries = custom_converter_from_entries([
        DictEntry::new("banana", "香蕉"),
        DictEntry::new("apple", "蘋果"),
    ]);

    assert_eq!(raw.convert("香蕉 蘋果 梨"), "banana apple pear");
    assert_eq!(entries.convert("banana apple"), "香蕉 蘋果");
}

#[test]
fn converter_reports_missing_and_unknown_locales() {
    assert_eq!(
        converter("", "cn").unwrap_err(),
        OpenCCError::MissingLocale { kind: "from" }
    );

    assert!(matches!(
        converter("missing", "cn").unwrap_err(),
        OpenCCError::UnknownLocale { kind: "from", .. }
    ));
}

#[test]
fn presets_restrict_supported_directions() {
    assert!(presets::cn2t::converter("cn", "tw2").is_ok());
    assert!(presets::cn2t::converter("tw", "cn").is_err());
    assert!(presets::t2cn::converter("tw2", "cn").is_ok());
    assert!(presets::t2cn::converter("cn", "tw").is_err());
}

#[test]
fn html_converter_converts_xml_compatible_html_and_restores() {
    let converter = custom_converter_from_string("hello HELLO|keywords KEYWORDS");
    let xml = r#"<html lang="zh"><head><meta name="description" content="hello"/><meta name="keywords" content="keywords"/></head><body><p>hello</p><img alt="hello"/><input type="button" value="hello"/><input type="text" value="hello"/><div class="ignore-opencc">hello</div><span lang="en">hello</span><script>hello</script><style>hello</style></body></html>"#;
    let mut html =
        opencc_rust::HtmlConverter::from_xml_str(converter, xml, "zh", "zh-Hant").unwrap();

    html.convert();
    let converted = html.to_xml_string().unwrap();

    assert!(converted.contains(r#"lang="zh-Hant""#));
    assert!(converted.contains(">HELLO</p>"));
    assert!(converted.contains(r#"content="HELLO""#));
    assert!(converted.contains(r#"content="KEYWORDS""#));
    assert!(converted.contains(r#"alt="HELLO""#));
    assert!(converted.contains(r#"value="HELLO""#));
    assert!(converted.contains(r#"<span lang="en">hello</span>"#));
    assert!(converted.contains(r#"<script>hello</script>"#));
    assert!(converted.contains(r#"<style>hello</style>"#));

    html.restore();
    let restored = html.to_xml_string().unwrap();
    assert!(restored.contains(r#"lang="zh""#));
    assert!(restored.contains(">hello</p>"));
}
