mod ast;
mod builtins;
mod cgen;
mod lexer;
mod parser;
mod scoped_collections;
mod semant;
mod token;

use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::exit;

use argparse::{ArgumentParser, Collect, StoreOption};
use indexmap::{IndexMap, IndexSet};

use crate::ast::{Class, TypeID};
use crate::builtins::initialize_classes;
use crate::cgen::cgen;
use crate::lexer::tokenize;
use crate::parser::parse;
use crate::semant::semant;

fn get_name(in_file_name: &String, file_no: u32) -> Result<&str, String> {
    match Path::new(in_file_name).file_name() {
        Some(in_file_name) => match in_file_name.to_str() {
            Some(in_file_name) => return Ok(&in_file_name),
            None => {
                return Err(format!(
                    "Cannot process input file name number {}.",
                    file_no
                ))
            }
        },
        None => {
            return Err(format!(
                "Cannot process input file name number {}.",
                file_no
            ))
        }
    };
}

fn coolc() -> Result<(), String> {
    let mut out_file_name: Option<String> = None; // Stores the output path or None if not specified
    let mut in_file_names: Vec<String> = Vec::new(); // Stores the paths of the source files

    {
        // Get arguments from the command line
        let mut ap = ArgumentParser::new();
        ap.set_description("Compile COOL to MIPS assembly");

        ap.refer(&mut out_file_name)
            .add_option(&["-o"], StoreOption, "Output file");

        ap.refer(&mut in_file_names)
            .add_argument("Source Files", Collect, "COOL source");

        ap.parse_args_or_exit();
    }

    if in_file_names.is_empty() {
        return Err("Must specify at least one source file".to_string());
    }

    // classes is a map to store the Class objects.
    // This function creates a map that stores the basic classes (Object, IO, Int, Bool, String)
    let mut classes: IndexMap<TypeID, Class> = initialize_classes();

    // Loop over the source files from the command line
    // File numbers are stored starting at 1.
    for (in_file_name, file_no) in in_file_names.iter().zip(1..) {
        // try to open input file
        let in_file = match File::open(&in_file_name) {
            Err(why) => {
                return Err(format!("Cannot open {} as input: {}", in_file_name, why));
            }
            Ok(in_file) => in_file,
        };

        // Tokenize the file
        let tokens = tokenize(in_file, in_file_name)?;

        let in_file_name = get_name(in_file_name, file_no)?;

        // Parse the tokens into class definitions.
        // The classes are added to the classes map.
        parse(tokens, in_file_name, file_no, &mut classes)?;
    }

    // Add semantic information to classes (see semant/mod.rs for more detail)
    // int_table : set of integer constants specified in the source
    // str_table : vector of string constants specified in the source
    let (classes, mut int_table, str_table): (IndexMap<TypeID, Class>, IndexSet<u32>, Vec<String>) =
        semant(classes)?;

    // add lengths of source files to int_table
    for (in_file_name, file_no) in in_file_names.iter().zip(1..) {
        let in_file_name = get_name(in_file_name, file_no)?;

        int_table.insert(in_file_name.len() as u32);
    }

    // add lengths of class names to int_table
    for class_name in classes.keys() {
        int_table.insert(class_name.len() as u32);
    }

    // convert int_table into sorted vector
    let mut int_table: Vec<u32> = int_table.into_iter().collect();
    int_table.sort_unstable();

    // determine output file name
    let out_file_name = match out_file_name {
        Some(out_file_name) => out_file_name, // If one is specified on the command line, use it
        None =>
        // If not, use the same name as the first source file but with .s
        {
            match Path::new(in_file_names.get(0).unwrap())
                .with_extension("s")
                .file_name()
            {
                Some(out_file_name) => match out_file_name.to_str() {
                    Some(out_file_name) => out_file_name.to_string(),
                    None => return Err(format!("Cannot process output file name.")),
                },

                None => return Err(format!("Cannot process output file name.")),
            }
        }
    };

    let mut out_file: Box<dyn Write> = {
        match File::create(&out_file_name) {
            // Open output file for writing.
            Err(why) => {
                return Err(format!("Cannot open {} as output: {}", out_file_name, why));
            }
            Ok(file) => Box::new(file),
        }
    };

    // Code generation - write out valid MIPS assembly
    cgen(
        &mut out_file,
        &in_file_names,
        &classes,
        &int_table,
        &str_table,
    )?;

    eprintln!("Successfully wrote {}.", out_file_name);

    Ok(())
}

fn main() {
    if let Err(s) = coolc() {
        eprintln!("{}", s);
        exit(1);
    }
}
