use crate::ast::{Class, Expression, ObjectID, TypeID};
use crate::token::Token;

use self::node::ParseStackNode;
use self::parse_table::{Action, State};
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

// Shift/Reduce parser
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

    fn reduce_02(
        &mut self,
        classes: &mut IndexMap<TypeID, Class>,
        in_file_name: &str,
    ) -> Result<(), String> {
        // class_list: class ';'

        self.pop(); // SemiColon
        let (name, class) = self.pop().into_class();

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

        let new_node = ParseStackNode::ClassList;

        let new_state = get_reduce_new_state_class_list(self.top_state());

        self.push(new_node, new_state);

        Ok(())
    }

    fn reduce_03(
        &mut self,
        classes: &mut IndexMap<TypeID, Class>,
        in_file_name: &str,
    ) -> Result<(), String> {
        // class_list: class_list class ';'

        self.pop(); // SemiColon
        let (name, class) = self.pop().into_class();
        let class_list = self.pop();

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

    fn reduce_05(&mut self, in_file_name: &str, file_no: u32) {
        // class: CLASS TYPEID '{' feature_list '}'

        self.pop(); // CloseBrace
        let features = self.pop();
        self.pop(); // OpenBrace
        let name = self.pop().into_type_id();
        let line_no = self.pop().line_no(); // CLASS

        let parent_name = Some(TypeID::root_class_name());

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

    fn reduce_06(&mut self, in_file_name: &str, file_no: u32) {
        // class: CLASS TYPEID INHERITS TYPEID '{' feature_list '}'

        self.pop(); // CloseBrace
        let features = self.pop();
        self.pop(); // OpenBrace
        let parent_name = Some(self.pop().into_type_id());
        self.pop(); // INHERITS
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

    fn reduce_07(&mut self) {
        // feature_list: /* empty */
        let new_node = ParseStackNode::FeatureList {
            attrs: Vec::new(),
            methods: IndexMap::new(),
        };

        let new_state = get_reduce_new_state_feature_list(self.top_state());

        self.push(new_node, new_state);
    }

    fn reduce_08(&mut self, in_file_name: &str) -> Result<(), String> {
        // feature_list: feature_list feature ';'

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

        let new_node = ParseStackNode::FeatureList { attrs, methods };

        let new_state = get_reduce_new_state_feature_list(self.top_state());

        self.push(new_node, new_state);

        Ok(())
    }

    fn reduce_10(&mut self) {
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

    fn reduce_11(&mut self) {
        // feature: OBJECTID ':' TYPEID

        let type_decl = self.pop().into_type_id();
        self.pop(); // Colon
        let (line_no, name) = self.pop().get_line_no_object_id();

        let new_node = ParseStackNode::Attr {
            line_no,
            name,
            type_decl,
            init: Expression::NoExpr,
        };

        let new_state = get_reduce_new_state_feature(self.top_state());

        self.push(new_node, new_state);
    }

    fn reduce_12(&mut self) {
        // feature: OBJECTID ':' TYPEID ASSIGN expression

        let init = self.pop().into_expression();
        self.pop(); // ASSIGN
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

    fn reduce_13(&mut self) {
        // formal_list_nE: formal

        let formal = self.pop().into_formal();

        let formals = vec![formal];

        let new_node = ParseStackNode::FormalListNotEmpty { formals };

        let new_state = get_reduce_new_state_formal_list_ne(self.top_state());

        self.push(new_node, new_state);
    }

    fn reduce_14(&mut self) {
        // formal_list_nE: formal_list_nE ',' formal

        let formal = self.pop().into_formal();
        self.pop(); // Comma
        let formal_list_ne = self.pop();

        let mut formals = formal_list_ne.extract_formals();

        formals.push(formal);

        let new_node = ParseStackNode::FormalListNotEmpty { formals };

        let new_state = get_reduce_new_state_formal_list_ne(self.top_state());

        self.push(new_node, new_state);
    }

    fn reduce_15(&mut self) {
        // formal_list: /* empty */
        let new_node = ParseStackNode::FormalList {
            formals: Vec::new(),
        };

        let new_state = get_reduce_new_state_formal_list(self.top_state());

        self.push(new_node, new_state);
    }

    fn reduce_16(&mut self) {
        // formal_list: formal_list_nE

        let formal_list_ne = self.pop();

        let formals = formal_list_ne.extract_formals();

        let new_node = ParseStackNode::FormalList { formals };

        let new_state = get_reduce_new_state_formal_list(self.top_state());

        self.push(new_node, new_state);
    }

    fn reduce_17(&mut self) {
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

    fn reduce_18(&mut self) {
        // branch_list: branch ';'

        self.pop(); // SemiColon
        let branch = self.pop().into_branch();

        let branches = vec![branch];

        let new_node = ParseStackNode::BranchList { branches };

        let new_state = get_reduce_new_state_branch_list(self.top_state());

        self.push(new_node, new_state);
    }

    fn reduce_19(&mut self) {
        // branch_list: branch_list branch ';'

        self.pop(); // SemiColon
        let branch = self.pop().into_branch();
        let branch_list = self.pop();

        let mut branches = branch_list.extract_branches();

        branches.push(branch);

        let new_node = ParseStackNode::BranchList { branches };

        let new_state = get_reduce_new_state_branch_list(self.top_state());

        self.push(new_node, new_state);
    }

    fn reduce_20(&mut self) {
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

    fn reduce_21(&mut self) {
        // expression_list_SC: expression ';'

        self.pop(); // SemiColon
        let expr = self.pop().into_expression();

        let exprs = vec![expr];

        let new_node = ParseStackNode::ExpressionListSemiColon { exprs };

        let new_state = get_reduce_new_state_expression_list_sc(self.top_state());

        self.push(new_node, new_state);
    }

    fn reduce_22(&mut self) {
        // expression_list_SC: expression_list_SC expression ';'

        self.pop(); // SemiColon
        let expr = self.pop().into_expression();
        let expr_list = self.pop();

        let mut exprs = expr_list.extract_expressions();

        exprs.push(expr);

        let new_node = ParseStackNode::ExpressionListSemiColon { exprs };

        let new_state = get_reduce_new_state_expression_list_sc(self.top_state());

        self.push(new_node, new_state);
    }

    fn reduce_24(&mut self) {
        // expression_list_C_nE: expression

        let expr = self.pop().into_expression();

        let exprs = vec![expr];

        let new_node = ParseStackNode::ExpressionListCommaNotEmpty { exprs };

        let new_state = get_reduce_new_state_expression_list_c_ne(self.top_state());

        self.push(new_node, new_state);
    }

    fn reduce_25(&mut self) {
        // expression_list_C_nE: expression_list_C_nE ',' expression

        let expr = self.pop().into_expression();
        self.pop(); // Comma
        let expr_list_c_ne = self.pop();

        let mut exprs = expr_list_c_ne.extract_expressions();

        exprs.push(expr);

        let new_node = ParseStackNode::ExpressionListCommaNotEmpty { exprs };

        let new_state = get_reduce_new_state_expression_list_c_ne(self.top_state());

        self.push(new_node, new_state);
    }

    fn reduce_26(&mut self) {
        // expression_list_C: /* empty */
        let new_node = ParseStackNode::ExpressionListComma { exprs: Vec::new() };

        let new_state = get_reduce_new_state_expression_list_c(self.top_state());

        self.push(new_node, new_state);
    }

    fn reduce_27(&mut self) {
        // expression_list_C: expression_list_C_nE

        let expr_list_c_ne = self.pop();

        let exprs = expr_list_c_ne.extract_expressions();

        let new_node = ParseStackNode::ExpressionListComma { exprs };

        let new_state = get_reduce_new_state_expression_list_c(self.top_state());

        self.push(new_node, new_state);
    }

    fn reduce_28(&mut self) {
        // expression_let: OBJECTID ':' TYPEID IN expression

        let body = Box::new(self.pop().into_expression());
        self.pop(); // IN
        let type_decl = self.pop().into_type_id();
        self.pop(); // Colon
        let (line_no, identifier) = self.pop().get_line_no_object_id();

        let init = Box::new(Expression::NoExpr);

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

    fn reduce_29(&mut self) {
        // expression_let: OBJECTID ':' TYPEID ASSIGN expression IN expression

        let body = Box::new(self.pop().into_expression());
        self.pop(); // IN
        let init = Box::new(self.pop().into_expression());
        self.pop(); // ASSIGN
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

    fn reduce_30(&mut self) {
        // expression_let: OBJECTID ':' TYPEID ',' expression_let

        let body = Box::new(self.pop().into_expression());
        self.pop(); // IN
        let type_decl = self.pop().into_type_id();
        self.pop(); // Colon
        let (line_no, identifier) = self.pop().get_line_no_object_id();

        let init = Box::new(Expression::NoExpr);

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

    fn reduce_31(&mut self) {
        // expression_let: OBJECTID ':' TYPEID ASSIGN expression ',' expression_let

        let body = Box::new(self.pop().into_expression());
        self.pop(); // IN
        let init = Box::new(self.pop().into_expression());
        self.pop(); // ASSIGN
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

    fn reduce_34(&mut self) {
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

    fn reduce_35(&mut self) {
        // expression: OBJECTID '(' expression_list_C ')'

        self.pop(); // CloseParen
        let expression_list_c = self.pop();
        self.pop(); // OpenParen
        let (line_no, name) = self.pop().get_line_no_object_id();

        let expr = Box::new(Expression::Object {
            line_no,
            name: ObjectID::new_self(),
            static_type: TypeID::new_no_type(),
        });

        let args = expression_list_c.extract_expressions();

        let new_node = ParseStackNode::Dispatch {
            line_no,
            expr,
            type_name: None,
            name,
            args,
        };

        let new_state = get_reduce_new_state_expression(self.top_state());

        self.push(new_node, new_state);
    }

    fn reduce_36(&mut self) {
        // expression: expression '.' OBJECTID '(' expression_list_C ')'

        self.pop(); // CloseParen
        let expression_list_c = self.pop();
        self.pop(); // OpenParen
        let name = self.pop().into_object_id();
        self.pop(); // Dot
        let expr = Box::new(self.pop().into_expression());

        let args = expression_list_c.extract_expressions();

        let new_node = ParseStackNode::Dispatch {
            line_no: expr.line_no(),
            expr,
            type_name: None,
            name,
            args,
        };

        let new_state = get_reduce_new_state_expression(self.top_state());

        self.push(new_node, new_state);
    }

    fn reduce_37(&mut self) {
        // expression: expression '@' TYPEID '.' OBJECTID '(' expression_list_C ')'

        self.pop(); // CloseParen
        let expression_list_c = self.pop();
        self.pop(); // OpenParen
        let name = self.pop().into_object_id();
        self.pop(); // Dot
        let type_name = Some(self.pop().into_type_id());
        self.pop(); // At
        let expr = Box::new(self.pop().into_expression());

        let args = expression_list_c.extract_expressions();

        let new_node = ParseStackNode::Dispatch {
            line_no: expr.line_no(),
            expr,
            type_name,
            name,
            args,
        };

        let new_state = get_reduce_new_state_expression(self.top_state());

        self.push(new_node, new_state);
    }

    fn reduce_38(&mut self) {
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

    fn reduce_39(&mut self) {
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

    fn reduce_40(&mut self) {
        // expression: '{' expression_list_SC '}'

        self.pop(); // CloseBrace
        let expression_list_sc = self.pop();
        let line_no = self.pop().line_no(); // OpenBrace

        let body = expression_list_sc.extract_expressions();

        let new_node = ParseStackNode::Block { line_no, body };

        let new_state = get_reduce_new_state_expression(self.top_state());

        self.push(new_node, new_state);
    }

    fn reduce_41(&mut self) {
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

    fn reduce_42(&mut self) {
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

    fn reduce_43(&mut self) {
        // expression: NEW TYPEID

        let type_name = self.pop().into_type_id();
        let line_no = self.pop().line_no(); // NEW

        let new_node = ParseStackNode::New { line_no, type_name };

        let new_state = get_reduce_new_state_expression(self.top_state());

        self.push(new_node, new_state);
    }

    fn reduce_44(&mut self) {
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

    fn reduce_49(&mut self) {
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

        // self.reduce_comp(CompType::LT);
    }

    fn reduce_52(&mut self) {
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

    fn reduce_53(&mut self) {
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

    fn reduce_54(&mut self) {
        // expression: '(' expression ')'

        self.pop(); // CloseParen
        let new_node = self.pop();
        self.pop(); // OpenParen

        let new_state = get_reduce_new_state_expression(self.top_state());

        self.push(new_node, new_state);
    }

    fn reduce_55(&mut self) {
        // expression: OBJECTID

        let (line_no, name) = self.pop().get_line_no_object_id();

        let new_node = ParseStackNode::Object { line_no, name };

        let new_state = get_reduce_new_state_expression(self.top_state());

        self.push(new_node, new_state);
    }

    fn reduce_56(&mut self) {
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

    fn reduce_57(&mut self) {
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

    fn reduce_58(&mut self) {
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

            Action::Reduce02 => {
                self.reduce_02(classes, in_file_name)?;
            }

            Action::Reduce03 => {
                self.reduce_03(classes, in_file_name)?;
            }

            Action::Reduce05 => {
                self.reduce_05(in_file_name, file_no);
            }

            Action::Reduce06 => {
                self.reduce_06(in_file_name, file_no);
            }

            Action::Reduce07 => {
                self.reduce_07();
            }

            Action::Reduce08 => {
                self.reduce_08(in_file_name)?;
            }

            Action::Reduce10 => {
                self.reduce_10();
            }

            Action::Reduce11 => {
                self.reduce_11();
            }

            Action::Reduce12 => {
                self.reduce_12();
            }

            Action::Reduce13 => {
                self.reduce_13();
            }

            Action::Reduce14 => {
                self.reduce_14();
            }

            Action::Reduce15 => {
                self.reduce_15();
            }

            Action::Reduce16 => {
                self.reduce_16();
            }

            Action::Reduce17 => {
                self.reduce_17();
            }

            Action::Reduce18 => {
                self.reduce_18();
            }

            Action::Reduce19 => {
                self.reduce_19();
            }

            Action::Reduce20 => {
                self.reduce_20();
            }

            Action::Reduce21 => {
                self.reduce_21();
            }

            Action::Reduce22 => {
                self.reduce_22();
            }

            Action::Reduce24 => {
                self.reduce_24();
            }

            Action::Reduce25 => {
                self.reduce_25();
            }

            Action::Reduce26 => {
                self.reduce_26();
            }

            Action::Reduce27 => {
                self.reduce_27();
            }

            Action::Reduce28 => {
                self.reduce_28();
            }

            Action::Reduce29 => {
                self.reduce_29();
            }

            Action::Reduce30 => {
                self.reduce_30();
            }

            Action::Reduce31 => {
                self.reduce_31();
            }

            Action::Reduce34 => {
                self.reduce_34();
            }

            Action::Reduce35 => {
                self.reduce_35();
            }

            Action::Reduce36 => {
                self.reduce_36();
            }

            Action::Reduce37 => {
                self.reduce_37();
            }

            Action::Reduce38 => {
                self.reduce_38();
            }

            Action::Reduce39 => {
                self.reduce_39();
            }

            Action::Reduce40 => {
                self.reduce_40();
            }

            Action::Reduce41 => {
                self.reduce_41();
            }

            Action::Reduce42 => {
                self.reduce_42();
            }

            Action::Reduce43 => {
                self.reduce_43();
            }

            Action::Reduce44 => {
                self.reduce_44();
            }

            Action::ReduceArith => {
                self.reduce_arith();
            }

            Action::Reduce49 => {
                self.reduce_49();
            }

            Action::ReduceComp => {
                self.reduce_comp();
            }

            Action::Reduce52 => {
                self.reduce_52();
            }

            Action::Reduce53 => {
                self.reduce_53();
            }

            Action::Reduce54 => {
                self.reduce_54();
            }

            Action::Reduce55 => {
                self.reduce_55();
            }

            Action::Reduce56 => {
                self.reduce_56();
            }

            Action::Reduce57 => {
                self.reduce_57();
            }

            Action::Reduce58 => {
                self.reduce_58();
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
