mod patterns;
mod process;

use std::cmp::{Ord, Ordering};
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

use self::patterns::{comment_pats, initial_pats, quote_pats, PatName, State};
use self::process::process;
use crate::token::Token;

// Represents a match of a regular expression and the length
#[derive(Clone, Copy)]
struct Match {
    pat_name: PatName,
    len: usize,
}

// Order matches from largest to smallest
// Use pattern order as a tie breaker
impl Ord for Match {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.len.cmp(&other.len) {
            Ordering::Greater => Ordering::Less,
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => self.pat_name.cmp(&other.pat_name),
        }
    }
}

impl PartialOrd for Match {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Match {
    fn eq(&self, other: &Self) -> bool {
        self.pat_name == other.pat_name && self.len == other.len
    }
}

impl Eq for Match {}

// Given a set of regular expressions and the sub-string to be processed
fn search_pats(pats: &[(Regex, PatName)], remain: &str) -> Match {
    let mut v: Vec<Match> = vec![];

    // Check all patterns
    for (pat, pat_name) in pats.iter() {
        if let Some(d) = pat.captures(remain) {
            // If this pattern matches add it to the list
            v.push(Match {
                pat_name: *pat_name,
                len: d.get(0).unwrap().end(),
            });
        }
    }

    // Each state's patterns include one that will match any char.
    // Therefore, it is guaranteed that there will be a match.
    *v.iter().min().unwrap()
}

// Transform a file to a queue of tokens
pub fn tokenize(in_file: File, in_file_name: &str) -> Result<VecDeque<Token>, String> {
    let mut tokens: VecDeque<Token> = VecDeque::new();

    let mut state_stack: Vec<State> = vec![State::Normal]; // Top state determines lexer behaviour
    let mut working_str: String = String::new(); // used to store string constant

    // Sets of patterns for each state
    let initial_pats = initial_pats();
    let comment_pats = comment_pats();
    let quote_pats = quote_pats();

    let mut line_no: i16 = 0; // Tracks the current line number being processed

    let mut buff_reader = BufReader::new(in_file);

    loop {
        let mut line = String::new();
        let mut start = 0; // the location in the current input string

        line_no += 1;

        match buff_reader.read_line(&mut line) {
            Ok(0) => break, // No more lines
            Ok(_) => {}
            Err(s) => return Err(format!("{} : {} - {}", in_file_name, line_no, s)),
        }

        // Only ASCII characters are valid in source
        for b in line.as_bytes().iter() {
            if b & 0x80 != 0x00 {
                return Err(format!(
                    "{} : {} - Non-ASCII character in source",
                    in_file_name, line_no
                ));
            }
        }

        // While there is remaining data in the input line
        while line.len() > start {
            // get the next match
            let Match { pat_name, len } = match state_stack.last().unwrap() {
                State::Normal => search_pats(&initial_pats, &line[start..]),
                State::Comment => search_pats(&comment_pats, &line[start..]),
                State::Quote => search_pats(&quote_pats, &line[start..]),
            };

            process(
                in_file_name,
                line_no,
                pat_name,
                &line[start..start + len],
                &mut state_stack,
                &mut tokens,
                &mut working_str,
            )?;

            start += len;
        }
    }

    // Ensure that the file ended in Normal state
    match state_stack.last().unwrap() {
        State::Normal => tokens.push_back(Token::End { line_no }), // Needed for the parser
        State::Comment => return Err(format!("{} : {} - EOF in comment", in_file_name, line_no)),
        State::Quote => {
            return Err(format!(
                "{} : {} - EOF in string constant",
                in_file_name, line_no
            ))
        }
    };

    Ok(tokens)
}
