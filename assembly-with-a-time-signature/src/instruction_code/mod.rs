use crate::instruction_code::unlinked::{Label, Operand, UnlinkedInstruction, UnlinkedInstructionStream};

pub mod unlinked;

pub struct Symbol {
    pub name: Label,
    pub offset: u64,
}

pub struct Relocation {
    pub offset: u64,
    pub label: Label,
}

pub struct Section {
    pub name: String,
    pub data: Vec<u8>,
    pub symbols: Vec<Symbol>,
    pub relocations: Vec<Relocation>,
}

pub fn serialize(instr: UnlinkedInstructionStream) -> Vec<Section> {
    let mut sections = Vec::new();

    for (name, content) in instr.sections {
        let mut sec = Section {
            name,
            data: vec![],
            symbols: vec![],
            relocations: vec![]
        };

        for i in content.instr {
            match i {
                UnlinkedInstruction::Normal { opcode, operands } => {
                    sec.data.push(opcode as u8);

                    if operands.len() == 0 {
                        continue
                    }

                    assert!(operands.len() <= 4);

                    let mut bts: u8 = 0;
                    for (idx, operand) in operands.iter().enumerate() {
                        if idx != 0 {
                            bts <<= 2;
                        }

                        bts |= match operand {
                            Operand::Immediate(_) => 0b00,
                            Operand::Note(_) => 0b01,
                            Operand::Label(_) => 0b10,
                            Operand::Register(_) => 0b11,
                        };
                    }
                    sec.data.push(bts);

                    for operand in operands {
                        let (data, relocs) = operand.serialize(sec.data.len() as u64);
                        sec.data.extend(data);
                        sec.relocations.extend(relocs);
                    }
                }
                UnlinkedInstruction::Label(l) => {
                    sec.symbols.push(Symbol {
                        name: l,
                        offset: sec.data.len() as u64
                    });
                }
                UnlinkedInstruction::Define(d) => {
                    sec.data.extend(d);
                }
            }
        }

        sections.push(sec)
    }

    sections
}

