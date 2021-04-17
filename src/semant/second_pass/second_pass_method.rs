use std::ops::Range;

use super::conforms;
use super::second_pass_expr::second_pass_expr;
use crate::ast::{Class, Method, ObjectID, TypeID};
use crate::scoped_collections::ScopedIndexMap;

use indexmap::IndexMap;

pub fn second_pass_method(
    Method {
        line_no,
        formals,
        return_type,
        expr,
    }: Method,
    method_name: &ObjectID,
    classes: &IndexMap<TypeID, Class>,
    current_class_name: &TypeID,
    file_name: &str,
    object_types: &mut ScopedIndexMap<ObjectID, TypeID>,
    method_sigs: &IndexMap<(TypeID, ObjectID), Method>,
    parent_names: &IndexMap<TypeID, TypeID>,
    families: &IndexMap<TypeID, Range<u32>>,
) -> Result<Method, String> {
    // Scope for the formals of the method
    object_types.enter_scope();

    for formal in formals.iter() {
        object_types.insert(formal.name.clone(), formal.type_decl.clone());
    }

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

    // Does the expr's static type conform to the declared return type?
    if !conforms(
        &expr.static_type(),
        &return_type,
        current_class_name,
        families,
    ) {
        return Err(format!(
            "{} : {} - Inferred return type {} of method {} does not conform to declared return type {}.",
            file_name,
            line_no,
            expr.static_type(),
            method_name,
            return_type
        ));
    }

    object_types.exit_scope();

    Ok(Method {
        line_no,
        formals,
        return_type,
        expr,
    })
}
