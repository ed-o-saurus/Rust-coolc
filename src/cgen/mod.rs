mod code;
mod emit;

use std::{fmt, io};

use crate::ast::{Class, ObjectID, TypeID};
use crate::scoped_collections::ScopedIndexMap;

use indexmap::IndexMap;

use self::code::code_methods;
use self::emit::*;

const WORD_SIZE: i16 = 4;
const LG_WORD_SIZE: i16 = 2;

const DEFAULT_OBJFIELDS: i16 = 3;

const TAG_LOCATION: MemLocation = MemLocation {
    reg: Register::ACC,
    offset: 0,
};

// const SIZE_OFFSET: i16 = 1;
const DISPTABLE_LOCATION: MemLocation = MemLocation {
    reg: Register::ACC,
    offset: 2,
};

static mut LABEL_VAL: u32 = 0;

#[derive(Copy, Clone)]
pub struct MemLocation {
    pub reg: Register,
    pub offset: i16,
}

#[derive(Copy, Clone)]
pub struct Label(u32);

impl Label {
    fn new() -> Label {
        unsafe {
            LABEL_VAL += 1;

            Label(LABEL_VAL - 1)
        }
    }
}

impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "label_{:010x}", self.0)
    }
}

// Translate an IO error if necessary
pub fn cgen(
    out_file: &mut Box<dyn io::Write>,
    in_file_names: &[String],
    classes: &IndexMap<TypeID, Class>,
    int_table: &[u32],
    str_table: &[String],
) -> Result<(), String> {
    match write_code(out_file, in_file_names, classes, int_table, str_table) {
        Err(s) => Err(s.to_string()),
        Ok(()) => Ok(()),
    }
}

// output MIPS assembly for entire program to out_file
fn write_code(
    out_file: &mut Box<dyn io::Write>,
    in_file_names: &[String],
    classes: &IndexMap<TypeID, Class>,
    int_table: &[u32],
    str_table: &[String],
) -> Result<(), io::Error> {
    let root_class_name: TypeID = TypeID::root_class_name();

    let int_tag = classes.get(&TypeID::new_int()).unwrap().tag();
    let bool_tag = classes.get(&TypeID::new_bool()).unwrap().tag();
    let string_tag = classes.get(&TypeID::new_string()).unwrap().tag();

    writeln!(out_file, "\t.data")?;
    emit_align(out_file, LG_WORD_SIZE)?;

    emit_global(out_file, "class_nameTab")?; // allow trap handler access these addresses
    emit_global(out_file, "Main_protObj")?;
    emit_global(out_file, "Int_protObj")?;
    emit_global(out_file, "String_protObj")?;
    emit_global(out_file, "bool_const0")?;
    emit_global(out_file, "bool_const1")?;
    emit_global(out_file, "_int_tag")?;
    emit_global(out_file, "_bool_tag")?;
    emit_global(out_file, "_string_tag")?;

    writeln!(out_file, "_int_tag:")?;
    emit_word(out_file, int_tag)?;

    writeln!(out_file, "_bool_tag:")?;
    emit_word(out_file, bool_tag)?;

    writeln!(out_file, "_string_tag:")?;
    emit_word(out_file, string_tag)?;

    gc_settings(out_file)?; // Garbage Collection initialization

    // Write out all string constants
    for (val, val_id) in str_table.iter().zip(0..) {
        emit_string_const(out_file, "str_const", string_tag, val, val_id)?;
    }

    // Write out all file names as string constants
    emit_string_const(out_file, "file_name", string_tag, &String::new(), 0)?;
    for (val, val_id) in in_file_names.iter().zip(1..) {
        emit_string_const(out_file, "file_name", string_tag, val, val_id)?;
    }

    // Write out all class names (in tag order)
    class_name_consts(out_file, classes, &root_class_name, string_tag)?;

    // Write integer constants
    for val in int_table.iter() {
        emit_int_const(out_file, int_tag, *val)?;
    }

    // Write bool constants
    emit_bool_const(out_file, bool_tag, false)?;
    emit_bool_const(out_file, bool_tag, true)?;

    // Write class name table (in tag order)
    writeln!(out_file, "class_nameTab:")?;
    for tag in 0..classes.len() {
        emit_word(out_file, format!("class_name_{:08x}", tag))?;
    }

    writeln!(out_file, "class_objTab:")?;
    class_obj_tab(out_file, classes, &root_class_name)?;

    dispatch_table(out_file, classes, &root_class_name)?;

    proto_obj(out_file, classes, &root_class_name, DEFAULT_OBJFIELDS)?;

    writeln!(out_file, "heap_start:")?;
    emit_word(out_file, 0)?;
    writeln!(out_file, "\t.text")?;
    emit_global(out_file, "Main_init")?; // More globals
    emit_global(out_file, "Int_init")?;
    emit_global(out_file, "String_init")?;
    emit_global(out_file, "Bool_init")?;
    emit_global(out_file, "Main.main")?;

    // methods for all objects
    let mut object_locations: ScopedIndexMap<ObjectID, MemLocation> = ScopedIndexMap::new();
    code_methods(out_file, classes, &root_class_name, &mut object_locations)?;

    Ok(())
}

