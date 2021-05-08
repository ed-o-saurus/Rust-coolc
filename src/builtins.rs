#![allow(non_upper_case_globals)]

use indexmap::IndexMap;

use crate::ast::{Attr, Class, Expression, Formal, Method, ObjectID, TypeID};
use std::ops::Range;

// The classes defined here are automatically included.
// They are defined in the runtime system (trap.handler).

// Default values for created classes
const file_name: String = String::new();
const file_no: u32 = 0;
const basic: bool = true;
const line_no: i16 = 0;
const attrs: Vec<Attr> = Vec::new();
const family: Range<u32> = 0..0;
const child_names: Vec<TypeID> = Vec::new();
const dispatch_table: Vec<(TypeID, ObjectID)> = Vec::new();
const formals: Vec<Formal> = Vec::new();
const expr: Expression = Expression::NoExpr;

// Create a classes map with the basic classes
pub fn initialize_classes() -> IndexMap<TypeID, Class> {
    let class_object = {
        let abort = Method {
            line_no,
            formals,
            return_type: TypeID::new_object(),
            expr,
        };

        let type_name = Method {
            line_no,
            formals,
            return_type: TypeID::new_string(),
            expr,
        };

        let copy = Method {
            line_no,
            formals,
            return_type: TypeID::new_self_type(),
            expr,
        };

        let mut methods: IndexMap<ObjectID, Method> = IndexMap::new();
        methods.insert(ObjectID::new("abort".to_string()), abort);
        methods.insert(ObjectID::new("type_name".to_string()), type_name);
        methods.insert(ObjectID::new("copy".to_string()), copy);

        Class {
            file_name,
            file_no,
            basic,
            line_no,
            parent_name: None,
            attrs,
            methods,
            family,
            child_names,
            method_name_to_pos: IndexMap::new(),
            dispatch_table,
        }
    };

    let class_io = {
        let out_string = Method {
            line_no,
            formals: vec![Formal {
                line_no,
                name: ObjectID::new("arg".to_string()),
                type_decl: TypeID::new_string(),
            }],
            return_type: TypeID::new_self_type(),
            expr,
        };

        let out_int = Method {
            line_no,
            formals: vec![Formal {
                line_no,
                name: ObjectID::new("arg".to_string()),
                type_decl: TypeID::new_int(),
            }],
            return_type: TypeID::new_self_type(),
            expr,
        };

        let in_string = Method {
            line_no,
            formals,
            return_type: TypeID::new_string(),
            expr,
        };

        let in_int = Method {
            line_no,
            formals,
            return_type: TypeID::new_int(),
            expr,
        };

        let mut methods: IndexMap<ObjectID, Method> = IndexMap::new();
        methods.insert(ObjectID::new("out_string".to_string()), out_string);
        methods.insert(ObjectID::new("out_int".to_string()), out_int);
        methods.insert(ObjectID::new("in_string".to_string()), in_string);
        methods.insert(ObjectID::new("in_int".to_string()), in_int);

        Class {
            file_name,
            file_no,
            basic,
            line_no,
            parent_name: Some(TypeID::new_object()),
            attrs,
            methods,
            family,
            child_names,
            method_name_to_pos: IndexMap::new(),
            dispatch_table,
        }
    };

    let class_int = {
        let val = Attr {
            line_no,
            name: ObjectID::new("_val".to_string()),
            type_decl: TypeID::new("_prim_slot".to_string()),
            init: Expression::NoExpr,
            self_offset: 0,
        };

        let methods: IndexMap<ObjectID, Method> = IndexMap::new();

        Class {
            file_name,
            file_no,
            basic,
            line_no,
            parent_name: Some(TypeID::new_object()),
            attrs: vec![val],
            methods,
            family,
            child_names,
            method_name_to_pos: IndexMap::new(),
            dispatch_table,
        }
    };

    let class_bool = {
        let val = Attr {
            line_no,
            name: ObjectID::new("_val".to_string()),
            type_decl: TypeID::new("_prim_slot".to_string()),
            init: Expression::NoExpr,
            self_offset: 0,
        };

        let methods: IndexMap<ObjectID, Method> = IndexMap::new();

        Class {
            file_name,
            file_no,
            basic,
            line_no,
            parent_name: Some(TypeID::new_object()),
            attrs: vec![val],
            methods,
            family,
            child_names,
            method_name_to_pos: IndexMap::new(),
            dispatch_table,
        }
    };

    let class_string = {
        let val = Attr {
            line_no,
            name: ObjectID::new("_val".to_string()),
            type_decl: TypeID::new_int(),
            init: Expression::NoExpr,
            self_offset: 0,
        };

        let str_field = Attr {
            line_no,
            name: ObjectID::new("_str_field".to_string()),
            type_decl: TypeID::new("_prim_slot".to_string()),
            init: Expression::NoExpr,
            self_offset: 0,
        };

        let length = Method {
            line_no,
            formals,
            return_type: TypeID::new_int(),
            expr,
        };

        let concat = Method {
            line_no,
            formals: vec![Formal {
                line_no,
                name: ObjectID::new("arg".to_string()),
                type_decl: TypeID::new_string(),
            }],
            return_type: TypeID::new_string(),
            expr,
        };

        let substr = Method {
            line_no,
            formals: vec![
                Formal {
                    line_no,
                    name: ObjectID::new("arg1".to_string()),
                    type_decl: TypeID::new_int(),
                },
                Formal {
                    line_no,
                    name: ObjectID::new("arg2".to_string()),
                    type_decl: TypeID::new_int(),
                },
            ],
            return_type: TypeID::new_string(),
            expr,
        };

        let mut methods: IndexMap<ObjectID, Method> = IndexMap::new();
        methods.insert(ObjectID::new("length".to_string()), length);
        methods.insert(ObjectID::new("concat".to_string()), concat);
        methods.insert(ObjectID::new("substr".to_string()), substr);

        Class {
            file_name,
            file_no,
            basic,
            line_no,
            parent_name: Some(TypeID::new_object()),
            attrs: vec![val, str_field],
            methods,
            family,
            child_names,
            method_name_to_pos: IndexMap::new(),
            dispatch_table,
        }
    };

    let mut classes: IndexMap<TypeID, Class> = IndexMap::new();
    classes.insert(TypeID::new_object(), class_object);
    classes.insert(TypeID::new_io(), class_io);
    classes.insert(TypeID::new_int(), class_int);
    classes.insert(TypeID::new_bool(), class_bool);
    classes.insert(TypeID::new_string(), class_string);

    classes
}
