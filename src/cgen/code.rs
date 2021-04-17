use std::io;

use indexmap::IndexMap;

use super::emit::*;
use crate::ast::{ArithOpType, Branch, Class, CompType, Expression, ObjectID, TypeID};
use crate::scoped_collections::ScopedIndexMap;

use super::{Label, MemLocation};
use super::{DISPTABLE_LOCATION, TAG_LOCATION};

// Output code for an expression
pub fn code_expr(
    out_file: &mut Box<dyn io::Write>,
    expr: &Expression,
    current_class: &Class,
    classes: &IndexMap<TypeID, Class>,
    object_locations: &mut ScopedIndexMap<ObjectID, MemLocation>,
    fp_offset: i16,
) -> Result<(), io::Error> {
    match expr {
        Expression::Assign { name, expr, .. } => {
            // Set ACC to point to the expr's value
            code_expr(
                out_file,
                expr,
                current_class,
                classes,
                object_locations,
                fp_offset,
            )?;

            emit_store_word(
                out_file,
                Register::ACC,
                *object_locations.get(name).unwrap(),
            )?;
        }
        Expression::Dispatch {
            line_no,
            expr,
            type_name,
            name,
            args,
            ..
        } => {
            // Push arguments onto stack
            for arg in args.iter() {
                code_expr(
                    out_file,
                    arg,
                    current_class,
                    classes,
                    object_locations,
                    fp_offset,
                )?;

                emit_push(out_file, Register::ACC)?;
            }

            // Evaluate expression and keep at ACC
            code_expr(
                out_file,
                expr,
                current_class,
                classes,
                object_locations,
                fp_offset,
            )?;

            let l = Label::new();

            // Dispatch on void
            emit_bnez(out_file, Register::ACC, l)?;

            emit_load_string(out_file, Register::ACC, "file_name", current_class.file_no)?;
            emit_load_imm(out_file, Register::T1, *line_no)?;
            emit_jal(out_file, "_dispatch_abort")?;

            emit_label_def(out_file, l)?;

            // Find the position of this method
            let method_pos = match type_name {
                Some(type_name) => {
                    // Static dispatch

                    emit_load_address(out_file, Register::T1, format!("{}_dispTab", type_name))?;

                    classes.get(type_name).unwrap()
                }

                None => {
                    // "Dynamic" dispatch

                    emit_load_word(out_file, Register::T1, DISPTABLE_LOCATION)?;

                    if expr.static_type().is_self_type() {
                        current_class
                    } else {
                        classes.get(&expr.static_type()).unwrap()
                    }
                }
            }
            .method_name_to_pos
            .get(name)
            .unwrap();

            // Load method added from dispatch table to T1
            emit_load_word(
                out_file,
                Register::T1,
                MemLocation {
                    reg: Register::T1,
                    offset: *method_pos,
                },
            )?;

            // jump to method
            emit_jalr(out_file, Register::T1)?;
        }
        Expression::Cond {
            pred,
            then_expr,
            else_expr,
            ..
        } => {
            let label_else = Label::new();
            let label_fi = Label::new();

            // Evaluate predicate
            code_expr(
                out_file,
                pred,
                current_class,
                classes,
                object_locations,
                fp_offset,
            )?;

            // Get boolean value returned
            emit_fetch_int(out_file, Register::T1, Register::ACC)?;

            // If false (zero) jump to else clause
            emit_beqz(out_file, Register::T1, label_else)?;

            // Evaluate then_expr
            code_expr(
                out_file,
                then_expr,
                current_class,
                classes,
                object_locations,
                fp_offset,
            )?;

            // Unconditional branch to end of if statement
            emit_branch(out_file, label_fi)?;
            emit_label_def(out_file, label_else)?;

            // Evaluate else_expr
            code_expr(
                out_file,
                else_expr,
                current_class,
                classes,
                object_locations,
                fp_offset,
            )?;

            emit_label_def(out_file, label_fi)?;
        }
        Expression::Loop { pred, body, .. } => {
            let label_start = Label::new();
            let label_end = Label::new();

            emit_label_def(out_file, label_start)?;

            // Evaluate predicate
            code_expr(
                out_file,
                pred,
                current_class,
                classes,
                object_locations,
                fp_offset,
            )?;

            // Get boolean value returned
            emit_fetch_int(out_file, Register::T1, Register::ACC)?;

            // if false (zero) jump to end
            emit_beqz(out_file, Register::T1, label_end)?;

            // Evaluate contents
            code_expr(
                out_file,
                body,
                current_class,
                classes,
                object_locations,
                fp_offset,
            )?;

            // Unconditional branch to beginning
            emit_branch(out_file, label_start)?;

            emit_label_def(out_file, label_end)?;

            // Set ACC to void
            emit_move(out_file, Register::ACC, Register::ZERO)?;
        }
        Expression::TypeCase {
            line_no,
            expr,
            branches,
            ..
        } => {
            // Evaluate expression
            code_expr(
                out_file,
                expr,
                current_class,
                classes,
                object_locations,
                fp_offset,
            )?;

            let label_notvoid = Label::new();
            emit_bnez(out_file, Register::ACC, label_notvoid)?; // jump over fail if not void

            // Load filename and line number for crash
            emit_load_string(out_file, Register::ACC, "file_name", current_class.file_no)?;
            emit_load_imm(out_file, Register::T1, *line_no as i16)?;
            emit_jal(out_file, "_case_abort2")?;

            emit_label_def(out_file, label_notvoid)?;

            // Load tag to T2
            emit_load_word(out_file, Register::T2, TAG_LOCATION)?;

            let label_end = Label::new();

            // Code each branch
            // Note that the order is already correct
            for branch in branches.iter() {
                code_branch(
                    out_file,
                    branch,
                    current_class,
                    classes,
                    object_locations,
                    fp_offset,
                    label_end,
                )?;
            }

            // If no branch succeeds
            emit_jal(out_file, "_case_abort")?;

            emit_label_def(out_file, label_end)?;
        }
        Expression::Block { body, .. } => {
            // Evaluate each expression in body
            for expr in body.iter() {
                code_expr(
                    out_file,
                    expr,
                    current_class,
                    classes,
                    object_locations,
                    fp_offset,
                )?;
            }
        }
        Expression::Let {
            identifier,
            type_decl,
            init,
            body,
            ..
        } => {
            object_locations.enter_scope(); // Scope for new variable

            // Evaluate init expression

            if let Expression::NoExpr = **init {
                if type_decl.is_int() {
                    emit_load_int(out_file, Register::ACC, 0)?;
                } else if type_decl.is_bool() {
                    emit_load_bool(out_file, Register::ACC, false)?;
                } else if type_decl.is_string() {
                    emit_load_string(out_file, Register::ACC, "str_const", 0)?;
                } else {
                    emit_load_imm(out_file, Register::ACC, 0)?; // Void
                }
            } else {
                code_expr(
                    out_file,
                    init,
                    current_class,
                    classes,
                    object_locations,
                    fp_offset,
                )?;
            }

            // Push to stack and save location relative to FP
            emit_push(out_file, Register::ACC)?;
            object_locations.insert(
                identifier.clone(),
                MemLocation {
                    reg: Register::FP,
                    offset: fp_offset,
                },
            );

            // Evaluate body
            code_expr(
                out_file,
                body,
                current_class,
                classes,
                object_locations,
                fp_offset - 1,
            )?;

            // Pop the result of init to nowhere
            emit_pop(out_file, Register::ZERO)?;

            object_locations.exit_scope();
        }
        Expression::ArithOp {
            expr_lhs,
            expr_rhs,
            arith_op_type,
            ..
        } => {
            // Push LHS to stack
            code_expr(
                out_file,
                expr_lhs,
                current_class,
                classes,
                object_locations,
                fp_offset,
            )?;

            emit_push(out_file, Register::ACC)?;

            // Evaluate RHS
            code_expr(
                out_file,
                expr_rhs,
                current_class,
                classes,
                object_locations,
                fp_offset,
            )?;

            // New integer (same value as RHS)
            emit_jal(out_file, "Object.copy")?;

            // Get the RHS value and put it in T1
            emit_fetch_int(out_file, Register::T1, Register::ACC)?;

            // Get the LHS from the top of the stack and put it in T2
            emit_load_word(
                out_file,
                Register::T2,
                MemLocation {
                    // TODO
                    reg: Register::SP,
                    offset: 1,
                },
            )?;

            // Put the LHS value in T3
            emit_fetch_int(out_file, Register::T3, Register::T2)?;

            // Execute the operation and put to result in T1
            match arith_op_type {
                ArithOpType::Add => {
                    emit_add(out_file, Register::T1, Register::T3, Register::T1)?;
                }
                ArithOpType::Sub => {
                    emit_sub(out_file, Register::T1, Register::T3, Register::T1)?;
                }
                ArithOpType::Mul => {
                    emit_mul(out_file, Register::T1, Register::T3, Register::T1)?;
                }
                ArithOpType::Div => {
                    emit_div(out_file, Register::T1, Register::T3, Register::T1)?;
                }
            }

            // Put the result from T1 into the new integer
            emit_store_int(out_file, Register::T1, Register::ACC)?;

            // Pop LHS to nowhere
            emit_pop(out_file, Register::ZERO)?;
        }
        Expression::Neg { expr, .. } => {
            // Evaluate
            code_expr(
                out_file,
                expr,
                current_class,
                classes,
                object_locations,
                fp_offset,
            )?;

            // New integer (same value)
            emit_jal(out_file, "Object.copy")?;

            // Get value
            emit_fetch_int(out_file, Register::T1, Register::ACC)?;

            // Negate current value
            emit_neg(out_file, Register::T1, Register::T1)?;

            // Store value
            emit_store_int(out_file, Register::T1, Register::ACC)?;
        }
        Expression::Comp {
            expr_lhs,
            expr_rhs,
            comp_type,
            ..
        } => {
            let label = Label::new();

            // Push LHS to stack
            code_expr(
                out_file,
                expr_lhs,
                current_class,
                classes,
                object_locations,
                fp_offset,
            )?;

            emit_push(out_file, Register::ACC)?;

            // Evaluate RHS
            code_expr(
                out_file,
                expr_rhs,
                current_class,
                classes,
                object_locations,
                fp_offset,
            )?;

            // New integer (same value as RHS)
            emit_jal(out_file, "Object.copy")?;

            // Get the RHS value and put it in T1
            emit_fetch_int(out_file, Register::T1, Register::ACC)?;

            // Get the LHS from the top of the stack and put it in T2
            emit_load_word(
                out_file,
                Register::T2,
                MemLocation {
                    // TODO
                    reg: Register::SP,
                    offset: 1,
                },
            )?;

            // Put the LHS value in T3
            emit_fetch_int(out_file, Register::T3, Register::T2)?;

            // Point ACC to true constant
            emit_load_bool(out_file, Register::ACC, true)?;

            // Execute the comparison
            // If true branch to end
            match comp_type {
                CompType::LT => {
                    emit_blt(out_file, Register::T3, Register::T1, label)?;
                }
                CompType::LEq => {
                    emit_bleq(out_file, Register::T3, Register::T1, label)?;
                }
            }

            // Point ACC to false constant
            emit_load_bool(out_file, Register::ACC, false)?;

            emit_label_def(out_file, label)?;

            // Pop LHS to nowhere
            emit_pop(out_file, Register::ZERO)?;
        }
        Expression::Eq {
            expr_lhs, expr_rhs, ..
        } => {
            let label = Label::new();

            // Push LHS to stack
            code_expr(
                out_file,
                expr_lhs,
                current_class,
                classes,
                object_locations,
                fp_offset,
            )?;

            emit_push(out_file, Register::ACC)?;

            // Evaluate RHS
            code_expr(
                out_file,
                expr_rhs,
                current_class,
                classes,
                object_locations,
                fp_offset,
            )?;

            // RHS to T2
            emit_move(out_file, Register::T2, Register::ACC)?;

            // LHS to T1
            emit_pop(out_file, Register::T1)?;

            emit_load_bool(out_file, Register::ACC, true)?;

            // Test for structural equality
            emit_beq(out_file, Register::T1, Register::T2, label)?;

            emit_load_bool(out_file, Register::A1, false)?;
            emit_jal(out_file, "equality_test")?; // jump to equality test
            emit_label_def(out_file, label)?;
        }
        Expression::Not { expr, .. } => {
            // Evaluate expression
            code_expr(
                out_file,
                expr,
                current_class,
                classes,
                object_locations,
                fp_offset,
            )?;

            let label = Label::new();
            emit_fetch_int(out_file, Register::T1, Register::ACC)?;
            emit_load_bool(out_file, Register::ACC, true)?;

            // if value is zero (false) jump to end
            emit_beqz(out_file, Register::T1, label)?;
            emit_load_bool(out_file, Register::ACC, false)?;
            emit_label_def(out_file, label)?;
        }
        Expression::IntConst { val, .. } => {
            emit_load_int(out_file, Register::ACC, *val)?;
        }
        Expression::BoolConst { val, .. } => {
            emit_load_bool(out_file, Register::ACC, *val)?;
        }
        Expression::StringConst { val_id, .. } => {
            emit_load_string(out_file, Register::ACC, "str_const", *val_id)?;
        }
        Expression::New { type_name, .. } => {
            if type_name.is_self_type() {
                // address of class_objTab to T1
                emit_load_address(out_file, Register::T1, "class_objTab".to_string())?;

                // set T2 to current class tag
                emit_load_word(out_file, Register::T2, TAG_LOCATION)?;

                // Multiply tag by 8 (2 WORDs)
                emit_sll(out_file, Register::T2, Register::T2, 3)?;

                // Get prototype and init method
                emit_addu(out_file, Register::T1, Register::T1, Register::T2)?;
                emit_move(out_file, Register::T3, Register::T1)?;
                emit_push(out_file, Register::T3)?;

                // Copy prototype
                emit_load_word(
                    out_file,
                    Register::ACC,
                    MemLocation {
                        reg: Register::T1,
                        offset: 0,
                    },
                )?;
                emit_jal(out_file, "Object.copy")?;

                emit_pop(out_file, Register::T3)?;

                // Call init method
                emit_load_word(
                    out_file,
                    Register::T1,
                    MemLocation {
                        reg: Register::T3,
                        offset: 1, // TODO
                    },
                )?;
                emit_jalr(out_file, Register::T1)?;
            } else {
                // Copy prototype object
                emit_load_address(out_file, Register::ACC, format!("{}_protObj", type_name))?;
                emit_jal(out_file, "Object.copy")?;

                // Initialize object
                emit_jal(out_file, &format!("{}_init", type_name))?;
            }
        }
        Expression::IsVoid { expr, .. } => {
            let label = Label::new();

            code_expr(
                out_file,
                expr,
                current_class,
                classes,
                object_locations,
                fp_offset,
            )?;

            emit_move(out_file, Register::T1, Register::ACC)?;
            emit_load_bool(out_file, Register::ACC, true)?;

            // Test if equal to zero (Void)
            emit_beqz(out_file, Register::T1, label)?;

            emit_load_bool(out_file, Register::ACC, false)?;
            emit_label_def(out_file, label)?;
        }
        Expression::NoExpr => {} // Never used
        Expression::Object { name, .. } => {
            if name.is_self() {
                // self always refers to SELF register
                emit_move(out_file, Register::ACC, Register::SELF)?;
            } else {
                // Lookup location

                emit_load_word(
                    out_file,
                    Register::ACC,
                    *object_locations.get(name).unwrap(),
                )?;
            }
        }
    }

    Ok(())
}

