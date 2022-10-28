//kinematics.rs
//an inefficient but very flexible module to handle kinematics, using integrals and derivatives

//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~//

pub enum Var {
    T,
    X,
    Y,
}

const UniqueUnits : usize = 3;
pub enum Unit {
    M,
    S,
    KG
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Units {
    pub exponents  : [i32; UniqueUnits]
}

impl Unit {
    pub fn Units(&self) -> Units {
        let mut ret = Units::empty();
        ret.exponents[*self as usize] = 1;
        ret
    }
}

impl Units {
    fn empty() -> Self {
        Units {
            exponents : [0; UniqueUnits]
        }
    }
}

impl std::ops::Mul for Units {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let mut exponents = [0; UniqueUnits];
        for i in 0..UniqueUnits {
            exponents[i] = self.exponents[i] + rhs.exponents[i];
        }
        Units {
            exponents
        }
    }
}

impl std::ops::Div for Units {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        let mut exponents = [0; UniqueUnits];
        for i in 0..UniqueUnits {
            exponents[i] = self.exponents[i] - rhs.exponents[i];
        }
        Units {
            exponents
        }
    }
}

//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~//

pub enum EvalFunctionError {
    InvalidUnits,
    InvalidVar,
    DividedByZero,
}

pub trait Function {
    fn var_units(&self) -> Units;
    fn var(&self) -> Var;
    fn eval(&self, input: f32) -> Result<f32>;
    fn check_input_eval(&self, input: f32, var : Var, units : Units) -> Result<f32,EvalFunctionError> {
        if self.var_units() == units {
            if self.var() == var {
                self.eval(input)
            } else {
                Err(EvalFunctionError::InvalidVar)
            }
        } else {
            Err(EvalFunctionError::InvalidUnits)
        }
    }
    fn check_units(&self) -> bool;
    fn compile(&self) -> fn(f32) -> f32;
    // fn simplify(&mut self);
    // fn simplification(&self) -> Self;
    // fn simplify_units(&mut self);
    // fn simplification_units(&self) -> Self;
}

//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~//

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Monomial {
    pub coefficient : f32,
    pub units_coefficient : Units,
    pub exponent : i32,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Polynomial {
    pub expression : Vec<Monomial>,
    pub var : Var,
    pub var_units : Units,
    pub final_units : Units,
}

impl Function for Polynomial {
    fn var_units(&self) -> Units {
        self.var_units
    }
    fn var(&self) -> Var {
        self.var
    }
    fn eval(&self, input: f32) -> Result<f32, EvalFunctionError> {
        let mut result = 0.0;
        for monomial in self.expression {
            result += monomial.coefficient * input.powi(monomial.exponent);
        }
        Ok(result)
    }
    //need to implement compilation to delta position closure 
}

//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~//

enum Functions {
    Polynomial(Polynomial),
}

//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~//

trait Integrable {
    fn integrate(&self, respect : Var) -> impl Function;
}

trait Differentiable {
    fn differentiate(&self, respect : Var) -> impl Function;
}

//need to implement integration and differentiation for polynomials

//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~//