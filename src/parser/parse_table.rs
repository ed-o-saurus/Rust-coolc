use crate::token::Token;

// This module contains information about the behaviour of the shift-reduce parse used to parse the token stream into an AST.

// The state component of the stack
#[derive(Copy, Clone)]
pub enum State {
    StateInitial, // Initial State matched with the Bottom Node
    State002,
    State004,
    State005,
    State007,
    State009,
    State010,
    State011,
    State012,
    State013,
    State014,
    State016,
    State017,
    State019,
    State020,
    State021,
    State022,
    State023,
    State024,
    State025,
    State026,
    State027,
    State028,
    State029,
    State030,
    State031,
    State032,
    State033,
    State034,
    State035,
    State036,
    State037,
    State038,
    State039,
    State040,
    State041,
    State042,
    State043,
    State044,
    State045,
    State046,
    State047,
    State048,
    State049,
    State050,
    State051,
    State052,
    State053,
    State054,
    State055,
    State057,
    State058,
    State059,
    State060,
    State061,
    State062,
    State063,
    State064,
    State065,
    State066,
    State068,
    State069,
    State070,
    State071,
    State072,
    State073,
    State074,
    State075,
    State076,
    State077,
    State078,
    State079,
    State080,
    State081,
    State084,
    State085,
    State086,
    State087,
    State088,
    State089,
    State090,
    State092,
    State093,
    State094,
    State095,
    State096,
    State097,
    State098,
    State099,
    State100,
    State101,
    State102,
    State103,
    State104,
    State105,
    State106,
    State109,
    State110,
    State111,
    State112,
    State113,
    State114,
    State115,
    State116,
    State117,
    State118,
    State119,
    State120,
    State121,
    State122,
    State123,
    State124,
    State125,
    State126,
    State127,
    State128,
    State129,
    State130,
    State131,
    State132,
    State133,
    State134,
    State135,
    State136,
    State137,
    State138,
    State139,
    State140,
    State141,
    State142,
    State143,
    State144,
    State145,
    State146,
    State147,
    State148,
}

// Action to be carried out by the parse as determined by the parse table
pub enum Action {
    Shift { new_state: State },
    Reduce02,
    Reduce03,
    Reduce05,
    Reduce06,
    Reduce07,
    Reduce08,
    Reduce10,
    Reduce11,
    Reduce12,
    Reduce13,
    Reduce14,
    Reduce15,
    Reduce16,
    Reduce17,
    Reduce18,
    Reduce19,
    Reduce20,
    Reduce21,
    Reduce22,
    Reduce24,
    Reduce25,
    Reduce26,
    Reduce27,
    Reduce28,
    Reduce29,
    Reduce30,
    Reduce31,
    Reduce34,
    Reduce35,
    Reduce36,
    Reduce37,
    Reduce38,
    Reduce39,
    Reduce40,
    Reduce41,
    Reduce42,
    Reduce43,
    Reduce44,
    ReduceArith,
    Reduce49,
    ReduceComp,
    Reduce52,
    Reduce53,
    Reduce54,
    Reduce55,
    Reduce56,
    Reduce57,
    Reduce58,
    Accept,
}

