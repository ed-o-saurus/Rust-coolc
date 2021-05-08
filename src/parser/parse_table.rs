use crate::token::Token;

// This module contains information about the behaviour of the shift-reduce parse used to parse the token stream into an AST.

// The state component of the stack
#[derive(Copy, Clone)]
pub enum State {
    StateInitial, // Initial State matched with the Bottom Node
    State001,
    State002,
    State003,
    State004,
    State005,
    State006,
    State007,
    State008,
    State009,
    State010,
    State011,
    State012,
    State013,
    State014,
    State015,
    State016,
    State017,
    State018,
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
    State056,
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
    State067,
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
    State082,
    State083,
    State084,
    State085,
    State086,
    State087,
    State088,
    State089,
    State090,
    State091,
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
    State107,
    State108,
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
}

pub enum DispatchType {
    OnSelf,
    OnExpr,
    Static,
}

// Action to be carried out by the parse as determined by the parse table
pub enum Action {
    Shift { new_state: State },
    ReduceClassList { is_empty: bool },
    ReduceClass { has_parent: bool },
    ReduceFeatureList { is_empty: bool },
    ReduceMethod,
    ReduceAttr { has_init: bool },
    ReduceFormalListNE { is_empty: bool },
    ReduceFormalList { is_empty: bool },
    ReduceFormal,
    ReduceBranchList { is_empty: bool },
    ReduceBranch,
    ReduceExprListSC { is_empty: bool },
    ReduceExprListCNE { is_empty: bool },
    ReduceExprListC { is_empty: bool },
    ReduceExprLet { has_init: bool },
    ReduceAssign,
    ReduceDispatch { dispatch_type: DispatchType },
    ReduceCond,
    ReduceWhile,
    ReduceBlock,
    ReduceLet,
    ReduceTypeCase,
    ReduceNew,
    ReduceIsVoid,
    ReduceArith,
    ReduceNeg,
    ReduceComp,
    ReduceEq,
    ReduceNot,
    ReduceParen,
    ReduceVarByName,
    ReduceIntConst,
    ReduceStrConst,
    ReduceBoolConst,
    Accept,
}

