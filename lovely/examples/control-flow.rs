use lovely::{
    self,
    bytecode::{decode, encode, Op},
    image,
    value::Value,
    vm::execute,
};
use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let example = vec![
        Op::Constant(Value::Number(0.0)),
        Op::Label(0),
        Op::Constant(Value::String("sorry no swearing".to_string())),
        Op::Print,
        Op::Constant(Value::Number(1.0)),
        Op::Add,
        Op::Dup,
        Op::Constant(Value::Number(5.0)),
        Op::Lt,
        Op::JmpIf(0),
        Op::Constant(Value::Number(42.0)),
        Op::Constant(Value::Number(36.0)),
        Op::Div,
        Op::Print,
        Op::Constant(Value::Number(20.0)),
        Op::Constant(Value::Number(12.0)),
        Op::Mul,
        Op::Print,
        Op::Constant(Value::String("hey there".to_string())),
        Op::Print,
        Op::Constant(Value::String("isn't this lovely".to_string())),
        Op::Print,
        Op::Constant(Value::String("isn't this beautiful".to_string())),
        Op::Print,
    ];

    let bytecode = encode(&example);

    let dt = image::draw(&bytecode);
    dt.write_png("output.png")?;

    let decoder = png::Decoder::new(File::open("output.png").unwrap());
    let mut reader = decoder.read_info().unwrap();
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).unwrap();
    let bytes = &buf[..info.buffer_size()];

    let bytecode = image::decode(bytes, info.width as usize);

    let example = decode(&bytecode);

    execute(&example);

    Ok(())
}
