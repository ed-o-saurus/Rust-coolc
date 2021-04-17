// The classes declared in this file are used to construct an abstract syntax tree

use indexmap::IndexMap;
use std::cmp::Eq;
use std::cmp::{Ord, Ordering};
use std::fmt;
use std::hash::Hash;
use std::ops::Range;

// Each class has a 'tag' - a unique integer value assign in DFS order
// Object class has a tag of 0
// The family field is the range of the tags in all of the class's descendants (including itself).

pub struct Class {
    pub file_name: String,
    pub file_no: u32,
    pub basic: bool, // Is this class provided by the runtime environment? (i.e. Object, Int, String, Bool, IO)
    pub line_no: i16,
    pub parent_name: Option<TypeID>,
    pub attrs: Vec<Attr>,                    // Attributes of the class
    pub methods: IndexMap<ObjectID, Method>, // Methods of this class indexed by their names
    pub family: Range<u32>,
    pub child_names: Vec<TypeID>, // The names of classes that inherit from this class
    pub method_name_to_pos: IndexMap<ObjectID, i16>, // Given the name of a method (native or inherited) gives the position in the dispatch table.
    pub dispatch_table: Vec<(TypeID, ObjectID)>, // A list of all available methods in position order
}

impl Class {
    // Get the tag of this class
    pub fn tag(&self) -> u32 {
        self.family.start
    }
}

// Class attributes
pub struct Attr {
    pub line_no: i16,
    pub name: ObjectID,
    pub type_decl: TypeID,
    pub init: Expression,
    pub self_offset: i16, // Offset from self pointer ($s0)
}

// Class methods
pub struct Method {
    pub line_no: i16,
    pub formals: Vec<Formal>, // Arguments and their types
    pub return_type: TypeID,
    pub expr: Expression,
}

// get_sig duplicates the information from a Method necessary to compare its signature to a dispatch
impl Method {
    pub fn get_sig(&self) -> Method {
        let Method {
            line_no,
            formals,
            return_type,
            ..
        } = self;

        Method {
            line_no: *line_no,
            formals: formals.clone(),
            return_type: return_type.clone(),
            expr: Expression::NoExpr, // No need to copy the contents
        }
    }
}

// Used to represent the arguments of a method declaration
#[derive(Clone)]
pub struct Formal {
    pub line_no: i16,
    pub name: ObjectID,
    pub type_decl: TypeID,
}

// One branch of a TypeCase statement
pub struct Branch {
    pub line_no: i16,
    pub name: ObjectID,
    pub type_decl: TypeID,
    pub expr: Expression,
    pub family: Range<u32>, // The family of the classes from name
}

impl Branch {
    // The (static) type of the returned expression
    pub fn static_type(&self) -> TypeID {
        self.expr.static_type()
    }
}

// Used to order the branches in a TypeCase expression
// This ordering ensures that the least case is checked before a higher case.
impl Ord for Branch {
    fn cmp(&self, other: &Self) -> Ordering {
        other.family.start.cmp(&self.family.start) // Note that the ordering is reversed
    }
}

impl PartialOrd for Branch {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Branch {
    fn eq(&self, other: &Self) -> bool {
        self.family.start == other.family.start
    }
}

impl Eq for Branch {}

// Type of arithmetic operation
pub enum ArithOpType {
    Add,
    Sub,
    Mul,
    Div,
}

impl fmt::Display for ArithOpType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ArithOpType::Add => write!(f, "+"),
            ArithOpType::Sub => write!(f, "-"),
            ArithOpType::Mul => write!(f, "*"),
            ArithOpType::Div => write!(f, "/"),
        }
    }
}

// Type of comparison operation
pub enum CompType {
    LT,
    LEq,
}

impl fmt::Display for CompType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CompType::LT => write!(f, "<"),
            CompType::LEq => write!(f, "<="),
        }
    }
}

pub enum Expression {
    Assign {
        line_no: i16,
        name: ObjectID,
        expr: Box<Expression>,
    },
    Dispatch {
        line_no: i16,
        expr: Box<Expression>,
        type_name: Option<TypeID>, // static dispatch if Some
        name: ObjectID,
        args: Vec<Expression>,
        static_type: TypeID,
    },
    Cond {
        line_no: i16,
        pred: Box<Expression>,
        then_expr: Box<Expression>,
        else_expr: Box<Expression>,
        static_type: TypeID,
    },
    Loop {
        line_no: i16,
        pred: Box<Expression>,
        body: Box<Expression>,
    },
    TypeCase {
        line_no: i16,
        expr: Box<Expression>,
        branches: Vec<Branch>,
        static_type: TypeID,
    },
    Block {
        line_no: i16,
        body: Vec<Expression>,
    },
    Let {
        line_no: i16,
        identifier: ObjectID,
        type_decl: TypeID,
        init: Box<Expression>,
        body: Box<Expression>,
    },
    ArithOp {
        line_no: i16,
        expr_lhs: Box<Expression>,
        expr_rhs: Box<Expression>,
        arith_op_type: ArithOpType,
    },
    Neg {
        line_no: i16,
        expr: Box<Expression>,
    },
    Comp {
        line_no: i16,
        expr_lhs: Box<Expression>,
        expr_rhs: Box<Expression>,
        comp_type: CompType,
    },
    Eq {
        line_no: i16,
        expr_lhs: Box<Expression>,
        expr_rhs: Box<Expression>,
    },
    Not {
        line_no: i16,
        expr: Box<Expression>,
    },
    IntConst {
        line_no: i16,
        val: u32,
    },
    BoolConst {
        line_no: i16,
        val: bool,
    },
    StringConst {
        line_no: i16,
        val: String,
        val_id: u32, // A unique value used to reference the string constant in the assembly output
    },
    New {
        line_no: i16,
        type_name: TypeID,
    },
    IsVoid {
        line_no: i16,
        expr: Box<Expression>,
    },
    NoExpr,
    Object {
        line_no: i16,
        name: ObjectID,
        static_type: TypeID,
    },
}

