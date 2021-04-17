#![allow(clippy::trivial_regex)]

use regex::Regex;

// The lexer maintains a stack of States.
// The highest state determine what set of regular expressions
// the lexer uses.

pub enum State {
    Normal,  // Normal operation (lexing code)
    Comment, // Inside a (*   *) comment - ignore contents
    Quote,   // Inside a quote - add contents to the working string
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
pub enum PatName {
    NormalAdd,
    NormalDiv,
    NormalSub,
    NormalMul,
    NormalEq,
    NormalLT,
    NormalDot,
    NormalNeg,
    NormalComma,
    NormalSemiColon,
    NormalColon,
    NormalOpenParen,
    NormalCloseParen,
    NormalAt,
    NormalOpenBrace,
    NormalCloseBrace,
    NormalWhiteSpace,
    NormalClass,
    NormalElse,
    NormalFi,
    NormalIf,
    NormalIn,
    NormalInherits,
    NormalLet,
    NormalLoop,
    NormalPool,
    NormalThen,
    NormalWhile,
    NormalCase,
    NormalEsac,
    NormalOf,
    NormalNew,
    NormalIsVoid,
    NormalNot,
    NormalDArrow,
    NormalLE,
    NormalAssign,
    NormalTrue,
    NormalFalse,
    NormalTypeID,
    NormalObjectID,
    NormalIntConst,
    NormalDashComment,
    NormalOpenComment,
    NormalCloseComment,
    NormalQuote,
    NormalBadChar,

    CommentOpenComment,
    CommentCloseComment,
    CommentChar,

