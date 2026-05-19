fn main() -> Result<(), Box<dyn std::error::Error>> {
    let text = "鼠标和软件可以连接到计算机网络。";
    let shape_only = opencc_rust::converter("cn", "t")?;
    let taiwan_words = opencc_rust::converter("cn", "tw2")?;

    println!("原文：{text}");
    println!("只轉字形 cn -> t：{}", shape_only.convert(text));
    println!("臺灣詞彙 cn -> tw2：{}", taiwan_words.convert(text));
    println!();
    println!("差異：cn -> t 保留原本詞彙；cn -> tw2 會轉成臺灣慣用詞。");

    Ok(())
}
