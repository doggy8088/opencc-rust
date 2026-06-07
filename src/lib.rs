mod dict_data;

use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;

const TW_PHRASES_CUSTOM_EXTRA: &str = r####"
網絡服務	網路服務
應用程序網關	應用程式閘道
鏡像文件	映像檔
保存更改	儲存變更
台球桌	撞球桌
文件名	檔名
文件系統	檔案系統
文件描述符	檔案描述子
函數調用	函式呼叫
渲染管線	算繪管線
內存分配	記憶體配置
網絡棧	網路堆疊
網絡適配器	網路介面卡
"####;
const TW_PHRASES_CUSTOM_EXTRA_REV: &str = r####"
網路服務	網絡服務
應用程式閘道	應用程序網關
映像檔	鏡像文件
儲存變更	保存更改
撞球桌	台球桌
檔名	文件名
檔案系統	文件系統
檔案描述子	文件描述符
函式呼叫	函數調用
算繪管線	渲染管線
記憶體配置	內存分配
網路堆疊	網絡棧
網路介面卡	網絡適配器
"####;


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpenCCError {
    MissingLocale { kind: &'static str },
    UnknownLocale { kind: &'static str, locale: String },
    NullDictionaryGroup,
    Xml(String),
}

impl fmt::Display for OpenCCError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingLocale { kind } => write!(f, "Please provide the `{kind}` option"),
            Self::UnknownLocale { kind, locale } => {
                write!(f, "Unknown locale `{locale}` for `{kind}` option")
            }
            Self::NullDictionaryGroup => write!(f, "Dictionary group cannot be null."),
            Self::Xml(message) => write!(f, "XML error: {message}"),
        }
    }
}

impl std::error::Error for OpenCCError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConverterOptions {
    pub from: String,
    pub to: String,
}

impl ConverterOptions {
    pub fn new(from: impl Into<String>, to: impl Into<String>) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
        }
    }
}

