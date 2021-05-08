use crate::ast::{Attr, Class, Expression, Method, ObjectID, TypeID};
use crate::token::Token;

use self::node::ParseStackNode;
use self::parse_table::{Action, DispatchType, State};
use std::collections::VecDeque;

mod node;
mod parse_table;

use self::parse_table::get_action;

use self::parse_table::get_reduce_new_state_branch;
use self::parse_table::get_reduce_new_state_branch_list;
use self::parse_table::get_reduce_new_state_class;
use self::parse_table::get_reduce_new_state_class_list;
use self::parse_table::get_reduce_new_state_expression;
use self::parse_table::get_reduce_new_state_expression_let;
use self::parse_table::get_reduce_new_state_expression_list_c;
use self::parse_table::get_reduce_new_state_expression_list_c_ne;
use self::parse_table::get_reduce_new_state_expression_list_sc;
use self::parse_table::get_reduce_new_state_feature;
use self::parse_table::get_reduce_new_state_feature_list;
use self::parse_table::get_reduce_new_state_formal;
use self::parse_table::get_reduce_new_state_formal_list;
use self::parse_table::get_reduce_new_state_formal_list_ne;

use indexmap::IndexMap;

// Shift / Reduce parser
struct Parser {
    tokens: VecDeque<Token>,
    stack: Vec<(ParseStackNode, State)>,
}

impl Parser {
    fn new(tokens: VecDeque<Token>) -> Parser {
        Parser {
            tokens,
            stack: vec![(ParseStackNode::Bottom, State::StateInitial)], // Initialize the stack
        }
    }

    fn top_state(&self) -> State {
        self.stack.last().unwrap().1
    }

    fn push(&mut self, node: ParseStackNode, state: State) {
        self.stack.push((node, state));
    }

    fn pop(&mut self) -> ParseStackNode {
        self.stack.pop().unwrap().0
    }

    // Remove a token and place its equivalent node on the stack
    fn shift(&mut self, state: State) {
        let token = self.tokens.pop_front().unwrap();

        let node = match token {
            Token::Add { line_no } => ParseStackNode::TermAdd { line_no },
            Token::Div { line_no } => ParseStackNode::TermDiv { line_no },
            Token::Sub { line_no } => ParseStackNode::TermSub { line_no },
            Token::Mul { line_no } => ParseStackNode::TermMul { line_no },
            Token::Eq { line_no } => ParseStackNode::TermEq { line_no },
            Token::LT { line_no } => ParseStackNode::TermLT { line_no },
            Token::Dot { line_no } => ParseStackNode::TermDot { line_no },
            Token::Neg { line_no } => ParseStackNode::TermNeg { line_no },
            Token::Comma { line_no } => ParseStackNode::TermComma { line_no },
            Token::SemiColon { line_no } => ParseStackNode::TermSemiColon { line_no },
            Token::Colon { line_no } => ParseStackNode::TermColon { line_no },
            Token::OpenParen { line_no } => ParseStackNode::TermOpenParen { line_no },
            Token::CloseParen { line_no } => ParseStackNode::TermCloseParen { line_no },
            Token::At { line_no } => ParseStackNode::TermAt { line_no },
            Token::OpenBrace { line_no } => ParseStackNode::TermOpenBrace { line_no },
            Token::CloseBrace { line_no } => ParseStackNode::TermCloseBrace { line_no },
            Token::Class { line_no } => ParseStackNode::TermClass { line_no },
            Token::Else { line_no } => ParseStackNode::TermElse { line_no },
            Token::Fi { line_no } => ParseStackNode::TermFi { line_no },
            Token::If { line_no } => ParseStackNode::TermIf { line_no },
            Token::In { line_no } => ParseStackNode::TermIn { line_no },
            Token::Inherits { line_no } => ParseStackNode::TermInherits { line_no },
            Token::Let { line_no } => ParseStackNode::TermLet { line_no },
            Token::Loop { line_no } => ParseStackNode::TermLoop { line_no },
            Token::Pool { line_no } => ParseStackNode::TermPool { line_no },
            Token::Then { line_no } => ParseStackNode::TermThen { line_no },
            Token::While { line_no } => ParseStackNode::TermWhile { line_no },
            Token::Case { line_no } => ParseStackNode::TermCase { line_no },
            Token::Esac { line_no } => ParseStackNode::TermEsac { line_no },
            Token::Of { line_no } => ParseStackNode::TermOf { line_no },
            Token::New { line_no } => ParseStackNode::TermNew { line_no },
            Token::IsVoid { line_no } => ParseStackNode::TermIsVoid { line_no },
            Token::Not { line_no } => ParseStackNode::TermNot { line_no },
            Token::DArrow { line_no } => ParseStackNode::TermDArrow { line_no },
            Token::LEq { line_no } => ParseStackNode::TermLEq { line_no },
            Token::Assign { line_no } => ParseStackNode::TermAssign { line_no },
            Token::BoolConst { line_no, val } => ParseStackNode::TermBoolConst { line_no, val },
            Token::TypeID { line_no, type_name } => {
                ParseStackNode::TermTypeID { line_no, type_name }
            }
            Token::ObjectID { line_no, obj_name } => {
                ParseStackNode::TermObjectID { line_no, obj_name }
            }
            Token::IntConst { line_no, val } => ParseStackNode::TermIntConst { line_no, val },
            Token::StrConst { line_no, val } => ParseStackNode::TermStrConst { line_no, val },
            Token::End { .. } => panic!("Cannot shift End token"),
        };

        self.push(node, state);
    }

