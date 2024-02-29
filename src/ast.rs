
// Some type aliases that are used to make the code more concise
pub type Stmt = Statement;
pub type Exp = Expression;
pub type Op = Operator;

#[derive(Debug, PartialEq)]
pub struct Program {
     pub functions: Vec<Function>,
     pub globals: Vec<Global>,
 }

#[derive(Debug, PartialEq)]
pub struct Function {
    pub name: String,
    pub params: Vec<Parameter>,
    pub body: Vec<Statement>,
    pub ret_type: String,
}


#[derive(Debug, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub ttype: String,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
   If(If),
   While(While),
   DoWhile(DoWhile),
   Let(Let),
   Asm(Asm),
   Return(Return),
   Assign(Assign),
   Call(Call),
}

#[derive(Debug, PartialEq)]
pub struct If {
    pub condition: Expression,
    pub body: Vec<Statement>,
    pub else_body: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub struct While {
    pub condition: Expression,
    pub body: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub struct DoWhile {
    pub condition: Expression,
    pub body: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub struct Let {
    pub name: String,
    pub ttype: String,
    pub value: Expression,
}

#[derive(Debug, PartialEq)]
pub struct Global {
    pub name: String,
    pub value: Expression,
}

#[derive(Debug, PartialEq)]
pub struct Asm {
    pub segments: Vec<ASMSegment>,
}

#[derive(Debug, PartialEq)]
pub enum ASMSegment {
  String(String),
  Variable(String),
  Newline,
}

#[derive(Debug, PartialEq)]
pub struct Return {
    pub value: Expression,
}

#[derive(Debug, PartialEq)]
pub struct Assign {
    pub name: String,
    pub value: Expression,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Int(i64),
    Var(String),
    BinOp(Box<Expression>, Operator, Box<Expression>),
    Call(Call),
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Ne,
    LT,
    GT,
}

#[derive(Debug, PartialEq)]
pub enum Type_ {
    U64
}

#[derive(Debug, PartialEq)]
pub struct Call {
    pub name: String,
    pub args: Vec<Expression>,
}

pub fn binop(left: Exp, op: Op, right: Exp) -> Exp {
    Exp::BinOp(Box::new(left), op, Box::new(right))
}
