use indexmap::{IndexMap, IndexSet};

use super::first_pass_expr::first_pass_expr;
use crate::ast::{Class, Formal, Method, ObjectID, TypeID};
use crate::scoped_collections::ScopedIndexMap;

use std::ops::Range;

pub fn first_pass_method(
    Method {
        line_no,
        formals,
        return_type,
        expr,
    }: Method,
    method_name: &ObjectID,
    classes: &IndexMap<TypeID, Class>,
    current_class_name: &TypeID,
    current_class_family: &Range<u32>,
    file_name: &str,
    int_table: &mut IndexSet<u32>,
    str_consts: &mut IndexMap<String, u32>,
    str_table: &mut Vec<String>,
    method_count: i16,
    method_name_to_pos: &mut ScopedIndexMap<ObjectID, i16>,
    dispatch_table: &mut ScopedIndexMap<i16, (TypeID, ObjectID)>,
) -> Result<(Method, bool), String> {
    let is_new: bool; // Is this method not overriding a previously declared method?

    match method_name_to_pos.get(&method_name) {
        Some(method_pos) => {
            is_new = false;

            let (old_class, _) = dispatch_table.get(method_pos).unwrap();
            let old_method = classes
                .get(old_class)
                .unwrap()
                .methods
                .get(method_name)
                .unwrap();

            dispatch_table.insert(
                *method_pos,
                (current_class_name.clone(), method_name.clone()),
            );

            method_compare(
                old_method,
                line_no,
                &method_name,
                &formals,
                &return_type,
                file_name,
            )?;
        }
        None => {
            is_new = true;

            method_name_to_pos.insert(method_name.clone(), method_count);
            dispatch_table.insert(
                method_count,
                (current_class_name.clone(), method_name.clone()),
            );
        }
    }

    let mut formal_names: IndexSet<ObjectID> = IndexSet::new();

    for formal in formals.iter() {
        if formal.type_decl.is_self_type() {
            return Err(format!(
                "{} : {} - Formal parameter {} cannot have type SELF_TYPE.",
                file_name, line_no, formal.name
            ));
        }

        if formal.name.is_self() {
            return Err(format!(
                "{} : {} - 'self' cannot be the name of a formal parameter.",
                file_name, line_no
            ));
        }

        if formal.type_decl != *current_class_name && !classes.contains_key(&formal.type_decl) {
            return Err(format!(
                "{} : {} - Class {} of formal parameter {} is undefined.",
                file_name, line_no, formal.type_decl, formal.name
            ));
        }

        if !formal_names.insert(formal.name.clone()) {
            return Err(format!(
                "{} : {} - Formal parameter {} is multiply defined.",
                file_name, line_no, formal.name
            ));
        }
    }

    if !return_type.is_self_type()
        && return_type != *current_class_name
        && !classes.contains_key(&return_type)
    {
        return Err(format!(
            "{} : {} - Undefined return type {} in method {}.",
            file_name, line_no, return_type, method_name
        ));
    }

    let expr = first_pass_expr(
        expr,
        classes,
        current_class_name,
        current_class_family,
        file_name,
        int_table,
        str_consts,
        str_table,
    )?;

    Ok((
        Method {
            line_no,
            formals,
            return_type,
            expr,
        },
        is_new,
    ))
}

// Ensure that a redefined method has the same signature of its predecessor
fn method_compare(
    old_method: &Method,
    line_no: i16,
    method_name: &ObjectID,
    formals: &[Formal],
    return_type: &TypeID,
    file_name: &str,
) -> Result<(), String> {
    if old_method.return_type != *return_type {
        return Err(format!(
            "{} : {} - In redefined method {}, return type {} is different from original return type {}.",
            file_name, line_no, method_name, return_type, old_method.return_type
        ));
    }

    if old_method.formals.len() != formals.len() {
        return Err(format!(
            "{} : {} - Incompatible number of formal parameters in redefined method {}.",
            file_name, line_no, method_name
        ));
    }

    for (old_formal, new_formal) in old_method.formals.iter().zip(formals.iter()) {
        if old_formal.type_decl != new_formal.type_decl {
            return Err(format!(
                "{} : {} - In redefined method {}, parameter type {} is different from original type {}.",
                file_name, line_no, method_name, new_formal.type_decl, old_formal.type_decl
            ));
        }
    }

    Ok(())
}