    // All reduce functions remove nodes from the stack, assemble them into one new node and push that node onto the stack.

    fn reduce_class_list(
        &mut self,
        classes: &mut IndexMap<TypeID, Class>,
        in_file_name: &str,
        is_empty: bool,
    ) -> Result<(), String> {
        // class_list:            class ';'
        // class_list: class_list class ';'

        self.pop(); // SemiColon
        let (name, class) = self.pop().into_class();

        let class_list = if is_empty {
            ParseStackNode::ClassList
        } else {
            self.pop()
        };

        let line_no = class.line_no;

        if let Some(prev_case) = classes.insert(name.clone(), class) {
            if prev_case.basic {
                return Err(format!(
                    "{} : {} - Redefinition of basic class {}.",
                    in_file_name, line_no, name
                ));
            } else {
                return Err(format!(
                    "{} : {} - Class {} was previously defined.",
                    in_file_name, line_no, name
                ));
            }
        }

        let new_node = class_list;

        let new_state = get_reduce_new_state_class_list(self.top_state());

        self.push(new_node, new_state);

        Ok(())
    }

    fn reduce_class(&mut self, in_file_name: &str, file_no: u32, has_parent: bool) {
        // class: CLASS TYPEID                 '{' feature_list '}'
        // class: CLASS TYPEID INHERITS TYPEID '{' feature_list '}'

        self.pop(); // CloseBrace
        let features = self.pop();
        self.pop(); // OpenBrace

        let parent_name = Some(if has_parent {
            let parent_name = self.pop().into_type_id();
            self.pop(); // INHERITS
            parent_name
        } else {
            TypeID::root_class_name()
        });

        let name = self.pop().into_type_id();
        let line_no = self.pop().line_no(); // CLASS

        let (attrs, methods) = features.extract_features();

        let new_node = ParseStackNode::Class {
            file_name: in_file_name.to_string(),
            file_no,
            line_no,
            name,
            parent_name,
            attrs,
            methods,
        };

        let new_state = get_reduce_new_state_class(self.top_state());

        self.push(new_node, new_state);
    }

    fn reduce_feature_list(&mut self, in_file_name: &str, is_empty: bool) -> Result<(), String> {
        // feature_list: /* empty */
        // feature_list: feature_list feature ';'

        let (attrs, methods) = if is_empty {
            (Vec::<Attr>::new(), IndexMap::<ObjectID, Method>::new())
        } else {
            self.pop(); // SemiColon
            let feature = self.pop();
            let features = self.pop();

            let (mut attrs, mut methods) = features.extract_features();

            match feature {
                ParseStackNode::Attr { .. } => {
                    attrs.push(feature.into_attr());
                }
                ParseStackNode::Method { line_no, .. } => {
                    let (name, method) = feature.into_method();

                    if methods.insert(name.clone(), method).is_some() {
                        return Err(format!(
                            "{} : {} - Method {} is multiply defined.",
                            in_file_name, line_no, name
                        ));
                    }
                }
                _ => {
                    panic!("Bad ParseStackNode type");
                }
            }

            (attrs, methods)
        };

        let new_node = ParseStackNode::FeatureList { attrs, methods };

        let new_state = get_reduce_new_state_feature_list(self.top_state());

        self.push(new_node, new_state);

        Ok(())
    }

