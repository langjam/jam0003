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
            let op: compiler::Op = inst[vm.ip].try_into().expect("Expected valid `Op`.");

            vm.ip += 1;

            use compiler::Op::*;
            match op {
                NoOp => panic!("NoOp encountered."),
                Move => {
                    let x = extract_value(&mut vm, &inst);
                    let y = extract_value(&mut vm, &inst);

                    vm.pen_position.x += x;
                    vm.pen_position.y += y;
                }
                Store => {
                    let reg = inst[vm.ip];
                    vm.ip += 1;

                    let new_value = extract_value(&mut vm, &inst);

                    vm.registers[reg as usize] = new_value;
                }
                Add => {
                    let reg = inst[vm.ip];
                    vm.ip += 1;

                    let a = extract_value(&mut vm, &inst);
                    let b = extract_value(&mut vm, &inst);

                    vm.registers[reg as usize] = a + b;
                }
                Subtract => {
                    let reg = inst[vm.ip];
                    vm.ip += 1;

                    let a = extract_value(&mut vm, &inst);
                    let b = extract_value(&mut vm, &inst);

                    vm.registers[reg as usize] = a - b;
                }
                Multiply => {
                    let reg = inst[vm.ip];
                    vm.ip += 1;

                    let a = extract_value(&mut vm, &inst);
                    let b = extract_value(&mut vm, &inst);

                    vm.registers[reg as usize] = a * b;
                }
                Divide => {
                    let reg = inst[vm.ip];
                    vm.ip += 1;

                    let a = extract_value(&mut vm, &inst);
                    let b = extract_value(&mut vm, &inst);

                    vm.registers[reg as usize] = a / b;
                }
                Stbg => {
                    vm.background_color[0] = extract_value(&mut vm, &inst);
                    vm.background_color[1] = extract_value(&mut vm, &inst);
                    vm.background_color[2] = extract_value(&mut vm, &inst);
                }
                Stps => {
                    vm.pen_position.x = extract_value(&mut vm, &inst);
                    vm.pen_position.y = extract_value(&mut vm, &inst);
                }
                Stcl => {
                    vm.pen_color[0] = extract_value(&mut vm, &inst);
                    vm.pen_color[1] = extract_value(&mut vm, &inst);
                    vm.pen_color[2] = extract_value(&mut vm, &inst);
                }
                Strd => {
                    vm.pen_radius = extract_value(&mut vm, &inst);
                }
                Cmp => {
                    let a = extract_value(&mut vm, &inst);
                    let b = extract_value(&mut vm, &inst);
                    vm.rc = match a.cmp(&b) {
                        std::cmp::Ordering::Less => -1,
                        std::cmp::Ordering::Equal => 0,
                        std::cmp::Ordering::Greater => 1,
                    };
                }
                Jmp => {
                    let jump = extract_integer(&mut vm, &inst);
                    vm.ip = vm.ip.wrapping_add((jump - 2) as usize); // -2 because the operand take 2 bytes
                }
                Jeq => {
                    let jump = extract_integer(&mut vm, &inst);
                    if vm.rc == 0 {
                        vm.ip = vm.ip.wrapping_add((jump - 2) as usize); // -2 because the operand take 2 bytes
                    }
                }
                Jne => {
                    let jump = extract_integer(&mut vm, &inst);
                    if vm.rc != 0 {
                        vm.ip = vm.ip.wrapping_add((jump - 2) as usize); // -2 because the operand take 2 bytes
                    }
                }
                Jlt => {
                    let jump = extract_integer(&mut vm, &inst);
                    if vm.rc == -1 {
                        vm.ip = vm.ip.wrapping_add((jump - 2) as usize); // -2 because the operand take 2 bytes
                    }
                }
                Jgt => {
                    let jump = extract_integer(&mut vm, &inst);
                    if vm.rc == 1 {
                        vm.ip = vm.ip.wrapping_add((jump - 2) as usize); // -2 because the operand take 2 bytes
                    }
                }
                Jle => {
                    let jump = extract_integer(&mut vm, &inst);
                    if vm.rc <= 0 {
                        vm.ip = vm.ip.wrapping_add((jump - 2) as usize); // -2 because the operand take 2 bytes
                    }
                }
                Jge => {
                    let jump = extract_integer(&mut vm, &inst);
                    if vm.rc >= 0 {
                        vm.ip = vm.ip.wrapping_add((jump - 2) as usize); // -2 because the operand take 2 bytes
                    }
                }
                Rect => {
                    let w = extract_value(&mut vm, &inst);
                    let h = extract_value(&mut vm, &inst);

                    let rect = TuxShape::Rect {
                        color: vm.pen_color,
                        origin: vm.pen_position,
                        width: w,
                        height: h,
                    };

                    vm.shapes.push(rect);
                }
                Line => {
                    let dx = extract_value(&mut vm, &inst);
                    let dy = extract_value(&mut vm, &inst);

                    let line = TuxShape::Line {
                        color: vm.pen_color,
                        origin: vm.pen_position,
                        dx,
                        dy,
                    };

                    vm.shapes.push(line);
                }
                Elps => {
                    let w = extract_value(&mut vm, &inst);
                    let h = extract_value(&mut vm, &inst);

                    let ellipse = TuxShape::Ellipse {
                        color: vm.pen_color,
                        origin: vm.pen_position,
                        width: w,
                        height: h,
                    };

                    vm.shapes.push(ellipse);
                }
                Vert => {
                    let x = extract_value(&mut vm, &inst);
                    let y = extract_value(&mut vm, &inst);

                    let vert = Position { x, y };

                    vm.vertices.push(vert);
                }
                Pgon => {
                    let pgon = TuxShape::Polygon {
                        color: vm.pen_color,
                        vertices: vm.vertices.drain(..).collect(),
                    };
                    vm.shapes.push(pgon);
                }
            }
        }

        let shapes = vm.shapes.iter();

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
                match shape {
                    TuxShape::Rect {
                        color,
                        origin,
                        width,
                        height,
                    } => rectangle_from_to(
                        [
                            color[0] as f32 / 255.0,
                            color[1] as f32 / 255.0,
                            color[2] as f32 / 255.0,
                            1.0,
                        ],
                        [origin.x as f64, origin.y as f64],
                        [(origin.x + width) as f64, (origin.y + height) as f64],
                        ctx.transform,
                        g,
                    ),
                    TuxShape::Line {
                        color,
                        origin,
                        dx,
                        dy,
                    } => line(
                        [
                            color[0] as f32 / 255.0,
                            color[1] as f32 / 255.0,
                            color[2] as f32 / 255.0,
                            1.0,
                        ],
                        vm.pen_radius as f64,
                        [
                            origin.x as f64,
                            origin.y as f64,
                            (origin.x + dx) as f64,
                            (origin.y + dy) as f64,
                        ],
                        ctx.transform,
                        g,
                    ),
                    TuxShape::Ellipse {
                        color,
                        origin,
                        width,
                        height,
                    } => ellipse_from_to(
                        [
                            color[0] as f32 / 255.0,
                            color[1] as f32 / 255.0,
                            color[2] as f32 / 255.0,
                            1.0,
                        ],
                        [origin.x as f64, origin.y as f64],
                        [(origin.x + width) as f64, (origin.y + height) as f64],
                        ctx.transform,
                        g,
                    ),
                    TuxShape::Polygon { color, vertices } => {
                        let verts: Vec<_> =
                            vertices.iter().map(|p| [p.x as f64, p.y as f64]).collect();
                        polygon(
                            [
                                color[0] as f32 / 255.0,
                                color[1] as f32 / 255.0,
                                color[2] as f32 / 255.0,
                                1.0,
                            ],
                            verts.as_slice(),
                            ctx.transform,
                            g,
                        );
                    }
                };
            }
        });
    }

    Ok(())
}

