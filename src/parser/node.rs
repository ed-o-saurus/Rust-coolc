use crate::ast::{
    ArithOpType, Attr, Branch, Class, CompType, Expression, Formal, Method, ObjectID, TypeID,
};

use indexmap::IndexMap;

// The types of nodes to be placed on the parse stack
pub enum ParseStackNode {
    // Terminals
    TermAdd {
        line_no: i16,
    },
    TermDiv {
        line_no: i16,
    },
    TermSub {
        line_no: i16,
    },
    TermMul {
        line_no: i16,
    },
    TermEq {
        line_no: i16,
    },
    TermLT {
        line_no: i16,
    },
    TermDot {
        line_no: i16,
    },
    TermNeg {
        line_no: i16,
    },
    TermComma {
        line_no: i16,
    },
    TermSemiColon {
        line_no: i16,
    },
    TermColon {
        line_no: i16,
    },
    TermOpenParen {
        line_no: i16,
    },
    TermCloseParen {
        line_no: i16,
    },
    TermAt {
        line_no: i16,
    },
    TermOpenBrace {
        line_no: i16,
    },
    TermCloseBrace {
        line_no: i16,
    },
    TermClass {
        line_no: i16,
    },
    TermElse {
        line_no: i16,
    },
    TermFi {
        line_no: i16,
    },
    TermIf {
        line_no: i16,
    },
    TermIn {
        line_no: i16,
    },
    TermInherits {
        line_no: i16,
    },
    TermLet {
        line_no: i16,
    },
    TermLoop {
        line_no: i16,
    },
    TermPool {
        line_no: i16,
    },
    TermThen {
        line_no: i16,
    },
    TermWhile {
        line_no: i16,
    },
    TermCase {
        line_no: i16,
    },
    TermEsac {
        line_no: i16,
    },
    TermOf {
        line_no: i16,
    },
    TermNew {
        line_no: i16,
    },
    TermIsVoid {
        line_no: i16,
    },
    TermNot {
        line_no: i16,
    },
    TermDArrow {
        line_no: i16,
    },
    TermLEq {
        line_no: i16,
    },
    TermAssign {
        line_no: i16,
    },
    TermBoolConst {
        line_no: i16,
        val: bool,
    },
    TermIntConst {
        line_no: i16,
        val: u32,
    },
    TermStrConst {
        line_no: i16,
        val: String,
    },
    TermTypeID {
        line_no: i16,
        type_name: String,
    },
    TermObjectID {
        line_no: i16,
        obj_name: String,
    },
    // Special
    Bottom,
    // Non-terminals
    ClassList,
    Class {
        file_name: String,
        file_no: u32,
        line_no: i16,
        name: TypeID,
        parent_name: Option<TypeID>,
        attrs: Vec<Attr>,
        methods: IndexMap<ObjectID, Method>,
    },
    FeatureList {
        attrs: Vec<Attr>,
        methods: IndexMap<ObjectID, Method>,
    },
    Method {
        // Feature type
        line_no: i16,
        name: ObjectID,
        formals: Vec<Formal>,
        return_type: TypeID,
        expr: Expression,
    },
    Attr {
        // Feature type
        line_no: i16,
        name: ObjectID,
        type_decl: TypeID,
        init: Expression,
    },
    FormalListNotEmpty {
        formals: Vec<Formal>,
    },
    FormalList {
        formals: Vec<Formal>,
    },
    Formal {
        line_no: i16,
        name: ObjectID,
        type_decl: TypeID,
    },
    BranchList {
        branches: Vec<Branch>,
    },
    Branch {
        line_no: i16,
        name: ObjectID,
        type_decl: TypeID,
        expr: Expression,
    },
    ExpressionListSemiColon {
        exprs: Vec<Expression>,
    },
    ExpressionListCommaNotEmpty {
        exprs: Vec<Expression>,
    },
    ExpressionListComma {
        exprs: Vec<Expression>,
    },
    ExpressionLet {
        // ExpressionLet type
        line_no: i16,
        identifier: ObjectID,
        type_decl: TypeID,
        init: Box<Expression>,
        body: Box<Expression>,
    },
    Assign {
        // Expr type
        line_no: i16,
        name: ObjectID,
        expr: Box<Expression>,
    },
    Dispatch {
        // Expr type
        line_no: i16,
        expr: Box<Expression>,
        type_name: Option<TypeID>,
        name: ObjectID,
        args: Vec<Expression>,
    },
    Cond {
        // Expr type
        line_no: i16,
        pred: Box<Expression>,
        then_expr: Box<Expression>,
        else_expr: Box<Expression>,
    },
    Loop {
        // Expr type
        line_no: i16,
        pred: Box<Expression>,
        body: Box<Expression>,
    },
    TypeCase {
        // Expr type
        line_no: i16,
        expr: Box<Expression>,
        branches: Vec<Branch>,
    },
    Block {
        // Expr type
        line_no: i16,
        body: Vec<Expression>,
    },
    Let {
        // Expr type
        line_no: i16,
        identifier: ObjectID,
        type_decl: TypeID,
        init: Box<Expression>,
        body: Box<Expression>,
    },
    ArithOp {
        // Expr type
        line_no: i16,
        expr_lhs: Box<Expression>,
        expr_rhs: Box<Expression>,
        arith_op_type: ArithOpType,
    },
    Neg {
        // Expr type
        line_no: i16,
        expr: Box<Expression>,
    },
    Comp {
        // Expr type
        line_no: i16,
        expr_lhs: Box<Expression>,
        expr_rhs: Box<Expression>,
        comp_type: CompType,
    },
    Eq {
        // Expr type
        line_no: i16,
        expr_lhs: Box<Expression>,
        expr_rhs: Box<Expression>,
    },
    Not {
        // Expr type
        line_no: i16,
        expr: Box<Expression>,
    },
    IntConst {
        // Expr type
        line_no: i16,
        val: u32,
    },
    BoolConst {
        // Expr type
        line_no: i16,
        val: bool,
    },
    StringConst {
        // Expr type
        line_no: i16,
        val: String,
    },
    New {
        // Expr type
        line_no: i16,
        type_name: TypeID,
    },
    IsVoid {
        // Expr type
        line_no: i16,
        expr: Box<Expression>,
    },
    Object {
        // Expr type
        line_no: i16,
        name: ObjectID,
    },
}

