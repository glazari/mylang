use crate::ast::*;

pub struct CheckedProgram {
    pub prog: Program,
    pub program_env: ProgEnv,
    pub function_envs: Vec<FuncEnv>,
}

pub struct ProgEnv {
    function_names: Vec<String>,
    function_params: Vec<u32>,
}

pub struct FuncEnv {
    pub function_params: Vec<String>,
    pub local_variables: Vec<String>,
}

impl CheckedProgram {
    pub fn check(prog: Program) -> Result<CheckedProgram, String> {
        let mut function_names = Vec::new();
        let mut function_params = Vec::new();
        let mut function_envs = Vec::new();

        for function in &prog.functions {
            function_names.push(function.name.clone());
            function_params.push(function.params.len() as u32);
        }

        let has_main = function_names.contains(&"main".to_string());
        if !has_main {
            return Err("No main function found".to_string());
        }

        let program_env = ProgEnv {
            function_names,
            function_params,
        };

        for function in &prog.functions {
            let function_env = Self::check_function(function, &program_env)?;
            function_envs.push(function_env);
        }

        Ok(CheckedProgram { prog, program_env, function_envs})
    }

    fn check_function(function: &Function, prog_env: &ProgEnv) -> Result<FuncEnv, String> {
        let mut function_params = Vec::new();
        let mut local_variables = Vec::new();

        for param in &function.params {
            if function_params.contains(&param.name) {
                return Err(format!(
                    "Duplicate parameter name {} in function {}",
                    param.name, function.name
                ));
            }
            function_params.push(param.name.clone());
        }

        for statement in &function.body {
            match statement {
                Stmt::Let(let_stmt) => {
                    if local_variables.contains(&let_stmt.name) {
                        return Err(format!(
                            "Duplicate variable name {} in function {}",
                            let_stmt.name, function.name
                        ));
                    }
                    local_variables.push(let_stmt.name.clone());
                }
                _ => {}
            }
        }

        let function_env = FuncEnv {
            function_params,
            local_variables,
        };

        Self::check_statements(&function.body, &function_env, &prog_env)?;

        Ok(function_env)
    }

    fn check_statement(stmt: &Stmt, f_env: &FuncEnv, p_env: &ProgEnv) -> Result<(), String> {
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
                Self::check_expression(&let_stmt.value, f_env, p_env)?;
            }
            Stmt::Asm(_) => {
                // No checks, programer is responsible for writing correct assembly
            }
            Stmt::Return(return_stmt) => {
                Self::check_expression(&return_stmt.value, f_env, p_env)?;
            }
            Stmt::Assign(assign_stmt) => {
                Self::check_expression(&assign_stmt.value, f_env, p_env)?;
            }
        }
        Ok(())
    }

    fn check_comparison_operator(op: &Exp) -> Result<(), String> {
        if let Exp::BinOp(_, op, _) = op {
            match op {
                Op::Eq | Op::Ne | Op::LT | Op::GT => return Ok(()),
                _ => return Err(format!("Invalid comparison expression: {:?}", op)),
            }
        }
        Err(format!("Invalid comparison expression: {:?}", op))
    }

    fn check_statements(stmts: &Vec<Stmt>, f_env: &FuncEnv, p_env: &ProgEnv) -> Result<(), String> {
        for statement in stmts {
            Self::check_statement(statement, f_env, p_env)?;
        }
        Ok(())
    }

    fn check_expression(exp: &Exp, f_env: &FuncEnv, p_env: &ProgEnv) -> Result<(), String> {
        // chect:
        // 1. all variables are defined
        // 2. all function calls are defined and have the correct number of arguments
        // 3. In the future check the types of the expression
        match exp {
            Exp::Int(_) => { /* No checks needed, until we add type checking */ }
            Exp::Var(variable) => {
                if !f_env.function_params.contains(&variable)
                    && !f_env.local_variables.contains(&variable)
                {
                    return Err(format!("Variable {} not found", variable));
                }
            }
            Exp::BinOp(e1, _op, e2) => {
                Self::check_expression(&e1, f_env, p_env)?;
                Self::check_expression(&e2, f_env, p_env)?;
            }
            Exp::Call(call) => {
                if !p_env.function_names.contains(&call.name) {
                    return Err(format!("Function {} not found", call.name));
                }
                let fun_index = p_env
                    .function_names
                    .iter()
                    .position(|x| x == &call.name)
                    .unwrap();
                let fun_params = p_env.function_params[fun_index];
                if fun_params != call.args.len() as u32 {
                    return Err(format!(
                        "Function {} takes {} parameters, {} given",
                        call.name,
                        fun_params,
                        call.args.len()
                    ));
                }
            }
        }
        Ok(())
    }
}
