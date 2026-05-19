use opencc_rust::{Dict, DictEntry, DictGroup, converter_factory, custom_converter_from_entries};

fn main() {
    let custom_only = custom_converter_from_entries([
        DictEntry::new("香蕉", "banana"),
        DictEntry::new("蘋果", "apple"),
        DictEntry::new("用户", "使用者"),
        DictEntry::new("用户界面", "使用者介面"),
    ]);

    println!("{}", custom_only.convert("香蕉、蘋果和用户界面"));

    let product_terms = Dict::from_entries([
        DictEntry::new("預設使用者介面", "預設 UI"),
        DictEntry::new("資料庫", "DB"),
    ]);

    let cn_to_tw_with_product_terms = converter_factory([
        opencc_rust::locale::from::cn(),
        opencc_rust::locale::to::tw2(),
        DictGroup::new([product_terms]),
    ]);

    println!(
        "{}",
        cn_to_tw_with_product_terms.convert("默认用户界面支持数据库和网络请求。")
    );
}
