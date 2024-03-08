use crate::ast::*;
use crate::file_info::{FI, underline_error};

#[derive(Debug)]
pub struct CheckedProgram {
    pub prog: Program,
    pub program_env: ProgEnv,
    pub function_envs: Vec<FuncEnv>,
}

#[derive(Debug)]
pub struct ProgEnv {
    fn_sigs: Vec<FuncSig>,
    pub global_values: Vec<i64>,
    pub globals_def: Vec<Variable>,
}

#[derive(Debug)]
pub struct FuncSig {
    pub name: String,
    pub params: Vec<Type_>,
    pub ret_type: Type_,
}

#[derive(Debug)]
pub struct Variable {
    pub name: String,
    pub ttype: Type_,
}

#[derive(Debug)]
pub struct FuncEnv {
    pub function_params: Vec<Variable>,
    pub local_variables: Vec<Variable>,
    pub ret_type: Type_,
}

#[derive(Debug)]
pub struct CheckError {
    pub msg: String,
    pub fi: FI,
}

impl CheckError {
    pub fn new(msg: &str, fi: FI) -> CheckError {
        CheckError { msg: msg.to_string(), fi }
    }
    pub fn pretty_print(&self, input: &str) {
        println!("{}", self.msg);
        println!("{}", underline_error(input, &self.fi));
    }
}

impl CheckedProgram {
    pub fn check(prog: Program) -> Result<CheckedProgram, CheckError> {
        let mut function_envs = Vec::new();
        let mut fn_sigs: Vec<FuncSig> = Vec::new();

        for function in &prog.functions {
            if fn_sigs.iter().any(|x| x.name == function.name) {
                return Err(CheckError::new(&format!("Duplicate function name {}", function.name), function.fi));
            }
            fn_sigs.push(FuncSig {
                name: function.name.clone(),
                params: function.params.iter().map(|x| x.ttype.clone()).collect(),
                ret_type: function.ret_type,
            });
        }

        let has_main = fn_sigs
            .iter()
            .any(|x| x.name == "main" && x.params.is_empty());
        if !has_main {
            return Err(CheckError::new("No main function found", FI::zero()));
        }

        let global_values = Self::resolve_global_values(&prog)?;
        let mut globals_def = Vec::new();
        for global in &prog.globals {
            globals_def.push(Variable {
                name: global.name.clone(),
                ttype: global.ttype.clone(),
            });
        }

        let program_env = ProgEnv {
            fn_sigs,
            global_values,
            globals_def,
        };

        for function in &prog.functions {
            let function_env = Self::check_function(function, &program_env)?;
            function_envs.push(function_env);
        }

        Ok(CheckedProgram {
            prog,
            program_env,
            function_envs,
        })
    }

    fn resolve_global_values(prog: &Program) -> Result<Vec<i64>, CheckError> {
        let dependencies: Vec<Vec<usize>> = Self::find_global_dependencies(prog);

        let mut state: Vec<usize> = vec![0; prog.globals.len()];
        // 0 - not visited, 1 - visiting, 2 - visited
        let mut stack = Vec::new();

        for i in 0..prog.globals.len() {
            if state[i] == 0 {
                Self::global_dfs(i, &dependencies, &mut state, &mut stack);
            }
        }

        let mut global_values = vec![0; prog.globals.len()]; // will be filled with the values of the globals
        let names: Vec<String> = prog.globals.iter().map(|x| x.name.clone()).collect();
        for i in 0..prog.globals.len() {
            let global = &prog.globals[stack[i] as usize];
            global_values[stack[i]] =
                Self::eval_global_expression(&global.value, &global_values, &names);
        }

        Ok(global_values)
    }

