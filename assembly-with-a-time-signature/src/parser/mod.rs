use lwb_parser::language;
use lwb_parser::language::Language;
use lwb_parser::sources::source_file::SourceFile;
use crate::parser::AST::Instrument;

pub mod ast;

language! {pub Asm at mod ast}

pub fn parse(input: &SourceFile) -> <Asm as Language>::Ast {
    Asm::parse(input)
}

#[cfg(test)]
mod tests {
    use lwb_parser::language::Language;
    use lwb_parser::sources::source_file::SourceFile;
    use crate::parser::Asm;

    #[test]
    pub fn test_language() {
        dbg!(Asm::parse(&SourceFile::new("
add %r0 #3 #4
test:
    sub %r1 #5 %r6
    jmp test
        ", "test.asm")));
    }
}

impl<M> From<Instrument<M>> for u8 {
    fn from(i: Instrument<M>) -> Self {
        match i {
            Instrument::Custom(_, n) => n.1.parse().unwrap(),
            Instrument::Piano(_) => 0,
            Instrument::Celesta(_) => 8,
            Instrument::Glockenspiel(_) => 9,
            Instrument::Musicbox(_) => 10,
            Instrument::Marimba(_) => 12,
            Instrument::Dulcimer(_) => 15,
            Instrument::Organ(_) => 16,
            Instrument::Accordion(_) => 21,
            Instrument::Harmonica(_) => 22,
            Instrument::Nylonguitar(_) => 24,
            Instrument::Steelguitar(_) => 25,
            Instrument::Distortionguitar(_) => 30,
            Instrument::Acousticbass(_) => 32,
            Instrument::Slapbass(_) => 36,
            Instrument::Violin(_) => 40,
            Instrument::Harp(_) => 46,
            Instrument::Timpani(_) => 47,
            Instrument::Strings(_) => 48,
            Instrument::Synthstrings(_) => 50,
            Instrument::Voiceoohs(_) => 53,
            Instrument::Synthvox(_) => 54,
            Instrument::Brass(_) => 61,
            Instrument::Altosax(_) => 65,
            Instrument::Tenorsax(_) => 66,
            Instrument::Oboe(_) => 68,
            Instrument::Enghorn(_) => 69,
            Instrument::Flute(_) => 73,
            Instrument::Panflute(_) => 75,
            Instrument::Whistle(_) => 78,
            Instrument::Ocarina(_) => 79,
            Instrument::Heavysquarewave(_) => 80,
            Instrument::Fantasia(_) => 88,
            Instrument::Warmpad(_) => 89,
            Instrument::Echodrops(_) => 102,
            Instrument::Startheme(_) => 103,
            Instrument::Sitar(_) => 104,
            Instrument::Banjo(_) => 105,
            Instrument::Kalimba(_) => 108,
            Instrument::Bagpipe(_) => 109,
            Instrument::Fiddle(_) => 110,
            Instrument::Steeldrum(_) => 114,
            Instrument::Bird(_) => 123,
            Instrument::Telephone(_) => 124,
            Instrument::Applause(_) => 126,
            Instrument::Gunshot(_) => 127,
        }
    }
}