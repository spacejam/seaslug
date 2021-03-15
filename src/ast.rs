pub struct Module {
    behavior: Behavior,
    items: Vec<Item>,
}

pub struct Struct {
    name: String,
    fields: Vec<Field>,
}

pub struct Enum {
    name: String,
    variants: Vec<Struct>,
}

pub struct Field {
    pub name: String,
    pub ty: Type,
}

pub struct Type;

pub struct Function {
    subfunctions: Vec<SubFunction>,
}

pub struct SubFunction {
    // implicit arity in arguments length
    name: String,
    ret: Type,
    arguments: Vec<Field>,
    statements: Vec<Statement>,
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
