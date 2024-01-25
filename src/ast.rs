
#[derive(Debug, PartialEq)]
pub struct Program {
     pub functions: Vec<Function>,
     //constants: Vec<Constant>,
 }

#[derive(Debug, PartialEq)]
pub struct Function {
    pub name: String,
    pub params: Vec<Parameter>,
    pub body: Vec<Statement>,
}


#[derive(Debug, PartialEq)]
pub struct Parameter {
    pub name: String,
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
}

#[derive(Debug, PartialEq)]
pub struct If {
    pub condition: Conditional,
    pub body: Vec<Statement>,
    pub else_body: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub struct While {
    pub condition: Conditional,
    pub body: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub struct DoWhile {
    pub condition: Conditional,
    pub body: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub struct Let {
    pub name: String,
    pub value: Expression,
}

#[derive(Debug, PartialEq)]
pub struct Asm {
    pub lines: Vec<String>,
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
pub enum Conditional {
    Eq(Term, Term),
    NE(Term, Term),
    LT(Term, Term),
    GT(Term, Term),
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Int(i64),
    Var(String),
    Addition(Box<Expression>, Box<Expression>),
    Subtraction(Box<Expression>, Box<Expression>),
    Term(Term),
    Add(Term, Term),
    Sub(Term, Term),
    Mul(Term, Term),
    Div(Term, Term),
    Call(Call),
}

#[derive(Debug, PartialEq)]
pub enum Term {
    Number(i64),
    Variable(String),
}

#[derive(Debug, PartialEq)]
pub struct Call {
    pub name: String,
    pub args: Vec<Term>,
}