    fn reduce_method(&mut self) {
        // feature: OBJECTID '(' formal_list ')' ':' TYPEID '{' expression '}'

        self.pop(); // CloseBrace
        let expr = self.pop().into_expression();
        self.pop(); // OpenBrace
        let return_type = self.pop().into_type_id();
        self.pop(); // Colon
        self.pop(); // CloseParen
        let formal_list = self.pop();
        self.pop(); // OpenParen
        let (line_no, name) = self.pop().get_line_no_object_id();

        let formals = formal_list.extract_formals();

        let new_node = ParseStackNode::Method {
            line_no,
            name,
            formals,
            return_type,
            expr,
        };

        let new_state = get_reduce_new_state_feature(self.top_state());

        self.push(new_node, new_state);
    }

    fn reduce_attr(&mut self, has_init: bool) {
        // feature: OBJECTID ':' TYPEID
        // feature: OBJECTID ':' TYPEID ASSIGN expression

        let init = if has_init {
            let init = self.pop().into_expression();
            self.pop(); // ASSIGN
            init
        } else {
            Expression::NoExpr
        };

        let type_decl = self.pop().into_type_id();
        self.pop(); // Colon
        let (line_no, name) = self.pop().get_line_no_object_id();

        let new_node = ParseStackNode::Attr {
            line_no,
            name,
            type_decl,
            init,
        };

        let new_state = get_reduce_new_state_feature(self.top_state());

        self.push(new_node, new_state);
    }

    fn reduce_formal_list_ne(&mut self, is_empty: bool) {
        // formal_list_nE:                    formal
        // formal_list_nE: formal_list_nE ',' formal

        let formal = self.pop().into_formal();
        let mut formals = if is_empty {
            Vec::new()
        } else {
            self.pop(); // Comma
            let formal_list_ne = self.pop();
            formal_list_ne.extract_formals()
        };

        formals.push(formal);

        let new_node = ParseStackNode::FormalListNotEmpty { formals };

        let new_state = get_reduce_new_state_formal_list_ne(self.top_state());

        self.push(new_node, new_state);
    }

    fn reduce_formal_list(&mut self, is_empty: bool) {
    	// formal_list : /* empty /*
    	// formal_list : formal_list_ne
    	
        let formals = if is_empty {
            Vec::new()
        } else {
            let formal_list_ne = self.pop();
            formal_list_ne.extract_formals()
        };

        let new_node = ParseStackNode::FormalList { formals };

        let new_state = get_reduce_new_state_formal_list(self.top_state());

        self.push(new_node, new_state);
    }

    fn reduce_formal(&mut self) {
        // formal: OBJECTID ':' TYPEID

        let type_decl = self.pop().into_type_id();
        self.pop(); // Colon
        let (line_no, name) = self.pop().get_line_no_object_id();

        let new_node = ParseStackNode::Formal {
            line_no,
            name,
            type_decl,
        };

        let new_state = get_reduce_new_state_formal(self.top_state());

        self.push(new_node, new_state);
    }

    fn reduce_branch_list(&mut self, is_empty: bool) {
        // branch_list:             branch ';'
        // branch_list: branch_list branch ';'

        self.pop(); // SemiColon
        let branch = self.pop().into_branch();

        let mut branches = if is_empty {
            Vec::new()
        } else {
            let branch_list = self.pop();
            branch_list.extract_branches()
        };

        branches.push(branch);

        let new_node = ParseStackNode::BranchList { branches };

        let new_state = get_reduce_new_state_branch_list(self.top_state());

        self.push(new_node, new_state);
    }

