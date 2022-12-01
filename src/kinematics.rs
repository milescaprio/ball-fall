//kinematics.rs
//an inefficient but very flexible module to handle kinematics, using integrals and derivatives

//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~//

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Var {
    T,
    X,
    Y,
    S, //undirectional position
}

const UniqueUnits : usize = 3;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
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
    pub fn units(&self) -> Units {
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
    pub fn pow(&self, exp : i32) -> Units {
        let mut ret = Units::empty();
        for i in 0..UniqueUnits {
            ret.exponents[i] = self.exponents[i] * exp;
        }
        ret
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

fn count_combinations(n: u64, r: u64) -> u64 {
    if r > n {
        0
    } else {
        (1..=r.min(n - r)).fold(1, |acc, val| acc * (n - val + 1) / val)
    }
}

//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~//

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum EvalFunctionError {
    OutsideDomain,
    Imaginary
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum FunctionCompatibilityError {
    InvalidUnits,
    InvalidVar,
}
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum FunctionInternalError {
    UnitMismatch,
    SpecificFunctionError(&'static str),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum FunctionError {
    FunctionCompatibilityError(FunctionCompatibilityError),
    EvalFunctionError(EvalFunctionError),
    FunctionInternalError(FunctionInternalError),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DiffrientiationError {
    ProhibitedRespect,
    FullyUndifferentiable,
    UnkownResultFormat,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum IntegrationError {
    ProhibitedRespect,
    FullyUnintegrable,
    UnkownResultFormat,
}

pub trait Function {
    fn var_units(&self) -> Units;
    fn final_units(&self) -> Units;
    fn var(&self) -> Var;
    //todo: add resulting var?
    fn check(&self) -> Result<(),FunctionInternalError>;
    fn check_recursive(&self) -> Result<(), FunctionInternalError>;
    fn compile(&self) -> Result<Box<dyn Fn(f32) -> Result<f32,EvalFunctionError>>, FunctionInternalError> {
        self.check_recursive()?;
        Ok(self.compile_unchecked())
    }
    fn compile_unchecked(&self) -> Box<dyn Fn(f32) -> Result<f32,EvalFunctionError>>;
    fn check_input(&self, var : Var, units : Units) -> Result<(),FunctionCompatibilityError> {
        if self.var_units() == units {
            if self.var() == var {
                Ok(())
            } else {
                Err(FunctionCompatibilityError::InvalidVar)
            }    
        } else {
            Err(FunctionCompatibilityError::InvalidUnits)
        }    
    }

    fn stretch_vert(&self, n : f32) -> Box<dyn Function>;
        //if your function is for some reason not able to handle coefficients, you can return a different type,
        //like a composite or product function type
    fn shift_vert(&self, n : f32) -> Box<dyn Function>;
    fn shift_hor(&self, n : f32) -> Box<dyn Function>;
    fn stereotype() -> Self where Self : Sized; //can only be called on a variant of Function not just a dyn Function type
    //DiffrientiationBehavior
    fn differentiated(&self, respect : Var) -> Result<Box<dyn Function>, DiffrientiationError>;
    // fn differentiate( &mut self, respect : Var) -> Result<(), DiffrientiationError> {
    //     self = self.differentiated(respect)?;
    // }
    //IntegrationBehavior
    fn integrated_c(&self, respect : Var, c : f32) -> Result<Box<dyn Function>, IntegrationError>;
    fn integrated(&self, respect : Var) -> Result<Box<dyn Function>, IntegrationError> {
        self.integrated_c(respect, 0.0)
    }
    fn debug(&self);
}    

//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~//

enum KinematicsFunctions {
    Polynomial(Polynomial),
    SumFunction(SumFunction),
}

//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~//

//todo: make these checked/unchecked, forcing implementation of both checked and unchecked for odd functions
//to figure out their own FullyUnintegrable/FullyUndifferentiable/UnknownResultFormat errors in the checked version or use make a PartialIntegrable for functions that are sometimes integrable and require an errored version

// pub trait IntegrationBehavior {
    // //A trait that specifies that the function has known integration behavior. Recommended to implement this trait to convenience other composite functions,
    // //as sometimes the integrals are propogated down. Even if it just returns an integration error it means it can be put into functions that require it
    // //in case they are possibly integrated
    // //Todo: make this more intuitive, to predict behavior of dyn Functions, only forcing integratability when needed
    // fn integrate_c(&self, respect : Var, c : f32) -> Result<Box<dyn CalcFunction>, IntegrationError>;
    // fn integrate(&self, respect : Var) -> Result<Box<dyn CalcFunction>, IntegrationError> {
    //     self.integrate_c(respect, 0.0)
    // }
    
// }

// pub trait DifferentiationBehavior {
//     //A trait that specifies that the function has known diffrientiation behavior. Recommended to implement this trait to convenience other composite functions,
//     //as sometimes the derivatives are propogated down. Even if it just returns a differentiation error it means it can be put into functions that require it
//     //in case they are possibly differentiated
//     //Todo: make this more intuitive, to predict behavior of dyn Functions, only forcing differentiability when needed
//     fn differentiate(&self, respect : Var) -> Result<Box<dyn CalcFunction>, DiffrientiationError>;
// }

// pub trait CalcFunction : Function + IntegrationBehavior + DifferentiationBehavior {}
// impl<T> CalcFunction for T where T: Function + IntegrationBehavior + DifferentiationBehavior {}

//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~//

//idea: split into maintainablefunction : function + integrationbehavior + differentiationbehavior + transformable
//for now, just make all function maintainablefunction

//impl<T> std::fmt::Debug for T where T : Function {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Monomial {
    pub coefficient : f32,
    pub units_coefficient : Units,
    pub exponent : i32,
}    

impl Monomial {
    pub fn init(coefficient : f32, units_coefficient : Units, exponent : i32) -> Self {
        Monomial {
            coefficient,
            units_coefficient,
            exponent
        }    
    }    
}    

#[derive(Debug, Clone, PartialEq)]
pub struct Polynomial {
    pub expression : Vec<Monomial>,
    pub var : Var,
    pub var_units : Units,
    pub final_units : Units,
}    

impl Polynomial {
    pub fn init(var: Var, var_units : Units, final_units : Units, terms : Vec<Monomial>) -> Self {
        Polynomial {
            expression : terms,
            var,
            var_units,
            final_units
        }    
    }    
}    

impl Function for Polynomial {
    fn var_units(&self) -> Units {
        self.var_units
    }    
    fn var(&self) -> Var {
        self.var
    }    
    fn final_units(&self) -> Units {
        self.final_units
    }
    fn check(&self) -> Result<(),FunctionInternalError> {
        let mut i = 0;
        for monomial in &self.expression {
            if monomial.units_coefficient * self.var_units().pow(monomial.exponent) != self.final_units {
                dbg!(monomial.units_coefficient);
                dbg!(self.var_units().pow(monomial.exponent));
                dbg!(self.final_units);
                return Err(FunctionInternalError::UnitMismatch);
            }    
            if monomial.exponent != i {
                if monomial.exponent < 0 {
                    return Err(FunctionInternalError::SpecificFunctionError("NegativeExponent"));
                }
                dbg!(self);
                return Err(FunctionInternalError::SpecificFunctionError("UnsortedPolynomial"))
            }
            i += 1;
        }    
        Ok(())
    }
    fn check_recursive(&self) -> Result<(), FunctionInternalError> {
        self.check()
    }
    fn compile_unchecked(&self) -> Box<dyn Fn(f32) -> Result<f32,EvalFunctionError>> {
        let mut evalexpr : Vec<f32> = Vec::new();
        for monomial in &self.expression {
            if evalexpr.len() < (monomial.exponent + 1) as usize {
                evalexpr.resize((monomial.exponent + 1) as usize, 0.0);
            }    
            evalexpr[monomial.exponent as usize] = monomial.coefficient;
        }    
        Box::new(move |input : f32| {
            let mut result = 0.0;
            for i in 0..evalexpr.len() {
                result += evalexpr[i] * input.powi(i as i32);
            }    
            Ok(result)
        })    
    }
    fn stretch_vert(&self, n : f32) -> Box<dyn Function> {
        let mut ret = self.clone();
        for monomial in &mut ret.expression {
            monomial.coefficient *= n;
        }    
        Box::new(ret)
    }
    fn shift_vert(&self, n : f32) -> Box<dyn Function> {
        let mut ret = self.clone();
        ret.expression[0].coefficient += n;
        Box::new(ret)
    }
    fn shift_hor(&self, n : f32) -> Box<dyn Function> {
        //shift the polynomial horizontally by n
        //substitute x = x - n and each exponent for each term 
        let mut new_expression : Vec<Monomial> = (0..self.expression.len()).map(|x| Monomial::init(0.0, self.final_units / self.var_units.pow(x as i32), x as i32)).collect();
        // for i in 0..self.expression.len() {
        //     new_expression.push(new_monomial);
        // }
        for monomial in &self.expression {
            let mut add_coefficients = Vec::with_capacity(self.expression.len());
            for i in 0..=(monomial.exponent as usize) {
                add_coefficients.push((-n).powi(monomial.exponent - i as i32) * count_combinations(monomial.exponent as u64, i as u64) as f32 * monomial.coefficient);
            }
            dbg!(&add_coefficients);
            for i in 0..=(monomial.exponent as usize) {
                new_expression[i].coefficient += add_coefficients[i];
            }
            //finish this
        }
        Box::new(Polynomial::init(self.var, self.var_units, self.final_units, new_expression))
    }
    fn stereotype() -> Self {
        Polynomial::init(Var::X, Unit::M.units(), Unit::M.units(), vec![Monomial::init(1.0, Unit::M.units(), 1)])
    }

    //DiffrientiationBehavior
    fn differentiated(&self, respect : Var) -> Result<Box<dyn Function>, DiffrientiationError> {
        if respect == self.var {
            let mut derivative : Vec<Monomial> = Vec::new();
            for monomial in &self.expression {
                if monomial.exponent != 0 {
                    derivative.push(Monomial::init(monomial.coefficient * monomial.exponent as f32, monomial.units_coefficient, monomial.exponent - 1));
                } 
            }    
            Ok(Box::new(Polynomial::init(self.var, self.var_units, self.final_units / self.var_units, derivative)))
        } else {
            Err(DiffrientiationError::ProhibitedRespect)
        }    
    }

    fn integrated_c(&self, respect : Var, c : f32) -> Result<Box<dyn Function>, IntegrationError> {
        if respect == self.var {
            let mut integral : Vec<Monomial> = Vec::new();
            integral.push(Monomial { coefficient: c, units_coefficient: self.final_units * self.var_units, exponent: 0 });
            for monomial in &self.expression {
                integral.push(Monomial::init(monomial.coefficient / (monomial.exponent + 1) as f32, monomial.units_coefficient, monomial.exponent + 1));
            }
            Ok(Box::new(Polynomial::init(self.var, self.var_units, self.final_units * self.var_units, integral)))
        } else {
            Err(IntegrationError::ProhibitedRespect)
        }    
    }

    fn debug(&self) {
        dbg!(&self.expression);
        dbg!(self.var);
        dbg!(self.var_units);
        dbg!(self.final_units);
    }
}    

// impl DifferentiationBehavior for Polynomial {
//     fn mult_const_calc(&self, n : f32) -> Box<dyn CalcFunction>; //this is a dumb solution, everything sorta fell apart with the CalcFunction, but I have nothing better right now
//     fn differentiate(&self, respect : Var) -> Result<Box<dyn CalcFunction>, DiffrientiationError> {
//         if respect == self.var {
//             let mut derivative : Vec<Monomial> = Vec::new();
//             for monomial in &self.expression {
//                 if monomial.exponent != 0 {
//                     derivative.push(Monomial::init(monomial.coefficient * monomial.exponent as f32, monomial.units_coefficient, monomial.exponent - 1));
//                 } 
//             }    
//             Ok(Box::new(Polynomial::init(self.var, self.var_units, self.final_units / self.var_units, derivative)))
//         } else {
//             Err(DiffrientiationError::ProhibitedRespect)
//         }    
//     }    
// }    

// impl IntegrationBehavior for Polynomial {
//     fn integrate_c(&self, respect : Var, c : f32) -> Result<Box<dyn CalcFunction>, IntegrationError> {
//         if respect == self.var {
//             let mut integral : Vec<Monomial> = Vec::new();
//             integral.push(Monomial { coefficient: c, units_coefficient: self.final_units * self.var_units, exponent: 0 });
//             for monomial in &self.expression {
//                 integral.push(Monomial::init(monomial.coefficient / (monomial.exponent + 1) as f32, monomial.units_coefficient, monomial.exponent + 1));
//             }
//             Ok(Box::new(Polynomial::init(self.var, self.var_units, self.final_units * self.var_units, integral)))
//         } else {
//             Err(IntegrationError::ProhibitedRespect)
//         }    
//     }    
// }

//#[derive(Clone, Debug)]
pub struct SumFunction {
    f1 : Box<dyn Function>,
    f2 : Box<dyn Function>,
    pub var : Var,
    pub var_units : Units,
    pub final_units : Units,
}

impl SumFunction {
    pub fn from_compatible(f1 : Box<dyn Function>, f2 : Box<dyn Function>) -> Result<Self,&'static str> {
        if f1.var() != f2.var() {
            return Err("Functions Contain Different Input Variables");
        }
        if f1.var_units() != f2.var_units() {
            return Err("Functions Contain Different Input Variable Units");
        }
        if f1.final_units() != f2.final_units() {
            return Err("Functions Contain Different Output Units");
        }
        Ok(SumFunction {
            var : f1.var(),
            var_units : f1.var_units(),
            final_units : f1.final_units(),
            f1 : f1,
            f2 : f2,
        })  
    }
}

impl Function for SumFunction {
    fn var(&self) -> Var {
        self.var
    }
    fn var_units(&self) -> Units {
        self.var_units
    }
    fn final_units(&self) -> Units {
        self.final_units
    }
    fn check(&self) -> Result<(), FunctionInternalError> {
        if  self.f1.var_units()   != self.var_units   || self.f2.var_units()   != self.var_units   ||
            self.f1.final_units() != self.final_units || self.f2.final_units() != self.final_units ||
            self.f1.var()         != self.var         || self.f2.var()         != self.var {
            return Err(FunctionInternalError::UnitMismatch);
        }
        Ok(())
    }
    fn check_recursive(&self) -> Result<(), FunctionInternalError> {
        self.f1.check()?;
        self.f2.check()?;
        self.check()
    }
    fn compile_unchecked(&self) -> Box<dyn Fn(f32) -> Result<f32,EvalFunctionError>> {
        let closure1 = self.f1.compile_unchecked();
        let closure2 = self.f2.compile_unchecked();
        Box::new(move |x| {
            Ok(closure1(x)? + closure2(x)?)
        })
    }
    fn stretch_vert(&self, n : f32) -> Box<dyn Function> {
        Box::new(SumFunction {
            var : self.var,
            var_units : self.var_units,
            final_units : self.final_units,
            f1 : self.f1.stretch_vert(n),
            f2 : self.f2.stretch_vert(n),
        })
    }
    fn shift_vert(&self, n : f32) -> Box<dyn Function> {
        Box::new(SumFunction {
            var : self.var,
            var_units : self.var_units,
            final_units : self.final_units,
            f1 : self.f1.shift_vert(n),
            f2 : self.f2.shift_vert(n),
        })
    }
    fn shift_hor(&self, n : f32) -> Box<dyn Function> {
        Box::new(SumFunction {
            var : self.var,
            var_units : self.var_units,
            final_units : self.final_units,
            f1 : self.f1.shift_hor(n),
            f2 : self.f2.shift_hor(n),
        })
    }
    fn stereotype() -> Self where Self : Sized {
        SumFunction {
            f1 : Box::new(Polynomial::stereotype()),
            f2 : Box::new(Polynomial::stereotype()),
            var : Var::X,
            var_units : Units::empty(),
            final_units : Units::empty(),
        }
    }
    fn differentiated(&self, respect : Var) -> Result<Box<dyn Function>, DiffrientiationError> {
        if respect == self.var {
            Ok(Box::new(SumFunction {
                var : self.var,
                var_units : self.var_units,
                final_units : self.final_units,
                f1 : self.f1.differentiated(respect)?,
                f2 : self.f2.differentiated(respect)?,
            }))
        } else {
            Err(DiffrientiationError::ProhibitedRespect)
        }    
    }  
    fn integrated_c(&self, respect : Var, c : f32) -> Result<Box<dyn Function>, IntegrationError> { //preferably the function that is easier to add c to should go in f2; use bigger/more complex function first
        if respect == self.var {
            Ok(Box::new(SumFunction {
                var : self.var,
                var_units : self.var_units,
                final_units : self.final_units,
                f1 : self.f1.integrated(respect)?,
                f2 : self.f2.integrated_c(respect, c)?,
            }))
        } else {
            Err(IntegrationError::ProhibitedRespect)
        }    
    }

    fn debug(&self) {
        self.f1.debug();
        self.f2.debug();
    }
}

// impl DifferentiationBehavior for SumFunction {
//     fn differentiate(&self, respect : Var) -> Result<Box<dyn Function>, DiffrientiationError> {
//         if respect == self.var {
//             Ok(Box::new(SumFunction {
//                 var : self.var,
//                 var_units : self.var_units,
//                 final_units : self.final_units,
//                 f1 : self.f1.differentiate(respect)?,
//                 f2 : self.f2.differentiate(respect)?,
//             }))
//         } else {
//             Err(DiffrientiationError::ProhibitedRespect)
//         }    
//     }    
// }    

// impl IntegrationBehavior for SumFunction {
//     fn integrate_c(&self, respect : Var, c : f32) -> Result<Box<dyn CalcFunction>, IntegrationError> { //preferably the function that is easier to add c to should go in f2; use bigger/more complex function first
//         if respect == self.var {
//             Ok(Box::new(SumFunction {
//                 var : self.var,
//                 var_units : self.var_units,
//                 final_units : self.final_units,
//                 f1 : self.f1.integrate(respect)?,
//                 f2 : self.f2.integrate_c(respect, c)?,
//             }))
//         } else {
//             Err(IntegrationError::ProhibitedRespect)
//         }    
//     }    
// }


//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~//

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn construct_units() {
        let meter = Unit::M;
        let meters = meter.units();
        assert_eq!(meters.exponents[Unit::M as usize], 1);
    }
    #[test]
    fn construct_monomial() {
        let meter = Unit::M;
        let meters = meter.units();
        let none = Units::empty();
        let slope = Monomial::init(2.0, none, 1);
        assert_eq!(slope.coefficient, 2.0);
        assert_eq!(slope.units_coefficient.exponents[meter as usize], 0);
        assert_eq!(slope.exponent, 1);
    }
    #[test]
    fn construct_polynomial() {
        let meter = Unit::M;
        let meters = meter.units();
        let none = Units::empty();
        let slope = Monomial::init(2.0, none, 1);
        let polynomial = Polynomial::init(Var::X, meters, meters, vec![slope]);
        assert_eq!(polynomial.expression[0].coefficient, 2.0);
        assert_eq!(polynomial.expression[0].units_coefficient.exponents[meter as usize], 0);
        assert_eq!(polynomial.expression[0].exponent, 1);
        assert_eq!(polynomial.var, Var::X);
        assert_eq!(polynomial.var_units.exponents[meter as usize], 1);
        assert_eq!(polynomial.final_units.exponents[meter as usize], 1);
    }
    #[test]
    fn polynomial_compile_eval() {
        let meter = Unit::M;
        let meters = meter.units();
        let none = Units::empty();
        let slope = Monomial::init(2.0, none, 1);
        let intercept = Monomial::init(3.0, meters, 0);
        let polynomial = Polynomial::init(Var::X, meters, meters, vec![intercept, slope]);
        assert_eq!(polynomial.compile().unwrap()(1.0).unwrap(), 5.0);
    }
    #[test]
    #[should_panic]
    fn polynomial_compile_fail() {
        let meter = Unit::M;
        let meters = meter.units();
        let none = Units::empty();
        let slope = Monomial::init(2.0, none, 1);
        let intercept = Monomial::init(3.0, meters, 0);
        let polynomial = Polynomial::init(Var::X, meters, meters, vec![intercept, slope]);
        assert_eq!(polynomial.compile().unwrap()(1.0).unwrap(), 1.0);
    }
    #[test]
    fn polynomial_differentiate() {
        let meter = Unit::M;
        let meters = meter.units();
        let none = Units::empty();
        let slope = Monomial::init(2.0, none, 1);
        let intercept = Monomial::init(3.0, meters, 0);
        let polynomial = Polynomial::init(Var::X, meters, meters, vec![intercept, slope]);
        let derivative = polynomial.differentiated(Var::X).unwrap();
        assert_eq!(derivative.compile().unwrap()(1.0).unwrap(), 2.0);
    }
    #[test]
    fn polynomial_integrate() {
        let meter = Unit::M;
        let meters = meter.units();
        let none = Units::empty();
        let slope = Monomial::init(2.0, none, 1);
        let intercept = Monomial::init(3.0, meters, 0);
        let polynomial = Polynomial::init(Var::X, meters, meters, vec![intercept, slope]);
        let integral = polynomial.integrated_c(Var::X, 0.0).unwrap();
        assert_eq!(integral.compile().unwrap()(1.0).unwrap(), 4.0);
    }
    #[test]
    fn polynomial_shift() {
        let meter = Unit::M;
        let meters = meter.units();
        let none = Units::empty();
        let c = Monomial::init(6.0, meters, 0);
        let b = Monomial::init(9.0, none, 1);
        let a = Monomial::init(4.0, meters.pow(-1), 2);
        let polynomial = Polynomial::init(Var::X, meters, meters, vec![c,b,a]);
        let shifted = polynomial.shift_hor(-1.0);
        shifted.debug();
        assert_eq!(shifted.compile().unwrap()(1.0).unwrap(), 6.0);
        //assert_eq!(shifted.compile().unwrap()(2.0).unwrap(), 74.0);
        //vec![Monomial::init(4.0, meters.pow(-1), 0), Monomial::init(17.0, none, 1), Monomial::init(9.0, meters, 2)]);
    }
}