// @TODO:
// use piston2d to do the image making
//

mod compiler;
mod evaluator;
mod parser;
mod utils;

fn main() -> Result<(), String> {
    let mut args = std::env::args().skip(1);
    let file_path = args
        .next()
        .ok_or("No file path given to tux interpreter.".to_string())?;
    let file_path = std::path::Path::new(&file_path);

    let source: &str = Box::leak(
        std::fs::read_to_string(&file_path)
            .or(Err(format!(
                "Failed to read file {}",
                file_path
                    .file_name()
                    .expect("Expected valid file path")
                    .to_str()
                    .expect("Expected file path to be valid UTF-8")
            )))?
            .into_boxed_str(),
    );

    for line in source.lines() {
        println!("{}", line);
    }

    println!("------------");

    let ir = match parser::parse(source) {
        Ok(ir) => ir,
        Err(err) => {
            eprintln!(
                "{}",
                utils::DisplayableError::new(
                    file_path
                        .file_name()
                        .expect("Expected valid file path")
                        .to_str()
                        .expect("Expected file path to be valid UTF-8")
                        .to_string(),
                    err
                )
            );
            return Ok(());
        }
    };
    println!("{:?}", ir);

    println!("------------");

    let inst = match compiler::compile(ir) {
        Ok(inst) => inst,
        Err(err) => {
            eprintln!(
                "{}",
                utils::DisplayableError::new(
                    file_path
                        .file_name()
                        .expect("Expected valid file path")
                        .to_str()
                        .expect("Expected file path to be valid UTF-8")
                        .to_string(),
                    err
                )
            );
            return Ok(());
        }
    };
    println!("{:#?}", inst);

    println!("------------");

    if let Err(err) = evaluator::evaluate(
        file_path
            .file_stem()
            .expect("Expected a file stem")
            .to_str()
            .expect("Expected stem to be valid UTF-8")
            .to_string(),
        inst,
    ) {
        eprintln!(
            "{}",
            utils::DisplayableError::new(
                file_path
                    .file_name()
                    .expect("Expected valid file path")
                    .to_str()
                    .expect("Expected file path to be valid UTF-8")
                    .to_string(),
                err
            )
        );
        return Ok(());
    }

    Ok(())
}
