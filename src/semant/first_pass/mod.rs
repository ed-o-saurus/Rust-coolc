mod first_pass_attr;
mod first_pass_branch;
mod first_pass_expr;
mod first_pass_method;

use indexmap::{IndexMap, IndexSet};

use self::first_pass_attr::first_pass_attr;
use self::first_pass_method::first_pass_method;
use crate::ast::{Attr, Class, Method, ObjectID, TypeID};
use crate::scoped_collections::{ScopedIndexMap, ScopedIndexSet};

pub fn first_pass(
    mut classes: IndexMap<TypeID, Class>,
    class_name: &TypeID,
    mut self_offset: i16,
    mut method_count: i16,
    int_table: &mut IndexSet<u32>,
    str_consts: &mut IndexMap<String, u32>,
    str_table: &mut Vec<String>,
    inherited_attrs: &mut ScopedIndexSet<ObjectID>,
    method_name_to_pos: &mut ScopedIndexMap<ObjectID, i16>,
    dispatch_table: &mut ScopedIndexMap<i16, (TypeID, ObjectID)>,
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
        ..
    } = classes.remove(class_name).unwrap();

    inherited_attrs.enter_scope();
    method_name_to_pos.enter_scope();
    dispatch_table.enter_scope();

    // process attributes
    let attrs = {
        let mut new_attrs: Vec<Attr> = Vec::new();
        for attr in attrs.drain(..) {
            new_attrs.push(first_pass_attr(
                attr,
                &classes,
                class_name,
                &family,
                &file_name,
                self_offset, // offset from SELF pointer
                int_table,
                str_consts,
                str_table,
                inherited_attrs,
            )?);

            self_offset += 1;
        }

        new_attrs
    };

    // process methods
    let methods = {
        let mut new_methods: IndexMap<ObjectID, Method> = IndexMap::new();
        for (method_name, method) in methods.drain(..) {
            let (method, is_new) = first_pass_method(
                method,
                &method_name,
                &classes,
                class_name,
                &family,
                &file_name,
                int_table,
                str_consts,
                str_table,
                method_count,
                method_name_to_pos,
                dispatch_table,
            )?;

            new_methods.insert(method_name, method);

            if is_new {
                method_count += 1;
            }
        }

        new_methods
    };

    let child_names_clone = child_names.clone();

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
            method_name_to_pos: method_name_to_pos.to_index_map(),
            dispatch_table: (0..method_count)
                .map(|i| dispatch_table.get(&i).unwrap().clone())
                .collect(),
        },
    );

    // Recursively descend to child classes
    for child_name in child_names_clone.iter() {
        classes = first_pass(
            classes,
            child_name,
            self_offset,
            method_count,
            int_table,
            str_consts,
            str_table,
            inherited_attrs,
            method_name_to_pos,
            dispatch_table,
        )?;
    }

    inherited_attrs.exit_scope();
    method_name_to_pos.exit_scope();
    dispatch_table.exit_scope();

    Ok(classes)
}
