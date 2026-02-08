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
            "score" | "s" => {
                let n = expect_one_int(call, ctx)?;
                Ok(vec![Effect::AddScore(n)])
            }
            "bank" | "b" => {
                let n = expect_one_int(call, ctx)?;
                Ok(vec![Effect::AddBankroll(n)])
            }
            "dbl" | "x" => {
                expect_arity(call, 0)?;
                Ok(vec![Effect::MulBankroll(2)])
            }
            "draw" | "d" => {
                let n = expect_one_int(call, ctx)?;
                Ok(vec![Effect::Draw(n)])
            }
            "tri" | "t" => {
                let n = expect_one_int(call, ctx)?;
                let v = tri(n)?;
                Ok(vec![Effect::SetAcc(v)])
            }
            "fibo" | "f" => {
                let n = expect_one_int(call, ctx)?;
                let v = fibo(n)?;
                Ok(vec![Effect::SetAcc(v)])
            }
            "clone" | "c" => {
                let n = expect_one_int(call, ctx)?;
                Ok(vec![Effect::Clone(n)])
            }
            "again" | "a" => {
                let n = expect_one_int(call, ctx)?;
                Ok(vec![Effect::Again(n)])
            }
            "mutate" | "m" => {
                expect_arity(call, 0)?;
                Ok(vec![Effect::Mutate])
            }
            "jam" | "j" => {
                let n = expect_one_int(call, ctx)?;
                Ok(vec![Effect::AddScore(n), Effect::Draw(1)])
            }
            "mint" | "i" => {
                let n = expect_one_int(call, ctx)?;
                Ok(vec![Effect::AddBankroll(n), Effect::Draw(1)])
            }
            "cash" | "v" => {
                let n = expect_one_int(call, ctx)?;
                let spend = n.checked_neg().ok_or(VmError::Overflow)?;
                Ok(vec![Effect::AddScore(n), Effect::AddBankroll(spend)])
            }
            "hedge" | "h" => {
                let n = expect_one_int(call, ctx)?;
                let score = ctx
                    .get("score")
                    .or_else(|| ctx.get("Q"))
                    .unwrap_or_default();
                let target = ctx
                    .get("target")
                    .or_else(|| ctx.get("T"))
                    .unwrap_or(i64::MAX);
                if score < target {
                    Ok(vec![Effect::AddScore(n)])
                } else {
                    Ok(vec![Effect::AddBankroll(n)])
                }
            }
            "wild" | "w" => {
                let n = expect_one_int(call, ctx)?;
                Ok(vec![Effect::Mutate, Effect::Again(n)])
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::kardlang::parse_program;

    struct TestCtx {
        values: HashMap<String, i64>,
    }

    impl TestCtx {
        fn from_pairs(pairs: &[(&str, i64)]) -> Self {
            let mut values = HashMap::new();
            for (k, v) in pairs {
                values.insert((*k).to_string(), *v);
            }
            Self { values }
        }
    }

    impl VmContext for TestCtx {
        fn get(&self, name: &str) -> Option<i64> {
            self.values.get(name).copied()
        }
    }

    fn parse_single_call(source: &str) -> Call {
        let program = parse_program(source).expect("program should parse");
        assert_eq!(program.calls.len(), 1);
        program.calls[0].clone()
    }

    #[test]
    fn short_aliases_and_combo_ops_emit_expected_effects() {
        let mut vm = Machine::new(Limits::default());
        let ctx = TestCtx::from_pairs(&[]);

        let jam = vm.eval_call(&parse_single_call("j(11)"), &ctx).unwrap();
        assert_eq!(jam, vec![Effect::AddScore(2), Effect::Draw(1)]);

        let mint = vm.eval_call(&parse_single_call("i(11)"), &ctx).unwrap();
        assert_eq!(mint, vec![Effect::AddBankroll(2), Effect::Draw(1)]);

        let cash = vm.eval_call(&parse_single_call("v(11)"), &ctx).unwrap();
        assert_eq!(cash, vec![Effect::AddScore(2), Effect::AddBankroll(-2)]);

        let dbl = vm.eval_call(&parse_single_call("x()"), &ctx).unwrap();
        assert_eq!(dbl, vec![Effect::MulBankroll(2)]);
    }

    #[test]
    fn hedge_switches_between_score_and_bank_modes() {
        let mut vm = Machine::new(Limits::default());

        let score_mode = vm
            .eval_call(
                &parse_single_call("h(11)"),
                &TestCtx::from_pairs(&[("score", 5), ("target", 10)]),
            )
            .unwrap();
        assert_eq!(score_mode, vec![Effect::AddScore(2)]);

        let bank_mode = vm
            .eval_call(
                &parse_single_call("h(11)"),
                &TestCtx::from_pairs(&[("score", 10), ("target", 10)]),
            )
            .unwrap();
        assert_eq!(bank_mode, vec![Effect::AddBankroll(2)]);
    }

    #[test]
    fn wild_alias_runs_mutate_then_replay() {
        let mut vm = Machine::new(Limits::default());
        let ctx = TestCtx::from_pairs(&[]);

        let wild = vm.eval_call(&parse_single_call("w(1)"), &ctx).unwrap();
        assert_eq!(wild, vec![Effect::Mutate, Effect::Again(1)]);
    }
}
