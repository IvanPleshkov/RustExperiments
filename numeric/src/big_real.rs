use crate::big_integer::BigInteger;
use crate::machine_integer::MachineInteger;

pub struct BigReal {
    mantissa: BigInteger,
    exponent: MachineInteger,
}
