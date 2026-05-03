use opencc_rust::{DictEntry, custom_converter_from_entries, custom_converter_from_string};

fn main() {
    let raw = custom_converter_from_string("香蕉 banana|蘋果 apple|梨 pear");
    println!("{}", raw.convert("香蕉 蘋果 梨"));

    let entries =
        custom_converter_from_entries([DictEntry::new("“", "「"), DictEntry::new("”", "」")]);
    println!("{}", entries.convert("悟空道:“师父又来了。”"));
}
