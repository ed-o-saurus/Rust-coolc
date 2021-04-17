use std::ops::Range;

use super::conforms;
use super::second_pass_expr::second_pass_expr;
use crate::ast::{Attr, Class, Method, ObjectID, TypeID};
use crate::scoped_collections::ScopedIndexMap;

use indexmap::IndexMap;

pub fn second_pass_attr(
    Attr {
        line_no,
        name,
        type_decl,
        init,
        self_offset,
    }: Attr,
    classes: &IndexMap<TypeID, Class>,
    current_class_name: &TypeID,
    file_name: &str,
    object_types: &mut ScopedIndexMap<ObjectID, TypeID>,
    method_sigs: &IndexMap<(TypeID, ObjectID), Method>,
    parent_names: &IndexMap<TypeID, TypeID>,
    families: &IndexMap<TypeID, Range<u32>>,
) -> Result<Attr, String> {
    let init = second_pass_expr(
        init,
        classes,
        current_class_name,
        file_name,
        object_types,
        method_sigs,
        parent_names,
        families,
    )?;

    if !conforms(
        &init.static_type(),
        &type_decl,
        current_class_name,
        families,
    ) {
        return Err(format!(
            "{} : {} - Type {} of assigned expression does not conform to declared type {} of identifier {}.",
            file_name,
            line_no,
            init.static_type(),
            type_decl,
            name
        ));
    }

    Ok(Attr {
        line_no,
        name,
        type_decl,
        init,
        self_offset,
    })
}
