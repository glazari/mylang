// Some type aliases that are used to make the code more concise
use crate::file_info::FI;

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
    pub ret_type: Type_,
    pub fi: FI,
}

#[derive(Debug, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub ttype: Type_,
    pub fi: FI,
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
    pub fi: FI,
}

#[derive(Debug, PartialEq)]
pub struct While {
    pub condition: Expression,
    pub body: Vec<Statement>,
    pub fi: FI,
}

#[derive(Debug, PartialEq)]
pub struct DoWhile {
    pub condition: Expression,
    pub body: Vec<Statement>,
    pub fi: FI,
}

#[derive(Debug, PartialEq)]
pub struct Let {
    pub name: String,
    pub ttype: Type_,
    pub value: Expression,
    pub fi: FI,
}

#[derive(Debug, PartialEq)]
pub struct Global {
    pub name: String,
    pub value: Expression,
    pub ttype: Type_,
    pub fi: FI,
}

#[derive(Debug, PartialEq)]
pub struct Asm {
    pub segments: Vec<ASMSegment>,
    pub fi: FI,
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
    pub fi: FI,
}

#[derive(Debug, PartialEq)]
pub struct Assign {
    pub name: String,
    pub value: Expression,
    pub fi: FI,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    U64(u64, FI),
    I64(i64, FI),
    Var(String, FI),
    BinOp(Box<Expression>, Operator, Box<Expression>, FI),
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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Type_ {
    U64(FI),
    I64(FI),
}

#[derive(Debug, PartialEq)]
pub struct Call {
    pub name: String,
    pub args: Vec<Expression>,
    pub fi: FI,
}

impl Expression {
    pub fn fi(&self) -> FI {
        match self {
            Expression::U64(_, fi) => *fi,
            Expression::I64(_, fi) => *fi,
            Expression::Var(_, fi) => *fi,
            Expression::BinOp(_, _, _, fi) => *fi,
            Expression::Call(call) => call.fi,
        }
    }
}

impl Statement {
    pub fn fi(&self) -> FI {
        match self {
            Statement::If(if_) => if_.fi,
            Statement::While(while_) => while_.fi,
            Statement::DoWhile(do_while) => do_while.fi,
            Statement::Let(let_) => let_.fi,
            Statement::Asm(asm) => asm.fi,
            Statement::Return(return_) => return_.fi,
            Statement::Assign(assign) => assign.fi,
            Statement::Call(call) => call.fi,
        }
    }
}

impl Type_ {
    pub fn fi(&self) -> FI {
        match self {
            Type_::U64(fi) => *fi,
            Type_::I64(fi) => *fi,
        }
    }
    pub fn zero(&self) -> Type_ {
        match self {
            Type_::U64(_) => Type_::U64(FI::zero()),
            Type_::I64(_) => Type_::I64(FI::zero()),
        }
    }
    pub fn eq(&self, other: &Type_) -> bool {
        let a = self.zero();
        let b = other.zero();
        a == b
    }
    pub fn neq(&self, other: &Type_) -> bool {
        !self.eq(other)
    }
}


pub fn binop(left: Exp, op: Op, right: Exp) -> Exp {
    let left_fi = left.fi();
    let right_fi = right.fi();
    let fi = left_fi.merge(&right_fi);
    Exp::BinOp(Box::new(left), op, Box::new(right), fi)
}