// What to do (shift, reduce, or accept based on the next token and the top parse state
pub fn get_action(top_state: State, next_token: &Token) -> Result<Action, i16> {
    let next_line_no = next_token.get_line_no();

    Ok(match top_state {
        State::StateInitial => match next_token {
            Token::Class { .. } => Action::Shift {
                new_state: State::State001,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State001 => match next_token {
            Token::TypeID { .. } => Action::Shift {
                new_state: State::State004,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State002 => match next_token {
            Token::Class { .. } => Action::Shift {
                new_state: State::State001,
            },
            Token::End { .. } => Action::Accept,
            _ => {
                return Err(next_line_no);
            }
        },

        State::State003 => match next_token {
            Token::SemiColon { .. } => Action::Shift {
                new_state: State::State006,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State004 => match next_token {
            Token::Inherits { .. } => Action::Shift {
                new_state: State::State007,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State008,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State005 => match next_token {
            Token::SemiColon { .. } => Action::Shift {
                new_state: State::State009,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State006 => Action::ReduceClassList { is_empty: true },

        State::State007 => match next_token {
            Token::TypeID { .. } => Action::Shift {
                new_state: State::State010,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State008 => match next_token {
            Token::ObjectID { .. } => Action::ReduceFeatureList { is_empty: true },
            Token::CloseBrace { .. } => Action::ReduceFeatureList { is_empty: true },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State009 => Action::ReduceClassList { is_empty: false },

        State::State010 => match next_token {
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State012,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State011 => match next_token {
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State013,
            },
            Token::CloseBrace { .. } => Action::Shift {
                new_state: State::State014,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State012 => match next_token {
            Token::ObjectID { .. } => Action::ReduceFeatureList { is_empty: true },
            Token::CloseBrace { .. } => Action::ReduceFeatureList { is_empty: true },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State013 => match next_token {
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State017,
            },
            Token::Colon { .. } => Action::Shift {
                new_state: State::State018,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State014 => Action::ReduceClass { has_parent: false },

        State::State015 => match next_token {
            Token::SemiColon { .. } => Action::Shift {
                new_state: State::State019,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State016 => match next_token {
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State013,
            },
            Token::CloseBrace { .. } => Action::Shift {
                new_state: State::State020,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State017 => match next_token {
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State021,
            },
            _ => Action::ReduceFormalList { is_empty: true },
        },

        State::State018 => match next_token {
            Token::TypeID { .. } => Action::Shift {
                new_state: State::State025,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State019 => Action::ReduceFeatureList { is_empty: false },

        State::State020 => Action::ReduceClass { has_parent: true },

        State::State021 => match next_token {
            Token::Colon { .. } => Action::Shift {
                new_state: State::State026,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State022 => match next_token {
            Token::Comma { .. } => Action::Shift {
                new_state: State::State027,
            },
            _ => Action::ReduceFormalList { is_empty: false },
        },

        State::State023 => match next_token {
            Token::CloseParen { .. } => Action::Shift {
                new_state: State::State028,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State024 => Action::ReduceFormalListNE { is_empty: true },

        State::State025 => match next_token {
            Token::Assign { .. } => Action::Shift {
                new_state: State::State029,
            },
            _ => Action::ReduceAttr { has_init: false },
        },

        State::State026 => match next_token {
            Token::TypeID { .. } => Action::Shift {
                new_state: State::State030,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State027 => match next_token {
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State021,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State028 => match next_token {
            Token::Colon { .. } => Action::Shift {
                new_state: State::State032,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State029 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State033,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State034,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State035,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State036,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State037,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State038,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State046,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State030 => Action::ReduceFormal,

        State::State031 => Action::ReduceFormalListNE { is_empty: false },

        State::State032 => match next_token {
            Token::TypeID { .. } => Action::Shift {
                new_state: State::State048,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State033 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State033,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State034,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State035,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State036,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State037,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State038,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State046,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State034 => match next_token {
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State050,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State035 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State033,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State034,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State035,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State036,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State037,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State038,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State046,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State036 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State033,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State034,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State035,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State036,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State037,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State038,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State046,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State037 => match next_token {
            Token::TypeID { .. } => Action::Shift {
                new_state: State::State054,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State038 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State033,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State034,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State035,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State036,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State037,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State038,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State046,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State039 => Action::ReduceStrConst,

        State::State040 => Action::ReduceIntConst,

        State::State041 => Action::ReduceBoolConst,

        State::State042 => match next_token {
            Token::Assign { .. } => Action::Shift {
                new_state: State::State056,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State057,
            },
            _ => Action::ReduceVarByName,
        },

        State::State043 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State033,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State034,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State035,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State036,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State037,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State038,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State046,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State044 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State033,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State034,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State035,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State036,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State037,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State038,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State046,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State045 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State033,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State034,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State035,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State036,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State037,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State038,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State046,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State046 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State033,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State034,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State035,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State036,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State037,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State038,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State046,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State047 => match next_token {
            Token::LEq { .. } => Action::Shift {
                new_state: State::State063,
            },
            Token::LT { .. } => Action::Shift {
                new_state: State::State064,
            },
            Token::Eq { .. } => Action::Shift {
                new_state: State::State065,
            },
            Token::Add { .. } => Action::Shift {
                new_state: State::State066,
            },
            Token::Sub { .. } => Action::Shift {
                new_state: State::State067,
            },
            Token::Mul { .. } => Action::Shift {
                new_state: State::State068,
            },
            Token::Div { .. } => Action::Shift {
                new_state: State::State069,
            },
            Token::At { .. } => Action::Shift {
                new_state: State::State070,
            },
            Token::Dot { .. } => Action::Shift {
                new_state: State::State071,
            },
            _ => Action::ReduceAttr { has_init: true },
        },

        State::State048 => match next_token {
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State072,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State049 => match next_token {
            Token::Then { .. } => Action::Shift {
                new_state: State::State073,
            },
            Token::LEq { .. } => Action::Shift {
                new_state: State::State063,
            },
            Token::LT { .. } => Action::Shift {
                new_state: State::State064,
            },
            Token::Eq { .. } => Action::Shift {
                new_state: State::State065,
            },
            Token::Add { .. } => Action::Shift {
                new_state: State::State066,
            },
            Token::Sub { .. } => Action::Shift {
                new_state: State::State067,
            },
            Token::Mul { .. } => Action::Shift {
                new_state: State::State068,
            },
            Token::Div { .. } => Action::Shift {
                new_state: State::State069,
            },
            Token::At { .. } => Action::Shift {
                new_state: State::State070,
            },
            Token::Dot { .. } => Action::Shift {
                new_state: State::State071,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State050 => match next_token {
            Token::Colon { .. } => Action::Shift {
                new_state: State::State074,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State051 => Action::ReduceLet,

        State::State052 => match next_token {
            Token::Loop { .. } => Action::Shift {
                new_state: State::State075,
            },
            Token::LEq { .. } => Action::Shift {
                new_state: State::State063,
            },
            Token::LT { .. } => Action::Shift {
                new_state: State::State064,
            },
            Token::Eq { .. } => Action::Shift {
                new_state: State::State065,
            },
            Token::Add { .. } => Action::Shift {
                new_state: State::State066,
            },
            Token::Sub { .. } => Action::Shift {
                new_state: State::State067,
            },
            Token::Mul { .. } => Action::Shift {
                new_state: State::State068,
            },
            Token::Div { .. } => Action::Shift {
                new_state: State::State069,
            },
            Token::At { .. } => Action::Shift {
                new_state: State::State070,
            },
            Token::Dot { .. } => Action::Shift {
                new_state: State::State071,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State053 => match next_token {
            Token::Of { .. } => Action::Shift {
                new_state: State::State076,
            },
            Token::LEq { .. } => Action::Shift {
                new_state: State::State063,
            },
            Token::LT { .. } => Action::Shift {
                new_state: State::State064,
            },
            Token::Eq { .. } => Action::Shift {
                new_state: State::State065,
            },
            Token::Add { .. } => Action::Shift {
                new_state: State::State066,
            },
            Token::Sub { .. } => Action::Shift {
                new_state: State::State067,
            },
            Token::Mul { .. } => Action::Shift {
                new_state: State::State068,
            },
            Token::Div { .. } => Action::Shift {
                new_state: State::State069,
            },
            Token::At { .. } => Action::Shift {
                new_state: State::State070,
            },
            Token::Dot { .. } => Action::Shift {
                new_state: State::State071,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State054 => Action::ReduceNew,

        State::State055 => match next_token {
            Token::At { .. } => Action::Shift {
                new_state: State::State070,
            },
            Token::Dot { .. } => Action::Shift {
                new_state: State::State071,
            },
            _ => Action::ReduceIsVoid,
        },

        State::State056 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State033,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State034,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State035,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State036,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State037,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State038,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State046,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State057 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State033,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State034,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State035,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State036,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State037,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State038,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State046,
            },
            _ => Action::ReduceExprListC { is_empty: true },
        },

        State::State058 => match next_token {
            Token::LEq { .. } => Action::Shift {
                new_state: State::State063,
            },
            Token::LT { .. } => Action::Shift {
                new_state: State::State064,
            },
            Token::Eq { .. } => Action::Shift {
                new_state: State::State065,
            },
            Token::Add { .. } => Action::Shift {
                new_state: State::State066,
            },
            Token::Sub { .. } => Action::Shift {
                new_state: State::State067,
            },
            Token::Mul { .. } => Action::Shift {
                new_state: State::State068,
            },
            Token::Div { .. } => Action::Shift {
                new_state: State::State069,
            },
            Token::At { .. } => Action::Shift {
                new_state: State::State070,
            },
            Token::Dot { .. } => Action::Shift {
                new_state: State::State071,
            },
            _ => Action::ReduceNot,
        },

        State::State059 => match next_token {
            Token::At { .. } => Action::Shift {
                new_state: State::State070,
            },
            Token::Dot { .. } => Action::Shift {
                new_state: State::State071,
            },
            _ => Action::ReduceNeg,
        },

        State::State060 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State033,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State034,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State035,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State036,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State037,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State038,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::CloseBrace { .. } => Action::Shift {
                new_state: State::State081,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State046,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State061 => match next_token {
            Token::LEq { .. } => Action::Shift {
                new_state: State::State063,
            },
            Token::LT { .. } => Action::Shift {
                new_state: State::State064,
            },
            Token::Eq { .. } => Action::Shift {
                new_state: State::State065,
            },
            Token::Add { .. } => Action::Shift {
                new_state: State::State066,
            },
            Token::Sub { .. } => Action::Shift {
                new_state: State::State067,
            },
            Token::Mul { .. } => Action::Shift {
                new_state: State::State068,
            },
            Token::Div { .. } => Action::Shift {
                new_state: State::State069,
            },
            Token::At { .. } => Action::Shift {
                new_state: State::State070,
            },
            Token::Dot { .. } => Action::Shift {
                new_state: State::State071,
            },
            Token::SemiColon { .. } => Action::Shift {
                new_state: State::State083,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State062 => match next_token {
            Token::LEq { .. } => Action::Shift {
                new_state: State::State063,
            },
            Token::LT { .. } => Action::Shift {
                new_state: State::State064,
            },
            Token::Eq { .. } => Action::Shift {
                new_state: State::State065,
            },
            Token::Add { .. } => Action::Shift {
                new_state: State::State066,
            },
            Token::Sub { .. } => Action::Shift {
                new_state: State::State067,
            },
            Token::Mul { .. } => Action::Shift {
                new_state: State::State068,
            },
            Token::Div { .. } => Action::Shift {
                new_state: State::State069,
            },
            Token::At { .. } => Action::Shift {
                new_state: State::State070,
            },
            Token::Dot { .. } => Action::Shift {
                new_state: State::State071,
            },
            Token::CloseParen { .. } => Action::Shift {
                new_state: State::State084,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State063 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State033,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State034,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State035,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State036,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State037,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State038,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State046,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State064 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State033,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State034,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State035,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State036,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State037,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State038,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State046,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State065 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State033,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State034,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State035,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State036,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State037,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State038,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State046,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State066 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State033,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State034,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State035,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State036,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State037,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State038,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State046,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State067 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State033,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State034,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State035,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State036,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State037,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State038,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State046,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State068 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State033,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State034,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State035,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State036,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State037,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State038,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State046,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State069 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State033,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State034,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State035,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State036,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State037,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State038,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State046,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State070 => match next_token {
            Token::TypeID { .. } => Action::Shift {
                new_state: State::State092,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State071 => match next_token {
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State093,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State072 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State033,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State034,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State035,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State036,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State037,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State038,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State046,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State073 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State033,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State034,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State035,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State036,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State037,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State038,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State046,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State074 => match next_token {
            Token::TypeID { .. } => Action::Shift {
                new_state: State::State096,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State075 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State033,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State034,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State035,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State036,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State037,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State038,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State046,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State076 => match next_token {
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State098,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State077 => match next_token {
            Token::LEq { .. } => Action::Shift {
                new_state: State::State063,
            },
            Token::LT { .. } => Action::Shift {
                new_state: State::State064,
            },
            Token::Eq { .. } => Action::Shift {
                new_state: State::State065,
            },
            Token::Add { .. } => Action::Shift {
                new_state: State::State066,
            },
            Token::Sub { .. } => Action::Shift {
                new_state: State::State067,
            },
            Token::Mul { .. } => Action::Shift {
                new_state: State::State068,
            },
            Token::Div { .. } => Action::Shift {
                new_state: State::State069,
            },
            Token::At { .. } => Action::Shift {
                new_state: State::State070,
            },
            Token::Dot { .. } => Action::Shift {
                new_state: State::State071,
            },
            _ => Action::ReduceAssign,
        },

        State::State078 => match next_token {
            Token::Comma { .. } => Action::Shift {
                new_state: State::State101,
            },
            _ => Action::ReduceExprListC { is_empty: false },
        },

        State::State079 => match next_token {
            Token::CloseParen { .. } => Action::Shift {
                new_state: State::State102,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State080 => match next_token {
            Token::LEq { .. } => Action::Shift {
                new_state: State::State063,
            },
            Token::LT { .. } => Action::Shift {
                new_state: State::State064,
            },
            Token::Eq { .. } => Action::Shift {
                new_state: State::State065,
            },
            Token::Add { .. } => Action::Shift {
                new_state: State::State066,
            },
            Token::Sub { .. } => Action::Shift {
                new_state: State::State067,
            },
            Token::Mul { .. } => Action::Shift {
                new_state: State::State068,
            },
            Token::Div { .. } => Action::Shift {
                new_state: State::State069,
            },
            Token::At { .. } => Action::Shift {
                new_state: State::State070,
            },
            Token::Dot { .. } => Action::Shift {
                new_state: State::State071,
            },
            _ => Action::ReduceExprListCNE { is_empty: true },
        },

        State::State081 => Action::ReduceBlock,

        State::State082 => match next_token {
            Token::LEq { .. } => Action::Shift {
                new_state: State::State063,
            },
            Token::LT { .. } => Action::Shift {
                new_state: State::State064,
            },
            Token::Eq { .. } => Action::Shift {
                new_state: State::State065,
            },
            Token::Add { .. } => Action::Shift {
                new_state: State::State066,
            },
            Token::Sub { .. } => Action::Shift {
                new_state: State::State067,
            },
            Token::Mul { .. } => Action::Shift {
                new_state: State::State068,
            },
            Token::Div { .. } => Action::Shift {
                new_state: State::State069,
            },
            Token::At { .. } => Action::Shift {
                new_state: State::State070,
            },
            Token::Dot { .. } => Action::Shift {
                new_state: State::State071,
            },
            Token::SemiColon { .. } => Action::Shift {
                new_state: State::State103,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State083 => Action::ReduceExprListSC { is_empty: true },

        State::State084 => Action::ReduceParen,

        State::State085 => match next_token {
            Token::Add { .. } => Action::Shift {
                new_state: State::State066,
            },
            Token::Sub { .. } => Action::Shift {
                new_state: State::State067,
            },
            Token::Mul { .. } => Action::Shift {
                new_state: State::State068,
            },
            Token::Div { .. } => Action::Shift {
                new_state: State::State069,
            },
            Token::At { .. } => Action::Shift {
                new_state: State::State070,
            },
            Token::Dot { .. } => Action::Shift {
                new_state: State::State071,
            },
            _ => Action::ReduceComp,
        },

        State::State086 => match next_token {
            Token::Add { .. } => Action::Shift {
                new_state: State::State066,
            },
            Token::Sub { .. } => Action::Shift {
                new_state: State::State067,
            },
            Token::Mul { .. } => Action::Shift {
                new_state: State::State068,
            },
            Token::Div { .. } => Action::Shift {
                new_state: State::State069,
            },
            Token::At { .. } => Action::Shift {
                new_state: State::State070,
            },
            Token::Dot { .. } => Action::Shift {
                new_state: State::State071,
            },
            _ => Action::ReduceComp,
        },

        State::State087 => match next_token {
            Token::Add { .. } => Action::Shift {
                new_state: State::State066,
            },
            Token::Sub { .. } => Action::Shift {
                new_state: State::State067,
            },
            Token::Mul { .. } => Action::Shift {
                new_state: State::State068,
            },
            Token::Div { .. } => Action::Shift {
                new_state: State::State069,
            },
            Token::At { .. } => Action::Shift {
                new_state: State::State070,
            },
            Token::Dot { .. } => Action::Shift {
                new_state: State::State071,
            },
            _ => Action::ReduceEq,
        },

        State::State088 => match next_token {
            Token::Mul { .. } => Action::Shift {
                new_state: State::State068,
            },
            Token::Div { .. } => Action::Shift {
                new_state: State::State069,
            },
            Token::At { .. } => Action::Shift {
                new_state: State::State070,
            },
            Token::Dot { .. } => Action::Shift {
                new_state: State::State071,
            },
            _ => Action::ReduceArith,
        },

        State::State089 => match next_token {
            Token::Mul { .. } => Action::Shift {
                new_state: State::State068,
            },
            Token::Div { .. } => Action::Shift {
                new_state: State::State069,
            },
            Token::At { .. } => Action::Shift {
                new_state: State::State070,
            },
            Token::Dot { .. } => Action::Shift {
                new_state: State::State071,
            },
            _ => Action::ReduceArith,
        },

        State::State090 => match next_token {
            Token::At { .. } => Action::Shift {
                new_state: State::State070,
            },
            Token::Dot { .. } => Action::Shift {
                new_state: State::State071,
            },
            _ => Action::ReduceArith,
        },

        State::State091 => match next_token {
            Token::At { .. } => Action::Shift {
                new_state: State::State070,
            },
            Token::Dot { .. } => Action::Shift {
                new_state: State::State071,
            },
            _ => Action::ReduceArith,
        },

        State::State092 => match next_token {
            Token::Dot { .. } => Action::Shift {
                new_state: State::State104,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State093 => match next_token {
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State105,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State094 => match next_token {
            Token::LEq { .. } => Action::Shift {
                new_state: State::State063,
            },
            Token::LT { .. } => Action::Shift {
                new_state: State::State064,
            },
            Token::Eq { .. } => Action::Shift {
                new_state: State::State065,
            },
            Token::Add { .. } => Action::Shift {
                new_state: State::State066,
            },
            Token::Sub { .. } => Action::Shift {
                new_state: State::State067,
            },
            Token::Mul { .. } => Action::Shift {
                new_state: State::State068,
            },
            Token::Div { .. } => Action::Shift {
                new_state: State::State069,
            },
            Token::At { .. } => Action::Shift {
                new_state: State::State070,
            },
            Token::Dot { .. } => Action::Shift {
                new_state: State::State071,
            },
            Token::CloseBrace { .. } => Action::Shift {
                new_state: State::State106,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State095 => match next_token {
            Token::Else { .. } => Action::Shift {
                new_state: State::State107,
            },
            Token::LEq { .. } => Action::Shift {
                new_state: State::State063,
            },
            Token::LT { .. } => Action::Shift {
                new_state: State::State064,
            },
            Token::Eq { .. } => Action::Shift {
                new_state: State::State065,
            },
            Token::Add { .. } => Action::Shift {
                new_state: State::State066,
            },
            Token::Sub { .. } => Action::Shift {
                new_state: State::State067,
            },
            Token::Mul { .. } => Action::Shift {
                new_state: State::State068,
            },
            Token::Div { .. } => Action::Shift {
                new_state: State::State069,
            },
            Token::At { .. } => Action::Shift {
                new_state: State::State070,
            },
            Token::Dot { .. } => Action::Shift {
                new_state: State::State071,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State096 => match next_token {
            Token::In { .. } => Action::Shift {
                new_state: State::State108,
            },
            Token::Assign { .. } => Action::Shift {
                new_state: State::State109,
            },
            Token::Comma { .. } => Action::Shift {
                new_state: State::State110,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State097 => match next_token {
            Token::Pool { .. } => Action::Shift {
                new_state: State::State111,
            },
            Token::LEq { .. } => Action::Shift {
                new_state: State::State063,
            },
            Token::LT { .. } => Action::Shift {
                new_state: State::State064,
            },
            Token::Eq { .. } => Action::Shift {
                new_state: State::State065,
            },
            Token::Add { .. } => Action::Shift {
                new_state: State::State066,
            },
            Token::Sub { .. } => Action::Shift {
                new_state: State::State067,
            },
            Token::Mul { .. } => Action::Shift {
                new_state: State::State068,
            },
            Token::Div { .. } => Action::Shift {
                new_state: State::State069,
            },
            Token::At { .. } => Action::Shift {
                new_state: State::State070,
            },
            Token::Dot { .. } => Action::Shift {
                new_state: State::State071,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State098 => match next_token {
            Token::Colon { .. } => Action::Shift {
                new_state: State::State112,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State099 => match next_token {
            Token::Esac { .. } => Action::Shift {
                new_state: State::State113,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State098,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State100 => match next_token {
            Token::SemiColon { .. } => Action::Shift {
                new_state: State::State115,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State101 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State033,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State034,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State035,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State036,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State037,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State038,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State046,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State102 => Action::ReduceDispatch {
            dispatch_type: DispatchType::OnSelf,
        },

        State::State103 => Action::ReduceExprListSC { is_empty: false },

        State::State104 => match next_token {
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State117,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State105 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State033,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State034,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State035,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State036,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State037,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State038,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State046,
            },
            _ => Action::ReduceExprListC { is_empty: true },
        },

        State::State106 => Action::ReduceMethod,

        State::State107 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State033,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State034,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State035,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State036,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State037,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State038,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State046,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State108 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State033,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State034,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State035,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State036,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State037,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State038,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State046,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State109 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State033,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State034,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State035,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State036,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State037,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State038,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State046,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State110 => match next_token {
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State050,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State111 => Action::ReduceWhile,

        State::State112 => match next_token {
            Token::TypeID { .. } => Action::Shift {
                new_state: State::State123,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State113 => Action::ReduceTypeCase,

        State::State114 => match next_token {
            Token::SemiColon { .. } => Action::Shift {
                new_state: State::State124,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State115 => Action::ReduceBranchList { is_empty: true },

        State::State116 => match next_token {
            Token::LEq { .. } => Action::Shift {
                new_state: State::State063,
            },
            Token::LT { .. } => Action::Shift {
                new_state: State::State064,
            },
            Token::Eq { .. } => Action::Shift {
                new_state: State::State065,
            },
            Token::Add { .. } => Action::Shift {
                new_state: State::State066,
            },
            Token::Sub { .. } => Action::Shift {
                new_state: State::State067,
            },
            Token::Mul { .. } => Action::Shift {
                new_state: State::State068,
            },
            Token::Div { .. } => Action::Shift {
                new_state: State::State069,
            },
            Token::At { .. } => Action::Shift {
                new_state: State::State070,
            },
            Token::Dot { .. } => Action::Shift {
                new_state: State::State071,
            },
            _ => Action::ReduceExprListCNE { is_empty: false },
        },

        State::State117 => match next_token {
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State125,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State118 => match next_token {
            Token::CloseParen { .. } => Action::Shift {
                new_state: State::State126,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State119 => match next_token {
            Token::Fi { .. } => Action::Shift {
                new_state: State::State127,
            },
            Token::LEq { .. } => Action::Shift {
                new_state: State::State063,
            },
            Token::LT { .. } => Action::Shift {
                new_state: State::State064,
            },
            Token::Eq { .. } => Action::Shift {
                new_state: State::State065,
            },
            Token::Add { .. } => Action::Shift {
                new_state: State::State066,
            },
            Token::Sub { .. } => Action::Shift {
                new_state: State::State067,
            },
            Token::Mul { .. } => Action::Shift {
                new_state: State::State068,
            },
            Token::Div { .. } => Action::Shift {
                new_state: State::State069,
            },
            Token::At { .. } => Action::Shift {
                new_state: State::State070,
            },
            Token::Dot { .. } => Action::Shift {
                new_state: State::State071,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State120 => match next_token {
            Token::LEq { .. } => Action::Shift {
                new_state: State::State063,
            },
            Token::LT { .. } => Action::Shift {
                new_state: State::State064,
            },
            Token::Eq { .. } => Action::Shift {
                new_state: State::State065,
            },
            Token::Add { .. } => Action::Shift {
                new_state: State::State066,
            },
            Token::Sub { .. } => Action::Shift {
                new_state: State::State067,
            },
            Token::Mul { .. } => Action::Shift {
                new_state: State::State068,
            },
            Token::Div { .. } => Action::Shift {
                new_state: State::State069,
            },
            Token::At { .. } => Action::Shift {
                new_state: State::State070,
            },
            Token::Dot { .. } => Action::Shift {
                new_state: State::State071,
            },
            _ => Action::ReduceExprLet { has_init: false },
        },

        State::State121 => match next_token {
            Token::In { .. } => Action::Shift {
                new_state: State::State128,
            },
            Token::LEq { .. } => Action::Shift {
                new_state: State::State063,
            },
            Token::LT { .. } => Action::Shift {
                new_state: State::State064,
            },
            Token::Eq { .. } => Action::Shift {
                new_state: State::State065,
            },
            Token::Add { .. } => Action::Shift {
                new_state: State::State066,
            },
            Token::Sub { .. } => Action::Shift {
                new_state: State::State067,
            },
            Token::Mul { .. } => Action::Shift {
                new_state: State::State068,
            },
            Token::Div { .. } => Action::Shift {
                new_state: State::State069,
            },
            Token::At { .. } => Action::Shift {
                new_state: State::State070,
            },
            Token::Dot { .. } => Action::Shift {
                new_state: State::State071,
            },
            Token::Comma { .. } => Action::Shift {
                new_state: State::State129,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State122 => Action::ReduceExprLet { has_init: false },

        State::State123 => match next_token {
            Token::DArrow { .. } => Action::Shift {
                new_state: State::State130,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State124 => Action::ReduceBranchList { is_empty: false },

        State::State125 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State033,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State034,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State035,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State036,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State037,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State038,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State046,
            },
            _ => Action::ReduceExprListC { is_empty: true },
        },

        State::State126 => Action::ReduceDispatch {
            dispatch_type: DispatchType::OnExpr,
        },

        State::State127 => Action::ReduceCond,

        State::State128 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State033,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State034,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State035,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State036,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State037,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State038,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State046,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State129 => match next_token {
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State050,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State130 => match next_token {
            Token::If { .. } => Action::Shift {
                new_state: State::State033,
            },
            Token::Let { .. } => Action::Shift {
                new_state: State::State034,
            },
            Token::While { .. } => Action::Shift {
                new_state: State::State035,
            },
            Token::Case { .. } => Action::Shift {
                new_state: State::State036,
            },
            Token::New { .. } => Action::Shift {
                new_state: State::State037,
            },
            Token::IsVoid { .. } => Action::Shift {
                new_state: State::State038,
            },
            Token::StrConst { .. } => Action::Shift {
                new_state: State::State039,
            },
            Token::IntConst { .. } => Action::Shift {
                new_state: State::State040,
            },
            Token::BoolConst { .. } => Action::Shift {
                new_state: State::State041,
            },
            Token::ObjectID { .. } => Action::Shift {
                new_state: State::State042,
            },
            Token::Not { .. } => Action::Shift {
                new_state: State::State043,
            },
            Token::Neg { .. } => Action::Shift {
                new_state: State::State044,
            },
            Token::OpenBrace { .. } => Action::Shift {
                new_state: State::State045,
            },
            Token::OpenParen { .. } => Action::Shift {
                new_state: State::State046,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State131 => match next_token {
            Token::CloseParen { .. } => Action::Shift {
                new_state: State::State135,
            },
            _ => {
                return Err(next_line_no);
            }
        },

        State::State132 => match next_token {
            Token::LEq { .. } => Action::Shift {
                new_state: State::State063,
            },
            Token::LT { .. } => Action::Shift {
                new_state: State::State064,
            },
            Token::Eq { .. } => Action::Shift {
                new_state: State::State065,
            },
            Token::Add { .. } => Action::Shift {
                new_state: State::State066,
            },
            Token::Sub { .. } => Action::Shift {
                new_state: State::State067,
            },
            Token::Mul { .. } => Action::Shift {
                new_state: State::State068,
            },
            Token::Div { .. } => Action::Shift {
                new_state: State::State069,
            },
            Token::At { .. } => Action::Shift {
                new_state: State::State070,
            },
            Token::Dot { .. } => Action::Shift {
                new_state: State::State071,
            },
            _ => Action::ReduceExprLet { has_init: true },
        },

        State::State133 => Action::ReduceExprLet { has_init: true },

        State::State134 => match next_token {
            Token::LEq { .. } => Action::Shift {
                new_state: State::State063,
            },
            Token::LT { .. } => Action::Shift {
                new_state: State::State064,
            },
            Token::Eq { .. } => Action::Shift {
                new_state: State::State065,
            },
            Token::Add { .. } => Action::Shift {
                new_state: State::State066,
            },
            Token::Sub { .. } => Action::Shift {
                new_state: State::State067,
            },
            Token::Mul { .. } => Action::Shift {
                new_state: State::State068,
            },
            Token::Div { .. } => Action::Shift {
                new_state: State::State069,
            },
            Token::At { .. } => Action::Shift {
                new_state: State::State070,
            },
            Token::Dot { .. } => Action::Shift {
                new_state: State::State071,
            },
            _ => Action::ReduceBranch,
        },

        State::State135 => Action::ReduceDispatch {
            dispatch_type: DispatchType::Static,
        },
    })
}

// These functions determine the state that a new non-terminal node should have on the parse stack.

pub fn get_reduce_new_state_class_list(top_state: State) -> State {
    match top_state {
        State::StateInitial => State::State002,
        _ => panic!("Impossible branch"),
    }
}

pub fn get_reduce_new_state_class(top_state: State) -> State {
    match top_state {
        State::StateInitial => State::State003,
        State::State002 => State::State005,
        _ => panic!("Impossible branch"),
    }
}

pub fn get_reduce_new_state_feature_list(top_state: State) -> State {
    match top_state {
        State::State008 => State::State011,
        State::State012 => State::State016,
        _ => panic!("Impossible branch"),
    }
}

pub fn get_reduce_new_state_feature(top_state: State) -> State {
    match top_state {
        State::State011 => State::State015,
        State::State016 => State::State015,
        _ => panic!("Impossible branch"),
    }
}

pub fn get_reduce_new_state_formal_list_ne(top_state: State) -> State {
    match top_state {
        State::State017 => State::State022,
        _ => panic!("Impossible branch"),
    }
}

pub fn get_reduce_new_state_formal_list(top_state: State) -> State {
    match top_state {
        State::State017 => State::State023,
        _ => panic!("Impossible branch"),
    }
}

pub fn get_reduce_new_state_formal(top_state: State) -> State {
    match top_state {
        State::State017 => State::State024,
        State::State027 => State::State031,
        _ => panic!("Impossible branch"),
    }
}

pub fn get_reduce_new_state_branch_list(top_state: State) -> State {
    match top_state {
        State::State076 => State::State099,
        _ => panic!("Impossible branch"),
    }
}

pub fn get_reduce_new_state_branch(top_state: State) -> State {
    match top_state {
        State::State076 => State::State100,
        State::State099 => State::State114,
        _ => panic!("Impossible branch"),
    }
}

pub fn get_reduce_new_state_expression_list_sc(top_state: State) -> State {
    match top_state {
        State::State045 => State::State060,
        _ => panic!("Impossible branch"),
    }
}

pub fn get_reduce_new_state_expression_list_c_ne(top_state: State) -> State {
    match top_state {
        State::State057 => State::State078,
        State::State105 => State::State078,
        State::State125 => State::State078,
        _ => panic!("Impossible branch"),
    }
}

pub fn get_reduce_new_state_expression_list_c(top_state: State) -> State {
    match top_state {
        State::State057 => State::State079,
        State::State105 => State::State118,
        State::State125 => State::State131,
        _ => panic!("Impossible branch"),
    }
}

pub fn get_reduce_new_state_expression_let(top_state: State) -> State {
    match top_state {
        State::State034 => State::State051,
        State::State110 => State::State122,
        State::State129 => State::State133,
        _ => panic!("Impossible branch"),
    }
}

pub fn get_reduce_new_state_expression(top_state: State) -> State {
    match top_state {
        State::State029 => State::State047,
        State::State033 => State::State049,
        State::State035 => State::State052,
        State::State036 => State::State053,
        State::State038 => State::State055,
        State::State043 => State::State058,
        State::State044 => State::State059,
        State::State045 => State::State061,
        State::State046 => State::State062,
        State::State056 => State::State077,
        State::State057 => State::State080,
        State::State060 => State::State082,
        State::State063 => State::State085,
        State::State064 => State::State086,
        State::State065 => State::State087,
        State::State066 => State::State088,
        State::State067 => State::State089,
        State::State068 => State::State090,
        State::State069 => State::State091,
        State::State072 => State::State094,
        State::State073 => State::State095,
        State::State075 => State::State097,
        State::State101 => State::State116,
        State::State105 => State::State080,
        State::State107 => State::State119,
        State::State108 => State::State120,
        State::State109 => State::State121,
        State::State125 => State::State080,
        State::State128 => State::State132,
        State::State130 => State::State134,
        _ => panic!("Impossible branch"),
    }
}