impl Expression {
    pub fn line_no(&self) -> i16 {
        match self {
            Expression::Assign { line_no, .. }
            | Expression::Dispatch { line_no, .. }
            | Expression::Cond { line_no, .. }
            | Expression::Loop { line_no, .. }
            | Expression::TypeCase { line_no, .. }
            | Expression::Block { line_no, .. }
            | Expression::Let { line_no, .. }
            | Expression::ArithOp { line_no, .. }
            | Expression::Neg { line_no, .. }
            | Expression::Comp { line_no, .. }
            | Expression::Eq { line_no, .. }
            | Expression::Not { line_no, .. }
            | Expression::IntConst { line_no, .. }
            | Expression::BoolConst { line_no, .. }
            | Expression::StringConst { line_no, .. }
            | Expression::New { line_no, .. }
            | Expression::IsVoid { line_no, .. }
            | Expression::Object { line_no, .. } => *line_no,
            _ => panic!("Bad expr type"),
        }
    }

    // Static type of the expression
    // Some expression types need to be set during semantic analysis
    pub fn static_type(&self) -> TypeID {
        match self {
            Expression::Assign { expr, .. } => expr.static_type(),
            Expression::Loop { .. } => TypeID::new_object(),
            Expression::Block { body, .. } => body.last().unwrap().static_type(),
            Expression::Let { body, .. } => body.static_type(),
            Expression::StringConst { .. } => TypeID::new_string(),
            Expression::New { type_name, .. } => type_name.clone(),
            Expression::NoExpr => TypeID::new_no_type(),

            Expression::Comp { .. }
            | Expression::Eq { .. }
            | Expression::Not { .. }
            | Expression::IsVoid { .. }
            | Expression::BoolConst { .. } => TypeID::new_bool(),

            Expression::ArithOp { .. } | Expression::Neg { .. } | Expression::IntConst { .. } => {
                TypeID::new_int()
            }

            Expression::Dispatch { static_type, .. }
            | Expression::Cond { static_type, .. }
            | Expression::TypeCase { static_type, .. }
            | Expression::Object { static_type, .. } => static_type.clone(),
        }
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct ObjectID {
    obj_name: String,
}

impl ObjectID {
    pub fn new(obj_name: String) -> ObjectID {
        ObjectID { obj_name }
    }

    pub fn new_self() -> ObjectID {
        ObjectID {
            obj_name: "self".to_string(),
        }
    }

    pub fn new_main() -> ObjectID {
        ObjectID {
            obj_name: "main".to_string(),
        }
    }

    pub fn is_self(&self) -> bool {
        self.obj_name == "self"
    }
}

impl fmt::Display for ObjectID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.obj_name)
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct TypeID {
    type_name: String,
}

impl TypeID {
    pub fn new(type_name: String) -> TypeID {
        TypeID { type_name }
    }

    // starting point for depth-first-search of the inheritance tree
    pub fn root_class_name() -> TypeID {
        TypeID {
            type_name: "Object".to_string(),
        }
    }

    pub fn new_object() -> TypeID {
        TypeID {
            type_name: "Object".to_string(),
        }
    }

    pub fn new_io() -> TypeID {
        TypeID {
            type_name: "IO".to_string(),
        }
    }

    pub fn new_int() -> TypeID {
        TypeID {
            type_name: "Int".to_string(),
        }
    }

    pub fn new_bool() -> TypeID {
        TypeID {
            type_name: "Bool".to_string(),
        }
    }

    pub fn new_string() -> TypeID {
        TypeID {
            type_name: "String".to_string(),
        }
    }

    pub fn new_main() -> TypeID {
        TypeID {
            type_name: "Main".to_string(),
        }
    }

    pub fn new_self_type() -> TypeID {
        TypeID {
            type_name: "SELF_TYPE".to_string(),
        }
    }

    pub fn new_no_type() -> TypeID {
        TypeID {
            type_name: "_no_type".to_string(),
        }
    }

    pub fn is_system_type(&self) -> bool {
        self.type_name.as_bytes()[0] == b'_'
    }

    pub fn is_self_type(&self) -> bool {
        self.type_name == "SELF_TYPE"
    }

    pub fn is_no_type(&self) -> bool {
        self.type_name == "_no_type"
    }

    pub fn is_int(&self) -> bool {
        self.type_name == "Int"
    }

    pub fn is_bool(&self) -> bool {
        self.type_name == "Bool"
    }

    pub fn is_string(&self) -> bool {
        self.type_name == "String"
    }

    pub fn len(&self) -> usize {
        self.type_name.len()
    }
}

impl fmt::Display for TypeID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.type_name)
    }
}
