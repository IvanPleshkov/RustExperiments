use crate::machine_integer::MachineInteger;
use crate::big_integer::BigInteger;
use crate::rational::Rational;
use crate::machine_real::MachineReal;
use crate::big_real::BigReal;
use crate::real_number::RealNumber;
use crate::complex::Complex;

pub enum ComplexNumber {
    MachineInteger(MachineInteger),
    BigInteger(BigInteger),
    MachineRational(Rational<MachineInteger>),
    BigRational(Rational<BigInteger>),
    MachineReal(MachineReal),
    BigReal(BigReal),
    
    ComplexMachineInteger(Complex<MachineInteger>),
    ComplexBigInteger(Complex<BigInteger>),
    ComplexMachineRational(Complex<Rational<MachineInteger>>),
    ComplexBigRational(Complex<Rational<BigInteger>>),
    ComplexMachineReal(Complex<MachineReal>),
    ComplexBigReal(Complex<BigReal>),
    
    Infinity,
}
