fn main() -> Result<(), Box<dyn std::error::Error>> {
    let converter = opencc_rust::converter("cn", "tw2")?;
    println!("{}", converter.convert("汉语"));
    Ok(())
}