// Output garbage collection settings
fn gc_settings(out_file: &mut Box<dyn io::Write>) -> Result<(), io::Error> {
    emit_global(out_file, "_MemMgr_INITIALIZER")?;
    writeln!(out_file, "_MemMgr_INITIALIZER:")?;
    emit_word(out_file, "_NoGC_Init")?;

    emit_global(out_file, "_MemMgr_COLLECTOR")?;
    writeln!(out_file, "_MemMgr_COLLECTOR:")?;
    emit_word(out_file, "_NoGC_Collect")?;

    emit_global(out_file, "_MemMgr_TEST")?;
    writeln!(out_file, "_MemMgr_TEST:")?;
    emit_word(out_file, 0)?;

    Ok(())
}

// Write out all class names (in tag order)
fn class_name_consts(
    out_file: &mut Box<dyn io::Write>,
    classes: &IndexMap<TypeID, Class>,
    class_name: &TypeID,
    string_tag: u32,
) -> Result<(), io::Error> {
    let class: &Class = classes.get(class_name).unwrap();

    emit_string_const(
        out_file,
        "class_name",
        string_tag,
        &class_name.to_string(),
        class.tag(),
    )?;

    for child_class_name in &class.child_names {
        class_name_consts(out_file, classes, &child_class_name, string_tag)?;
    }

    Ok(())
}

// Class Object tab
fn class_obj_tab(
    out_file: &mut Box<dyn io::Write>,
    classes: &IndexMap<TypeID, Class>,
    class_name: &TypeID,
) -> Result<(), io::Error> {
    let class: &Class = classes.get(class_name).unwrap();

    emit_word(out_file, format!("{}_protObj", class_name))?;
    emit_word(out_file, format!("{}_init", class_name))?;

    for child_class_name in &class.child_names {
        class_obj_tab(out_file, classes, &child_class_name)?;
    }

    Ok(())
}

// list each class's methods in order
fn dispatch_table(
    out_file: &mut Box<dyn io::Write>,
    classes: &IndexMap<TypeID, Class>,
    class_name: &TypeID,
) -> Result<(), io::Error> {
    let class: &Class = classes.get(class_name).unwrap();

    writeln!(out_file, "{}_dispTab:", class_name)?;

    for (class_name, method_name) in &class.dispatch_table {
        emit_word(out_file, format!("{}.{}", class_name, method_name))?;
    }

    for child_class_name in &class.child_names {
        dispatch_table(out_file, classes, &child_class_name)?;
    }

    Ok(())
}

// Create a prototype object for each class
// This object has the structure of an object in this class but its attributes are not set
fn proto_obj(
    out_file: &mut Box<dyn io::Write>,
    classes: &IndexMap<TypeID, Class>,
    class_name: &TypeID,
    mut size: i16,
) -> Result<(), io::Error> {
    let class: &Class = classes.get(class_name).unwrap();

    size += class.attrs.len() as i16;

    emit_gc_tag(out_file)?;
    writeln!(out_file, "{}_protObj:", class_name)?;
    emit_word(out_file, class.tag())?;
    emit_word(out_file, size as u32)?;

    emit_word(out_file, format!("{}_dispTab", class_name))?;

    proto_attrs(out_file, classes, class_name)?;

    for child_class_name in &class.child_names {
        proto_obj(out_file, classes, &child_class_name, size)?;
    }

    Ok(())
}

// Attributes for a prototype object
fn proto_attrs(
    out_file: &mut Box<dyn io::Write>,
    classes: &IndexMap<TypeID, Class>,
    class_name: &TypeID,
) -> Result<(), io::Error> {
    let class: &Class = classes.get(class_name).unwrap();

    if let Some(parent_name) = &class.parent_name {
        proto_attrs(out_file, classes, parent_name)?;
    }

    for attr in class.attrs.iter() {
        if attr.type_decl.is_int() {
            emit_word(out_file, "int_const_00000000")?; // 0
        } else if attr.type_decl.is_bool() {
            emit_word(out_file, "bool_const0")?; // False
        } else if attr.type_decl.is_string() {
            emit_word(out_file, "str_const_00000000")?; // Empty String ("")
        } else {
            emit_word(out_file, 0)?; // Void
        }
    }

    Ok(())
}
