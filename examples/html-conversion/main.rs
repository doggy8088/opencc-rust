fn main() -> Result<(), Box<dyn std::error::Error>> {
    let xml = r#"
<html lang="zh-HK">
  <body>
    <h1>漢語轉換示例</h1>
    <p lang="zh-HK">伺服器與網絡服務已啟動。</p>
    <p lang="en">This paragraph should stay unchanged.</p>
  </body>
</html>
"#;

    let converter = opencc_rust::converter("hk", "cn")?;
    let mut html = opencc_rust::HtmlConverter::from_xml_str(converter, xml, "zh-HK", "zh-CN")?;

    html.convert();
    println!("轉換後：");
    println!("{}", html.to_xml_string()?);

    html.restore();
    println!();
    println!("還原後：");
    println!("{}", html.to_xml_string()?);

    Ok(())
}
