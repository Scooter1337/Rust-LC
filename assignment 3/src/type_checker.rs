// L.A. (Luca) Verheul - S3704041
// Sat 30 Dec 2023

// Import handy dbg! macro (shadowing std::dbg! macro)

use crate::{
    dbg,
    parser::{Expression, Judgement, Type},
};

use std::collections::HashSet;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
enum TypeCheckError {
    UnknownType(String),
    MismatchedTypes(String),
}

impl Display for TypeCheckError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            TypeCheckError::UnknownType(t) => write!(f, "Unknown type: {}", t),
            TypeCheckError::MismatchedTypes(t) => {
                write!(f, "Mismatched types, unknown type: {}", t)
            }
        }
    }
}

type TypeCheckResult = std::result::Result<(), TypeCheckError>;

/// Returns Ok(()) if the judgement is well-typed, Err(TypeCheckError) otherwise.
fn _type_check(judgement: &Judgement) -> TypeCheckResult {
    // Extract expression and type
    let Judgement::Judgement(expr, typ) = judgement;

    // Check if free variables (unknown types) are present, and count types
    let mut free = HashSet::new();
    let mut typeset: HashSet<String> = HashSet::new();
    free_vars(expr, &mut free, &mut HashSet::new(), &mut typeset);
    dbg!(&free, &typeset);

    if !free.is_empty() {
        return Err(TypeCheckError::UnknownType(
            free.into_iter().collect::<Vec<String>>().join(", "),
        ));
    }

    // Check if all types are known
    match check_judgement_type(typ, &typeset) {
        Ok(_) => Ok(()),
        Err(err_code) => Err(err_code),
    }
}

// borrowed from ass2, modified to also keep track of types
fn free_vars(
    expression: &Expression,
    free: &mut HashSet<String>,
    abstr_vars: &mut HashSet<String>,
    typeset: &mut HashSet<String>,
) {
    match expression {
        Expression::Application(lexpr, rexpr) => {
            free_vars(lexpr, free, abstr_vars, typeset);
            free_vars(rexpr, free, abstr_vars, typeset);
        }
        Expression::Abstraction(var, typ, body) => {
            // If this abstraction adds the variable, also remove it.
            if abstr_vars.insert(var.clone()) {
                free_vars(body, free, abstr_vars, typeset);
                abstr_vars.remove(var);
            } else {
                // the variable is already in the set, so should not be removed
                free_vars(body, free, abstr_vars, typeset);
            }

            // add the type to the set of types
            collect_types(typ, typeset);
        }
        Expression::Variable(varname) => {
            if !abstr_vars.contains(varname) {
                free.insert(varname.clone());
            }
        }
    }
}

/// Collects all types in a type, and adds them to the typeset.
fn collect_types(typ: &Type, typeset: &mut HashSet<String>) {
    match typ {
        Type::Function(t1, t2) => {
            collect_types(t1, typeset);
            collect_types(t2, typeset);
        }
        Type::Variable(t) => {
            typeset.insert(t.clone());
        }
    }
}

/// Check all types in a judgement's type, and return an error if any of them are unknown.
fn check_judgement_type(typ: &Type, typeset: &HashSet<String>) -> TypeCheckResult {
    match typ {
        Type::Function(t1, t2) => {
            check_judgement_type(t1, typeset)?;
            check_judgement_type(t2, typeset)?;
        }
        Type::Variable(t) => {
            if !typeset.contains(t) {
                return Err(TypeCheckError::MismatchedTypes(t.clone()));
            }
        }
    }

    Ok(())
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

pub(crate) fn bench_type_check(judgement: &Judgement) {
    _type_check(judgement).unwrap();
}

pub(crate) fn manual_type_check(judgement: &Judgement) -> bool {
    match _type_check(judgement) {
        Ok(_) => true,
        Err(err_code) => {
            println!(
                "Invalid judgement [{}] caught during typechecking!",
                err_code
            );
            false
        }
    }
}
