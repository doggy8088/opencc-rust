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
    trie.load_dict_str("a b|invalid|c d|Web 平台庫\tWeb 平台函式庫");

    assert_eq!(trie.convert("a"), "b");
    assert_eq!(trie.convert("c"), "d");
    assert_eq!(trie.convert("Web 平台庫"), "Web 平台函式庫");
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
fn built_in_converter_converts_preferred_taiwan_terms_to_tw2() {
    let converter = converter("cn", "tw2").unwrap();
    let cases = [
        ("视频", "影片"),
        ("音频", "音訊"),
        ("软件", "軟體"),
        ("硬件", "硬體"),
        ("程序", "程式"),
        ("进程", "行程"),
        ("进程间通信", "行程間通訊"),
        ("线程", "執行緒"),
        ("数据", "資料"),
        ("数据库", "資料庫"),
        ("电子邮件", "電子郵件"),
        ("网络", "網路"),
        ("网络服务", "網路服務"),
        ("应用程序网关", "應用程式閘道"),
        ("镜像文件", "映像檔"),
        ("保存更改", "儲存變更"),
        ("信息", "資訊"),
        ("质量", "品質"),
        ("用户", "使用者"),
        ("默认", "預設"),
        ("创建", "建立"),
        ("实现", "實作"),
        ("运行", "執行"),
        ("发布", "發表"),
        ("屏幕", "螢幕"),
        ("界面", "介面"),
        ("文档", "文件"),
        ("操作系统", "作業系統"),
        ("剑指", "針對"),
        ("痛点", "要害"),
        ("硬伤", "罩門"),
    ];

    for (source, expected) in cases {
        assert_eq!(converter.convert(source), expected, "{source}");
    }
}

#[test]
fn built_in_converter_handles_cn_to_tw2_edge_cases() {
    let converter = converter("cn", "tw2").unwrap();
    let cases = [
        ("数据结构数据库", "資料結構資料庫"),
        ("命令行工具", "命令列工具"),
        ("响应式编程响应头", "回應式程式設計回應標頭"),
        ("Web 平台库", "Web 平台函式庫"),
        ("for 循环和while 循环", "for 迴圈和while 迴圈"),
        ("「类」", "「類別」"),
        ("类。", "類別。"),
        ("“数据库”, “网络请求”", "“資料庫”, “網路請求”"),
        ("项目设置：默认值", "專案設定：預設值"),
        ("软件发布", "軟體發表"),
        ("发布响应式编程教程", "發表回應式程式設計課程"),
        ("发布数据库迁移脚本", "發表資料庫遷移指令碼"),
        ("文件名和文件系统", "檔名和檔案系統"),
        ("文件描述符和函数调用", "檔案描述子和函式呼叫"),
        ("渲染管线和内存分配", "算繪管線和記憶體配置"),
        ("网络栈和网络适配器", "網路堆疊和網路介面卡"),
        ("爆发", "爆發"),
        ("千钧一发", "千鈞一髮"),
        ("一触即发", "一觸即發"),
        ("百发百中", "百發百中"),
        ("爆发发布", "爆發發表"),
    ];

    for (source, expected) in cases {
        assert_eq!(converter.convert(source), expected, "{source}");
    }
}

#[test]
fn built_in_converter_keeps_cn_to_tw2_project_item_contexts() {
    let converter = converter("cn", "tw2").unwrap();
    let cases = [
        ("项目", "項目"),
        ("清单项目", "清單項目"),
        ("每个项目", "每個項目"),
    ];

    for (source, expected) in cases {
        assert_eq!(converter.convert(source), expected, "{source}");
    }
}

#[test]
fn built_in_converter_converts_cn_to_tw2_project_compounds() {
    let converter = converter("cn", "tw2").unwrap();
    let cases = [
        ("项目文件夹", "專案資料夾"),
        ("项目的", "專案的"),
        ("项目目录", "專案目錄"),
        ("项目管理", "專案管理"),
        ("项目设置", "專案設定"),
    ];

    for (source, expected) in cases {
        assert_eq!(converter.convert(source), expected, "{source}");
    }
}