    fn reduce_branch(&mut self) {
        // branch: OBJECTID ':' TYPEID DARROW expression

        let expr = self.pop().into_expression();
        self.pop(); // DARROW
        let type_decl = self.pop().into_type_id();
        self.pop(); // Colon
        let (line_no, name) = self.pop().get_line_no_object_id();

        let new_node = ParseStackNode::Branch {
            line_no,
            name,
            type_decl,
            expr,
        };

        let new_state = get_reduce_new_state_branch(self.top_state());

        self.push(new_node, new_state);
    }

    fn reduce_expr_list_sc(&mut self, is_empty: bool) {
        // expression_list_SC:                    expression ';'
        // expression_list_SC: expression_list_SC expression ';'

        self.pop(); // SemiColon
        let expr = self.pop().into_expression();

        let mut exprs = if is_empty {
            Vec::new()
        } else {
            let expr_list = self.pop();
            expr_list.extract_expressions()
        };

        exprs.push(expr);

        let new_node = ParseStackNode::ExpressionListSemiColon { exprs };

        let new_state = get_reduce_new_state_expression_list_sc(self.top_state());

        self.push(new_node, new_state);
    }

    fn reduce_expr_list_c_ne(&mut self, is_empty: bool) {
        // expression_list_C_nE: expression
        // expression_list_C_nE: expression_list_C_nE ',' expression

        let expr = self.pop().into_expression();

        let mut exprs = if is_empty {
            Vec::new()
        } else {
            self.pop(); // Comma
            let expr_list_c_ne = self.pop();
            expr_list_c_ne.extract_expressions()
        };

        exprs.push(expr);

        let new_node = ParseStackNode::ExpressionListCommaNotEmpty { exprs };

        let new_state = get_reduce_new_state_expression_list_c_ne(self.top_state());

        self.push(new_node, new_state);
    }

    fn reduce_expr_list_c(&mut self, is_empty: bool) {
        // expression_list_C: /* empty */
        // expression_list_C: expression_list_C_nE

        let exprs = if is_empty {
            Vec::new()
        } else {
            let expr_list_c_ne = self.pop();

            expr_list_c_ne.extract_expressions()
        };

        let new_node = ParseStackNode::ExpressionListComma { exprs };

        let new_state = get_reduce_new_state_expression_list_c(self.top_state());

        self.push(new_node, new_state);
    }

    fn reduce_expr_let(&mut self, has_init: bool) {
        // expression_let: OBJECTID ':' TYPEID                   IN expression
        // expression_let: OBJECTID ':' TYPEID ASSIGN expression IN expression

        let body = Box::new(self.pop().into_expression());
        self.pop(); // IN or Comma

        let init = Box::new(if has_init {
            let init = self.pop().into_expression();
            self.pop(); // ASSIGN
            init
        } else {
            Expression::NoExpr
        });

        let type_decl = self.pop().into_type_id();
        self.pop(); // Colon
        let (line_no, identifier) = self.pop().get_line_no_object_id();

        let new_node = ParseStackNode::ExpressionLet {
            line_no,
            identifier,
            type_decl,
            init,
            body,
        };

        let new_state = get_reduce_new_state_expression_let(self.top_state());

        self.push(new_node, new_state);
    }

    fn reduce_assign(&mut self) {
        // expression: OBJECTID ASSIGN expression

        let expr = Box::new(self.pop().into_expression());
        self.pop(); // ASSIGN
        let (line_no, name) = self.pop().get_line_no_object_id();

        let new_node = ParseStackNode::Assign {
            line_no,
            name,
            expr,
        };

        let new_state = get_reduce_new_state_expression(self.top_state());

        self.push(new_node, new_state);
    }

