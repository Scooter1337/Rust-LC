// L.A. (Luca) Verheul - S3704041
// Mon 25 Dec 2023

use crate::dbg;
use std::collections::HashSet;
use std::fmt::{Display, Formatter, Result};
use std::sync::atomic::AtomicUsize;

use crate::parser::Expression;

#[allow(unused)]
#[derive(Debug, PartialEq, Eq)]
pub(super) enum ReduceError {
    ReductionOutOfBounds,
    BetaReductionOnNonAbstraction,
}

const MAX_REDUCTIONS: usize = 10000;

impl Display for ReduceError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            ReduceError::ReductionOutOfBounds => {
                write!(
                    f,
                    "Reduction out of bounds, more than {} reduction steps",
                    MAX_REDUCTIONS
                )
            }
            ReduceError::BetaReductionOnNonAbstraction => {
                write!(f, "Beta reduction on non abstraction")
            }
        }
    }
}

type ReduceResult = std::result::Result<Expression, ReduceError>;

// Global counter for alpha conversion variable names
// An easy way to make sure that the variable names are always unique
static CUSTOM_VARNAME_COUNTER: AtomicUsize = AtomicUsize::new(1);

fn unique_varname(old_name: &str) -> String {
    let count = CUSTOM_VARNAME_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    format!("{}{}", old_name, count)
}

fn alpha(variable: &str, expr: Expression) -> (String, Expression) {
    let new_name = unique_varname(variable);
    (
        new_name.to_owned(),
        substitution(expr, variable, &Expression::Variable(new_name)),
    )
}

fn beta(abstraction: Expression, expr: Expression) -> ReduceResult {
    if let Expression::Abstraction(var, body) = abstraction {
        Ok(substitution(*body, &var, &expr))
    } else {
        Err(ReduceError::BetaReductionOnNonAbstraction)
    }
}

fn substitution(expr: Expression, symbol: &str, subexp: &Expression) -> Expression {
    match expr {
        Expression::Application(lexpr, rexpr) => Expression::Application(
            Box::new(substitution(*lexpr, symbol, subexp)),
            Box::new(substitution(*rexpr, symbol, subexp)),
        ),
        Expression::Abstraction(var, body) => {
            if is_free_var(subexp, &var) {
                // alpha conversion
                let alpha = alpha(&var, *body);
                Expression::Abstraction(alpha.0, Box::new(substitution(alpha.1, symbol, subexp)))
            } else {
                Expression::Abstraction(var, Box::new(substitution(*body, symbol, subexp)))
            }
        }
        Expression::Variable(varname) => {
            if varname == symbol {
                subexp.clone()
            } else {
                Expression::Variable(varname)
            }
        }
    }
}

fn is_free_var(expression: &Expression, symbol: &str) -> bool {
    let mut free = HashSet::new();
    _free_vars(expression, &mut free, &mut HashSet::new());
    free.contains(symbol)
}

fn _free_vars(
    expression: &Expression,
    free: &mut HashSet<String>,
    abstr_vars: &mut HashSet<String>,
) {
    match expression {
        Expression::Application(lexpr, rexpr) => {
            _free_vars(lexpr, free, abstr_vars);
            _free_vars(rexpr, free, abstr_vars);
        }
        Expression::Abstraction(var, body) => {
            // If this abstraction adds the variable, also remove it.
            if abstr_vars.insert(var.clone()) {
                _free_vars(body, free, abstr_vars);
                abstr_vars.remove(var);
            } else {
                // the variable is already in the set, so should not be removed
                _free_vars(body, free, abstr_vars);
            }
        }
        Expression::Variable(varname) => {
            if !abstr_vars.contains(varname) {
                free.insert(varname.clone());
            }
        }
    }
}

/// Counts the number of reductions
///
static REDUCE_COUNTER: AtomicUsize = AtomicUsize::new(1);

fn _reduce(expr: Expression) -> ReduceResult {
    if REDUCE_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst) > MAX_REDUCTIONS {
        return Err(ReduceError::ReductionOutOfBounds);
    }

    if let Expression::Application(lexpr, rexpr) = expr {
        let lexpr = _reduce(*lexpr)?;
        if let Expression::Abstraction(var, body) = &lexpr {
            _reduce(beta(lexpr, *rexpr)?)
        } else {
            Ok(Expression::Application(
                Box::new(lexpr),
                Box::new(_reduce(*rexpr)?),
            ))
        }
    } else if let Expression::Abstraction(var, body) = expr {
        Ok(Expression::Abstraction(var, Box::new(_reduce(*body)?)))
    } else {
        Ok(expr)
    }
}

/// Reduce the expression
/// If there is an error, prints an error and exits the program.
///
/// # Arguments
/// * `expr` - The expression to reduce
///
/// # Returns
/// The reduced expression
///
/// # Error
/// "Error [{err_code}] caught during reducing on line {idx}!"
pub(super) fn reduce(expr: Expression, idx: usize) -> Expression {
    CUSTOM_VARNAME_COUNTER.store(1, std::sync::atomic::Ordering::SeqCst);
    REDUCE_COUNTER.store(1, std::sync::atomic::Ordering::SeqCst);
    let reduction = _reduce(expr);
    dbg!(&reduction);
    match reduction {
        Ok(expr) => expr,
        Err(err) => {
            eprintln!(
                "Error [{}] caught during reducing on line {}!.",
                err,
                idx + 1
            );
            std::process::exit(2);
        }
    }
}

/// Reduce the expression \
/// Only used for benchmarking \
/// Unwraps the result, so panics if there is an error, for ultimate speed
pub(crate) fn bench_reduce(expr: Expression) -> Expression {
    CUSTOM_VARNAME_COUNTER.store(1, std::sync::atomic::Ordering::SeqCst);
    REDUCE_COUNTER.store(1, std::sync::atomic::Ordering::SeqCst);
    _reduce(expr).unwrap()
}

/// Reduce the expression \
/// If there is an error, returns None
/// Only used for manual mode, where we want to keep reducing even if there is an error
/// (does not exit with code 1 on error)
pub(crate) fn manual_reduce(expr: Expression) -> Option<Expression> {
    CUSTOM_VARNAME_COUNTER.store(1, std::sync::atomic::Ordering::SeqCst);
    REDUCE_COUNTER.store(1, std::sync::atomic::Ordering::SeqCst);
    let reduction = _reduce(expr);
    dbg!(&reduction);
    match reduction {
        Ok(expr) => expr.into(),
        Err(err) => {
            eprintln!("Error [{}] caught during reducing.", err);
            None
        }
    }
}
