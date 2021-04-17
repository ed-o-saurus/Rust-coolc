use std::ops::Range;

use super::second_pass_expr::second_pass_expr;
use crate::ast::{Branch, Class, Method, ObjectID, TypeID};
use crate::scoped_collections::ScopedIndexMap;

use indexmap::IndexMap;

pub fn second_pass_branch(
    Branch {
        line_no,
        name,
        type_decl,
        expr,
        family,
    }: Branch,
    classes: &IndexMap<TypeID, Class>,
    current_class_name: &TypeID,
    file_name: &str,
    object_types: &mut ScopedIndexMap<ObjectID, TypeID>,
    method_sigs: &IndexMap<(TypeID, ObjectID), Method>,
    parent_names: &IndexMap<TypeID, TypeID>,
    families: &IndexMap<TypeID, Range<u32>>,
) -> Result<Branch, String> {
    object_types.enter_scope();

    object_types.insert(name.clone(), type_decl.clone());

    let expr = second_pass_expr(
        expr,
        classes,
        current_class_name,
        file_name,
        object_types,
        method_sigs,
        parent_names,
        families,
    )?;

    object_types.exit_scope();

    Ok(Branch {
        line_no,
        name,
        type_decl,
        expr,
        family,
    })
}