    fn eval_global_expression(exp: &Exp, global_values: &Vec<i64>, names: &Vec<String>) -> i64 {
        match exp {
            Exp::U64(n, _) => *n,
            Exp::Var(var, _) => {
                let index = names.iter().position(|x| x == var);
                global_values[index.unwrap()]
            }
            Exp::BinOp(e1, op, e2, _) => {
                let v1 = Self::eval_global_expression(e1, global_values, names);
                let v2 = Self::eval_global_expression(e2, global_values, names);
                match op {
                    Op::Add => v1 + v2,
                    Op::Sub => v1 - v2,
                    Op::Mul => v1 * v2,
                    Op::Div => v1 / v2,
                    Op::Mod => v1 % v2,
                    Op::Eq => panic!("Comparison operators not allowed in global expressions"),
                    Op::Ne => panic!("Comparison operators not allowed in global expressions"),
                    Op::LT => panic!("Comparison operators not allowed in global expressions"),
                    Op::GT => panic!("Comparison operators not allowed in global expressions"),
                }
            }
            Exp::Call(_) => {
                panic!("Function calls not allowed in global expressions");
            }
        }
    }

    fn global_dfs(
        n: usize,
        deps: &Vec<Vec<usize>>,
        state: &mut Vec<usize>,
        stack: &mut Vec<usize>,
    ) {
        state[n] = 1;
        for dep in &deps[n] {
            if state[*dep] == 0 {
                Self::global_dfs(*dep, deps, state, stack);
            } else if state[*dep] == 1 {
                panic!("Cyclic dependency in global variables");
            }
        }
        state[n] = 2;
        stack.push(n);
    }

    fn find_global_dependencies(prog: &Program) -> Vec<Vec<usize>> {
        let mut dependencies = Vec::with_capacity(prog.globals.len());
        for global in &prog.globals {
            dependencies.push(Self::vars_in_global_expression(
                &global.value,
                &prog.globals,
            ));
        }
        dependencies
    }

    fn vars_in_global_expression(exp: &Exp, globals: &Vec<Global>) -> Vec<usize> {
        match exp {
            Exp::U64(_, _) => Vec::new(),
            Exp::Var(var, _) => {
                let index = globals.iter().position(|x| x.name == *var);
                Vec::from([index.unwrap()])
            }
            Exp::BinOp(e1, _op, e2, _) => {
                let mut vars = Self::vars_in_global_expression(e1, globals);
                vars.append(&mut Self::vars_in_global_expression(e2, globals));
                vars
            }
            Exp::Call(_call) => {
                panic!("Function calls not allowed in global expressions");
            }
        }
    }

    fn check_function(function: &Function, prog_env: &ProgEnv) -> Result<FuncEnv, CheckError> {
        let mut function_params = Vec::new();
        let mut local_variables = Vec::new();

        for param in &function.params {
            let p_var = Variable {
                name: param.name.clone(),
                ttype: param.ttype.clone(),
            };

            if function_params
                .iter()
                .any(|x: &Variable| x.name == param.name)
            {
                return Err(
                    CheckError::new(&format!("Duplicate parameter name {} in function {}", param.name, function.name), param.fi)
                );
            }
            function_params.push(p_var);
        }

        for statement in &function.body {
            match statement {
                Stmt::Let(let_stmt) => {
                    let var = Variable {
                        name: let_stmt.name.clone(),
                        ttype: let_stmt.ttype.clone(),
                    };
                    if local_variables
                        .iter()
                        .any(|x: &Variable| x.name == let_stmt.name)
                    {
                        return Err(
                            CheckError::new(&format!("Duplicate variable name {} in function {}", let_stmt.name, function.name), let_stmt.fi)
                        );
                    }
                    local_variables.push(var);
                }
                _ => {}
            }
        }

        let function_env = FuncEnv {
            function_params,
            local_variables,
            ret_type: function.ret_type.clone(),
        };

        Self::check_statements(&function.body, &function_env, &prog_env)?;

        Ok(function_env)
    }

