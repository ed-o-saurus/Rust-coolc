use indexmap::{IndexMap, IndexSet};

use super::first_pass_expr::first_pass_expr;
use crate::ast::{Attr, Class, ObjectID, TypeID};
use crate::scoped_collections::ScopedIndexSet;

use std::ops::Range;

pub fn first_pass_attr(
    Attr {
        line_no,
        name,
        type_decl,
        init,
        ..
    }: Attr,
    classes: &IndexMap<TypeID, Class>,
    current_class_name: &TypeID,
    current_class_family: &Range<u32>,
    file_name: &str,
    self_offset: i16, // offset from SELF pointer
    int_table: &mut IndexSet<u32>,
    str_consts: &mut IndexMap<String, u32>,
    str_table: &mut Vec<String>,
    inherited_attrs: &mut ScopedIndexSet<ObjectID>,
) -> Result<Attr, String> {
    if name.is_self() {
        return Err(format!(
            "{} : {} - 'self' cannot be the name of an attribute.",
            file_name, line_no
        ));
    }

    if inherited_attrs.contains_top_scope(&name) {
        return Err(format!(
            "{} : {} - Attribute {} is multiply defined in class.",
            file_name, line_no, name
        ));
    }

    if inherited_attrs.contains(&name) {
        return Err(format!(
            "{} : {} - Attribute {} is an attribute of an inherited class.",
            file_name, line_no, name
        ));
    }

    inherited_attrs.insert(name.clone());

    if type_decl != *current_class_name
        && !classes.contains_key(&type_decl)
        && !type_decl.is_system_type()
        && !type_decl.is_self_type()
    {
        return Err(format!(
            "{} : {} - Class {} of attribute {} is undefined.",
            file_name, line_no, type_decl, name
        ));
    }

    let init = first_pass_expr(
        init,
        classes,
        current_class_name,
        current_class_family,
        file_name,
        int_table,
        str_consts,
        str_table,
    )?;

    Ok(Attr {
        line_no,
        name,
        type_decl,
        init,
        self_offset,
    })
}
