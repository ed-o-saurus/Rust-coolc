use std::ops::Range;

use super::second_pass_branch::second_pass_branch;
use super::{conforms, least_upper_bound};
use crate::ast::{Branch, Class, Expression, Method, ObjectID, TypeID};
use crate::scoped_collections::ScopedIndexMap;

use indexmap::IndexMap;

pub fn second_pass_expr(
    expr: Expression,
    classes: &IndexMap<TypeID, Class>,
    current_class_name: &TypeID,
    file_name: &str,
    object_types: &mut ScopedIndexMap<ObjectID, TypeID>,
    method_sigs: &IndexMap<(TypeID, ObjectID), Method>,
    parent_names: &IndexMap<TypeID, TypeID>,
    families: &IndexMap<TypeID, Range<u32>>,
) -> Result<Expression, String> {
    Ok(match expr {
        Expression::Assign {
            line_no,
            name,
            expr,
        } => {
            let expr = Box::new(second_pass_expr(
                *expr,
                classes,
                current_class_name,
                file_name,
                object_types,
                method_sigs,
                parent_names,
                families,
            )?);

            match object_types.get(&name) {
                Some(t) => {
                    if !conforms(&expr.static_type(), t, current_class_name, families) {
                        return Err(format!(
                                "{} : {} - Type {} of assigned expression does not conform to declared type {} of identifier {}.",
                                file_name,
                                line_no,
                                &expr.static_type(),
                                t,
                                name
                            ));
                    }
                }
                None => {
                    return Err(format!(
                        "{} : {} - Assignment to undeclared variable {}.",
                        file_name, line_no, name
                    ))
                }
            }

            Expression::Assign {
                line_no,
                name,
                expr,
            }
        }

        Expression::Dispatch {
            line_no,
            expr,
            type_name,
            name,
            mut args,
            ..
        } => {
            let expr = Box::new(second_pass_expr(
                *expr,
                classes,
                current_class_name,
                file_name,
                object_types,
                method_sigs,
                parent_names,
                families,
            )?);

            let type_name = match type_name {
                Some(type_name) => {
                    if !conforms(
                        &expr.static_type(),
                        &type_name,
                        current_class_name,
                        families,
                    ) {
                        return Err(format!(
                            "{} : {} - Expression type {} does not conform to declared static dispatch type {}.",
                            file_name,
                            line_no,
                            expr.static_type(),
                            type_name
                        ));
                    }

                    Some(type_name)
                }
                None => None,
            };

            let args = {
                let mut new_args: Vec<Expression> = Vec::new();
                for arg in args.drain(..) {
                    new_args.push(second_pass_expr(
                        arg,
                        classes,
                        current_class_name,
                        file_name,
                        object_types,
                        method_sigs,
                        parent_names,
                        families,
                    )?);
                }

                new_args
            };

            let (type_name, method_id) = match type_name {
                Some(type_name) => (Some(type_name.clone()), (type_name, name.clone())),
                None => {
                    let mut class_name = expr.static_type();
                    if class_name.is_self_type() {
                        class_name = current_class_name.clone();
                    }

                    (None, (class_name, name.clone()))
                }
            };

            let method_sig = match method_sigs.get(&method_id) {
                Some(method_sig) => method_sig,
                None => {
                    return Err(format!(
                        "{} : {} - Dispatch to undefined method {}.",
                        file_name, line_no, name
                    ));
                }
            };

            let mut t = method_sig.return_type.clone();
            if t.is_self_type() && !expr.static_type().is_self_type() {
                t = expr.static_type();
            }

            let static_type = t;

            if args.len() != method_sig.formals.len() {
                return Err(format!(
                    "{} : {} - Method {} called with wrong number of arguments.",
                    file_name, line_no, name
                ));
            }

            for (arg, formal) in args.iter().zip(method_sig.formals.iter()) {
                if !conforms(
                    &arg.static_type(),
                    &formal.type_decl,
                    current_class_name,
                    families,
                ) {
                    return Err(format!(
                        "{} : {} - In call of method {}, type {} of parameter a does not conform to declared type {}.",
                        file_name,
                        line_no,
                        name,
                        arg.static_type(),
                        formal.type_decl
                    ));
                }
            }

            Expression::Dispatch {
                line_no,
                expr,
                type_name,
                name,
                args,
                static_type,
            }
        }

        Expression::Cond {
            line_no,
            pred,
            then_expr,
            else_expr,
            ..
        } => {
            let pred = Box::new(second_pass_expr(
                *pred,
                classes,
                current_class_name,
                file_name,
                object_types,
                method_sigs,
                parent_names,
                families,
            )?);

            if !pred.static_type().is_bool() {
                return Err(format!(
                    "{} : {} - Predicate of 'if' does not have type Bool.",
                    file_name, line_no
                ));
            }

            let then_expr = Box::new(second_pass_expr(
                *then_expr,
                classes,
                current_class_name,
                file_name,
                object_types,
                method_sigs,
                parent_names,
                families,
            )?);

            let else_expr = Box::new(second_pass_expr(
                *else_expr,
                classes,
                current_class_name,
                file_name,
                object_types,
                method_sigs,
                parent_names,
                families,
            )?);

            let static_type = least_upper_bound(
                &then_expr.static_type(),
                &else_expr.static_type(),
                current_class_name,
                parent_names,
                families,
            );

            Expression::Cond {
                line_no,
                pred,
                then_expr,
                else_expr,
                static_type,
            }
        }
        Expression::Loop {
            line_no,
            pred,
            body,
        } => {
            let pred = Box::new(second_pass_expr(
                *pred,
                classes,
                current_class_name,
                file_name,
                object_types,
                method_sigs,
                parent_names,
                families,
            )?);

            let body = Box::new(second_pass_expr(
                *body,
                classes,
                current_class_name,
                file_name,
                object_types,
                method_sigs,
                parent_names,
                families,
            )?);

            if !pred.static_type().is_bool() {
                return Err(format!(
                    "{} : {} - Loop condition does not have type Bool.",
                    file_name, line_no
                ));
            }

            Expression::Loop {
                line_no,
                pred,
                body,
            }
        }
        Expression::TypeCase {
            line_no,
            expr,
            mut branches,
            ..
        } => {
            let expr = Box::new(second_pass_expr(
                *expr,
                classes,
                current_class_name,
                file_name,
                object_types,
                method_sigs,
                parent_names,
                families,
            )?);

            let branches = {
                let mut new_branches: Vec<Branch> = Vec::new();
                for branch in branches.drain(..) {
                    let branch = second_pass_branch(
                        branch,
                        classes,
                        current_class_name,
                        file_name,
                        object_types,
                        method_sigs,
                        parent_names,
                        families,
                    )?;

                    new_branches.push(branch);
                }

                new_branches
            };

            let mut static_type = branches.first().unwrap().static_type();

            for branch in branches.iter() {
                static_type = least_upper_bound(
                    &static_type,
                    &branch.static_type(),
                    current_class_name,
                    parent_names,
                    families,
                );
            }

            Expression::TypeCase {
                line_no,
                expr,
                branches,
                static_type,
            }
        }
        Expression::Block {
            line_no, mut body, ..
        } => {
            let body = {
                let mut new_body: Vec<Expression> = Vec::new();
                for expr in body.drain(..) {
                    new_body.push(second_pass_expr(
                        expr,
                        classes,
                        current_class_name,
                        file_name,
                        object_types,
                        method_sigs,
                        parent_names,
                        families,
                    )?);
                }

                new_body
            };

            Expression::Block { line_no, body }
        }
        Expression::Let {
            line_no,
            identifier,
            type_decl,
            init,
            body,
            ..
        } => {
            let init = Box::new(second_pass_expr(
                *init,
                classes,
                current_class_name,
                file_name,
                object_types,
                method_sigs,
                parent_names,
                families,
            )?);

            if !conforms(
                &init.static_type(),
                &type_decl,
                current_class_name,
                families,
            ) {
                return Err(format!(
                    "{} : {} - Inferred type {} of initialization of {} does not conform to identifier's declared type {}.",
                    file_name,
                    line_no,
                    init.static_type(),
                    identifier,
                    type_decl
                ));
            }

            object_types.enter_scope();

            object_types.insert(identifier.clone(), type_decl.clone());

            let body = Box::new(second_pass_expr(
                *body,
                classes,
                current_class_name,
                file_name,
                object_types,
                method_sigs,
                parent_names,
                families,
            )?);

            object_types.exit_scope();

            Expression::Let {
                line_no,
                identifier,
                type_decl,
                init,
                body,
            }
        }
        Expression::ArithOp {
            line_no,
            expr_lhs,
            expr_rhs,
            arith_op_type,
        } => {
            let expr_lhs = Box::new(second_pass_expr(
                *expr_lhs,
                classes,
                current_class_name,
                file_name,
                object_types,
                method_sigs,
                parent_names,
                families,
            )?);

            let expr_rhs = Box::new(second_pass_expr(
                *expr_rhs,
                classes,
                current_class_name,
                file_name,
                object_types,
                method_sigs,
                parent_names,
                families,
            )?);

            if !expr_lhs.static_type().is_int() || !expr_lhs.static_type().is_int() {
                return Err(format!(
                    "{} : {} - non-Int arguments: {} {} {}",
                    file_name,
                    line_no,
                    expr_lhs.static_type(),
                    arith_op_type,
                    expr_rhs.static_type()
                ));
            }

            Expression::ArithOp {
                line_no,
                expr_lhs,
                expr_rhs,
                arith_op_type,
            }
        }
        Expression::Neg { line_no, expr } => {
            let expr = Box::new(second_pass_expr(
                *expr,
                classes,
                current_class_name,
                file_name,
                object_types,
                method_sigs,
                parent_names,
                families,
            )?);

            if !expr.static_type().is_int() {
                return Err(format!(
                    "{} : {} - Argument of '~' has type {} instead of Int",
                    file_name,
                    line_no,
                    expr.static_type()
                ));
            }

            Expression::Neg { line_no, expr }
        }
        Expression::Comp {
            line_no,
            expr_lhs,
            expr_rhs,
            comp_type,
        } => {
            let expr_lhs = Box::new(second_pass_expr(
                *expr_lhs,
                classes,
                current_class_name,
                file_name,
                object_types,
                method_sigs,
                parent_names,
                families,
            )?);

            let expr_rhs = Box::new(second_pass_expr(
                *expr_rhs,
                classes,
                current_class_name,
                file_name,
                object_types,
                method_sigs,
                parent_names,
                families,
            )?);

            if !expr_lhs.static_type().is_int() || !expr_lhs.static_type().is_int() {
                return Err(format!(
                    "{} : {} - non-Int arguments: {} {} {}",
                    file_name,
                    line_no,
                    expr_lhs.static_type(),
                    comp_type,
                    expr_rhs.static_type()
                ));
            }

            Expression::Comp {
                line_no,
                expr_lhs,
                expr_rhs,
                comp_type,
            }
        }
        Expression::Eq {
            line_no,
            expr_lhs,
            expr_rhs,
        } => {
            let expr_lhs = Box::new(second_pass_expr(
                *expr_lhs,
                classes,
                current_class_name,
                file_name,
                object_types,
                method_sigs,
                parent_names,
                families,
            )?);

            let expr_rhs = Box::new(second_pass_expr(
                *expr_rhs,
                classes,
                current_class_name,
                file_name,
                object_types,
                method_sigs,
                parent_names,
                families,
            )?);

            if expr_lhs.static_type().is_int() && !expr_rhs.static_type().is_int() {
                return Err(format!(
                    "{} : {} - Illegal comparison with a basic type.",
                    file_name, line_no
                ));
            }
            if !expr_lhs.static_type().is_int() && expr_rhs.static_type().is_int() {
                return Err(format!(
                    "{} : {} - Illegal comparison with a basic type.",
                    file_name, line_no
                ));
            }

            if expr_lhs.static_type().is_string() && !expr_rhs.static_type().is_string() {
                return Err(format!(
                    "{} : {} - Illegal comparison with a basic type.",
                    file_name, line_no
                ));
            }
            if !expr_lhs.static_type().is_string() && expr_rhs.static_type().is_string() {
                return Err(format!(
                    "{} : {} - Illegal comparison with a basic type.",
                    file_name, line_no
                ));
            }

            if expr_lhs.static_type().is_bool() && !expr_rhs.static_type().is_bool() {
                return Err(format!(
                    "{} : {} - Illegal comparison with a basic type.",
                    file_name, line_no
                ));
            }
            if !expr_lhs.static_type().is_bool() && expr_rhs.static_type().is_bool() {
                return Err(format!(
                    "{} : {} - Illegal comparison with a basic type.",
                    file_name, line_no
                ));
            }

            Expression::Eq {
                line_no,
                expr_lhs,
                expr_rhs,
            }
        }
        Expression::Not { line_no, expr } => {
            let expr = Box::new(second_pass_expr(
                *expr,
                classes,
                current_class_name,
                file_name,
                object_types,
                method_sigs,
                parent_names,
                families,
            )?);

            if !expr.static_type().is_bool() {
                return Err(format!(
                    "{} : {} - Argument of 'not' has type {} instead of Bool.",
                    file_name,
                    line_no,
                    expr.static_type()
                ));
            }

            Expression::Not { line_no, expr }
        }
        Expression::IntConst { .. } => expr,
        Expression::BoolConst { .. } => expr,
        Expression::StringConst { .. } => expr,
        Expression::New { .. } => expr,
        Expression::IsVoid { line_no, expr } => {
            let expr = Box::new(second_pass_expr(
                *expr,
                classes,
                current_class_name,
                file_name,
                object_types,
                method_sigs,
                parent_names,
                families,
            )?);

            Expression::IsVoid { line_no, expr }
        }
        Expression::NoExpr => expr,
        Expression::VarByName { line_no, name, .. } => {
            let static_type = if name.is_self() {
                TypeID::new_self_type()
            } else {
                match object_types.get(&name) {
                    Some(t) => t.clone(),
                    None => {
                        return Err(format!(
                            "{} : {} - Undeclared identifier {}.",
                            file_name, line_no, name
                        ))
                    }
                }
            };

            Expression::VarByName {
                line_no,
                name,
                static_type,
            }
        }
    })
}