// Code one branch of a TypeCase
pub fn code_branch(
    out_file: &mut Box<dyn io::Write>,
    Branch {
        name, expr, family, ..
    }: &Branch,
    current_class: &Class,
    classes: &IndexMap<TypeID, Class>,
    object_locations: &mut ScopedIndexMap<ObjectID, MemLocation>,
    fp_offset: i16,
    label_end: Label,
) -> Result<(), io::Error> {
    let label_end_branch = Label::new();

    // If tag is outside family, this branch doesn't match
    emit_blti(out_file, Register::T2, family.start, label_end_branch)?;
    emit_bgei(out_file, Register::T2, family.end, label_end_branch)?;

    object_locations.enter_scope(); // Scope for new variable

    // Save expression to name
    emit_push(out_file, Register::ACC)?;
    object_locations.insert(
        name.clone(),
        MemLocation {
            reg: Register::FP,
            offset: fp_offset,
        },
    );

    // Evaluate expression
    code_expr(
        out_file,
        expr,
        current_class,
        classes,
        object_locations,
        fp_offset - 1,
    )?;

    emit_pop(out_file, Register::ZERO)?;

    object_locations.exit_scope();

    emit_branch(out_file, label_end)?;
    emit_label_def(out_file, label_end_branch)?;

    Ok(())
}