impl ParseStackNode {
    pub fn line_no(&self) -> i16 {
        match self {
            ParseStackNode::TermAdd { line_no }
            | ParseStackNode::TermDiv { line_no }
            | ParseStackNode::TermSub { line_no }
            | ParseStackNode::TermMul { line_no }
            | ParseStackNode::TermEq { line_no }
            | ParseStackNode::TermLT { line_no }
            | ParseStackNode::TermDot { line_no }
            | ParseStackNode::TermNeg { line_no }
            | ParseStackNode::TermComma { line_no }
            | ParseStackNode::TermSemiColon { line_no }
            | ParseStackNode::TermColon { line_no }
            | ParseStackNode::TermOpenParen { line_no }
            | ParseStackNode::TermCloseParen { line_no }
            | ParseStackNode::TermAt { line_no }
            | ParseStackNode::TermOpenBrace { line_no }
            | ParseStackNode::TermCloseBrace { line_no }
            | ParseStackNode::TermClass { line_no }
            | ParseStackNode::TermElse { line_no }
            | ParseStackNode::TermFi { line_no }
            | ParseStackNode::TermIf { line_no }
            | ParseStackNode::TermIn { line_no }
            | ParseStackNode::TermInherits { line_no }
            | ParseStackNode::TermLet { line_no }
            | ParseStackNode::TermLoop { line_no }
            | ParseStackNode::TermPool { line_no }
            | ParseStackNode::TermThen { line_no }
            | ParseStackNode::TermWhile { line_no }
            | ParseStackNode::TermCase { line_no }
            | ParseStackNode::TermEsac { line_no }
            | ParseStackNode::TermOf { line_no }
            | ParseStackNode::TermNew { line_no }
            | ParseStackNode::TermIsVoid { line_no }
            | ParseStackNode::TermNot { line_no }
            | ParseStackNode::TermDArrow { line_no }
            | ParseStackNode::TermLEq { line_no }
            | ParseStackNode::TermAssign { line_no }
            | ParseStackNode::TermBoolConst { line_no, .. }
            | ParseStackNode::TermTypeID { line_no, .. }
            | ParseStackNode::TermObjectID { line_no, .. }
            | ParseStackNode::TermIntConst { line_no, .. }
            | ParseStackNode::TermStrConst { line_no, .. }
            | ParseStackNode::Class { line_no, .. }
            | ParseStackNode::Method { line_no, .. }
            | ParseStackNode::Attr { line_no, .. }
            | ParseStackNode::Formal { line_no, .. }
            | ParseStackNode::Branch { line_no, .. }
            | ParseStackNode::ExpressionLet { line_no, .. }
            | ParseStackNode::Assign { line_no, .. }
            | ParseStackNode::Dispatch { line_no, .. }
            | ParseStackNode::Cond { line_no, .. }
            | ParseStackNode::Loop { line_no, .. }
            | ParseStackNode::TypeCase { line_no, .. }
            | ParseStackNode::Block { line_no, .. }
            | ParseStackNode::Let { line_no, .. }
            | ParseStackNode::ArithOp { line_no, .. }
            | ParseStackNode::Neg { line_no, .. }
            | ParseStackNode::Comp { line_no, .. }
            | ParseStackNode::Eq { line_no, .. }
            | ParseStackNode::Not { line_no, .. }
            | ParseStackNode::IntConst { line_no, .. }
            | ParseStackNode::BoolConst { line_no, .. }
            | ParseStackNode::StringConst { line_no, .. }
            | ParseStackNode::New { line_no, .. }
            | ParseStackNode::IsVoid { line_no, .. }
            | ParseStackNode::Object { line_no, .. } => *line_no,
            _ => panic!("No line_no for this node type"),
        }
    }

