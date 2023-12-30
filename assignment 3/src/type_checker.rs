// L.A. (Luca) Verheul - S3704041
// Sat 30 Dec 2023

// Import handy dbg! macro (shadowing std::dbg! macro)

use crate::{
    dbg,
    parser::{Expression, Judgement},
};

use std::collections::HashSet;
use std::fmt::{Display, Formatter, Result};

enum TypeCheckError {
    UnknownType(String),
    MismatchedTypes,
}

impl Display for TypeCheckError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            TypeCheckError::UnknownType(t) => write!(f, "Unknown type: {}", t),
            TypeCheckError::MismatchedTypes => write!(f, "Mismatched types"),
        }
    }
}

type TypeCheckResult = std::result::Result<(), TypeCheckError>;

/// Returns Ok(()) if the judgement is well-typed, Err(TypeCheckError) otherwise.
fn _type_check(judgement: &Judgement) -> TypeCheckResult {
    // Extract expression and type
    let Judgement::Judgement(expr, typ) = judgement;

    // Check if free variables (unknown types) are present
    let mut free = HashSet::new();
    free_vars(expr, &mut free, &mut HashSet::new());
    dbg!(&free);

    if !free.is_empty() {
        return Err(TypeCheckError::UnknownType(
            free.into_iter().collect::<Vec<String>>().join(", "),
        ));
    }
    Ok(())
}

// borrowed from ass2
fn free_vars(
    expression: &Expression,
    free: &mut HashSet<String>,
    abstr_vars: &mut HashSet<String>,
) {
    dbg!(&expression, &abstr_vars, &free);
    match expression {
        Expression::Application(lexpr, rexpr) => {
            free_vars(lexpr, free, abstr_vars);
            free_vars(rexpr, free, abstr_vars);
        }
        Expression::Abstraction(var, _type, body) => {
            // If this abstraction adds the variable, also remove it.
            if abstr_vars.insert(var.clone()) {
                free_vars(body, free, abstr_vars);
                abstr_vars.remove(var);
            } else {
                // the variable is already in the set, so should not be removed
                free_vars(body, free, abstr_vars);
            }
        }
        Expression::Variable(varname) => {
            if !abstr_vars.contains(varname) {
                free.insert(varname.clone());
            }
        }
    }
}

pub(super) fn type_check(judgement: &Judgement, idx: usize) {
    match _type_check(judgement) {
        Ok(_) => (),
        Err(err_code) => {
            eprintln!(
                "Invalid judgement [{}] caught during parsing on line {}!",
                err_code,
                idx + 1
            );

            std::process::exit(1);
        }
    }
}
