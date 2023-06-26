/// A RISC-V standard or compressed machine instruction.
/// ```rust
/// use rysk::{Instruction, Register};
/// // srai x17,x7,19
/// let r = Instruction::new(0x4133d893);
/// assert_eq!(r.opcode(), 0b0010011);
/// assert_eq!(r.funct3(), 0b101);
/// assert_eq!(r.funct7(), 0b0100000);
/// assert_eq!(r.rd(), Register::X17);
/// assert_eq!(r.rs1(), Register::X7);
/// assert_eq!(r.rs2(), Register::X19);
/// // jalr x31,1234(x11)
/// let i = Instruction::new(0x4d258fe7);
/// assert_eq!(i.opcode(), 0b1100111);
/// assert_eq!(i.funct3(), 0b000);
/// assert_eq!(i.rd(), Register::X31);
/// assert_eq!(i.rs1(), Register::X11);
/// assert_eq!(i.i_immediate(), 1234);
/// // sw x27,2047(x31)
/// let s = Instruction::new(0x7fbfafa3);
/// assert_eq!(s.opcode(), 0b0100011);
/// assert_eq!(s.funct3(), 0b010);
/// assert_eq!(s.rs1(), Register::X31);
/// assert_eq!(s.rs2(), Register::X27);
/// assert_eq!(s.s_immediate(), 0x7FF);
/// // bgeu x10,x8,-4
/// let b = Instruction::new(0xfe857ee3);
/// assert_eq!(b.opcode(), 0b1100011);
/// assert_eq!(b.funct3(), 0b111);
/// assert_eq!(b.rs1(), Register::X10);
/// assert_eq!(b.rs2(), Register::X8);
/// assert_eq!(b.b_immediate(), -4i32 as u32);
/// // lui x15,0xDEAD4
/// let u = Instruction::new(0xdead47b7);
/// assert_eq!(u.opcode(), 0b0110111 );
/// assert_eq!(u.rd(), Register::X15);
/// assert_eq!(u.u_immediate(), 0xDEAD_4000);
/// // jal x7,0xDEAD4
/// let j = Instruction::new(0x2d5de3ef);
/// assert_eq!(j.opcode(), 0b1101111);
/// assert_eq!(j.rd(), Register::X7);
/// assert_eq!(j.j_immediate(), 0xDEAD4);
/// ```
#[derive(Clone, Copy)]
pub struct Instruction(u32);
impl Instruction {
    /// Creates a new 32-bit standard instruction.
    #[inline]
    pub fn new(instruction: u32) -> Self {
        Self(instruction)
    }
    /// Takes variable-length instruction parcels and returns the instruction
    /// and a bit indicating that the instruction was compressed.
    pub fn from_parcels(parcels: [u16; 2]) -> (Self, bool) {
        if parcels[0] & 0b11 != 0b11 {
            // Compressed encoding
            (Self(parcels[0] as u32), true)
        } else {
            // Standard encoding
            let [a, b] = parcels[0].to_le_bytes();
            let [c, d] = parcels[1].to_le_bytes();
            (Self(u32::from_le_bytes([a, b, c, d])), false)
        }
    }
    /// Returns `true` if the instruction is a compressed 16-bit instruction.
    #[inline]
    pub fn compressed(self) -> bool {
        self.0 & 0b11 != 0b11
    }
    /// Returns `true` if the instruction is a standard 32-bit instruction.
    #[inline]
    pub fn standard(self) -> bool {
        self.0 & 0b11 == 0b11
    }