    // These functions extract the contents from a node.
    // Most of them only work on one or two node types.

    pub fn extract_features(self) -> (Vec<Attr>, IndexMap<ObjectID, Method>) {
        if let ParseStackNode::FeatureList { attrs, methods } = self {
            (attrs, methods)
        } else {
            panic!("Bad node type")
        }
    }

    pub fn extract_formals(self) -> Vec<Formal> {
        if let ParseStackNode::FormalListNotEmpty { formals }
        | ParseStackNode::FormalList { formals } = self
        {
            formals
        } else {
            panic!("Bad node type")
        }
    }

    pub fn extract_branches(self) -> Vec<Branch> {
        if let ParseStackNode::BranchList { branches } = self {
            branches
        } else {
            panic!("Bad node type")
        }
    }

    pub fn extract_expressions(self) -> Vec<Expression> {
        if let ParseStackNode::ExpressionListSemiColon { exprs }
        | ParseStackNode::ExpressionListCommaNotEmpty { exprs }
        | ParseStackNode::ExpressionListComma { exprs } = self
        {
            exprs
        } else {
            panic!("Bad node type")
        }
    }

    pub fn get_line_no_object_id(self) -> (i16, ObjectID) {
        if let ParseStackNode::TermObjectID { line_no, obj_name } = self {
            (line_no, ObjectID::new(obj_name))
        } else {
            panic!("Bad node type")
        }
    }

    // These functions convert a Node into a type that appears in the AST

    pub fn into_class(self) -> (TypeID, Class) {
        if let ParseStackNode::Class {
            file_name,
            file_no,
            line_no,
            name,
            parent_name,
            attrs,
            methods,
        } = self
        {
            (
                name,
                Class {
                    file_name,
                    file_no,
                    basic: false,
                    line_no,
                    parent_name,
                    attrs,
                    methods,
                    family: 0..0,
                    child_names: vec![],
                    method_name_to_pos: IndexMap::new(),
                    dispatch_table: Vec::new(),
                },
            )
        } else {
            panic!("Bad node type")
        }
    }

    pub fn into_attr(self) -> Attr {
        if let ParseStackNode::Attr {
            line_no,
            name,
            type_decl,
            init,
        } = self
        {
            Attr {
                line_no,
                name,
                type_decl,
                init,
                self_offset: 0,
            }
        } else {
            panic!("Bad node type");
        }
    }

    pub fn into_method(self) -> (ObjectID, Method) {
        if let ParseStackNode::Method {
            line_no,
            name,
            formals,
            return_type,
            expr,
        } = self
        {
            (
                name,
                Method {
                    line_no,
                    formals,
                    return_type,
                    expr,
                },
            )
        } else {
            panic!("Bad node type");
        }
    }

    pub fn into_formal(self) -> Formal {
        if let ParseStackNode::Formal {
            line_no,
            name,
            type_decl,
        } = self
        {
            Formal {
                line_no,
                name,
                type_decl,
            }
        } else {
            panic!("Bad node type");
        }
    }

