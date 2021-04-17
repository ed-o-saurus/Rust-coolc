use std::collections::VecDeque;

use super::patterns::PatName;
use super::State;
use crate::token::Token;

// When passes the information about pattern match,
// Update tokens, working_str, and state_stack as appropriate

pub fn process(
    in_file_name: &str,
    line_no: i16,
    pat_name: PatName,
    lexeme: &str,
    state_stack: &mut Vec<State>,
    tokens: &mut VecDeque<Token>,
    working_str: &mut String,
) -> Result<(), String> {
    match pat_name {
        // In normal state, add a based on the pattern match token
        PatName::NormalAdd => tokens.push_back(Token::Add { line_no }),
        PatName::NormalDiv => tokens.push_back(Token::Div { line_no }),
        PatName::NormalSub => tokens.push_back(Token::Sub { line_no }),
        PatName::NormalMul => tokens.push_back(Token::Mul { line_no }),
        PatName::NormalEq => tokens.push_back(Token::Eq { line_no }),
        PatName::NormalLT => tokens.push_back(Token::LT { line_no }),
        PatName::NormalDot => tokens.push_back(Token::Dot { line_no }),
        PatName::NormalNeg => tokens.push_back(Token::Neg { line_no }),
        PatName::NormalComma => tokens.push_back(Token::Comma { line_no }),
        PatName::NormalSemiColon => tokens.push_back(Token::SemiColon { line_no }),
        PatName::NormalColon => tokens.push_back(Token::Colon { line_no }),
        PatName::NormalOpenParen => tokens.push_back(Token::OpenParen { line_no }),
        PatName::NormalCloseParen => tokens.push_back(Token::CloseParen { line_no }),
        PatName::NormalAt => tokens.push_back(Token::At { line_no }),
        PatName::NormalOpenBrace => tokens.push_back(Token::OpenBrace { line_no }),
        PatName::NormalCloseBrace => tokens.push_back(Token::CloseBrace { line_no }),
        PatName::NormalWhiteSpace => {} // Take no action for a white space
        PatName::NormalClass => tokens.push_back(Token::Class { line_no }),
        PatName::NormalElse => tokens.push_back(Token::Else { line_no }),
        PatName::NormalFi => tokens.push_back(Token::Fi { line_no }),
        PatName::NormalIf => tokens.push_back(Token::If { line_no }),
        PatName::NormalIn => tokens.push_back(Token::In { line_no }),
        PatName::NormalInherits => tokens.push_back(Token::Inherits { line_no }),
        PatName::NormalLet => tokens.push_back(Token::Let { line_no }),
        PatName::NormalLoop => tokens.push_back(Token::Loop { line_no }),
        PatName::NormalPool => tokens.push_back(Token::Pool { line_no }),
        PatName::NormalThen => tokens.push_back(Token::Then { line_no }),
        PatName::NormalWhile => tokens.push_back(Token::While { line_no }),
        PatName::NormalCase => tokens.push_back(Token::Case { line_no }),
        PatName::NormalEsac => tokens.push_back(Token::Esac { line_no }),
        PatName::NormalOf => tokens.push_back(Token::Of { line_no }),
        PatName::NormalNew => tokens.push_back(Token::New { line_no }),
        PatName::NormalIsVoid => tokens.push_back(Token::IsVoid { line_no }),
        PatName::NormalNot => tokens.push_back(Token::Not { line_no }),
        PatName::NormalDArrow => tokens.push_back(Token::DArrow { line_no }),
        PatName::NormalLE => tokens.push_back(Token::LEq { line_no }),
        PatName::NormalAssign => tokens.push_back(Token::Assign { line_no }),
        PatName::NormalTrue => tokens.push_back(Token::BoolConst { line_no, val: true }),
        PatName::NormalFalse => tokens.push_back(Token::BoolConst {
            line_no,
            val: false,
        }),
        PatName::NormalTypeID => tokens.push_back(Token::TypeID {
            line_no,
            type_name: lexeme.to_string(),
        }),
        PatName::NormalObjectID => tokens.push_back(Token::ObjectID {
            line_no,
            obj_name: lexeme.to_string(),
        }),
        PatName::NormalIntConst => match lexeme.parse::<u32>() {
            Ok(val) => tokens.push_back(Token::IntConst { line_no, val }),
            Err(_) => {
                return Err(
                    format!( // If the string of digits cannot be represented by a u32
                    "{} : {} - {} exceedes 32 bits",
                    in_file_name, line_no, lexeme
                ),
                );
            }
        },
        PatName::NormalDashComment => {} // Ignore comment
        PatName::NormalOpenComment => state_stack.push(State::Comment), // Enter Comment state
        PatName::NormalCloseComment => {
            // Error if there is a close comment without an open comment
            return Err(format!("{} : {} - Unmatched *)", in_file_name, line_no));
        }
        PatName::NormalQuote => {
            // Begin a string
            state_stack.push(State::Quote); // Enter Quote state
            working_str.clear();
        }
        PatName::NormalBadChar => {
            // Any other unrecognized character is an error
            return Err(format!(
                "{} : {} -  Unexpected character : '{}'",
                in_file_name, line_no, lexeme
            ));
        }

        PatName::CommentOpenComment => state_stack.push(State::Comment), // (* .. *) comments can be nested
        PatName::CommentCloseComment => {
            // Exit the current state if the comment ends
            state_stack.pop();
        }
        PatName::CommentChar => {} // Ignore all other characters

        PatName::QuoteNull | PatName::QuoteEscNull => {
            // NUL characters are not allowed in quotes
            return Err(format!(
                "{} : {} - String contains null character.",
                in_file_name, line_no
            ));
        }
        PatName::QuoteQuote => {
            // When the quote ends,
            state_stack.pop(); // exit the quote state
            tokens.push_back(Token::StrConst {
                // Add a string constant
                line_no,
                val: working_str.clone(),
            })
        }
        PatName::QuoteChar => working_str.push_str(lexeme), // Add character to working string
        PatName::QuoteEscNewLine => working_str.push('\n'), // Add new line
        PatName::QuoteBackSpace => working_str.push('\x08'), // Add backspace
        PatName::QuoteTab => working_str.push('\t'),        // Add tab
        PatName::QuoteFormFeed => working_str.push('\x0c'), // Add formfeed
        PatName::QuoteEscChar => working_str.push_str(&lexeme[1..2]), // Any other escaped character
        PatName::QuoteNewLine => {
            // In case of an unescaped new line - raise an error
            state_stack.pop();
            return Err(format!(
                "{} : {} - Unterminated string constant",
                in_file_name, line_no
            ));
        }
    };

    Ok(())
}