    /// Return the standard opcode.
    /// ```rust
    /// assert_eq!(rysk::Instruction::new(0x4d258fe7).opcode(), 0b1100111);
    /// ```
    #[inline]
    pub fn opcode(self) -> u8 {
        (self.0 & 0b0111_1111) as u8
    }
    /// Return the compressed opcode.
    #[inline]
    pub fn op(self) -> u8 {
        (self.0 & 0b0000_0011) as u8
    }
    /// Return the standard funct3.
    #[inline]
    pub fn funct3(self) -> u8 {
        ((self.0 & 0x0000_7000) >> 12) as u8
    }
    /// Return the standard funct7.
    #[inline]
    pub fn funct7(self) -> u8 {
        ((self.0 & 0xFE00_0000) >> 25) as u8
    }
    /// Return the compressed funct2.
    #[inline]
    pub fn compressed_funct2(self) -> u8 {
        ((self.0 & 0x0060) >> 5) as u8
    }
    /// Return the compressed funct3.
    #[inline]
    pub fn compressed_funct3(self) -> u8 {
        ((self.0 & 0xE000) >> 13) as u8
    }
    /// Return the compressed funct4.
    #[inline]
    pub fn compressed_funct4(self) -> u8 {
        ((self.0 & 0xF000) >> 12) as u8
    }
    /// Return the compressed funct6.
    #[inline]
    pub fn compressed_funct6(self) -> u8 {
        ((self.0 & 0xFC00) >> 10) as u8
    }

    /// Return the standard & compressed destination register number.
    #[inline]
    pub fn rd(self) -> Register {
        // Safety: The register number is <= 5-bit and cannot exceed 31.
        unsafe { Register::new_unchecked(
            ((self.0 & 0x0000_0F80) >> 7) as u8
        )}
    }
    /// Return the first standard source register number.
    #[inline]
    pub fn rs1(self) -> Register {
        // Safety: The register number is 5-bit and cannot exceed 31.
        unsafe { Register::new_unchecked(
            ((self.0 & 0x000F_8000) >> 15) as u8
        )}
    }
    /// Return the second standard source register number.
    #[inline]
    pub fn rs2(self) -> Register {
        // Safety: The register number is 5-bit and cannot exceed 31.
        unsafe { Register::new_unchecked(
            ((self.0 & 0x01F0_0000) >> 20) as u8
        )}
    }
    /// Return the first compressed full-sized source register number.
    #[inline]
    pub fn compressed_rs1(self) -> Register {
        // Safety: The register number is 5-bit and cannot exceed 31.
        unsafe { Register::new_unchecked(
            ((self.0 & 0x0F80) >> 7) as u8
        )}
    }
    /// Return the second compressed full-sized source register number.
    #[inline]
    pub fn compressed_rs2(self) -> Register {
        // Safety: The register number is 5-bit and cannot exceed 31.
        unsafe { Register::new_unchecked(
            ((self.0 & 0x007C) >> 2) as u8
        )}
    }
    /// Return the first compressed half-sized source register number.
    #[inline]
    pub fn crs1(self) -> Register {
        // Safety: The register number is 3-bit and cannot exceed 31.
        unsafe { Register::new_unchecked(
            ((self.0 & 0x0380) >> 7) as u8 + 8
        )}
    }
    /// Return the second compressed half-sized source register number.
    #[inline]
    pub fn crs2(self) -> Register {
        // Safety: The register number is 3-bit and cannot exceed 31.
        unsafe { Register::new_unchecked(
            ((self.0 & 0x001C) >> 2) as u8 + 8
        )}
    }

