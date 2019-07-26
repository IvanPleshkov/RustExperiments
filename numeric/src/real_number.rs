use crate::machine_integer::MachineInteger;
use crate::big_integer::BigInteger;
use crate::rational::Rational;
use crate::machine_real::MachineReal;
use crate::big_real::BigReal;

pub enum RealNumber {
    MachineInteger(MachineInteger),
    BigInteger(BigInteger),
    MachineRational(Rational<MachineInteger>),
    BigRational(Rational<BigInteger>),
    MachineReal(MachineReal),
    BigReal(BigReal),
}