    pub fn into_branch(self) -> Branch {
        if let ParseStackNode::Branch {
            line_no,
            name,
            type_decl,
            expr,
        } = self
        {
            Branch {
                line_no,
                name,
                type_decl,
                expr,
                family: 0..0,
            }
        } else {
            panic!("Bad node type");
        }
    }

    pub fn into_expression(self) -> Expression {
        let static_type: TypeID = TypeID::new_no_type();

        match self {
            ParseStackNode::ExpressionLet {
                line_no,
                identifier,
                type_decl,
                init,
                body,
            } => Expression::Let {
                line_no,
                identifier,
                type_decl,
                init,
                body,
            },
            ParseStackNode::Assign {
                line_no,
                name,
                expr,
            } => Expression::Assign {
                line_no,
                name,
                expr,
            },
            ParseStackNode::Dispatch {
                line_no,
                expr,
                type_name,
                name,
                args,
            } => Expression::Dispatch {
                line_no,
                expr,
                type_name,
                name,
                args,
                static_type,
            },
            ParseStackNode::Cond {
                line_no,
                pred,
                then_expr,
                else_expr,
            } => Expression::Cond {
                line_no,
                pred,
                then_expr,
                else_expr,
                static_type,
            },
            ParseStackNode::Loop {
                line_no,
                pred,
                body,
            } => Expression::Loop {
                line_no,
                pred,
                body,
            },
            ParseStackNode::TypeCase {
                line_no,
                expr,
                branches,
            } => Expression::TypeCase {
                line_no,
                expr,
                branches,
                static_type,
            },
            ParseStackNode::Block { line_no, body } => Expression::Block { line_no, body },
            ParseStackNode::Let {
                line_no,
                identifier,
                type_decl,
                init,
                body,
            } => Expression::Let {
                line_no,
                identifier,
                type_decl,
                init,
                body,
            },
            ParseStackNode::ArithOp {
                line_no,
                expr_lhs,
                expr_rhs,
                arith_op_type,
            } => Expression::ArithOp {
                line_no,
                expr_lhs,
                expr_rhs,
                arith_op_type,
            },
            ParseStackNode::Neg { line_no, expr } => Expression::Neg { line_no, expr },
            ParseStackNode::Comp {
                line_no,
                expr_lhs,
                expr_rhs,
                comp_type,
            } => Expression::Comp {
                line_no,
                expr_lhs,
                expr_rhs,
                comp_type,
            },
            ParseStackNode::Eq {
                line_no,
                expr_lhs,
                expr_rhs,
            } => Expression::Eq {
                line_no,
                expr_lhs,
                expr_rhs,
            },
            ParseStackNode::Not { line_no, expr } => Expression::Not { line_no, expr },
            ParseStackNode::IntConst { line_no, val } => Expression::IntConst { line_no, val },
            ParseStackNode::BoolConst { line_no, val } => Expression::BoolConst { line_no, val },
            ParseStackNode::StringConst { line_no, val } => Expression::StringConst {
                line_no,
                val,
                val_id: 0, // This is set later
            },
            ParseStackNode::New { line_no, type_name } => Expression::New { line_no, type_name },
            ParseStackNode::IsVoid { line_no, expr } => Expression::IsVoid { line_no, expr },
            ParseStackNode::Object { line_no, name } => Expression::Object {
                line_no,
                name,
                static_type,
            },
            _ => panic!("Bad node type"),
        }
    }

    pub fn into_comp_type(self) -> CompType {
        match self {
            ParseStackNode::TermLT { .. } => CompType::LT,
            ParseStackNode::TermLEq { .. } => CompType::LEq,
            _ => panic!("Bad node type"),
        }
    }

    pub fn into_arith_type(self) -> ArithOpType {
        match self {
            ParseStackNode::TermAdd { .. } => ArithOpType::Add,
            ParseStackNode::TermSub { .. } => ArithOpType::Sub,
            ParseStackNode::TermMul { .. } => ArithOpType::Mul,
            ParseStackNode::TermDiv { .. } => ArithOpType::Div,
            _ => panic!("Bad node type"),
        }
    }

    pub fn into_object_id(self) -> ObjectID {
        if let ParseStackNode::TermObjectID { obj_name, .. } = self {
            ObjectID::new(obj_name)
        } else {
            panic!("Bad node type")
        }
    }

    pub fn into_type_id(self) -> TypeID {
        if let ParseStackNode::TermTypeID { type_name, .. } = self {
            TypeID::new(type_name)
        } else {
            panic!("Bad node type")
        }
    }
}
