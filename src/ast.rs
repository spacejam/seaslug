pub struct Module {
    pub behavior: Behavior,
    pub items: Vec<Item>,
}

pub struct Struct {
    pub name: String,
    pub fields: Vec<Field>,
}

pub struct Enum {
    pub name: String,
    pub variants: Vec<Struct>,
}

pub struct Field {
    pub name: String,
    pub ty: Type,
}

pub struct Type;

pub struct Function {
    pub subfunctions: Vec<SubFunction>,
}

pub struct SubFunction {
    // implicit arity in arguments length
    pub name: String,
    pub ret: Type,
    pub arguments: Vec<Field>,
    pub statements: Vec<Statement>,
}

pub enum Statement {
    Assign(String, Expression),
    Expression(Expression),
}

pub enum Expression {
    Call,
    Case,
}

pub enum Item {
    Import { path: String },
    Export { name: String, arity: usize },
    Struct(Struct),
    Function(Function),
}

pub enum Behavior {
    Http,
    Grpc,
    StateMachine,
}