// What to do (shift, reduce, or accept based on the next token and the top parse state
pub fn get_action(top_state: State, next_token: &Token) -> Result<Action, i16> {
    let next_line_no = next_token.get_line_no();

    Ok(match top_state {
        State::StateInitial => match next_token {
            Token::Class { .. } => Action::Shift {
                new_state: State::State002,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State002 => match next_token {
            Token::TypeID { .. } => Action::Shift {
                new_state: State::State007,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State004 => match next_token {
            Token::Class { .. } => Action::Shift {
                new_state: State::State002,
            },
            Token::End { .. } => Action::Accept,
            _ => {
                return Err(next_line_no);
            }
        },

        State::State005 => match next_token {
            Token::SemiColon { .. } => Action::Shift {
                new_state: State::State010,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State007 => match next_token {
            Token::Inherits { .. } => Action::Shift {
                new_state: State::State011,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State012,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State009 => match next_token {
            Token::SemiColon { .. } => Action::Shift {
                new_state: State::State013,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State010 => Action::Reduce02,

        State::State011 => match next_token {
            Token::TypeID { .. } => Action::Shift {
                new_state: State::State014,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State012 => match next_token {
            Token::ObjectID { .. } => Action::Reduce07,
            Token::CloseBrace { .. } => Action::Reduce07,
            _ => {
                return Err(next_line_no);
            }
        },

        State::State013 => Action::Reduce03,

        State::State014 => match next_token {
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State017,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State016 => match next_token {
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State019,
            },
            Token::CloseBrace { .. } => Action::Shift {
                new_state: State::State020,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State017 => match next_token {
            Token::ObjectID { .. } => Action::Reduce07,
            Token::CloseBrace { .. } => Action::Reduce07,
            _ => {
                return Err(next_line_no);
            }
        },

        State::State019 => match next_token {
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State023,
            },
            Token::Colon { .. } => Action::Shift {
                new_state: State::State024,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State020 => Action::Reduce05,

        State::State021 => match next_token {
            Token::SemiColon { .. } => Action::Shift {
                new_state: State::State025,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State022 => match next_token {
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State019,
            },
            Token::CloseBrace { .. } => Action::Shift {
                new_state: State::State026,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State023 => match next_token {
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State027,
            },
            _ => Action::Reduce15,
        },

        State::State024 => match next_token {
            Token::TypeID { .. } => Action::Shift {
                new_state: State::State031,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State025 => Action::Reduce08,

        State::State026 => Action::Reduce06,

        State::State027 => match next_token {
            Token::Colon { .. } => Action::Shift {
                new_state: State::State032,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State028 => match next_token {
            Token::Comma { .. } => Action::Shift {
                new_state: State::State033,
            },
            _ => Action::Reduce16,
        },

        State::State029 => match next_token {
            Token::CloseParen { .. } => Action::Shift {
                new_state: State::State034,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State030 => Action::Reduce13,

        State::State031 => match next_token {
            Token::Assign { .. } => Action::Shift {
                new_state: State::State035,
            },
            _ => Action::Reduce11,
        },

        State::State032 => match next_token {
            Token::TypeID { .. } => Action::Shift {
                new_state: State::State036,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State033 => match next_token {
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State027,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State034 => match next_token {
            Token::Colon { .. } => Action::Shift {
                new_state: State::State038,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State035 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State046,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State047,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State048,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State049,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State050,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State051,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State052,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State036 => Action::Reduce17,

        State::State037 => Action::Reduce14,

        State::State038 => match next_token {
            Token::TypeID { .. } => Action::Shift {
                new_state: State::State054,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State039 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State046,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State047,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State048,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State049,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State050,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State051,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State052,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State040 => match next_token {
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State057,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State041 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State046,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State047,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State048,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State049,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State050,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State051,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State052,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State042 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State046,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State047,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State048,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State049,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State050,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State051,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State052,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State043 => match next_token {
            Token::TypeID { .. } => Action::Shift {
                new_state: State::State061,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State044 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State046,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State047,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State048,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State049,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State050,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State051,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State052,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State045 => Action::Reduce57,

        State::State046 => Action::Reduce56,

        State::State047 => Action::Reduce58,

        State::State048 => match next_token {
            Token::Assign { .. } => Action::Shift {
                new_state: State::State063,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State064,
            },
            _ => Action::Reduce55,
        },

        State::State049 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State046,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State047,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State048,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State049,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State050,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State051,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State052,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State050 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State046,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State047,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State048,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State049,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State050,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State051,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State052,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State051 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State046,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State047,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State048,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State049,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State050,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State051,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State052,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State052 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State046,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State047,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State048,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State049,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State050,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State051,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State052,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State053 => match next_token {
            Token::LEq { .. } => Action::Shift {
                new_state: State::State071,
            },
            Token::LT { .. } => Action::Shift {
                new_state: State::State072,
            },
            Token::Eq { .. } => Action::Shift {
                new_state: State::State073,
            },
            Token::Add { .. } => Action::Shift {
                new_state: State::State074,
            },
            Token::Sub { .. } => Action::Shift {
                new_state: State::State075,
            },
            Token::Mul { .. } => Action::Shift {
                new_state: State::State076,
            },
            Token::Div { .. } => Action::Shift {
                new_state: State::State077,
            },
            Token::At { .. } => Action::Shift {
                new_state: State::State078,
            },
            Token::Dot { .. } => Action::Shift {
                new_state: State::State079,
            },
            _ => Action::Reduce12,
        },

        State::State054 => match next_token {
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State080,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State055 => match next_token {
            Token::Then { .. } => Action::Shift {
                new_state: State::State081,
            },
            Token::LEq { .. } => Action::Shift {
                new_state: State::State071,
            },
            Token::LT { .. } => Action::Shift {
                new_state: State::State072,
            },
            Token::Eq { .. } => Action::Shift {
                new_state: State::State073,
            },
            Token::Add { .. } => Action::Shift {
                new_state: State::State074,
            },
            Token::Sub { .. } => Action::Shift {
                new_state: State::State075,
            },
            Token::Mul { .. } => Action::Shift {
                new_state: State::State076,
            },
            Token::Div { .. } => Action::Shift {
                new_state: State::State077,
            },
            Token::At { .. } => Action::Shift {
                new_state: State::State078,
            },
            Token::Dot { .. } => Action::Shift {
                new_state: State::State079,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State057 => match next_token {
            Token::Colon { .. } => Action::Shift {
                new_state: State::State084,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State058 => Action::Reduce41,

        State::State059 => match next_token {
            Token::Loop { .. } => Action::Shift {
                new_state: State::State085,
            },
            Token::LEq { .. } => Action::Shift {
                new_state: State::State071,
            },
            Token::LT { .. } => Action::Shift {
                new_state: State::State072,
            },
            Token::Eq { .. } => Action::Shift {
                new_state: State::State073,
            },
            Token::Add { .. } => Action::Shift {
                new_state: State::State074,
            },
            Token::Sub { .. } => Action::Shift {
                new_state: State::State075,
            },
            Token::Mul { .. } => Action::Shift {
                new_state: State::State076,
            },
            Token::Div { .. } => Action::Shift {
                new_state: State::State077,
            },
            Token::At { .. } => Action::Shift {
                new_state: State::State078,
            },
            Token::Dot { .. } => Action::Shift {
                new_state: State::State079,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State060 => match next_token {
            Token::Of { .. } => Action::Shift {
                new_state: State::State086,
            },
            Token::LEq { .. } => Action::Shift {
                new_state: State::State071,
            },
            Token::LT { .. } => Action::Shift {
                new_state: State::State072,
            },
            Token::Eq { .. } => Action::Shift {
                new_state: State::State073,
            },
            Token::Add { .. } => Action::Shift {
                new_state: State::State074,
            },
            Token::Sub { .. } => Action::Shift {
                new_state: State::State075,
            },
            Token::Mul { .. } => Action::Shift {
                new_state: State::State076,
            },
            Token::Div { .. } => Action::Shift {
                new_state: State::State077,
            },
            Token::At { .. } => Action::Shift {
                new_state: State::State078,
            },
            Token::Dot { .. } => Action::Shift {
                new_state: State::State079,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State061 => Action::Reduce43,

        State::State062 => match next_token {
            Token::At { .. } => Action::Shift {
                new_state: State::State078,
            },
            Token::Dot { .. } => Action::Shift {
                new_state: State::State079,
            },
            _ => Action::Reduce44,
        },

        State::State063 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State046,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State047,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State048,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State049,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State050,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State051,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State052,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State064 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State046,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State047,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State048,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State049,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State050,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State051,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State052,
            },
            _ => Action::Reduce26,
        },

        State::State065 => match next_token {
            Token::LEq { .. } => Action::Shift {
                new_state: State::State071,
            },
            Token::LT { .. } => Action::Shift {
                new_state: State::State072,
            },
            Token::Eq { .. } => Action::Shift {
                new_state: State::State073,
            },
            Token::Add { .. } => Action::Shift {
                new_state: State::State074,
            },
            Token::Sub { .. } => Action::Shift {
                new_state: State::State075,
            },
            Token::Mul { .. } => Action::Shift {
                new_state: State::State076,
            },
            Token::Div { .. } => Action::Shift {
                new_state: State::State077,
            },
            Token::At { .. } => Action::Shift {
                new_state: State::State078,
            },
            Token::Dot { .. } => Action::Shift {
                new_state: State::State079,
            },
            _ => Action::Reduce53,
        },

        State::State066 => match next_token {
            Token::At { .. } => Action::Shift {
                new_state: State::State078,
            },
            Token::Dot { .. } => Action::Shift {
                new_state: State::State079,
            },
            _ => Action::Reduce49,
        },

        State::State068 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State046,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State047,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State048,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State049,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State050,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State051,
            },
            Token::CloseBrace { .. } => Action::Shift {
                new_state: State::State092,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State052,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State069 => match next_token {
            Token::LEq { .. } => Action::Shift {
                new_state: State::State071,
            },
            Token::LT { .. } => Action::Shift {
                new_state: State::State072,
            },
            Token::Eq { .. } => Action::Shift {
                new_state: State::State073,
            },
            Token::Add { .. } => Action::Shift {
                new_state: State::State074,
            },
            Token::Sub { .. } => Action::Shift {
                new_state: State::State075,
            },
            Token::Mul { .. } => Action::Shift {
                new_state: State::State076,
            },
            Token::Div { .. } => Action::Shift {
                new_state: State::State077,
            },
            Token::At { .. } => Action::Shift {
                new_state: State::State078,
            },
            Token::Dot { .. } => Action::Shift {
                new_state: State::State079,
            },
            Token::SemiColon { .. } => Action::Shift {
                new_state: State::State094,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State070 => match next_token {
            Token::LEq { .. } => Action::Shift {
                new_state: State::State071,
            },
            Token::LT { .. } => Action::Shift {
                new_state: State::State072,
            },
            Token::Eq { .. } => Action::Shift {
                new_state: State::State073,
            },
            Token::Add { .. } => Action::Shift {
                new_state: State::State074,
            },
            Token::Sub { .. } => Action::Shift {
                new_state: State::State075,
            },
            Token::Mul { .. } => Action::Shift {
                new_state: State::State076,
            },
            Token::Div { .. } => Action::Shift {
                new_state: State::State077,
            },
            Token::At { .. } => Action::Shift {
                new_state: State::State078,
            },
            Token::Dot { .. } => Action::Shift {
                new_state: State::State079,
            },
            Token::CloseParen { .. } => Action::Shift {
                new_state: State::State095,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State071 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State046,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State047,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State048,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State049,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State050,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State051,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State052,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State072 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State046,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State047,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State048,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State049,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State050,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State051,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State052,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State073 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State046,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State047,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State048,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State049,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State050,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State051,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State052,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State074 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State046,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State047,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State048,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State049,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State050,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State051,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State052,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State075 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State046,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State047,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State048,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State049,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State050,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State051,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State052,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State076 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State046,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State047,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State048,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State049,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State050,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State051,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State052,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State077 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State046,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State047,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State048,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State049,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State050,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State051,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State052,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State078 => match next_token {
            Token::TypeID { .. } => Action::Shift {
                new_state: State::State103,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State079 => match next_token {
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State104,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State080 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State046,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State047,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State048,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State049,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State050,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State051,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State052,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State081 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State046,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State047,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State048,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State049,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State050,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State051,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State052,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State084 => match next_token {
            Token::TypeID { .. } => Action::Shift {
                new_state: State::State109,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State085 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State046,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State047,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State048,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State049,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State050,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State051,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State052,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State086 => match next_token {
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State111,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State087 => match next_token {
            Token::LEq { .. } => Action::Shift {
                new_state: State::State071,
            },
            Token::LT { .. } => Action::Shift {
                new_state: State::State072,
            },
            Token::Eq { .. } => Action::Shift {
                new_state: State::State073,
            },
            Token::Add { .. } => Action::Shift {
                new_state: State::State074,
            },
            Token::Sub { .. } => Action::Shift {
                new_state: State::State075,
            },
            Token::Mul { .. } => Action::Shift {
                new_state: State::State076,
            },
            Token::Div { .. } => Action::Shift {
                new_state: State::State077,
            },
            Token::At { .. } => Action::Shift {
                new_state: State::State078,
            },
            Token::Dot { .. } => Action::Shift {
                new_state: State::State079,
            },
            _ => Action::Reduce34,
        },

        State::State088 => match next_token {
            Token::Comma { .. } => Action::Shift {
                new_state: State::State114,
            },
            _ => Action::Reduce27,
        },

        State::State089 => match next_token {
            Token::CloseParen { .. } => Action::Shift {
                new_state: State::State115,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State090 => match next_token {
            Token::LEq { .. } => Action::Shift {
                new_state: State::State071,
            },
            Token::LT { .. } => Action::Shift {
                new_state: State::State072,
            },
            Token::Eq { .. } => Action::Shift {
                new_state: State::State073,
            },
            Token::Add { .. } => Action::Shift {
                new_state: State::State074,
            },
            Token::Sub { .. } => Action::Shift {
                new_state: State::State075,
            },
            Token::Mul { .. } => Action::Shift {
                new_state: State::State076,
            },
            Token::Div { .. } => Action::Shift {
                new_state: State::State077,
            },
            Token::At { .. } => Action::Shift {
                new_state: State::State078,
            },
            Token::Dot { .. } => Action::Shift {
                new_state: State::State079,
            },
            _ => Action::Reduce24,
        },

        State::State092 => Action::Reduce40,

        State::State093 => match next_token {
            Token::LEq { .. } => Action::Shift {
                new_state: State::State071,
            },
            Token::LT { .. } => Action::Shift {
                new_state: State::State072,
            },
            Token::Eq { .. } => Action::Shift {
                new_state: State::State073,
            },
            Token::Add { .. } => Action::Shift {
                new_state: State::State074,
            },
            Token::Sub { .. } => Action::Shift {
                new_state: State::State075,
            },
            Token::Mul { .. } => Action::Shift {
                new_state: State::State076,
            },
            Token::Div { .. } => Action::Shift {
                new_state: State::State077,
            },
            Token::At { .. } => Action::Shift {
                new_state: State::State078,
            },
            Token::Dot { .. } => Action::Shift {
                new_state: State::State079,
            },
            Token::SemiColon { .. } => Action::Shift {
                new_state: State::State116,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State094 => Action::Reduce21,

        State::State095 => Action::Reduce54,

        State::State096 => match next_token {
            Token::Add { .. } => Action::Shift {
                new_state: State::State074,
            },
            Token::Sub { .. } => Action::Shift {
                new_state: State::State075,
            },
            Token::Mul { .. } => Action::Shift {
                new_state: State::State076,
            },
            Token::Div { .. } => Action::Shift {
                new_state: State::State077,
            },
            Token::At { .. } => Action::Shift {
                new_state: State::State078,
            },
            Token::Dot { .. } => Action::Shift {
                new_state: State::State079,
            },
            _ => Action::ReduceComp,
        },

        State::State097 => match next_token {
            Token::Add { .. } => Action::Shift {
                new_state: State::State074,
            },
            Token::Sub { .. } => Action::Shift {
                new_state: State::State075,
            },
            Token::Mul { .. } => Action::Shift {
                new_state: State::State076,
            },
            Token::Div { .. } => Action::Shift {
                new_state: State::State077,
            },
            Token::At { .. } => Action::Shift {
                new_state: State::State078,
            },
            Token::Dot { .. } => Action::Shift {
                new_state: State::State079,
            },
            _ => Action::ReduceComp,
        },

        State::State098 => match next_token {
            Token::Add { .. } => Action::Shift {
                new_state: State::State074,
            },
            Token::Sub { .. } => Action::Shift {
                new_state: State::State075,
            },
            Token::Mul { .. } => Action::Shift {
                new_state: State::State076,
            },
            Token::Div { .. } => Action::Shift {
                new_state: State::State077,
            },
            Token::At { .. } => Action::Shift {
                new_state: State::State078,
            },
            Token::Dot { .. } => Action::Shift {
                new_state: State::State079,
            },
            _ => Action::Reduce52,
        },

        State::State099 => match next_token {
            Token::Mul { .. } => Action::Shift {
                new_state: State::State076,
            },
            Token::Div { .. } => Action::Shift {
                new_state: State::State077,
            },
            Token::At { .. } => Action::Shift {
                new_state: State::State078,
            },
            Token::Dot { .. } => Action::Shift {
                new_state: State::State079,
            },
            _ => Action::ReduceArith,
        },

        State::State100 => match next_token {
            Token::Mul { .. } => Action::Shift {
                new_state: State::State076,
            },
            Token::Div { .. } => Action::Shift {
                new_state: State::State077,
            },
            Token::At { .. } => Action::Shift {
                new_state: State::State078,
            },
            Token::Dot { .. } => Action::Shift {
                new_state: State::State079,
            },
            _ => Action::ReduceArith,
        },

        State::State101 => match next_token {
            Token::At { .. } => Action::Shift {
                new_state: State::State078,
            },
            Token::Dot { .. } => Action::Shift {
                new_state: State::State079,
            },
            _ => Action::ReduceArith,
        },

        State::State102 => match next_token {
            Token::At { .. } => Action::Shift {
                new_state: State::State078,
            },
            Token::Dot { .. } => Action::Shift {
                new_state: State::State079,
            },
            _ => Action::ReduceArith,
        },

        State::State103 => match next_token {
            Token::Dot { .. } => Action::Shift {
                new_state: State::State117,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State104 => match next_token {
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State118,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State105 => match next_token {
            Token::LEq { .. } => Action::Shift {
                new_state: State::State071,
            },
            Token::LT { .. } => Action::Shift {
                new_state: State::State072,
            },
            Token::Eq { .. } => Action::Shift {
                new_state: State::State073,
            },
            Token::Add { .. } => Action::Shift {
                new_state: State::State074,
            },
            Token::Sub { .. } => Action::Shift {
                new_state: State::State075,
            },
            Token::Mul { .. } => Action::Shift {
                new_state: State::State076,
            },
            Token::Div { .. } => Action::Shift {
                new_state: State::State077,
            },
            Token::At { .. } => Action::Shift {
                new_state: State::State078,
            },
            Token::Dot { .. } => Action::Shift {
                new_state: State::State079,
            },
            Token::CloseBrace { .. } => Action::Shift {
                new_state: State::State119,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State106 => match next_token {
            Token::Else { .. } => Action::Shift {
                new_state: State::State120,
            },
            Token::LEq { .. } => Action::Shift {
                new_state: State::State071,
            },
            Token::LT { .. } => Action::Shift {
                new_state: State::State072,
            },
            Token::Eq { .. } => Action::Shift {
                new_state: State::State073,
            },
            Token::Add { .. } => Action::Shift {
                new_state: State::State074,
            },
            Token::Sub { .. } => Action::Shift {
                new_state: State::State075,
            },
            Token::Mul { .. } => Action::Shift {
                new_state: State::State076,
            },
            Token::Div { .. } => Action::Shift {
                new_state: State::State077,
            },
            Token::At { .. } => Action::Shift {
                new_state: State::State078,
            },
            Token::Dot { .. } => Action::Shift {
                new_state: State::State079,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State109 => match next_token {
            Token::In { .. } => Action::Shift {
                new_state: State::State121,
            },
            Token::Assign { .. } => Action::Shift {
                new_state: State::State122,
            },
            Token::Comma { .. } => Action::Shift {
                new_state: State::State123,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State110 => match next_token {
            Token::Pool { .. } => Action::Shift {
                new_state: State::State124,
            },
            Token::LEq { .. } => Action::Shift {
                new_state: State::State071,
            },
            Token::LT { .. } => Action::Shift {
                new_state: State::State072,
            },
            Token::Eq { .. } => Action::Shift {
                new_state: State::State073,
            },
            Token::Add { .. } => Action::Shift {
                new_state: State::State074,
            },
            Token::Sub { .. } => Action::Shift {
                new_state: State::State075,
            },
            Token::Mul { .. } => Action::Shift {
                new_state: State::State076,
            },
            Token::Div { .. } => Action::Shift {
                new_state: State::State077,
            },
            Token::At { .. } => Action::Shift {
                new_state: State::State078,
            },
            Token::Dot { .. } => Action::Shift {
                new_state: State::State079,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State111 => match next_token {
            Token::Colon { .. } => Action::Shift {
                new_state: State::State125,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State112 => match next_token {
            Token::Esac { .. } => Action::Shift {
                new_state: State::State126,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State111,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State113 => match next_token {
            Token::SemiColon { .. } => Action::Shift {
                new_state: State::State128,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State114 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State046,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State047,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State048,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State049,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State050,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State051,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State052,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State115 => Action::Reduce35,

        State::State116 => Action::Reduce22,

        State::State117 => match next_token {
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State130,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State118 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State046,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State047,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State048,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State049,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State050,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State051,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State052,
            },
            _ => Action::Reduce26,
        },

        State::State119 => Action::Reduce10,

        State::State120 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State046,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State047,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State048,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State049,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State050,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State051,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State052,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State121 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State046,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State047,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State048,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State049,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State050,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State051,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State052,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State122 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State046,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State047,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State048,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State049,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State050,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State051,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State052,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State123 => match next_token {
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State057,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State124 => Action::Reduce39,

        State::State125 => match next_token {
            Token::TypeID { .. } => Action::Shift {
                new_state: State::State136,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State126 => Action::Reduce42,

        State::State127 => match next_token {
            Token::SemiColon { .. } => Action::Shift {
                new_state: State::State137,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State128 => Action::Reduce18,

        State::State129 => match next_token {
            Token::LEq { .. } => Action::Shift {
                new_state: State::State071,
            },
            Token::LT { .. } => Action::Shift {
                new_state: State::State072,
            },
            Token::Eq { .. } => Action::Shift {
                new_state: State::State073,
            },
            Token::Add { .. } => Action::Shift {
                new_state: State::State074,
            },
            Token::Sub { .. } => Action::Shift {
                new_state: State::State075,
            },
            Token::Mul { .. } => Action::Shift {
                new_state: State::State076,
            },
            Token::Div { .. } => Action::Shift {
                new_state: State::State077,
            },
            Token::At { .. } => Action::Shift {
                new_state: State::State078,
            },
            Token::Dot { .. } => Action::Shift {
                new_state: State::State079,
            },
            _ => Action::Reduce25,
        },

        State::State130 => match next_token {
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State138,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State131 => match next_token {
            Token::CloseParen { .. } => Action::Shift {
                new_state: State::State139,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State132 => match next_token {
            Token::Fi { .. } => Action::Shift {
                new_state: State::State140,
            },
            Token::LEq { .. } => Action::Shift {
                new_state: State::State071,
            },
            Token::LT { .. } => Action::Shift {
                new_state: State::State072,
            },
            Token::Eq { .. } => Action::Shift {
                new_state: State::State073,
            },
            Token::Add { .. } => Action::Shift {
                new_state: State::State074,
            },
            Token::Sub { .. } => Action::Shift {
                new_state: State::State075,
            },
            Token::Mul { .. } => Action::Shift {
                new_state: State::State076,
            },
            Token::Div { .. } => Action::Shift {
                new_state: State::State077,
            },
            Token::At { .. } => Action::Shift {
                new_state: State::State078,
            },
            Token::Dot { .. } => Action::Shift {
                new_state: State::State079,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State133 => match next_token {
            Token::LEq { .. } => Action::Shift {
                new_state: State::State071,
            },
            Token::LT { .. } => Action::Shift {
                new_state: State::State072,
            },
            Token::Eq { .. } => Action::Shift {
                new_state: State::State073,
            },
            Token::Add { .. } => Action::Shift {
                new_state: State::State074,
            },
            Token::Sub { .. } => Action::Shift {
                new_state: State::State075,
            },
            Token::Mul { .. } => Action::Shift {
                new_state: State::State076,
            },
            Token::Div { .. } => Action::Shift {
                new_state: State::State077,
            },
            Token::At { .. } => Action::Shift {
                new_state: State::State078,
            },
            Token::Dot { .. } => Action::Shift {
                new_state: State::State079,
            },
            _ => Action::Reduce28,
        },

        State::State134 => match next_token {
            Token::In { .. } => Action::Shift {
                new_state: State::State141,
            },
            Token::LEq { .. } => Action::Shift {
                new_state: State::State071,
            },
            Token::LT { .. } => Action::Shift {
                new_state: State::State072,
            },
            Token::Eq { .. } => Action::Shift {
                new_state: State::State073,
            },
            Token::Add { .. } => Action::Shift {
                new_state: State::State074,
            },
            Token::Sub { .. } => Action::Shift {
                new_state: State::State075,
            },
            Token::Mul { .. } => Action::Shift {
                new_state: State::State076,
            },
            Token::Div { .. } => Action::Shift {
                new_state: State::State077,
            },
            Token::At { .. } => Action::Shift {
                new_state: State::State078,
            },
            Token::Dot { .. } => Action::Shift {
                new_state: State::State079,
            },
            Token::Comma { .. } => Action::Shift {
                new_state: State::State142,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State135 => Action::Reduce30,

        State::State136 => match next_token {
            Token::DArrow { .. } => Action::Shift {
                new_state: State::State143,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State137 => Action::Reduce19,

        State::State138 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State046,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State047,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State048,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State049,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State050,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State051,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State052,
            },
            _ => Action::Reduce26,
        },

        State::State139 => Action::Reduce36,

        State::State140 => Action::Reduce38,

        State::State141 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State046,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State047,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State048,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State049,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State050,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State051,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State052,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State142 => match next_token {
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State057,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State143 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State046,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State047,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State048,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State049,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State050,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State051,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State052,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State144 => match next_token {
            Token::CloseParen { .. } => Action::Shift {
                new_state: State::State148,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State145 => match next_token {
            Token::LEq { .. } => Action::Shift {
                new_state: State::State071,
            },
            Token::LT { .. } => Action::Shift {
                new_state: State::State072,
            },
            Token::Eq { .. } => Action::Shift {
                new_state: State::State073,
            },
            Token::Add { .. } => Action::Shift {
                new_state: State::State074,
            },
            Token::Sub { .. } => Action::Shift {
                new_state: State::State075,
            },
            Token::Mul { .. } => Action::Shift {
                new_state: State::State076,
            },
            Token::Div { .. } => Action::Shift {
                new_state: State::State077,
            },
            Token::At { .. } => Action::Shift {
                new_state: State::State078,
            },
            Token::Dot { .. } => Action::Shift {
                new_state: State::State079,
            },
            _ => Action::Reduce29,
        },

        State::State146 => Action::Reduce31,

        State::State147 => match next_token {
            Token::LEq { .. } => Action::Shift {
                new_state: State::State071,
            },
            Token::LT { .. } => Action::Shift {
                new_state: State::State072,
            },
            Token::Eq { .. } => Action::Shift {
                new_state: State::State073,
            },
            Token::Add { .. } => Action::Shift {
                new_state: State::State074,
            },
            Token::Sub { .. } => Action::Shift {
                new_state: State::State075,
            },
            Token::Mul { .. } => Action::Shift {
                new_state: State::State076,
            },
            Token::Div { .. } => Action::Shift {
                new_state: State::State077,
            },
            Token::At { .. } => Action::Shift {
                new_state: State::State078,
            },
            Token::Dot { .. } => Action::Shift {
                new_state: State::State079,
            },
            _ => Action::Reduce20,
        },

        State::State148 => Action::Reduce37,
    })
}

// These functions determine the state that a new non-terminal node should have on the parse stack.

pub fn get_reduce_new_state_class_list(top_state: State) -> State {
    match top_state {
        State::StateInitial => State::State004,
        _ => panic!("Impossible branch"),
    }
}

pub fn get_reduce_new_state_class(top_state: State) -> State {
    match top_state {
        State::StateInitial => State::State005,
        State::State004 => State::State009,
        _ => panic!("Impossible branch"),
    }
}

pub fn get_reduce_new_state_feature_list(top_state: State) -> State {
    match top_state {
        State::State012 => State::State016,
        State::State017 => State::State022,
        _ => panic!("Impossible branch"),
    }
}

pub fn get_reduce_new_state_feature(top_state: State) -> State {
    match top_state {
        State::State016 => State::State021,
        State::State022 => State::State021,
        _ => panic!("Impossible branch"),
    }
}

pub fn get_reduce_new_state_formal_list_ne(top_state: State) -> State {
    match top_state {
        State::State023 => State::State028,
        _ => panic!("Impossible branch"),
    }
}

pub fn get_reduce_new_state_formal_list(top_state: State) -> State {
    match top_state {
        State::State023 => State::State029,
        _ => panic!("Impossible branch"),
    }
}

pub fn get_reduce_new_state_formal(top_state: State) -> State {
    match top_state {
        State::State023 => State::State030,
        State::State033 => State::State037,
        _ => panic!("Impossible branch"),
    }
}

pub fn get_reduce_new_state_branch_list(top_state: State) -> State {
    match top_state {
        State::State086 => State::State112,
        _ => panic!("Impossible branch"),
    }
}

pub fn get_reduce_new_state_branch(top_state: State) -> State {
    match top_state {
        State::State086 => State::State113,
        State::State112 => State::State127,
        _ => panic!("Impossible branch"),
    }
}

pub fn get_reduce_new_state_expression_list_sc(top_state: State) -> State {
    match top_state {
        State::State051 => State::State068,
        _ => panic!("Impossible branch"),
    }
}

pub fn get_reduce_new_state_expression_list_c_ne(top_state: State) -> State {
    match top_state {
        State::State064 => State::State088,
        State::State118 => State::State088,
        State::State138 => State::State088,
        _ => panic!("Impossible branch"),
    }
}

pub fn get_reduce_new_state_expression_list_c(top_state: State) -> State {
    match top_state {
        State::State064 => State::State089,
        State::State118 => State::State131,
        State::State138 => State::State144,
        _ => panic!("Impossible branch"),
    }
}

pub fn get_reduce_new_state_expression_let(top_state: State) -> State {
    match top_state {
        State::State040 => State::State058,
        State::State123 => State::State135,
        State::State142 => State::State146,
        _ => panic!("Impossible branch"),
    }
}

pub fn get_reduce_new_state_expression(top_state: State) -> State {
    match top_state {
        State::State035 => State::State053,
        State::State039 => State::State055,
        State::State041 => State::State059,
        State::State042 => State::State060,
        State::State044 => State::State062,
        State::State049 => State::State065,
        State::State050 => State::State066,
        State::State051 => State::State069,
        State::State052 => State::State070,
        State::State063 => State::State087,
        State::State064 => State::State090,
        State::State068 => State::State093,
        State::State071 => State::State096,
        State::State072 => State::State097,
        State::State073 => State::State098,
        State::State074 => State::State099,
        State::State075 => State::State100,
        State::State076 => State::State101,
        State::State077 => State::State102,
        State::State080 => State::State105,
        State::State081 => State::State106,
        State::State085 => State::State110,
        State::State114 => State::State129,
        State::State118 => State::State090,
        State::State120 => State::State132,
        State::State121 => State::State133,
        State::State122 => State::State134,
        State::State138 => State::State090,
        State::State141 => State::State145,
        State::State143 => State::State147,
        _ => panic!("Impossible branch"),
    }
}