    fn reduce_dispatch(&mut self, dispatch_type: DispatchType) {
        // expression:                           OBJECTID '(' expression_list_C ')'
        // expression: expression            '.' OBJECTID '(' expression_list_C ')'
        // expression: expression '@' TYPEID '.' OBJECTID '(' expression_list_C ')'

        self.pop(); // CloseParen
        let expression_list_c = self.pop();
        self.pop(); // OpenParen
        let (line_no, name) = self.pop().get_line_no_object_id();
        let type_name = None;

        let (line_no, expr, type_name, name) = match dispatch_type {
            DispatchType::OnSelf => {
                let expr = Box::new(Expression::VarByName {
                    line_no,
                    name: ObjectID::new_self(),
                    static_type: TypeID::new_no_type(),
                });

                (line_no, expr, type_name, name)
            }
            DispatchType::OnExpr => {
                self.pop(); // Dot

                let expr = Box::new(self.pop().into_expression());
                let line_no = expr.line_no();

                (line_no, expr, type_name, name)
            }
            DispatchType::Static => {
                self.pop(); // Dot
                let type_name = Some(self.pop().into_type_id());
                self.pop(); // At
                let expr = Box::new(self.pop().into_expression());

                let line_no = expr.line_no();

                (line_no, expr, type_name, name)
            }
        };

        let args = expression_list_c.extract_expressions();

        let new_node = ParseStackNode::Dispatch {
            line_no,
            expr,
            type_name,
            name,
            args,
        };

        let new_state = get_reduce_new_state_expression(self.top_state());

        self.push(new_node, new_state);
    }

    fn reduce_cond(&mut self) {
        // expression: IF expression THEN expression ELSE expression FI

        self.pop(); // FI
        let else_expr = Box::new(self.pop().into_expression());
        self.pop(); // ELSE
        let then_expr = Box::new(self.pop().into_expression());
        self.pop(); // THEN
        let pred = Box::new(self.pop().into_expression());
        let line_no = self.pop().line_no(); // IF

        let new_node = ParseStackNode::Cond {
            line_no,
            pred,
            then_expr,
            else_expr,
        };

        let new_state = get_reduce_new_state_expression(self.top_state());

        self.push(new_node, new_state);
    }

    fn reduce_while(&mut self) {
        // expression: WHILE expression LOOP expression POOL

        self.pop(); // POOL
        let body = Box::new(self.pop().into_expression());
        self.pop(); // LOOP
        let pred = Box::new(self.pop().into_expression());
        let line_no = self.pop().line_no(); // WHILE

        let new_node = ParseStackNode::Loop {
            line_no,
            pred,
            body,
        };

        let new_state = get_reduce_new_state_expression(self.top_state());

        self.push(new_node, new_state);
    }

    fn reduce_block(&mut self) {
        // expression: '{' expression_list_SC '}'

        self.pop(); // CloseBrace
        let expression_list_sc = self.pop();
        let line_no = self.pop().line_no(); // OpenBrace

        let body = expression_list_sc.extract_expressions();

        let new_node = ParseStackNode::Block { line_no, body };

        let new_state = get_reduce_new_state_expression(self.top_state());

        self.push(new_node, new_state);
    }

    fn reduce_let(&mut self) {
        // expression: LET expression_let

        let expr_let = self.pop();
        self.pop(); // LET

        let new_node = if let ParseStackNode::ExpressionLet {
            line_no,
            identifier,
            type_decl,
            init,
            body,
        } = expr_let
        {
            ParseStackNode::Let {
                line_no,
                identifier,
                type_decl,
                init,
                body,
            }
        } else {
            panic!("Bad ParseStackNode type");
        };

        let new_state = get_reduce_new_state_expression(self.top_state());

        self.push(new_node, new_state);
    }

    fn reduce_type_case(&mut self) {
        // expression: CASE expression OF branch_list ESAC

        self.pop(); // ESAC
        let branch_list = self.pop();
        self.pop(); // OF
        let expr = Box::new(self.pop().into_expression());
        let line_no = self.pop().line_no(); // CASE

        let branches = branch_list.extract_branches();

        let new_node = ParseStackNode::TypeCase {
            line_no,
            expr,
            branches,
        };

        let new_state = get_reduce_new_state_expression(self.top_state());

        self.push(new_node, new_state);
    }

    fn reduce_new(&mut self) {
        // expression: NEW TYPEID

        let type_name = self.pop().into_type_id();
        let line_no = self.pop().line_no(); // NEW

        let new_node = ParseStackNode::New { line_no, type_name };

        let new_state = get_reduce_new_state_expression(self.top_state());

        self.push(new_node, new_state);
    }

    fn reduce_is_void(&mut self) {
        // expression: ISVOID expression

        let expr = Box::new(self.pop().into_expression());
        let line_no = self.pop().line_no(); // ISVOID

        let new_node = ParseStackNode::IsVoid { line_no, expr };

        let new_state = get_reduce_new_state_expression(self.top_state());

        self.push(new_node, new_state);
    }

