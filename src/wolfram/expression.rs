use wolfram::numeric::integer::Integer;
use wolfram::numeric::rational::Rational;
use wolfram::numeric::real::Real;
use wolfram::numeric::complex::Complex;
use wolfram::sparse_array::SparseArray;

#[derive(Clone, Debug)]
pub enum Expression {
    Expression(Vec<Expression>),
    Symbol(String),
    Integer(Integer),
    Rational(Rational),
    Real(Real),
    Complex(Complex),
    Boolean(bool),
    String(String),
    SparseArray(SparseArray),
}

impl Expression {
    pub fn head(&self) -> String {
        match (self) {
            Expression::Expression(_)   => String::from("Expression"),
            Expression::Symbol(_)       => String::from("Symbol"),
            Expression::Integer(_)      => String::from("Integer"),
            Expression::Rational(_)     => String::from("Rational"),
            Expression::Real(_)         => String::from("Real"),
            Expression::Complex(_)      => String::from("Complex"),
            Expression::Boolean(_)      => String::from("Boolean"),
            Expression::String(_)       => String::from("String"),
            Expression::SparseArray(_)  => String::from("SparseArray"),
        }
    }

    pub fn fullform(&self) -> String {
        match (self) {
            Expression::Expression(_)   => String::from("Expression"),
            Expression::Symbol(e)       => String::from("Symbol[") + e + "]",
            Expression::Integer(e)      => String::from("Integer"),
            Expression::Rational(e)     => String::from("Rational"),
            Expression::Real(e)         => String::from("Real"),
            Expression::Complex(e)      => String::from("Complex"),
            Expression::Boolean(e)      => String::from("Boolean"),
            Expression::String(e)       => String::from("String"),
            Expression::SparseArray(e)  => String::from("SparseArray"),
        }
    }
}
