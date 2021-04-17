mod build_inheritiance_tree;
mod first_pass;
mod second_pass;
mod verify_main;

use indexmap::{IndexMap, IndexSet};
use std::ops::Range;

use self::build_inheritiance_tree::build_inheritiance_tree;
use self::first_pass::first_pass;
use self::second_pass::second_pass;
use self::verify_main::verify_main;
use crate::ast::{Class, Method, ObjectID, TypeID};
use crate::scoped_collections::{ScopedIndexMap, ScopedIndexSet};

const ATTR_OFFSET_START: i16 = 3;

pub fn semant(
    classes: IndexMap<TypeID, Class>,
) -> Result<(IndexMap<TypeID, Class>, IndexSet<u32>, Vec<String>), String> {
    let mut int_table: IndexSet<u32> = IndexSet::new(); // All integer constants
    let mut str_consts: IndexMap<String, u32> = IndexMap::new(); // All string constants mapped to unique index
    let mut str_table: Vec<String> = Vec::new(); // Inverse of str_conts

    // These constants are needed because they are default values for attributes and in 'let' statements.
    int_table.insert(0);
    str_consts.insert(String::new(), 0);
    str_table.push(String::new());

    // Ensure that the classes are valid and that there is no inheritance cycle
    // Set the children field of the Class structs
    let classes: IndexMap<TypeID, Class> = build_inheritiance_tree(classes)?;

    // Ensure that there is a Main.main() method
    verify_main(&classes)?;

    // Keep track of inherited attributes
    let mut inherited_attrs: ScopedIndexSet<ObjectID> = ScopedIndexSet::new();

    // Maps method name to its position in the dispatch table
    let mut method_name_to_pos: ScopedIndexMap<ObjectID, i16> = ScopedIndexMap::new();

    // Keeps track of the method at a given position in the dispatch table
    let mut dispatch_table: ScopedIndexMap<i16, (TypeID, ObjectID)> = ScopedIndexMap::new();

    let root_class_name = TypeID::root_class_name();

    // Fill tables, ensure explicitly mentioned classes are valid, and
    // set attribute offsets and method positions in dispatch tables.
    let classes = first_pass(
        classes,
        &root_class_name,
        ATTR_OFFSET_START,
        0, // method_count
        &mut int_table,
        &mut str_consts,
        &mut str_table,
        &mut inherited_attrs,
        &mut method_name_to_pos,
        &mut dispatch_table,
    )?;

    // Map objects to their static type
    let mut object_types: ScopedIndexMap<ObjectID, TypeID> = ScopedIndexMap::new();

    // Build an index map of method signatures
    let mut method_sigs: IndexMap<(TypeID, ObjectID), Method> = IndexMap::new();
    for (class_name, class) in classes.iter() {
        for (implementing_class, method_name) in class.dispatch_table.iter() {
            method_sigs.insert(
                (class_name.clone(), method_name.clone()),
                classes
                    .get(implementing_class)
                    .unwrap()
                    .methods
                    .get(method_name)
                    .unwrap()
                    .get_sig(),
            );
        }
    }

    // Map of class name to parent name
    let mut parent_names: IndexMap<TypeID, TypeID> = IndexMap::new();
    for (class_name, class) in classes.iter() {
        if let Some(parent_name) = &class.parent_name {
            parent_names.insert(class_name.clone(), parent_name.clone());
        }
    }

    // Map of class name to family
    let families: IndexMap<TypeID, Range<u32>> = classes
        .iter()
        .map(|(class_name, class)| (class_name.clone(), class.family.clone()))
        .collect();

    // Set static types
    // Ensure proper type comformality
    let classes = second_pass(
        classes,
        &root_class_name,
        &mut object_types,
        &method_sigs,
        &parent_names,
        &families,
    )?;

    Ok((classes, int_table, str_table))
}
