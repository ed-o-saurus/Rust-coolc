use std::ops::Range;

use crate::ast::{Class, TypeID};

use indexmap::IndexMap;

// Set the children field of the Class structs
// Ensure that all inheritance is valid

pub fn build_inheritiance_tree(
    mut classes: IndexMap<TypeID, Class>,
) -> Result<IndexMap<TypeID, Class>, String> {
    let mut ret_classes: IndexMap<TypeID, Class> = IndexMap::new(); // Returned value

    // map to store children fields of types
    // Will be combined with the classes later
    let mut children_by_parent: IndexMap<TypeID, Vec<TypeID>> = IndexMap::new();

    for (class_name, class) in classes.drain(..) {
        let file_name = class.file_name.clone();
        let line_no = class.line_no;

        children_by_parent.insert(class_name.clone(), Vec::new());

        if class.parent_name == Some(TypeID::new_bool()) {
            return Err(format!(
                "{} : {} - Class {} cannot inherit class Bool.",
                file_name, line_no, class_name
            ));
        }

        if class.parent_name == Some(TypeID::new_string()) {
            return Err(format!(
                "{} : {} - Class {} cannot inherit class String.",
                file_name, line_no, class_name
            ));
        }

        if class.parent_name == Some(TypeID::new_int()) {
            return Err(format!(
                "{} : {} - Class {} cannot inherit class Int.",
                file_name, line_no, class_name
            ));
        }

        if class.parent_name == Some(TypeID::new_self_type()) {
            return Err(format!(
                "{} : {} - Class {} cannot inherit class SELF_TYPE.",
                file_name, line_no, class_name
            ));
        }

        if class_name.is_self_type() {
            return Err(format!(
                "{} : {} - Redefinition of basic class SELF_TYPE.",
                file_name, line_no
            ));
        }

        // Move data from classes to the ret_clasees
        ret_classes.insert(class_name, class);
    }

    for (child_name, child) in ret_classes.iter() {
        if let Some(parent_name) = &child.parent_name {
            // If the class has a parent (Only Object does not.)
            match children_by_parent.get_mut(parent_name) {
                Some(child_names) => {
                    child_names.push(child_name.clone());
                }
                None => {
                    return Err(format!(
                        "{} : {} - Class {} inherits from an undefined class {}.",
                        child.file_name, child.line_no, child_name, parent_name
                    ))
                }
            }
        }
    }

    for (parent_name, child_names) in children_by_parent.iter() {
        // Set child_names in the ret_classes
        ret_classes.get_mut(parent_name).unwrap().child_names = child_names.clone();
    }

    // family values by class name
    let mut families: IndexMap<TypeID, Range<u32>> = IndexMap::new();

    // Get families of all classes
    // A family is a range of tags that are a classes descendants.
    set_families(&ret_classes, TypeID::root_class_name(), &mut families, 0);

    // Look for classes that weren't found in the DFS
    for (class_name, class) in ret_classes.iter() {
        if families.get(class_name).is_none() {
            return Err(format!(
                "{} : {} - Class {}, or an ancestor of {}, is involved in an inheritance cycle.",
                class.file_name, class.line_no, class_name, class_name
            ));
        }
    }

    for (class_name, family) in families.drain(..) {
        // Set family in ret_classes
        ret_classes.get_mut(&class_name).unwrap().family = family;
    }

    Ok(ret_classes)
}

// Determine families of all classes via a DFS
fn set_families(
    classes: &IndexMap<TypeID, Class>,
    class_name: TypeID,
    families: &mut IndexMap<TypeID, Range<u32>>,
    mut i: u32,
) -> u32 {
    let tag = i;

    i += 1;

    for class_name in classes.get(&class_name).unwrap().child_names.iter() {
        i = set_families(classes, class_name.clone(), families, i);
    }

    families.insert(class_name, tag..i);

    i
}
