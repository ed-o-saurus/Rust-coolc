// All of the tokens that are produced by the lexer

// line_no is the line number where the token appears

pub enum Token {
    // Characters
    Add { line_no: i16 },
    Div { line_no: i16 },
    Sub { line_no: i16 },
    Mul { line_no: i16 },
    Eq { line_no: i16 },
    LT { line_no: i16 },
    Dot { line_no: i16 },
    Neg { line_no: i16 },
    Comma { line_no: i16 },
    SemiColon { line_no: i16 },
    Colon { line_no: i16 },
    OpenParen { line_no: i16 },
    CloseParen { line_no: i16 },
    At { line_no: i16 },
    OpenBrace { line_no: i16 },
    CloseBrace { line_no: i16 },

    // Key words
    Class { line_no: i16 },
    Else { line_no: i16 },
    Fi { line_no: i16 },
    If { line_no: i16 },
    In { line_no: i16 },
    Inherits { line_no: i16 },
    Let { line_no: i16 },
    Loop { line_no: i16 },
    Pool { line_no: i16 },
    Then { line_no: i16 },
    While { line_no: i16 },
    Case { line_no: i16 },
    Esac { line_no: i16 },
    Of { line_no: i16 },
    New { line_no: i16 },
    IsVoid { line_no: i16 },
    Not { line_no: i16 },

    // Character sets
    DArrow { line_no: i16 }, // =>
    LEq { line_no: i16 },    // <=
    Assign { line_no: i16 }, // <-

    // Constants
    BoolConst { line_no: i16, val: bool },
    IntConst { line_no: i16, val: u32 },
    StrConst { line_no: i16, val: String },

    // IDs
    TypeID { line_no: i16, type_name: String },
    ObjectID { line_no: i16, obj_name: String },

    // Placed at the end of all file outputs
    // Needed for the parser
    End { line_no: i16 },
}

impl Token {
    pub fn get_line_no(&self) -> i16 {
        match self {
            Token::Add { line_no }
            | Token::Div { line_no }
            | Token::Sub { line_no }
            | Token::Mul { line_no }
            | Token::Eq { line_no }
            | Token::LT { line_no }
            | Token::Dot { line_no }
            | Token::Neg { line_no }
            | Token::Comma { line_no }
            | Token::SemiColon { line_no }
            | Token::Colon { line_no }
            | Token::OpenParen { line_no }
            | Token::CloseParen { line_no }
            | Token::At { line_no }
            | Token::OpenBrace { line_no }
            | Token::CloseBrace { line_no }
            | Token::Class { line_no }
            | Token::Else { line_no }
            | Token::Fi { line_no }
            | Token::If { line_no }
            | Token::In { line_no }
            | Token::Inherits { line_no }
            | Token::Let { line_no }
            | Token::Loop { line_no }
            | Token::Pool { line_no }
            | Token::Then { line_no }
            | Token::While { line_no }
            | Token::Case { line_no }
            | Token::Esac { line_no }
            | Token::Of { line_no }
            | Token::New { line_no }
            | Token::IsVoid { line_no }
            | Token::Not { line_no }
            | Token::DArrow { line_no }
            | Token::LEq { line_no }
            | Token::Assign { line_no }
            | Token::BoolConst { line_no, .. }
            | Token::TypeID { line_no, .. }
            | Token::ObjectID { line_no, .. }
            | Token::IntConst { line_no, .. }
            | Token::StrConst { line_no, .. }
            | Token::End { line_no } => *line_no,
        }
    }
}
