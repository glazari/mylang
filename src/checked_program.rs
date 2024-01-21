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

        Ok(CheckedProgram {
            prog,
            program_env,
            function_envs,
        })
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
                Statement::Let(let_statement) => {
                    if local_variables.contains(&let_statement.name) {
                        return Err(format!(
                            "Duplicate variable name {} in function {}",
                            let_statement.name, function.name
                        ));
                    }
                    local_variables.push(let_statement.name.clone());
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

    fn check_statement(statement: &Statement, f_env: &FuncEnv, p_env: &ProgEnv) -> Result<(), String> {
        match statement {
            Statement::If(if_statement) => {
                Self::check_conditional(&if_statement.condition, f_env, p_env)?;
                Self::check_statements(&if_statement.body, f_env, p_env)?;
                Self::check_statements(&if_statement.else_body, f_env, p_env)?;
            }
            Statement::While(while_statement) => {
                Self::check_conditional(&while_statement.condition, f_env, p_env)?;
                Self::check_statements(&while_statement.body, f_env, p_env)?;
            }
            Statement::DoWhile(do_while_statement) => {
                Self::check_conditional(&do_while_statement.condition, f_env, p_env)?;
                Self::check_statements(&do_while_statement.body, f_env, p_env)?;
            }
            Statement::Let(let_statement) => {
                Self::check_expression(&let_statement.value, f_env, p_env)?;
            }
            Statement::Asm(_) => {
               // No checks, programer is responsible for writing correct assembly
            }
            Statement::Return(return_statement) => {
                Self::check_expression(&return_statement.value, f_env, p_env)?;
            }
            Statement::Assign(assign_statement) => {
                Self::check_expression(&assign_statement.value, f_env, p_env)?;
            }
        }
        Ok(())
    }

    fn check_statements(statements: &Vec<Statement>, f_env: &FuncEnv, p_env: &ProgEnv) -> Result<(), String> {
        for statement in statements {
            Self::check_statement(statement, f_env, p_env)?;
        }
        Ok(())
    }

    fn check_conditional(conditional: &Conditional, f_env: &FuncEnv, p_env: &ProgEnv) -> Result<(), String> {
        match conditional {
           Conditional::Eq(term1, term2) => Self::check_term_pair(term1, term2, f_env, p_env)?,
           Conditional::NE(term1, term2) => Self::check_term_pair(term1, term2, f_env, p_env)?,
           Conditional::LT(term1, term2) => Self::check_term_pair(term1, term2, f_env, p_env)?,
           Conditional::GT(term1, term2) => Self::check_term_pair(term1, term2, f_env, p_env)?, 
        }
        Ok(())
    }
    
    fn check_term(term: &Term, f_env: &FuncEnv, _p_env: &ProgEnv) -> Result<(), String> {
        match term {
            Term::Number(_) => {}
            Term::Variable(variable) => {
                if !f_env.function_params.contains(&variable) && !f_env.local_variables.contains(&variable) {
                    return Err(format!("Variable {} not found", variable));
                }
            }
        }
        Ok(())
    }

    fn check_expression(expression: &Expression, f_env: &FuncEnv, p_env: &ProgEnv) -> Result<(), String> {
        match expression {
            Expression::Term(term) => Self::check_term(term, f_env, p_env)?,
            Expression::Add(term1, term2) => Self::check_term_pair(term1, term2, f_env, p_env)?, 
            Expression::Sub(term1, term2) => Self::check_term_pair(term1, term2, f_env, p_env)?, 
            Expression::Mul(term1, term2) => Self::check_term_pair(term1, term2, f_env, p_env)?,
            Expression::Div(term1, term2) => Self::check_term_pair(term1, term2, f_env, p_env)?,
            Expression::Call(call) => {
                if !p_env.function_names.contains(&call.name) {
                    return Err(format!("Function {} not found", call.name));
                }
                let fun_index = p_env.function_names.iter().position(|x| x == &call.name).unwrap();
                let fun_params = p_env.function_params[fun_index];
                if fun_params != call.args.len() as u32 {
                    return Err(format!("Function {} takes {} parameters, {} given", call.name, fun_params, call.args.len()));
                }

            }
        }
        Ok(())
    }

    fn check_term_pair(term1: &Term, term2: &Term, f_env: &FuncEnv, p_env: &ProgEnv) -> Result<(), String> {
        Self::check_term(term1, f_env, p_env)?;
        Self::check_term(term2, f_env, p_env)?;
        Ok(())
    }

}
