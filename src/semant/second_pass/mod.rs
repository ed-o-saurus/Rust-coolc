mod second_pass_attr;
mod second_pass_branch;
mod second_pass_expr;
mod second_pass_method;

use std::ops::Range;

use self::second_pass_attr::second_pass_attr;
use self::second_pass_method::second_pass_method;
use crate::ast::{Attr, Class, Method, ObjectID, TypeID};
use crate::scoped_collections::ScopedIndexMap;

use indexmap::IndexMap;

pub fn second_pass(
    mut classes: IndexMap<TypeID, Class>,
    class_name: &TypeID,
    object_types: &mut ScopedIndexMap<ObjectID, TypeID>,
    method_sigs: &IndexMap<(TypeID, ObjectID), Method>,
    parent_names: &IndexMap<TypeID, TypeID>,
    families: &IndexMap<TypeID, Range<u32>>,
) -> Result<IndexMap<TypeID, Class>, String> {
    let Class {
        file_name,
        file_no,
        basic,
        line_no,
        parent_name,
        mut attrs,
        mut methods,
        family,
        child_names,
        method_name_to_pos,
        dispatch_table,
    } = classes.remove(class_name).unwrap();

    let child_names_clone = child_names.clone();

    object_types.enter_scope();
    for attr in attrs.iter() {
        object_types.insert(attr.name.clone(), attr.type_decl.clone());
    }

    let attrs = {
        let mut new_attrs: Vec<Attr> = Vec::new();
        for attr in attrs.drain(..) {
            new_attrs.push(second_pass_attr(
                attr,
                &classes,
                class_name,
                &file_name,
                object_types,
                method_sigs,
                parent_names,
                families,
            )?);
        }

        new_attrs
    };

    let methods = {
        let mut new_methods: IndexMap<ObjectID, Method> = IndexMap::new();
        for (method_name, method) in methods.drain(..) {
            let method = second_pass_method(
                method,
                &method_name,
                &classes,
                class_name,
                &file_name,
                object_types,
                method_sigs,
                parent_names,
                families,
            )?;
            new_methods.insert(method_name, method);
        }

        new_methods
    };

    classes.insert(
        class_name.clone(),
        Class {
            file_name,
            file_no,
            basic,
            line_no,
            parent_name,
            attrs,
            methods,
            family,
            child_names,
            method_name_to_pos,
            dispatch_table,
        },
    );

    // Recursively descend to child classes
    for child_name in child_names_clone.iter() {
        classes = second_pass(
            classes,
            child_name,
            object_types,
            method_sigs,
            parent_names,
            families,
        )?;
    }

    object_types.exit_scope();

    Ok(classes)
}

// Does class #1 conform to class #2 ?
// Is class #1 a descendant of class #2 ?
pub fn conforms(
    class_name1: &TypeID,
    class_name2: &TypeID,
    current_class_name: &TypeID,
    families: &IndexMap<TypeID, Range<u32>>,
) -> bool {
    if class_name1.is_no_type() {
        return true;
    }

    if class_name1.is_self_type() && class_name2.is_self_type() {
        return true;
    }

    if class_name2.is_self_type() {
        return false;
    }

    let family = families.get(class_name2).unwrap();

    let tag = if class_name1.is_self_type() {
        families.get(current_class_name)
    } else {
        families.get(class_name1)
    }
    .unwrap()
    .start;

    family.contains(&tag)
}

// Return the name of the least class that both class_name1 and class_name2 conform to
pub fn least_upper_bound(
    class_name1: &TypeID,
    class_name2: &TypeID,
    current_class_name: &TypeID,
    parent_names: &IndexMap<TypeID, TypeID>,
    families: &IndexMap<TypeID, Range<u32>>,
) -> TypeID {
    if class_name1.is_self_type() && class_name2.is_self_type() {
        return TypeID::new_self_type();
    }

    if class_name1.is_self_type() {
        return least_upper_bound(
            current_class_name,
            class_name2,
            current_class_name,
            parent_names,
            families,
        );
    }

    if class_name2.is_self_type() {
        return least_upper_bound(
            class_name1,
            current_class_name,
            current_class_name,
            parent_names,
            families,
        );
    }

    if conforms(class_name1, class_name2, current_class_name, families) {
        return class_name1.clone();
    }

    if conforms(class_name2, class_name1, current_class_name, families) {
        return class_name2.clone();
    }

    least_upper_bound(
        parent_names.get(class_name1).unwrap(),
        class_name2,
        current_class_name,
        parent_names,
        families,
    )
}
