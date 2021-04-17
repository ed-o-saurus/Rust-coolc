use indexmap::IndexMap;

use crate::ast::{Class, ObjectID, TypeID};

// Ensure that there is a proper Main class and main method
pub fn verify_main(classes: &IndexMap<TypeID, Class>) -> Result<(), String> {
    let main_class = match classes.get(&TypeID::new_main()) {
        Some(main_class) => main_class,
        None => return Err("Class Main is not defined.".to_string()),
    };

    let main_method = match main_class.methods.get(&ObjectID::new_main()) {
        Some(main_method) => main_method,
        None => {
            return Err(format!(
                "{} : {} - No 'main' method in class Main.",
                main_class.file_name, main_class.line_no
            ));
        }
    };

    if !main_method.formals.is_empty() {
        return Err(format!(
            "{} : {} - 'main' method in class Main should have no arguments.",
            main_class.file_name, main_method.line_no
        ));
    }

    Ok(())
}
