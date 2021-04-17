use indexmap::{IndexMap, IndexSet};

use super::first_pass_branch::first_pass_branch;
use crate::ast::{Branch, Class, Expression, TypeID};

use std::ops::Range;

pub fn first_pass_expr(
    expr: Expression,
    classes: &IndexMap<TypeID, Class>,
    current_class_name: &TypeID,
    current_class_family: &Range<u32>,
    file_name: &str,
    int_table: &mut IndexSet<u32>,
    str_consts: &mut IndexMap<String, u32>,
    str_table: &mut Vec<String>,
) -> Result<Expression, String> {
    Ok(match expr {
        Expression::Assign {
            line_no,
            name,
            expr,
        } => {
            let expr = Box::new(first_pass_expr(
                *expr,
                classes,
                current_class_name,
                current_class_family,
                file_name,
                int_table,
                str_consts,
                str_table,
            )?);

            if name.is_self() {
                return Err(format!(
                    "{} : {} - Cannot assign to 'self'.",
                    file_name, line_no
                ));
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
            static_type,
        } => {
            let expr = Box::new(first_pass_expr(
                *expr,
                classes,
                current_class_name,
                current_class_family,
                file_name,
                int_table,
                str_consts,
                str_table,
            )?);

            let args = {
                let mut new_args: Vec<Expression> = Vec::new();
                for arg in args.drain(..) {
                    new_args.push(first_pass_expr(
                        arg,
                        classes,
                        current_class_name,
                        current_class_family,
                        file_name,
                        int_table,
                        str_consts,
                        str_table,
                    )?);
                }

                new_args
            };

            let type_name = match type_name {
                Some(type_name) => {
                    // static dispatch
                    if type_name.is_self_type() {
                        return Err(format!(
                            "{} : {} - Static dispatch to SELF_TYPE.",
                            file_name, line_no
                        ));
                    }

                    if type_name != *current_class_name && !classes.contains_key(&type_name) {
                        return Err(format!(
                            "{} : {} - Static dispatch to undefined class {}.",
                            file_name, line_no, type_name
                        ));
                    }

                    Some(type_name)
                }
                None => None, // "dynamic" dispatch - nothing to check this time
            };

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
            static_type,
        } => {
            let pred = Box::new(first_pass_expr(
                *pred,
                classes,
                current_class_name,
                current_class_family,
                file_name,
                int_table,
                str_consts,
                str_table,
            )?);

            let then_expr = Box::new(first_pass_expr(
                *then_expr,
                classes,
                current_class_name,
                current_class_family,
                file_name,
                int_table,
                str_consts,
                str_table,
            )?);

            let else_expr = Box::new(first_pass_expr(
                *else_expr,
                classes,
                current_class_name,
                current_class_family,
                file_name,
                int_table,
                str_consts,
                str_table,
            )?);

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
            let pred = Box::new(first_pass_expr(
                *pred,
                classes,
                current_class_name,
                current_class_family,
                file_name,
                int_table,
                str_consts,
                str_table,
            )?);

            let body = Box::new(first_pass_expr(
                *body,
                classes,
                current_class_name,
                current_class_family,
                file_name,
                int_table,
                str_consts,
                str_table,
            )?);

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
            static_type,
        } => {
            let mut branch_types: IndexSet<TypeID> = IndexSet::new();

            let expr = Box::new(first_pass_expr(
                *expr,
                classes,
                current_class_name,
                current_class_family,
                file_name,
                int_table,
                str_consts,
                str_table,
            )?);

            let branches = {
                let mut new_branches: Vec<Branch> = Vec::new();
                for branch in branches.drain(..) {
                    let branch = first_pass_branch(
                        branch,
                        classes,
                        current_class_name,
                        current_class_family,
                        file_name,
                        int_table,
                        str_consts,
                        str_table,
                    )?;

                    if !branch_types.insert(branch.type_decl.clone()) {
                        return Err(format!(
                            "{} : {} - Duplicate branch {} in case statement.",
                            file_name, branch.line_no, branch.type_decl
                        ));
                    }

                    new_branches.push(branch);
                }

                new_branches.sort_unstable();

                new_branches
            };

            Expression::TypeCase {
                line_no,
                expr,
                branches,
                static_type,
            }
        }
        Expression::Block { line_no, mut body } => {
            let body = {
                let mut new_body: Vec<Expression> = Vec::new();
                for expr in body.drain(..) {
                    new_body.push(first_pass_expr(
                        expr,
                        classes,
                        current_class_name,
                        current_class_family,
                        file_name,
                        int_table,
                        str_consts,
                        str_table,
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
        } => {
            if identifier.is_self() {
                return Err(format!(
                    "{} : {} - 'self' cannot be bound in a 'let' expression.",
                    file_name, line_no
                ));
            }

            let init = Box::new(first_pass_expr(
                *init,
                classes,
                current_class_name,
                current_class_family,
                file_name,
                int_table,
                str_consts,
                str_table,
            )?);

            let body = Box::new(first_pass_expr(
                *body,
                classes,
                current_class_name,
                current_class_family,
                file_name,
                int_table,
                str_consts,
                str_table,
            )?);

            if !type_decl.is_self_type()
                && type_decl != *current_class_name
                && !classes.contains_key(&type_decl)
            {
                return Err(format!(
                    "{} : {} - Class {} of let-bound identifier {} is undefined.",
                    file_name, line_no, type_decl, identifier
                ));
            }

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
            let expr_lhs = Box::new(first_pass_expr(
                *expr_lhs,
                classes,
                current_class_name,
                current_class_family,
                file_name,
                int_table,
                str_consts,
                str_table,
            )?);

            let expr_rhs = Box::new(first_pass_expr(
                *expr_rhs,
                classes,
                current_class_name,
                current_class_family,
                file_name,
                int_table,
                str_consts,
                str_table,
            )?);

            Expression::ArithOp {
                line_no,
                expr_lhs,
                expr_rhs,
                arith_op_type,
            }
        }
        Expression::Neg { line_no, expr } => {
            let expr = Box::new(first_pass_expr(
                *expr,
                classes,
                current_class_name,
                current_class_family,
                file_name,
                int_table,
                str_consts,
                str_table,
            )?);

            Expression::Neg { line_no, expr }
        }
        Expression::Comp {
            line_no,
            expr_lhs,
            expr_rhs,
            comp_type,
        } => {
            let expr_lhs = Box::new(first_pass_expr(
                *expr_lhs,
                classes,
                current_class_name,
                current_class_family,
                file_name,
                int_table,
                str_consts,
                str_table,
            )?);

            let expr_rhs = Box::new(first_pass_expr(
                *expr_rhs,
                classes,
                current_class_name,
                current_class_family,
                file_name,
                int_table,
                str_consts,
                str_table,
            )?);

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
            let expr_lhs = Box::new(first_pass_expr(
                *expr_lhs,
                classes,
                current_class_name,
                current_class_family,
                file_name,
                int_table,
                str_consts,
                str_table,
            )?);

            let expr_rhs = Box::new(first_pass_expr(
                *expr_rhs,
                classes,
                current_class_name,
                current_class_family,
                file_name,
                int_table,
                str_consts,
                str_table,
            )?);

            Expression::Eq {
                line_no,
                expr_lhs,
                expr_rhs,
            }
        }
        Expression::Not { line_no, expr } => {
            let expr = Box::new(first_pass_expr(
                *expr,
                classes,
                current_class_name,
                current_class_family,
                file_name,
                int_table,
                str_consts,
                str_table,
            )?);

            Expression::Not { line_no, expr }
        }
        Expression::IntConst { val, .. } => {
            int_table.insert(val);

            expr
        }
        Expression::BoolConst { .. } => expr,
        Expression::StringConst { line_no, val, .. } => {
            let val_id = match str_consts.get(&val) {
                Some(val_id) => *val_id, // If this string is already in the table
                None => {
                    int_table.insert(val.len() as u32);

                    str_table.push(val.clone());
                    str_consts.insert(val.clone(), (str_table.len() - 1) as u32);
                    (str_table.len() - 1) as u32
                }
            };

            Expression::StringConst {
                line_no,
                val,
                val_id,
            }
        }
        Expression::New { line_no, type_name } => {
            if !type_name.is_self_type()
                && type_name != *current_class_name
                && !classes.contains_key(&type_name)
            {
                return Err(format!(
                    "{} : {} - 'new' used with undefined class {}.",
                    file_name, line_no, type_name
                ));
            }

            Expression::New { line_no, type_name }
        }
        Expression::IsVoid { line_no, expr } => {
            let expr = Box::new(first_pass_expr(
                *expr,
                classes,
                current_class_name,
                current_class_family,
                file_name,
                int_table,
                str_consts,
                str_table,
            )?);

            Expression::IsVoid { line_no, expr }
        }
        Expression::NoExpr => expr,
        Expression::Object { .. } => expr,
    })
}
