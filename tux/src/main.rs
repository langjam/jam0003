mod parser;

fn main() -> Result<(), String> {
    let mut args = std::env::args().skip(1);
    let file_name = args
        .next()
        .ok_or("Hey! I need a source file to interpret!")?;

    let source: &str = Box::leak(
        std::fs::read_to_string(file_name)
            .or(Err("AHH! Bad source file!".to_string()))?
            .into_boxed_str(),
    );

    for line in source.lines() {
        println!("{}", line);
    }

    println!("------------");

    parser::parse(source)?;

    Ok(())
}