    fn reduce_arith(&mut self) {
        // expression: expression arith_op_type expression

        let expr_rhs = Box::new(self.pop().into_expression());
        let arith_op_type = self.pop().into_arith_type();
        let expr_lhs = Box::new(self.pop().into_expression());

        let new_node = ParseStackNode::ArithOp {
            line_no: expr_lhs.line_no(),
            expr_lhs,
            expr_rhs,
            arith_op_type,
        };

        let new_state = get_reduce_new_state_expression(self.top_state());

        self.push(new_node, new_state);
    }

    fn reduce_neg(&mut self) {
        // expression: '~' expression

        let expr = Box::new(self.pop().into_expression());
        self.pop(); // Neg

        let new_node = ParseStackNode::Neg {
            line_no: expr.line_no(),
            expr,
        };

        let new_state = get_reduce_new_state_expression(self.top_state());

        self.push(new_node, new_state);
    }

    fn reduce_comp(&mut self) {
        // expression: expression comp_type expression

        let expr_rhs = Box::new(self.pop().into_expression());
        let comp_type = self.pop().into_comp_type();
        let expr_lhs = Box::new(self.pop().into_expression());

        let new_node = ParseStackNode::Comp {
            line_no: expr_lhs.line_no(),
            expr_lhs,
            expr_rhs,
            comp_type,
        };

        let new_state = get_reduce_new_state_expression(self.top_state());

        self.push(new_node, new_state);
    }

    fn reduce_eq(&mut self) {
        // expression: expression '=' expression

        let expr_rhs = Box::new(self.pop().into_expression());
        self.pop(); // Eq
        let expr_lhs = Box::new(self.pop().into_expression());

        let new_node = ParseStackNode::Eq {
            line_no: expr_lhs.line_no(),
            expr_lhs,
            expr_rhs,
        };

        let new_state = get_reduce_new_state_expression(self.top_state());

        self.push(new_node, new_state);
    }

    fn reduce_not(&mut self) {
        // expression: NOT expression

        let expr = Box::new(self.pop().into_expression());
        self.pop(); // NOT

        let new_node = ParseStackNode::Not {
            line_no: expr.line_no(),
            expr,
        };

        let new_state = get_reduce_new_state_expression(self.top_state());

        self.push(new_node, new_state);
    }

    fn reduce_paren(&mut self) {
        // expression: '(' expression ')'

        self.pop(); // CloseParen
        let new_node = self.pop();
        self.pop(); // OpenParen

        let new_state = get_reduce_new_state_expression(self.top_state());

        self.push(new_node, new_state);
    }

    fn reduce_var_by_name(&mut self) {
        // expression: OBJECTID

        let (line_no, name) = self.pop().get_line_no_object_id();

        let new_node = ParseStackNode::VarByName { line_no, name };

        let new_state = get_reduce_new_state_expression(self.top_state());

        self.push(new_node, new_state);
    }

    fn reduce_int_const(&mut self) {
        // expression: INT_CONST

        let e = self.pop();

        let new_node = if let ParseStackNode::TermIntConst { line_no, val } = e {
            ParseStackNode::IntConst { line_no, val }
        } else {
            panic!("Bad ParseStackNode type")
        };

        let new_state = get_reduce_new_state_expression(self.top_state());

        self.push(new_node, new_state);
    }

    fn reduce_str_const(&mut self) {
        // expression: STR_CONST

        let e = self.pop();

        let new_node = if let ParseStackNode::TermStrConst { line_no, val } = e {
            ParseStackNode::StringConst { line_no, val }
        } else {
            panic!("Bad ParseStackNode type")
        };

        let new_state = get_reduce_new_state_expression(self.top_state());

        self.push(new_node, new_state);
    }

    fn reduce_bool_const(&mut self) {
        // expression: BOOL_CONST

        let e = self.pop();

        let new_node = if let ParseStackNode::TermBoolConst { line_no, val } = e {
            ParseStackNode::BoolConst { line_no, val }
        } else {
            panic!("Bad ParseStackNode type")
        };

        let new_state = get_reduce_new_state_expression(self.top_state());

        self.push(new_node, new_state);
    }