    QuoteNull,
    QuoteEscNull,
    QuoteQuote,
    QuoteChar,
    QuoteEscNewLine,
    QuoteBackSpace,
    QuoteTab,
    QuoteFormFeed,
    QuoteEscChar,
    QuoteNewLine,
}

// regular expression and their names for Normal state
pub fn initial_pats() -> Vec<(Regex, PatName)> {
    vec![
        (Regex::new(r"^\+").unwrap(), PatName::NormalAdd),
        (Regex::new(r"^/").unwrap(), PatName::NormalDiv),
        (Regex::new(r"^\-").unwrap(), PatName::NormalSub),
        (Regex::new(r"^\*").unwrap(), PatName::NormalMul),
        (Regex::new(r"^=").unwrap(), PatName::NormalEq),
        (Regex::new(r"^<").unwrap(), PatName::NormalLT),
        (Regex::new(r"^\.").unwrap(), PatName::NormalDot),
        (Regex::new(r"^\~").unwrap(), PatName::NormalNeg),
        (Regex::new(r"^,").unwrap(), PatName::NormalComma),
        (Regex::new(r"^;").unwrap(), PatName::NormalSemiColon),
        (Regex::new(r"^:").unwrap(), PatName::NormalColon),
        (Regex::new(r"^\(").unwrap(), PatName::NormalOpenParen),
        (Regex::new(r"^\)").unwrap(), PatName::NormalCloseParen),
        (Regex::new(r"^@").unwrap(), PatName::NormalAt),
        (Regex::new(r"^\{").unwrap(), PatName::NormalOpenBrace),
        (Regex::new(r"^\}").unwrap(), PatName::NormalCloseBrace),
        (
            Regex::new(r"^[ \t\f\r\v\n]").unwrap(),
            PatName::NormalWhiteSpace,
        ),
        (Regex::new(r"^(?i:CLASS)").unwrap(), PatName::NormalClass),
        (Regex::new(r"^(?i:ELSE)").unwrap(), PatName::NormalElse),
        (Regex::new(r"^(?i:FI)").unwrap(), PatName::NormalFi),
        (Regex::new(r"^(?i:IF)").unwrap(), PatName::NormalIf),
        (Regex::new(r"^(?i:IN)").unwrap(), PatName::NormalIn),
        (
            Regex::new(r"^(?i:INHERITS)").unwrap(),
            PatName::NormalInherits,
        ),
        (Regex::new(r"^(?i:LET)").unwrap(), PatName::NormalLet),
        (Regex::new(r"^(?i:LOOP)").unwrap(), PatName::NormalLoop),
        (Regex::new(r"^(?i:POOL)").unwrap(), PatName::NormalPool),
        (Regex::new(r"^(?i:THEN)").unwrap(), PatName::NormalThen),
        (Regex::new(r"^(?i:WHILE)").unwrap(), PatName::NormalWhile),
        (Regex::new(r"^(?i:CASE)").unwrap(), PatName::NormalCase),
        (Regex::new(r"^(?i:ESAC)").unwrap(), PatName::NormalEsac),
        (Regex::new(r"^(?i:OF)").unwrap(), PatName::NormalOf),
        (Regex::new(r"^(?i:NEW)").unwrap(), PatName::NormalNew),
        (Regex::new(r"^(?i:ISVOID)").unwrap(), PatName::NormalIsVoid),
        (Regex::new(r"^(?i:NOT)").unwrap(), PatName::NormalNot),
        (Regex::new(r"^=>").unwrap(), PatName::NormalDArrow),
        (Regex::new(r"^<=").unwrap(), PatName::NormalLE),
        (Regex::new(r"^<-").unwrap(), PatName::NormalAssign),
        (Regex::new(r"^t(?i:rue)").unwrap(), PatName::NormalTrue),
        (Regex::new(r"^f(?i:alse)").unwrap(), PatName::NormalFalse),
        (
            Regex::new(r"^[A-Z][A-Za-z0-9_]*").unwrap(),
            PatName::NormalTypeID,
        ),
        (
            Regex::new(r"^[a-z][A-Za-z0-9_]*").unwrap(),
            PatName::NormalObjectID,
        ),
        (Regex::new(r"^[0-9]+").unwrap(), PatName::NormalIntConst),
        (Regex::new(r"^\-\-.*").unwrap(), PatName::NormalDashComment),
        (Regex::new(r"^\(\*").unwrap(), PatName::NormalOpenComment),
        (Regex::new(r"^\*\)").unwrap(), PatName::NormalCloseComment),
        (Regex::new(r#"^""#).unwrap(), PatName::NormalQuote),
        (Regex::new(r"^.").unwrap(), PatName::NormalBadChar),
    ]
}

// regular expression and their names for Comment state
pub fn comment_pats() -> Vec<(Regex, PatName)> {
    vec![
        (Regex::new(r"^\(\*").unwrap(), PatName::CommentOpenComment),
        (Regex::new(r"^\*\)").unwrap(), PatName::CommentCloseComment),
        (Regex::new(r"^(.|\n)").unwrap(), PatName::CommentChar),
    ]
}

// regular expression and their names for Quote state
pub fn quote_pats() -> Vec<(Regex, PatName)> {
    vec![
        (Regex::new("^\0").unwrap(), PatName::QuoteNull),
        (Regex::new("^\\\\\0").unwrap(), PatName::QuoteEscNull),
        (Regex::new(r#"^""#).unwrap(), PatName::QuoteQuote),
        (Regex::new(r#"^[^"\n]"#).unwrap(), PatName::QuoteChar),
        (Regex::new(r"^\\\n").unwrap(), PatName::QuoteEscNewLine),
        (Regex::new(r"^\\b").unwrap(), PatName::QuoteBackSpace),
        (Regex::new(r"^\\t").unwrap(), PatName::QuoteTab),
        (Regex::new(r"^\\n").unwrap(), PatName::QuoteEscNewLine),
        (Regex::new(r"^\\f").unwrap(), PatName::QuoteFormFeed),
        (Regex::new(r"^\\.").unwrap(), PatName::QuoteEscChar),
        (Regex::new(r"^\n").unwrap(), PatName::QuoteNewLine),
    ]
}