#[test]
fn built_in_converter_handles_cn_to_tw2_mixed_punctuation_and_unicode_edges() {
    let converter = converter("cn", "tw2").unwrap();
    let cases = [
        ("控制台打印日志", "輸出到 Console記錄"),
        ("元数据 API", "Metadata API"),
        ("类（ Class ）加载器", "類別（ Class ）載入器"),
        ("（视频）", "（影片）"),
        ("数据库🚀网络请求", "資料庫🚀網路請求"),
    ];

    for (source, expected) in cases {
        assert_eq!(converter.convert(source), expected, "{source}");
    }
}

#[test]
fn built_in_converter_handles_cn_to_tw2_regional_orthography_and_sentence_edges() {
    let converter = converter("cn", "tw2").unwrap();
    let cases = [
        ("台湾台球桌", "台灣撞球桌"),
        ("折叠粘土", "折疊黏土"),
        (
            "默认用户界面支持数据库和网络请求。",
            "預設使用者介面支援資料庫和網路請求。",
        ),
        ("命令行工具加载配置文件。", "命令列工具載入組態檔。"),
        (
            "创建软件项目目录和项目设置。",
            "建立軟體專案目錄和專案設定。",
        ),
        (
            "调试器显示调用堆栈和断点。",
            "偵錯工具顯示呼叫堆疊和中斷點。",
        ),
        (
            "响应式编程教程包含缓存策略。",
            "回應式程式設計課程包含快取策略。",
        ),
    ];

    for (source, expected) in cases {
        assert_eq!(converter.convert(source), expected, "{source}");
    }
}

#[test]
fn built_in_converter_documents_cn_to_tw2_release_publish_current_behavior() {
    let converter = converter("cn", "tw2").unwrap();
    let cases = [("发布公告", "發表公告"), ("发布新版本", "發表新版本")];

    for (source, expected) in cases {
        assert_eq!(converter.convert(source), expected, "{source}");
    }
}

#[test]
fn built_in_converter_uses_ping_tai_not_ping_tai_in_tw2() {
    let cn_to_tw2 = converter("cn", "tw2").unwrap();
    let cases = [
        // bare 平台 (simplified) → 平台 (tw2), never 平臺
        ("平台", "平台"),
        // common platform compounds
        ("跨平台", "跨平台"),
        ("软件平台", "軟體平台"),
        ("作业平台", "作業平台"),
        // library compounds still expand 庫→函式庫
        ("Web 平台库", "Web 平台函式庫"),
        ("全平台库列表", "全平台函式庫列表"),
        ("原生平台库", "原生平台函式庫"),
    ];
    for (source, expected) in cases {
        assert_eq!(cn_to_tw2.convert(source), expected, "cn→tw2: {source}");
    }

    // tw2 → cn reverse: 平台 maps back to 平台
    let tw2_to_cn = converter("tw2", "cn").unwrap();
    let rev_cases = [
        ("跨平台", "跨平台"),
        ("軟體平台", "软件平台"),
    ];
    for (source, expected) in rev_cases {
        assert_eq!(tw2_to_cn.convert(source), expected, "tw2→cn: {source}");
    }
}

#[test]
fn built_in_converter_converts_tw_to_cn() {
    let converter = converter("tw", "cn").unwrap();

    assert_eq!(converter.convert("漢語"), "汉语");
}

#[test]
fn built_in_converter_converts_tw2_technical_phrases_to_cn() {
    let converter = converter("tw2", "cn").unwrap();
    let cases = [
        ("檔名和檔案系統", "文件名和文件系统"),
        ("檔案描述子和函式呼叫", "文件描述符和函数调用"),
        ("算繪管線和記憶體配置", "渲染管线和内存分配"),
        ("網路堆疊和網路介面卡", "网络栈和网络适配器"),
    ];

    for (source, expected) in cases {
        assert_eq!(converter.convert(source), expected, "{source}");
    }
}

#[test]
fn built_in_converter_handles_tw2_to_cn_project_terms() {
    let converter = converter("tw2", "cn").unwrap();
    let cases = [
        ("專案", "专案"),
        ("專案資料夾", "项目文件夹"),
        ("專案的", "项目的"),
        ("專案目錄", "项目目录"),
        ("專案管理", "项目管理"),
        ("專案設定", "项目设置"),
    ];

    for (source, expected) in cases {
        assert_eq!(converter.convert(source), expected, "{source}");
    }
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
