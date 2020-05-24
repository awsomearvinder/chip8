use chip8_parser::parser;
fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = std::env::args().collect();
    let file = std::fs::read(&args[1])?;
    let op_codes = parser::parse(&file).unwrap();
    for i in op_codes {
        println! {"{}",i};
    }
    Ok(())
}