    /// Return the I-type immediate value.
    #[inline]
    pub fn i_immediate(self) -> u32 {
        ((self.0 as i32) >> 20) as u32
    }
    /// Return the S-type immediate value.
    pub fn s_immediate(self) -> u32 {
        (((self.0 as i32) >> 20) as u32 & !0b1_1111) | ((self.0 >> 7) & 0b1_1111)
    }
    /// Return the B-type immediate value.
    pub fn b_immediate(self) -> u32 {
        (((self.0 as i32) >> 19) as u32 & 0xFFFF_F000) |
        ((self.0 << 4)                  & 0b1000_0000_0000) |
        ((self.0 >> 20)                 & 0b0111_1110_0000) |
        ((self.0 >> 7)                  & 0b0000_0001_1110)
    }
    /// Return the U-type immediate value.
    pub fn u_immediate(self) -> u32 {
        self.0 & 0xFFFF_F000
    }
    /// Return the J-type immediate value.
    pub fn j_immediate(self) -> u32 {
        (((self.0 as i32) >> 11) as u32 & 0xFFF0_0000) |
        (self.0                         & 0x000F_F000) |
        ((self.0 >> 8)                  & 0b1000_0000_0000) |
        ((self.0 >> 20)                 & 0b0111_1111_1110)
    }
    
}
impl core::fmt::Debug for Instruction {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        struct Bin<const N: usize>(u8);
        impl<const N: usize> core::fmt::Debug for Bin<N> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "0b{:0N$b}", self.0)
            }
        }
        struct Hex<const N: usize>(u32);
        impl<const N: usize> core::fmt::Debug for Hex<N> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "0x{:0N$X}", self.0)
            }
        }
        enum RawInstruction {
            Standard(u32),
            Compressed(u16)
        }
        impl core::fmt::Debug for RawInstruction {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                match self {
                    Self::Standard(i) => write!(f, "Standard(0x{i:08X})"),
                    Self::Compressed(i) => write!(f, "Compressed(0x{i:04X})")
                }
            }
        }
        let mut dbg = f.debug_struct("Instruction");
        if self.compressed() {
            dbg.field("value", &RawInstruction::Compressed(self.0 as u16))
                .field("opcode", &Bin::<2>(self.op()))
                .field("funct2", &Bin::<2>(self.compressed_funct2()))
                .field("funct3", &Bin::<3>(self.compressed_funct3()))
                .field("funct4", &Bin::<4>(self.compressed_funct4()))
                .field("funct6", &Bin::<6>(self.compressed_funct6()))
                .field("rd_rs1", &self.compressed_rs1())
                .field("rs2", &self.compressed_rs2())
                .field("crd_crs1'", &self.crs1())
                .field("crd_crs2'", &self.crs2());
            
        } else {
            dbg.field("value", &RawInstruction::Standard(self.0))
                .field("opcode", &Bin::<7>(self.opcode()))
                .field("funct3", &Bin::<3>(self.funct3()))
                .field("funct7", &Bin::<7>(self.funct7()))
                .field("rd", &self.rd())
                .field("rs1", &self.rs1())
                .field("rs2", &self.rs2())
                .field("i_immediate", &Hex::<8>(self.i_immediate()))
                .field("s_immediate", &Hex::<8>(self.s_immediate()))
                .field("b_immediate", &Hex::<8>(self.b_immediate()))
                .field("u_immediate", &Hex::<8>(self.u_immediate()))
                .field("j_immediate", &Hex::<8>(self.j_immediate()));
        };
        dbg.finish()
    }
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Register {
    X0 = 0,
    X1 = 1,
    X2 = 2,
    X3 = 3,
    X4 = 4,
    X5 = 5,
    X6 = 6,
    X7 = 7,
    X8 = 8,
    X9 = 9,
    X10 = 10,
    X11 = 11,
    X12 = 12,
    X13 = 13,
    X14 = 14,
    X15 = 15,
    X16 = 16,
    X17 = 17,
    X18 = 18,
    X19 = 19,
    X20 = 20,
    X21 = 21,
    X22 = 22,
    X23 = 23,
    X24 = 24,
    X25 = 25,
    X26 = 26,
    X27 = 27,
    X28 = 28,
    X29 = 29,
    X30 = 30,
    X31 = 31
}
impl Register {
    /// Convert the register number to the register or [`None`] if `num > 31`.
    #[inline]
    pub fn new(num: u8) -> Option<Self> {
        if num > 31 {
            None
        } else {
            Some(unsafe { Self::new_unchecked(num) })
        }
    }
    /// Convert the register number to the register.
    /// # Safety
    /// It is undefined behaviour for `num` to be greater than 31.
    #[inline(always)]
    pub unsafe fn new_unchecked(num: u8) -> Self {
        core::mem::transmute(num)
    }
}
impl core::fmt::Debug for Register {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "x{}", *self as u8)
    }
}
