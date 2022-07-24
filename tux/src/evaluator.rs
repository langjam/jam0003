use crate::compiler;
use crate::utils::*;

use piston_window::*;

pub const NUM_REGISTERS: usize = 16;

pub fn evaluate(program_name: String, inst: compiler::Instructions) -> Result<()> {
    let mut window: PistonWindow = WindowSettings::new(program_name, (640, 480))
        .exit_on_esc(true)
        .build()
        .map_err(|_| Error::new(CodeLocation::new(0, 0), "Failed to open program window."))?;

    let mut event_loop_settings = EventSettings::new();
    event_loop_settings.lazy = true;

    window.events.set_event_settings(event_loop_settings);

    let mut vm = VM::new();

    while let Some(e) = window.next() {
        while vm.ip < inst.len() {
            let op: compiler::Op = inst[vm.ip]
                .try_into()
                .map_err(|err| Error::new(CodeLocation::new(0, 0), err))?;

            vm.ip += 1;

            use compiler::Op::*;
            match op {
                NoOp => return Err(Error::new(CodeLocation::new(0, 0), "NoOp encountered.")),
                Move => {
                    let x = extract_integer(&mut vm, &inst);
                    let y = extract_integer(&mut vm, &inst);

                    vm.pen_position.x += x;
                    vm.pen_position.y += y;
                }
                Store => {
                    let reg = inst[vm.ip];
                    vm.ip += 1;

                    let new_value = extract_integer(&mut vm, &inst);

                    vm.registers[reg as usize] = new_value;
                }
                Add => {
                    let reg = inst[vm.ip];
                    vm.ip += 1;

                    let a = extract_integer(&mut vm, &inst);
                    let b = extract_integer(&mut vm, &inst);

                    vm.registers[reg as usize] = a + b;
                }
                Stbg => {
                    vm.background_color[0] = extract_integer(&mut vm, &inst);
                    vm.background_color[1] = extract_integer(&mut vm, &inst);
                    vm.background_color[2] = extract_integer(&mut vm, &inst);
                }
                Stps => {
                    vm.pen_position.x = extract_integer(&mut vm, &inst);
                    vm.pen_position.y = extract_integer(&mut vm, &inst);
                }
                Stcl => {
                    vm.pen_color[0] = extract_integer(&mut vm, &inst);
                    vm.pen_color[1] = extract_integer(&mut vm, &inst);
                    vm.pen_color[2] = extract_integer(&mut vm, &inst);
                }
                Strd => {
                    vm.pen_radius = extract_integer(&mut vm, &inst);
                }
                Rect => {
                    let w = extract_integer(&mut vm, &inst);
                    let h = extract_integer(&mut vm, &inst);

                    let rect = TuxShape {
                        kind: ShapeKind::Rect,
                        color: vm.pen_color,
                        top_left: vm.pen_position,
                        width: w,
                        height: h,
                    };

                    vm.shapes.push(rect);
                }
                Line => {
                    let w = extract_integer(&mut vm, &inst);
                    let h = extract_integer(&mut vm, &inst);

                    let line = TuxShape {
                        kind: ShapeKind::Line,
                        color: vm.pen_color,
                        top_left: vm.pen_position,
                        width: w,
                        height: h,
                    };

                    vm.shapes.push(line);
                }
            }
        }

        let shapes = vm.shapes.iter();

        // 2. draw the stuff
        window.draw_2d(&e, move |ctx, g, _| {
            clear(
                [
                    vm.background_color[0] as f32 / 255.0,
                    vm.background_color[1] as f32 / 255.0,
                    vm.background_color[2] as f32 / 255.0,
                    1.0,
                ],
                g,
            );

            for shape in shapes {
                match shape.kind {
                    ShapeKind::Rect => rectangle_from_to(
                        [
                            shape.color[0] as f32 / 255.0,
                            shape.color[1] as f32 / 255.0,
                            shape.color[2] as f32 / 255.0,
                            1.0,
                        ],
                        [shape.top_left.x as f64, shape.top_left.y as f64],
                        [
                            (shape.top_left.x + shape.width) as f64,
                            (shape.top_left.y + shape.height) as f64,
                        ],
                        ctx.transform,
                        g,
                    ),
                    ShapeKind::Line => line(
                        [
                            shape.color[0] as f32 / 255.0,
                            shape.color[1] as f32 / 255.0,
                            shape.color[2] as f32 / 255.0,
                            1.0,
                        ],
                        vm.pen_radius as f64,
                        [
                            shape.top_left.x as f64,
                            shape.top_left.y as f64,
                            (shape.top_left.x + shape.width) as f64,
                            (shape.top_left.y + shape.height) as f64,
                        ],
                        ctx.transform,
                        g,
                    ),
                };
            }
        });
    }

    Ok(())
}

fn extract_integer(vm: &mut VM, inst: &compiler::Instructions) -> i16 {
    if inst[vm.ip] == 0 {
        vm.ip += 1;

        let byte1 = inst[vm.ip];
        vm.ip += 1;
        let byte2 = inst[vm.ip];
        vm.ip += 1;

        let mut value = byte1 as i16;
        value |= ((byte2 as u16) << 8) as i16;

        value
    } else {
        vm.ip += 1;

        let reg = inst[vm.ip];
        vm.ip += 1;

        let value = vm.registers[reg as usize];

        value
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct Position {
    x: i16,
    y: i16,
}

#[derive(Clone, Copy, Debug)]
struct TuxShape {
    kind: ShapeKind,
    color: [i16; 3],
    top_left: Position,
    width: i16,
    height: i16,
}

#[derive(Clone, Copy, Debug)]
enum ShapeKind {
    Rect,
    Line,
}

struct VM {
    // Evaluation
    ip: usize,
    registers: [i16; NUM_REGISTERS],

    // Drawing
    pen_position: Position,
    pen_color: [i16; 3],
    pen_radius: i16,
    background_color: [i16; 3],
    shapes: Vec<TuxShape>,
}

impl VM {
    fn new() -> Self {
        Self {
            ip: Default::default(),
            registers: Default::default(),
            pen_position: Default::default(),
            pen_color: Default::default(),
            pen_radius: Default::default(),
            background_color: Default::default(),
            shapes: Default::default(),
        }
    }
}
