#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Syntax {
    // Entrypoints
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Zero,
    File,
    // Paths
    Exit,
    VerticalConnector,
    HorizontalConnector,
    IntersectingConnector,
    Floor,
    // Operations
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Max,
    Min,
    GreaterThan,
    LessThan,
    Equal,
}

impl Syntax {
    pub fn symbol(character: Syntax) -> char {
        match character {
            // Entrypoints
            Syntax::One => '1',
            Syntax::Two => '2',
            Syntax::Three => '3',
            Syntax::Four => '4',
            Syntax::Five => '5',
            Syntax::Six => '6',
            Syntax::Seven => '7',
            Syntax::Eight => '8',
            Syntax::Nine => '9',
            Syntax::Zero => '0',
            Syntax::File => 'F',
            // Paths
            Syntax::Exit => 'W',
            Syntax::VerticalConnector => '|',
            Syntax::HorizontalConnector => '_',
            Syntax::IntersectingConnector => '#',
            Syntax::Floor => '.',
            // Operations
            Syntax::Add => '+',
            Syntax::Subtract => '-',
            Syntax::Multiply => '*',
            Syntax::Divide => '/',
            Syntax::Modulo => '%',
            Syntax::Max => '^',
            Syntax::Min => 'v',
            Syntax::GreaterThan => '>',
            Syntax::LessThan => '<',
            Syntax::Equal => '=',
        }
    }

    // TODO: Make a function that verifies that the syntax is used with the
    // right number of paramaters

    pub fn get_symbol(self) -> char {
        Self::symbol(self)
    }
}

/// Convert a character to a syntax
impl From<char> for Syntax {
    fn from(character: char) -> Syntax {
        match character {
            // Entrypoints
            '1' => Syntax::One,
            '2' => Syntax::Two,
            '3' => Syntax::Three,
            '4' => Syntax::Four,
            '5' => Syntax::Five,
            '6' => Syntax::Six,
            '7' => Syntax::Seven,
            '8' => Syntax::Eight,
            '9' => Syntax::Nine,
            '0' => Syntax::Zero,
            'F' => Syntax::File,
            // Paths
            'W' => Syntax::Exit,
            '|' => Syntax::VerticalConnector,
            '_' => Syntax::HorizontalConnector,
            '.' => Syntax::Floor,
            '#' => Syntax::IntersectingConnector,
            // Operations
            '+' => Syntax::Add,
            '-' => Syntax::Subtract,
            '*' => Syntax::Multiply,
            '/' => Syntax::Divide,
            '%' => Syntax::Modulo,
            '^' => Syntax::Max,
            'v' => Syntax::Min,
            '>' => Syntax::GreaterThan,
            '<' => Syntax::LessThan,
            '=' => Syntax::Equal,

            _ => panic!("Unknown character"),
        }
    }
}