fn extract_integer(vm: &mut VM, inst: &compiler::Instructions) -> i16 {
    let byte1 = inst[vm.ip];
    vm.ip += 1;
    let byte2 = inst[vm.ip];
    vm.ip += 1;

    let mut value = byte1 as i16;
    value |= ((byte2 as u16) << 8) as i16;

    value
}

fn extract_value(vm: &mut VM, inst: &compiler::Instructions) -> i16 {
    if inst[vm.ip] == 0 {
        vm.ip += 1;
        extract_integer(vm, inst)
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

#[derive(Clone, Debug)]
enum TuxShape {
    Rect {
        color: [i16; 3],
        origin: Position,
        width: i16,
        height: i16,
    },
    Line {
        color: [i16; 3],
        origin: Position,
        dx: i16,
        dy: i16,
    },
    Ellipse {
        color: [i16; 3],
        origin: Position,
        width: i16,
        height: i16,
    },
    Polygon {
        color: [i16; 3],
        vertices: Vec<Position>,
    },
}

struct VM {
    // Evaluation
    ip: usize,
    registers: [i16; NUM_REGISTERS],
    rc: i8,

    // Drawing
    pen_position: Position,
    pen_color: [i16; 3],
    pen_radius: i16,
    background_color: [i16; 3],
    shapes: Vec<TuxShape>,
    vertices: Vec<Position>,
}

impl VM {
    fn new() -> Self {
        Self {
            ip: Default::default(),
            registers: Default::default(),
            rc: Default::default(),
            pen_position: Default::default(),
            pen_color: Default::default(),
            pen_radius: Default::default(),
            background_color: Default::default(),
            shapes: Default::default(),
            vertices: Default::default(),
        }
    }
}
