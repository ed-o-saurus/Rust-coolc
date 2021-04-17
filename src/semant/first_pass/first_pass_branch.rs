use indexmap::{IndexMap, IndexSet};

use super::first_pass_expr::first_pass_expr;
use crate::ast::{Branch, Class, TypeID};

use std::ops::Range;

pub fn first_pass_branch(
    Branch {
        line_no,
        name,
        type_decl,
        expr,
        ..
    }: Branch,
    classes: &IndexMap<TypeID, Class>,
    current_class_name: &TypeID,
    current_class_family: &Range<u32>,
    file_name: &str,
    int_table: &mut IndexSet<u32>,
    str_consts: &mut IndexMap<String, u32>,
    str_table: &mut Vec<String>,
) -> Result<Branch, String> {
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

    if name.is_self() {
        return Err(format!(
            "{} : {} - 'self' bound in 'case'.",
            file_name, line_no
        ));
    }

    let family: Range<u32> = if type_decl == *current_class_name {
        current_class_family.clone()
    } else {
        match classes.get(&type_decl) {
            Some(class) => class.family.clone(),
            None => {
                return Err(format!(
                    "{} : {} : Class {} of case branch is undefined.",
                    file_name, line_no, type_decl
                ))
            }
        }
    };

    Ok(Branch {
        line_no,
        name,
        type_decl,
        expr,
        family,
    })
}
