fn main() -> Result<(), Box<dyn std::error::Error>> {
    let converter = opencc_rust::converter("hk", "cn")?;
    let xml = "<html lang='zh-HK'><body><p lang='zh-HK'>漢語</p></body></html>";
    let mut html = opencc_rust::HtmlConverter::from_xml_str(converter, xml, "zh-HK", "zh-CN")?;

    html.convert();
    println!("{}", html.to_xml_string()?);
    html.restore();

    Ok(())
}
