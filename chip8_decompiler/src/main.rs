use chip8_parser::parser;
fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = std::env::args().collect();
    let file = std::fs::read(&args[1])?;
    let opCodes = parser::parse(&file).unwrap();
    eprintln!("{}", opCodes.len());
    for i in opCodes {
        println! {"{:?}",i};
    }
    Ok(())
}