    fn check_statement(stmt: &Stmt, f_env: &FuncEnv, p_env: &ProgEnv) -> Result<(), CheckError> {
        match stmt {
            Stmt::If(if_stmt) => {
                // Top level of the condition must be a comparison
                Self::check_comparison_operator(&if_stmt.condition)?;
                Self::check_expression(&if_stmt.condition, f_env, p_env)?;
                Self::check_statements(&if_stmt.body, f_env, p_env)?;
                Self::check_statements(&if_stmt.else_body, f_env, p_env)?;
            }
            Stmt::While(while_stmt) => {
                Self::check_comparison_operator(&while_stmt.condition)?;
                Self::check_expression(&while_stmt.condition, f_env, p_env)?;
                Self::check_statements(&while_stmt.body, f_env, p_env)?;
            }
            Stmt::DoWhile(do_while_stmt) => {
                Self::check_comparison_operator(&do_while_stmt.condition)?;
                Self::check_expression(&do_while_stmt.condition, f_env, p_env)?;
                Self::check_statements(&do_while_stmt.body, f_env, p_env)?;
            }
            Stmt::Let(let_stmt) => {
                let exp_type = Self::check_expression(&let_stmt.value, f_env, p_env)?;
                if exp_type.neq(&let_stmt.ttype) {
                    return Err(
                        CheckError::new(&format!("Type mismatch in let statement: {:?} and {:?}", exp_type, let_stmt.ttype), let_stmt.fi)
                    );
                }
            }
            Stmt::Asm(_) => {} // No checks, programer is responsible for writing correct assembly
            Stmt::Return(return_stmt) => {
                let exp_type = Self::check_expression(&return_stmt.value, f_env, p_env)?;
                if exp_type.neq(&f_env.ret_type) {
                    return Err(
                        CheckError::new(&format!("Type mismatch in return statement: {:?} and {:?}", exp_type, f_env.ret_type), return_stmt.fi)
                    );
                }
            }
            Stmt::Assign(assign_stmt) => {
                let exp_type = Self::check_expression(&assign_stmt.value, f_env, p_env)?;
                let var = p_env
                    .get_var(&assign_stmt.name, f_env)
                    .ok_or(
                        CheckError::new(&format!("Variable {} not found", assign_stmt.name), assign_stmt.fi)
                    )?;
                if exp_type.neq(&var.ttype) {
                    return Err(
                        CheckError::new(&format!("Type mismatch in assignment: {:?} and {:?}", exp_type, var.ttype), assign_stmt.fi)
                    );
                }
            }
            Stmt::Call(call) => {
                Self::check_call(call, f_env, p_env)?;
            }
        }
        Ok(())
    }

    fn check_comparison_operator(exp: &Exp) -> Result<(), CheckError> {
        if let Exp::BinOp(_, op, _, _) = exp {
            match op {
                Op::Eq | Op::Ne | Op::LT | Op::GT => return Ok(()),
                _ => return Err(
                    CheckError::new(&format!("Invalid comparison expression: {:?}", op), exp.fi())
                ),
            }
        }
        Err(
            CheckError::new(&format!("Invalid comparison expression: {:?}", exp), exp.fi())
        )
    }

    fn check_statements(stmts: &Vec<Stmt>, f_env: &FuncEnv, p_env: &ProgEnv) -> Result<(), CheckError> {
        for statement in stmts {
            Self::check_statement(statement, f_env, p_env)?;
        }
        Ok(())
    }

    fn check_expression(exp: &Exp, f_env: &FuncEnv, p_env: &ProgEnv) -> Result<Type_, CheckError> {
        // chect:
        // 1. all variables are defined
        // 2. all function calls are defined and have the correct number of arguments
        // 3. In the future check the types of the expression
        let ttype = match exp {
            Exp::U64(_, _) => Type_::U64(FI::zero()),
            Exp::Var(variable, _) => {
                p_env
                    .get_var(variable, f_env)
                    .ok_or(
                        CheckError::new(&format!("Variable {} not found", variable), exp.fi())
                    )?
                    .ttype
            }
            Exp::BinOp(e1, _op, e2, _) => {
                let ltype = Self::check_expression(&e1, f_env, p_env)?;
                let rtype = Self::check_expression(&e2, f_env, p_env)?;
                if ltype.neq(&rtype) {
                    return Err(
                        CheckError::new(&format!("Type mismatch in binary operation: {:?} and {:?}", ltype, rtype), exp.fi())
                    );
                }
                ltype
            }
            Exp::Call(call) => Self::check_call(call, f_env, p_env)?,
        };
        Ok(ttype)
    }