    // Carry out a shift or a reduce

    fn execute_action(
        &mut self,
        action: Action,
        in_file_name: &str,
        file_no: u32,
        classes: &mut IndexMap<TypeID, Class>,
    ) -> Result<(), String> {
        match action {
            Action::Shift { new_state } => {
                self.shift(new_state);
            }

            Action::ReduceClassList { is_empty } => {
                self.reduce_class_list(classes, in_file_name, is_empty)?;
            }

            Action::ReduceClass { has_parent } => {
                self.reduce_class(in_file_name, file_no, has_parent);
            }

            Action::ReduceFeatureList { is_empty } => {
                self.reduce_feature_list(in_file_name, is_empty)?;
            }

            Action::ReduceMethod => {
                self.reduce_method();
            }

            Action::ReduceAttr { has_init } => {
                self.reduce_attr(has_init);
            }

            Action::ReduceFormalListNE { is_empty } => {
                self.reduce_formal_list_ne(is_empty);
            }

            Action::ReduceFormalList { is_empty } => {
                self.reduce_formal_list(is_empty);
            }

            Action::ReduceFormal => {
                self.reduce_formal();
            }

            Action::ReduceBranchList { is_empty } => {
                self.reduce_branch_list(is_empty);
            }

            Action::ReduceBranch => {
                self.reduce_branch();
            }

            Action::ReduceExprListSC { is_empty } => {
                self.reduce_expr_list_sc(is_empty);
            }

            Action::ReduceExprListCNE { is_empty } => {
                self.reduce_expr_list_c_ne(is_empty);
            }

            Action::ReduceExprListC { is_empty } => {
                self.reduce_expr_list_c(is_empty);
            }

            Action::ReduceExprLet { has_init } => {
                self.reduce_expr_let(has_init);
            }

            Action::ReduceAssign => {
                self.reduce_assign();
            }

            Action::ReduceDispatch { dispatch_type } => {
                self.reduce_dispatch(dispatch_type);
            }

            Action::ReduceCond => {
                self.reduce_cond();
            }

            Action::ReduceWhile => {
                self.reduce_while();
            }

            Action::ReduceBlock => {
                self.reduce_block();
            }

            Action::ReduceLet => {
                self.reduce_let();
            }

            Action::ReduceTypeCase => {
                self.reduce_type_case();
            }

            Action::ReduceNew => {
                self.reduce_new();
            }

            Action::ReduceIsVoid => {
                self.reduce_is_void();
            }

            Action::ReduceArith => {
                self.reduce_arith();
            }

            Action::ReduceNeg => {
                self.reduce_neg();
            }

            Action::ReduceComp => {
                self.reduce_comp();
            }

            Action::ReduceEq => {
                self.reduce_eq();
            }

            Action::ReduceNot => {
                self.reduce_not();
            }

            Action::ReduceParen => {
                self.reduce_paren();
            }

            Action::ReduceVarByName => {
                self.reduce_var_by_name();
            }

            Action::ReduceIntConst => {
                self.reduce_int_const();
            }

            Action::ReduceStrConst => {
                self.reduce_str_const();
            }

            Action::ReduceBoolConst => {
                self.reduce_bool_const();
            }

            Action::Accept => {
                panic!("Cannot execute");
            }
        };

        Ok(())
    }
}

// Parse a queue of tokens into an AST
pub fn parse(
    tokens: VecDeque<Token>,
    in_file_name: &str,
    file_no: u32,
    classes: &mut IndexMap<TypeID, Class>,
) -> Result<(), String> {
    let mut parser = Parser::new(tokens);

    if let Token::End { .. } = parser.tokens.front().unwrap() {
        return Err(format!("{} - Empty file", in_file_name));
    }

    loop {
        match get_action(parser.top_state(), parser.tokens.front().unwrap()) {
            Ok(Action::Accept) => {
                break Ok(());
            }
            Ok(action) => parser.execute_action(action, in_file_name, file_no, classes)?,
            Err(line_no) => return Err(format!("{} : {} - Syntax error", in_file_name, line_no)),
        }
    }
}
