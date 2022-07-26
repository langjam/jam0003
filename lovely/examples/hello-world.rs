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
        Op::Constant(Value::String("hello world".to_string())),
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
