pub enum Flags6502 {
    C = (1 << 0), //Carry Bit
    Z = (1 << 1), //Zero
    I = (1 << 2), //Interrupt Disable
    D = (1 << 3), //Decimal
    B = (1 << 4), //The B Flag.
    U = (1 << 5), //Useless
    V = (1 << 6), //Overflow
    N = (1 << 7), //Negative
} 


pub fn SetFlag(status: &mut u8, f: Flags6502, b: bool){

    if b
    {
        *status |= f as u8;
    }
    else
    {
        *status &= !(f as u8);
    }
}