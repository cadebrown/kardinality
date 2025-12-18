use thiserror::Error;

use crate::kardlang::{BinOp, Call, Expr};

use super::{Effect, Limits};

pub trait VmContext {
    fn get(&self, name: &str) -> Option<i64>;
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum VmError {
    #[error("unknown function: {0}")]
    UnknownFunction(String),

    #[error("unknown register: {0}")]
    UnknownRegister(String),

    #[error("wrong arity for {name}: expected {expected}, got {got}")]
    WrongArity {
        name: String,
        expected: usize,
        got: usize,
    },

    #[error("integer overflow")]
    Overflow,

    #[error("execution aborted: exceeded max steps ({max_steps})")]
    StepLimitExceeded { max_steps: usize },
}

#[derive(Debug, Clone)]
pub struct Machine {
    steps: usize,
    limits: Limits,
}

impl Machine {
    pub fn new(limits: Limits) -> Self {
        Self { steps: 0, limits }
    }

    pub fn eval_call<C: VmContext>(
        &mut self,
        call: &Call,
        ctx: &C,
    ) -> Result<Vec<Effect>, VmError> {
        self.steps += 1;
        if self.steps > self.limits.max_steps {
            return Err(VmError::StepLimitExceeded {
                max_steps: self.limits.max_steps,
            });
        }

        match call.name.as_str() {
            "score" => {
                let n = expect_one_int(call, ctx)?;
                Ok(vec![Effect::AddScore(n)])
            }
            "bank" => {
                let n = expect_one_int(call, ctx)?;
                Ok(vec![Effect::AddBankroll(n)])
            }
            "dbl" => {
                expect_arity(call, 0)?;
                Ok(vec![Effect::MulBankroll(2)])
            }
            "draw" => {
                let n = expect_one_int(call, ctx)?;
                Ok(vec![Effect::Draw(n)])
            }
            "tri" => {
                let n = expect_one_int(call, ctx)?;
                let v = tri(n)?;
                Ok(vec![Effect::SetAcc(v)])
            }
            "fibo" => {
                let n = expect_one_int(call, ctx)?;
                let v = fibo(n)?;
                Ok(vec![Effect::SetAcc(v)])
            }
            "clone" => {
                let n = expect_one_int(call, ctx)?;
                Ok(vec![Effect::Clone(n)])
            }
            "again" => {
                let n = expect_one_int(call, ctx)?;
                Ok(vec![Effect::Again(n)])
            }
            "mutate" => {
                expect_arity(call, 0)?;
                Ok(vec![Effect::Mutate])
            }
            other => Err(VmError::UnknownFunction(other.to_string())),
        }
    }
}

fn expect_arity(call: &Call, expected: usize) -> Result<(), VmError> {
    let got = call.args.len();
    if got != expected {
        return Err(VmError::WrongArity {
            name: call.name.clone(),
            expected,
            got,
        });
    }
    Ok(())
}

fn expect_one_int<C: VmContext>(call: &Call, ctx: &C) -> Result<i64, VmError> {
    expect_arity(call, 1)?;
    eval_expr(&call.args[0], ctx)
}

fn eval_expr<C: VmContext>(expr: &Expr, ctx: &C) -> Result<i64, VmError> {
    match expr {
        Expr::Int(n, _) => Ok(*n),
        Expr::Var(name, _) => ctx
            .get(name)
            .ok_or_else(|| VmError::UnknownRegister(name.clone())),
        Expr::Group(inner, _) => eval_expr(inner, ctx),
        Expr::Binary { op, lhs, rhs, .. } => {
            let a = eval_expr(lhs, ctx)?;
            let b = eval_expr(rhs, ctx)?;
            match op {
                BinOp::Add => a.checked_add(b).ok_or(VmError::Overflow),
                BinOp::Mul => a.checked_mul(b).ok_or(VmError::Overflow),
            }
        }
    }
}

fn tri(n: i64) -> Result<i64, VmError> {
    if n <= 0 {
        return Ok(0);
    }
    let a = n;
    let b = n.checked_add(1).ok_or(VmError::Overflow)?;

    // Compute a*b/2 without losing precision.
    if a % 2 == 0 {
        a.checked_div(2)
            .and_then(|x| x.checked_mul(b))
            .ok_or(VmError::Overflow)
    } else {
        b.checked_div(2)
            .and_then(|x| x.checked_mul(a))
            .ok_or(VmError::Overflow)
    }
}

fn fibo(n: i64) -> Result<i64, VmError> {
    let mut n = n;
    if n <= 0 {
        return Ok(0);
    }

    // Clamp to keep it deterministic and non-explosive.
    n = n.min(48);

    let mut a: i64 = 0;
    let mut b: i64 = 1;
    for _ in 0..n {
        let next = a.checked_add(b).ok_or(VmError::Overflow)?;
        a = b;
        b = next;
    }
    Ok(a)
}
