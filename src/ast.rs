
pub struct Program {
     pub functions: Vec<Function>,
     //constants: Vec<Constant>,
 }

pub struct Function {
    pub name: String,
    pub params: Vec<Parameter>,
    pub body: Vec<Statement>,
}


pub struct Parameter {
    pub name: String,
}

#[derive(Debug)]
pub enum Statement {
   If(If),
   While(While),
   DoWhile(DoWhile),
   Let(Let),
   Asm(Asm),
   Return(Return),
   Assign(Assign),
}

#[derive(Debug)]
pub struct If {
    pub condition: Conditional,
    pub body: Vec<Statement>,
    pub else_body: Vec<Statement>,
}

#[derive(Debug)]
pub struct While {
    pub condition: Conditional,
    pub body: Vec<Statement>,
}

#[derive(Debug)]
pub struct DoWhile {
    pub condition: Conditional,
    pub body: Vec<Statement>,
}

#[derive(Debug)]
pub struct Let {
    pub name: String,
    pub value: Expression,
}

#[derive(Debug)]
pub struct Asm {
    pub lines: Vec<String>,
}

#[derive(Debug)]
pub struct Return {
    pub value: Expression,
}

#[derive(Debug)]
pub struct Assign {
    pub name: String,
    pub value: Expression,
}

#[derive(Debug)]
pub enum Conditional {
    Eq(Term, Term),
    NE(Term, Term),
    LT(Term, Term),
    GT(Term, Term),
}

#[derive(Debug)]
pub enum Expression {
    Term(Term),
    Add(Term, Term),
    Sub(Term, Term),
    Mul(Term, Term),
    Div(Term, Term),
    Call(Call),
}

#[derive(Debug)]
pub enum Term {
    Number(i32),
    Variable(String),
}

#[derive(Debug)]
pub struct Call {
    pub name: String,
    pub args: Vec<Term>,
}
