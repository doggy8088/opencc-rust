fn main() -> Result<(), Box<dyn std::error::Error>> {
    let text = "鼠标、软件、打印机和服务器都连接到计算机网络。";

    println!("原文：{text}");
    println!();

    for target in ["t", "tw", "tw2", "twp", "hk", "jp"] {
        let converter = opencc_rust::converter("cn", target)?;
        println!("cn -> {target:<3} {}", converter.convert(text));
    }

    println!();
    println!("選擇建議：只轉字形用 t；臺灣產品介面多半用 tw2；面向香港使用者可用 hk。");

    Ok(())
}