    fn check_call(call: &Call, f_env: &FuncEnv, p_env: &ProgEnv) -> Result<Type_, CheckError> {
        let fn_sig = p_env
            .get_signature(&call.name)
            .ok_or(
                CheckError::new(&format!("Function {} not found", call.name), call.fi)
            )?;

        if fn_sig.params.len() != call.args.len() {
            return Err(
                CheckError::new(&format!("Function {} takes {} parameters, {} given", call.name, fn_sig.params.len(), call.args.len()), call.fi)
                );
        }

        for (i, arg) in call.args.iter().enumerate() {
            let exp_type = Self::check_expression(arg, f_env, p_env)?;
            if exp_type.neq(&fn_sig.params[i]) {
                return Err(
                    CheckError::new(&format!("Type mismatch in function call: {:?} and {:?}", exp_type, fn_sig.params[i]), arg.fi())
                );
            }
        }

        Ok(fn_sig.ret_type)
    }
}

impl ProgEnv {
    pub fn get_signature(&self, name: &str) -> Option<&FuncSig> {
        self.fn_sigs.iter().find(|x| x.name == name)
    }

    pub fn get_global_def(&self, name: &str) -> Option<&Variable> {
        self.globals_def.iter().find(|x| x.name == name)
    }

    pub fn get_var<'a>(&'a self, name: &str, f_env: &'a FuncEnv) -> Option<&'a Variable> {
        if let Some(var) = f_env.get_var(name) {
            return Some(var);
        }
        self.get_global_def(name)
    }
}

impl FuncEnv {
    fn get_var(&self, name: &str) -> Option<&Variable> {
        if let Some(var) = self.function_params.iter().find(|x| x.name == name) {
            return Some(var);
        }
        if let Some(var) = self.local_variables.iter().find(|x| x.name == name) {
            return Some(var);
        }
        None
    }

    pub fn get_local_pos(&self, name: &str) -> Option<usize> {
        return self.local_variables.iter().position(|x| x.name == name);
    }
    pub fn get_param_pos(&self, name: &str) -> Option<usize> {
        return self.function_params.iter().position(|x| x.name == name);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parser::parse_program;
    use crate::tokenizer::tokenize;

    fn check_program(input: &str) -> Result<CheckedProgram, CheckError> {
        let ast = match parse_program(tokenize(input)) {
            Ok(ast) => ast,
            Err(e) => {
                e.pretty_print(input);
                panic!("Error in parsing");
            }
        };

        CheckedProgram::check(ast)
    }

    #[test]
    fn test_check_program_errors() {
        assert_some_error(
            "Duplicate variable name in function",
            r#"
        fn main() -> u64 {
            let a: u64 = 5;
            let a: u64 = 6;
        }
        "#,
        );

        assert_some_error(
            "Duplicate parameter name in function",
            r#"fn main(a: u64, a: u64) -> u64 {}"#,
        );

        assert_some_error("No main function found", r#""#);

        assert_some_error(
            "Duplicate function name",
            r#"
        fn main() -> u64 {}
        fn add() -> u64 {}
        fn add() -> u64 {}
        "#,
        );

        assert_some_error(
            "Variable not found",
            r#"
        fn main() -> u64 {
            a = 5;
        }
        "#,
        );
    }

    #[test]
    fn test_type_mismatches() {
        assert_some_error(
            "Type mismatch in let statement",
            r#"fn main() -> u64 {
            let a: u64 = 5;
            let b: i64 = a + 5;
            }
            "#,
        );

        assert_some_error(
            "Type mismatch in return statement",
            r#"fn main() -> u64 {}
            fn add(a: u64) -> i64 {
                return a;
            }
            "#,
        );

        assert_some_error(
            "Type mismatch in binary operation",
            r#"fn main() -> u64 {}
            fn add(a: u64, b: i64) -> u64 {
                let c: u64 = a + b;
            }
            "#,
        );
        assert_some_error(
            "Type mismatch in parameter",
            r#"fn main() -> u64 {
            let a: u64 = 5;
            add(a);
            }
            fn add(a: i64) -> u64 {}
            "#,
        );

        assert_some_error(
            "Type mismatch in function return",
            r#"fn main() -> u64 {
            let a: u64 = add(5);
            }
            fn add(a: u64) -> i64 {}
            "#,
        );
    }

    fn assert_some_error(msg: &str, input: &str) {
        let got = check_program(input);
        match got {
            Ok(_) => {
                panic!(
                    "Expected error: {}\n\ngot program:\n{:#?}",
                    msg,
                    got.unwrap()
                );
            }
            Err(_) => {}
        }
    }
}
