use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use object::{Architecture, BinaryFormat, Endianness, RelocationEncoding, RelocationKind, SectionKind, SymbolFlags, SymbolKind, SymbolScope};
use crate::instruction_code::{Section, serialize};
use crate::instruction_code::unlinked::UnlinkedInstructionStream;
use object::write::{Object, Relocation, Symbol, SymbolSection};
use std::io::Error as IoError;

pub struct ElfFile {
    pub(crate) f: File
}

impl ElfFile {
    pub fn create(name: impl AsRef<Path>) -> Result<ElfFile, IoError> {
        Ok(Self {
            f: File::create(name)?
        })

    }

    pub fn open(name: impl AsRef<Path>) -> Result<ElfFile, IoError> {
        Ok(Self {
            f: File::open(name)?
        })
    }

    pub fn write_unlinked(&mut self, instr: UnlinkedInstructionStream) -> Result<(), Box<dyn Error>>{
        let sections = serialize(instr);

        let mut obj = Object::new(
            BinaryFormat::Elf,
            Architecture::Music,
            Endianness::Little,
        );

        let mut symbol_ids = HashMap::new();
        let mut relocations_store = HashMap::new();

        for Section { name, data, symbols, relocations} in sections {
            let kind = match name.as_str() {
                "song" => SectionKind::Text,
                "instrument" => SectionKind::Data,
                "data" => SectionKind::Data,
                "rodata" => SectionKind::ReadOnlyData,
                "text" => SectionKind::Text,
                _ => SectionKind::Text,
            };

            let sec_id = obj.add_section(
                vec![],
                name.into_bytes(),
                kind
            );

            obj.append_section_data(sec_id, &data, 1);

            for sym in symbols {
                let id = obj.add_symbol(Symbol {
                    name: sym.name.as_bytes().to_vec(),
                    value: sym.offset,
                    size: 8,
                    kind: SymbolKind::Label,
                    scope: SymbolScope::Linkage,
                    weak: false,
                    section: SymbolSection::Section(sec_id),
                    flags: SymbolFlags::None
                });

                symbol_ids.insert(sym.name, (id, sec_id));
            }
            relocations_store.insert(sec_id, relocations);
        }

        for (sec_id, rels) in relocations_store {
            for rel in rels {
                let sym_id = symbol_ids.get(&rel.label)
                    .map(|i| i.0)
                    .unwrap_or_else(|| {
                        obj.add_symbol(Symbol {
                            name: rel.label.into_bytes(),
                            value: 0,
                            size: 8,
                            kind: SymbolKind::Unknown,
                            scope: SymbolScope::Linkage,
                            weak: false,
                            section: SymbolSection::Undefined,
                            flags: SymbolFlags::None
                        })
                    });
                    // .ok_or::<Box<dyn Error>>(format!("no symbol named {} found", rel.label).into())?
                    // .clone();

                obj.add_relocation(sec_id, Relocation {
                    offset: rel.offset,
                    size: 64,
                    kind: RelocationKind::Absolute,
                    encoding: RelocationEncoding::Generic,
                    symbol: sym_id,
                    addend: 0
                })?;
            }
        }

        obj.write_stream(&mut self.f)?;

        Ok(())
    }
}