impl Default for ConverterOptions {
    fn default() -> Self {
        Self::new("", "")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DictEntry {
    pub source: String,
    pub target: String,
}

impl DictEntry {
    pub fn new(source: impl Into<String>, target: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            target: target.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Dict {
    Raw(Cow<'static, str>),
    Entries(Vec<DictEntry>),
}

impl Dict {
    pub fn from_static(data: &'static str) -> Self {
        Self::Raw(Cow::Borrowed(data))
    }

    pub fn from_string(data: impl Into<String>) -> Self {
        Self::Raw(Cow::Owned(data.into()))
    }

    pub fn from_entries(entries: impl IntoIterator<Item = DictEntry>) -> Self {
        Self::Entries(entries.into_iter().collect())
    }

    fn load_into(&self, trie: &mut Trie) {
        match self {
            Self::Raw(data) => trie.load_dict_str(data),
            Self::Entries(entries) => trie.load_entries(entries),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DictGroup {
    dicts: Vec<Dict>,
}

impl DictGroup {
    pub fn new(dicts: impl IntoIterator<Item = Dict>) -> Self {
        Self {
            dicts: dicts.into_iter().collect(),
        }
    }

    pub fn from_strings(dicts: impl IntoIterator<Item = &'static str>) -> Self {
        Self::new(dicts.into_iter().map(Dict::from_static))
    }

    pub fn from_entry_sets(dicts: impl IntoIterator<Item = Vec<DictEntry>>) -> Self {
        Self::new(dicts.into_iter().map(Dict::from_entries))
    }

    pub fn concat(&self, dict: Dict) -> Self {
        let mut dicts = self.dicts.clone();
        dicts.push(dict);
        Self { dicts }
    }

    pub fn concat_group(&self, dicts: impl IntoIterator<Item = Dict>) -> Self {
        let mut combined = self.dicts.clone();
        combined.extend(dicts);
        Self { dicts: combined }
    }

    pub fn len(&self) -> usize {
        self.dicts.len()
    }

    pub fn is_empty(&self) -> bool {
        self.dicts.is_empty()
    }

    fn load_into(&self, trie: &mut Trie) {
        for dict in &self.dicts {
            dict.load_into(trie);
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Node {
    children: HashMap<char, Node>,
    value: Option<String>,
}

#[derive(Debug, Default, Clone)]
pub struct Trie {
    root: Node,
}

impl Trie {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_word(&mut self, source: &str, target: &str) {
        let mut node = &mut self.root;
        for ch in source.chars() {
            node = node.children.entry(ch).or_default();
        }
        node.value = Some(target.to_owned());
    }

    pub fn load_dict_str(&mut self, dict: &str) {
        for line in dict.split(|c| c == '|' || c == '\n') {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let (source, target) = if let Some((source, target)) = line.split_once('\t') {
                (source, target)
            } else if let Some((source, target)) = line.split_once(' ') {
                (source, target)
            } else {
                continue;
            };

            self.add_word(source, target);
        }
    }

    pub fn load_entries(&mut self, dict: &[DictEntry]) {
        for entry in dict {
            self.add_word(&entry.source, &entry.target);
        }
    }

    pub fn load_dict_group(&mut self, group: &DictGroup) {
        group.load_into(self);
    }

    pub fn convert(&self, input: &str) -> String {
        if input.is_empty() {
            return String::new();
        }

        let chars: Vec<char> = input.chars().collect();
        let mut result = String::with_capacity(input.len());
        let mut pending = String::new();
        let mut i = 0;

        while i < chars.len() {
            let mut node = &self.root;
            let mut matched_end = None;
            let mut matched_value: Option<&str> = None;
            let mut j = i;

            while j < chars.len() {
                let Some(next) = node.children.get(&chars[j]) else {
                    break;
                };

                j += 1;
                node = next;

                if let Some(value) = node.value.as_deref() {
                    matched_end = Some(j);
                    matched_value = Some(value);
                }
            }

            if let Some(end) = matched_end {
                result.push_str(&pending);
                pending.clear();
                result.push_str(matched_value.unwrap_or_default());
                i = end;
            } else {
                pending.push(chars[i]);
                i += 1;
            }
        }

        result.push_str(&pending);
        result
    }
}

#[derive(Debug, Clone)]
pub struct LocalePreset {
    from: HashMap<&'static str, DictGroup>,
    to: HashMap<&'static str, DictGroup>,
}

impl LocalePreset {
    pub fn new(
        from: HashMap<&'static str, DictGroup>,
        to: HashMap<&'static str, DictGroup>,
    ) -> Self {
        Self { from, to }
    }

    pub fn from(&self) -> &HashMap<&'static str, DictGroup> {
        &self.from
    }

    pub fn to(&self) -> &HashMap<&'static str, DictGroup> {
        &self.to
    }
}

#[derive(Debug, Clone)]
pub struct Converter {
    tries: Vec<Trie>,
}

impl Converter {
    pub fn new(dict_groups: impl IntoIterator<Item = DictGroup>) -> Self {
        let tries = dict_groups
            .into_iter()
            .map(|group| {
                let mut trie = Trie::new();
                trie.load_dict_group(&group);
                trie
            })
            .collect();

        Self { tries }
    }

    pub fn convert(&self, input: &str) -> String {
        self.tries
            .iter()
            .fold(input.to_owned(), |current, trie| trie.convert(&current))
    }
}

pub fn converter(from: &str, to: &str) -> Result<Converter, OpenCCError> {
    converter_with_options(&ConverterOptions::new(from, to))
}

pub fn converter_with_options(options: &ConverterOptions) -> Result<Converter, OpenCCError> {
    converter_builder(locale::preset())(options)
}

pub fn converter_builder(
    locale_preset: LocalePreset,
) -> impl Fn(&ConverterOptions) -> Result<Converter, OpenCCError> {
    move |options| {
        let mut dict_groups = Vec::with_capacity(2);
        add_dict_group("from", &options.from, &locale_preset.from, &mut dict_groups)?;
        add_dict_group("to", &options.to, &locale_preset.to, &mut dict_groups)?;
        Ok(converter_factory(dict_groups))
    }
}

pub fn converter_factory(dict_groups: impl IntoIterator<Item = DictGroup>) -> Converter {
    Converter::new(dict_groups)
}

pub fn custom_converter_from_string(dict: impl Into<String>) -> Converter {
    converter_factory([DictGroup::new([Dict::from_string(dict)])])
}

pub fn custom_converter_from_entries(entries: impl IntoIterator<Item = DictEntry>) -> Converter {
    converter_factory([DictGroup::new([Dict::from_entries(entries)])])
}

fn add_dict_group(
    kind: &'static str,
    locale: &str,
    map: &HashMap<&'static str, DictGroup>,
    dict_groups: &mut Vec<DictGroup>,
) -> Result<(), OpenCCError> {
    if locale.trim().is_empty() {
        return Err(OpenCCError::MissingLocale { kind });
    }

    if locale == "t" {
        return Ok(());
    }

    let Some(group) = map.get(locale) else {
        return Err(OpenCCError::UnknownLocale {
            kind,
            locale: locale.to_owned(),
        });
    };

    dict_groups.push(group.clone());
    Ok(())
}

pub mod locale {
    use super::{
        DictGroup, LocalePreset, TW_PHRASES_CUSTOM_EXTRA, TW_PHRASES_CUSTOM_EXTRA_REV, dict_data,
    };
    use std::collections::HashMap;

    pub mod from {
        use super::{DictGroup, TW_PHRASES_CUSTOM_EXTRA_REV, dict_data};

        pub fn cn() -> DictGroup {
            DictGroup::from_strings([dict_data::ST_CHARACTERS, dict_data::ST_PHRASES])
        }

        pub fn hk() -> DictGroup {
            DictGroup::from_strings([
                dict_data::HK_VARIANTS_REV,
                dict_data::HK_VARIANTS_REV_PHRASES,
            ])
        }

        pub fn tw() -> DictGroup {
            DictGroup::from_strings([
                dict_data::TW_VARIANTS_REV,
                dict_data::TW_VARIANTS_REV_PHRASES,
            ])
        }

        pub fn tw2() -> DictGroup {
            DictGroup::from_strings([
                dict_data::TW_VARIANTS_REV,
                TW_PHRASES_CUSTOM_EXTRA_REV,
                dict_data::TW_PHRASES_CUSTOM_REV,
            ])
        }

        pub fn twp() -> DictGroup {
            DictGroup::from_strings([
                dict_data::TW_VARIANTS_REV,
                dict_data::TW_VARIANTS_REV_PHRASES,
                dict_data::TW_PHRASES_REV,
            ])
        }

        pub fn jp() -> DictGroup {
            DictGroup::from_strings([
                dict_data::JP_VARIANTS_REV,
                dict_data::JP_SHINJITAI_CHARACTERS,
                dict_data::JP_SHINJITAI_PHRASES,
            ])
        }
    }

    pub mod to {
        use super::{DictGroup, TW_PHRASES_CUSTOM_EXTRA, dict_data};

        pub fn cn() -> DictGroup {
            DictGroup::from_strings([dict_data::TS_CHARACTERS, dict_data::TS_PHRASES])
        }

        pub fn hk() -> DictGroup {
            DictGroup::from_strings([dict_data::HK_VARIANTS])
        }

        pub fn tw() -> DictGroup {
            DictGroup::from_strings([dict_data::TW_VARIANTS])
        }

        pub fn tw2() -> DictGroup {
            DictGroup::from_strings([
                dict_data::TW_VARIANTS,
                TW_PHRASES_CUSTOM_EXTRA,
                dict_data::TW_PHRASES_CUSTOM,
            ])
        }

        pub fn twp() -> DictGroup {
            DictGroup::from_strings([
                dict_data::TW_VARIANTS,
                dict_data::TW_PHRASES_IT,
                dict_data::TW_PHRASES_NAME,
                dict_data::TW_PHRASES_OTHER,
            ])
        }

        pub fn jp() -> DictGroup {
            DictGroup::from_strings([dict_data::JP_VARIANTS])
        }
    }

    pub fn from_map() -> HashMap<&'static str, DictGroup> {
        HashMap::from([
            ("cn", from::cn()),
            ("hk", from::hk()),
            ("tw", from::tw()),
            ("tw2", from::tw2()),
            ("twp", from::twp()),
            ("jp", from::jp()),
        ])
    }

    pub fn to_map() -> HashMap<&'static str, DictGroup> {
        HashMap::from([
            ("cn", to::cn()),
            ("hk", to::hk()),
            ("tw", to::tw()),
            ("tw2", to::tw2()),
            ("twp", to::twp()),
            ("jp", to::jp()),
        ])
    }

    pub fn preset() -> LocalePreset {
        LocalePreset::new(from_map(), to_map())
    }
}

pub mod presets {
    use super::{
        Converter, ConverterOptions, LocalePreset, OpenCCError, converter_builder, locale,
    };
    use std::collections::HashMap;

    pub mod full {
        use super::{
            Converter, ConverterOptions, LocalePreset, OpenCCError, converter_builder, locale,
        };

        pub fn locale() -> LocalePreset {
            locale::preset()
        }

        pub fn converter(from: &str, to: &str) -> Result<Converter, OpenCCError> {
            converter_with_options(&ConverterOptions::new(from, to))
        }

        pub fn converter_with_options(
            options: &ConverterOptions,
        ) -> Result<Converter, OpenCCError> {
            converter_builder(locale())(options)
        }
    }

    pub mod cn2t {
        use super::{
            Converter, ConverterOptions, HashMap, LocalePreset, OpenCCError, converter_builder,
            locale,
        };

        pub fn locale() -> LocalePreset {
            LocalePreset::new(
                HashMap::from([("cn", locale::from::cn())]),
                HashMap::from([
                    ("hk", locale::to::hk()),
                    ("tw", locale::to::tw()),
                    ("tw2", locale::to::tw2()),
                    ("twp", locale::to::twp()),
                    ("jp", locale::to::jp()),
                ]),
            )
        }

        pub fn converter(from: &str, to: &str) -> Result<Converter, OpenCCError> {
            converter_with_options(&ConverterOptions::new(from, to))
        }

        pub fn converter_with_options(
            options: &ConverterOptions,
        ) -> Result<Converter, OpenCCError> {
            converter_builder(locale())(options)
        }
    }

    pub mod t2cn {
        use super::{
            Converter, ConverterOptions, HashMap, LocalePreset, OpenCCError, converter_builder,
            locale,
        };

        pub fn locale() -> LocalePreset {
            LocalePreset::new(
                HashMap::from([
                    ("hk", locale::from::hk()),
                    ("tw", locale::from::tw()),
                    ("tw2", locale::from::tw2()),
                    ("twp", locale::from::twp()),
                    ("jp", locale::from::jp()),
                ]),
                HashMap::from([("cn", locale::to::cn())]),
            )
        }

        pub fn converter(from: &str, to: &str) -> Result<Converter, OpenCCError> {
            converter_with_options(&ConverterOptions::new(from, to))
        }

        pub fn converter_with_options(
            options: &ConverterOptions,
        ) -> Result<Converter, OpenCCError> {
            converter_builder(locale())(options)
        }
    }
}

pub struct HtmlConverter {
    converter: Converter,
    root: xmltree::Element,
    original: xmltree::Element,
    from_lang_tag: String,
    to_lang_tag: String,
}

impl HtmlConverter {
    pub fn from_xml_str(
        converter: Converter,
        xml: &str,
        from_lang_tag: impl Into<String>,
        to_lang_tag: impl Into<String>,
    ) -> Result<Self, OpenCCError> {
        let root = xmltree::Element::parse(xml.as_bytes())
            .map_err(|err| OpenCCError::Xml(err.to_string()))?;
        Ok(Self {
            converter,
            original: root.clone(),
            root,
            from_lang_tag: from_lang_tag.into(),
            to_lang_tag: to_lang_tag.into(),
        })
    }

    pub fn convert(&mut self) {
        convert_element(
            &self.converter,
            &mut self.root,
            false,
            &self.from_lang_tag,
            &self.to_lang_tag,
        );
    }

    pub fn restore(&mut self) {
        self.root = self.original.clone();
    }

    pub fn root(&self) -> &xmltree::Element {
        &self.root
    }

    pub fn to_xml_string(&self) -> Result<String, OpenCCError> {
        let mut output = Vec::new();
        self.root
            .write(&mut output)
            .map_err(|err| OpenCCError::Xml(err.to_string()))?;
        String::from_utf8(output).map_err(|err| OpenCCError::Xml(err.to_string()))
    }
}

fn convert_element(
    converter: &Converter,
    element: &mut xmltree::Element,
    mut lang_matched: bool,
    from_lang_tag: &str,
    to_lang_tag: &str,
) {
    if has_ignore_class(element) {
        return;
    }

    if let Some(lang) = element.attributes.get_mut("lang") {
        if lang == from_lang_tag {
            lang_matched = true;
            *lang = to_lang_tag.to_owned();
        } else if !lang.is_empty() {
            lang_matched = false;
        }
    }

    if lang_matched {
        if element.name.eq_ignore_ascii_case("script") || element.name.eq_ignore_ascii_case("style")
        {
            return;
        }

        if element.name.eq_ignore_ascii_case("meta") {
            let name = element.attributes.get("name").cloned();
            if matches!(name.as_deref(), Some(name) if name.eq_ignore_ascii_case("description") || name.eq_ignore_ascii_case("keywords"))
            {
                if let Some(content) = element.attributes.get_mut("content") {
                    *content = converter.convert(content);
                }
            }
        } else if element.name.eq_ignore_ascii_case("img") {
            if let Some(alt) = element.attributes.get_mut("alt") {
                *alt = converter.convert(alt);
            }
        } else if element.name.eq_ignore_ascii_case("input") {
            let input_type = element.attributes.get("type").cloned();
            if matches!(input_type.as_deref(), Some(kind) if kind.eq_ignore_ascii_case("button")) {
                if let Some(value) = element.attributes.get_mut("value") {
                    *value = converter.convert(value);
                }
            }
        }
    }

    for child in &mut element.children {
        match child {
            xmltree::XMLNode::Element(child_element) => {
                convert_element(
                    converter,
                    child_element,
                    lang_matched,
                    from_lang_tag,
                    to_lang_tag,
                );
            }
            xmltree::XMLNode::Text(text) if lang_matched => {
                *text = converter.convert(text);
            }
            _ => {}
        }
    }
}

fn has_ignore_class(element: &xmltree::Element) -> bool {
    element
        .attributes
        .get("class")
        .map(|class| class.split_whitespace().any(|item| item == "ignore-opencc"))
        .unwrap_or(false)
}
