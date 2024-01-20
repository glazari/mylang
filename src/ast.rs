
struct Program {
     functions: Vec<Function>,
     //constants: Vec<Constant>,
 }

struct Function {
    name: String,
    params: Vec<Parameter>,
    body: Vec<Statement>,
}


struct Parameter {
    name: String,
}

enum Statement {
   If(If),
   While(While),
   DoWhile(DoWhile),
   Let(Let),
   Asm(Asm),
   Return(Return),
   Assign(Assign),
}

struct If {
    condition: Conditional,
    body: Vec<Statement>,
    else_body: Vec<Statement>,
}

struct While {
    condition: Conditional,
    body: Vec<Statement>,
}

struct DoWhile {
    condition: Conditional,
    body: Vec<Statement>,
}

struct Let {
    name: String,
    value: Expression,
}

struct Asm {
    lines: Vec<String>,
}

struct Return {
    value: Expression,
}

struct Assign {
    name: String,
    value: Expression,
}

enum Conditional {
    Eq(Term, Term),
    NE(Term, Term),
    LT(Term, Term),
    GT(Term, Term),
}

enum Expression {
    Term(Term),
    Add(Term, Term),
    Sub(Term, Term),
    Mul(Term, Term),
    Div(Term, Term),
    Call(Call),
}

enum Term {
    Number(i32),
    Variable(String),
}

struct Call {
    name: String,
    args: Vec<Term>,
}
