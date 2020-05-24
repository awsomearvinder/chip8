use std::fmt::Display;

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq)]
pub enum OpCode {
    SYS_ADDR(Addr),
    CLS,
    RET,
    JP_ADDR(Addr),
    CALL_ADDR(Addr),
    SE_REGISTER_BYTE(Register, Byte),
    SNE_REGISTER_BYTE(Register, Byte),
    SE_REGISTER_REGISTER(Register, Register),
    LD_REGISTER_BYTE(Register, Byte),
    ADD_REGISTER_BYTE(Register, Byte),
    LD_REGISTER_REGISTER(Register, Register),
    OR_REGISTER_REGISTER(Register, Register),
    AND_REGISTER_REGISTER(Register, Register),
    XOR_REGISTER_REGISTER(Register, Register),
    ADD_REGISTER_REGISTER(Register, Register),
    SUB_REGISTER_REGISTER(Register, Register),
    SHR_REGISTER_REGISTER(Register, Register),
    SUBN_REGISTER_REGISTER(Register, Register),
    SHL_REGISTER_REGISTER(Register, Register),
    SNE_REGISTER_REGISTER(Register, Register),
    LD_IREGISTER_ADDR(Addr),
    JP_V0_ADDR(Addr),
    RND_REGISTER_BYTE(Register, Byte),
    DRW_REGISTER_REGISTER_NIBBLE(Register, Register, Byte),
    SKP_REGISTER(Register),
    SKNP_REGISTER(Register),
    LD_REGISTER_TIMER(Register),
    LD_REGISTER_KEY(Register),
    LD_TIMER_REGISTER(Register),
    LD_SOUND_REGISTER(Register),
    ADD_IREGISTER_REGISTER(Register),
    LD_REGISTER_SPRITE(Register),
    LD_BCD_REGISTER_IREGISTER(Register),
    LD_IREGISTER_REGISTER(Register),
    LD_REGISTER_IREGISTER(Register),
}
impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::SYS_ADDR(Addr(val)) => write!(f, "SYS {:#03X}", val),
            Self::CLS => write!(f, "CLS"),
            Self::RET => write!(f, "RET"),
            Self::JP_ADDR(Addr(val)) => write!(f, "JP {:#03X}", val),
            Self::CALL_ADDR(Addr(val)) => write!(f, "CALL {:#03X}", val),
            Self::SE_REGISTER_BYTE(Register(reg), Byte(byte)) => {
                write!(f, "SE V{:X} #{:02X}", reg, byte)
            }
            Self::SNE_REGISTER_BYTE(Register(reg), Byte(byte)) => {
                write!(f, "SNE V{:X} #{:02X}", reg, byte)
            }
            Self::SE_REGISTER_REGISTER(Register(reg), Register(reg2)) => {
                write!(f, "SE V{:X} V{:X}", reg, reg2)
            }
            Self::LD_REGISTER_BYTE(Register(reg), Byte(byte)) => {
                write!(f, "LD V{:X} #{:02X}", reg, byte)
            }
            Self::ADD_REGISTER_BYTE(Register(reg), Byte(byte)) => {
                write!(f, "ADD V{:X} #{:02X}", reg, byte)
            }
            Self::LD_REGISTER_REGISTER(Register(reg), Register(reg2)) => {
                write!(f, "LD V{:X} V{:X}", reg, reg2)
            }
            Self::OR_REGISTER_REGISTER(Register(reg), Register(reg2)) => {
                write!(f, "OR V{:X} V{:X}", reg, reg2)
            }
            Self::AND_REGISTER_REGISTER(Register(reg), Register(reg2)) => {
                write!(f, "AND V{:X} V{:X}", reg, reg2)
            }
            Self::XOR_REGISTER_REGISTER(Register(reg), Register(reg2)) => {
                write!(f, "XOR V{:X} V{:X}", reg, reg2)
            }
            Self::ADD_REGISTER_REGISTER(Register(reg), Register(reg2)) => {
                write!(f, "ADD V{:X} V{:X}", reg, reg2)
            }
            Self::SUB_REGISTER_REGISTER(Register(reg), Register(reg2)) => {
                write!(f, "SUB V{:X} V{:X}", reg, reg2)
            }
            Self::SHR_REGISTER_REGISTER(Register(reg), Register(reg2)) => {
                write!(f, "SHR V{:X} V{:X}", reg, reg2)
            }
            Self::SUBN_REGISTER_REGISTER(Register(reg), Register(reg2)) => {
                write!(f, "SUBN V{:X} V{:X}", reg, reg2)
            }
            Self::SHL_REGISTER_REGISTER(Register(reg), Register(reg2)) => {
                write!(f, "SHL V{:X} V{:X}", reg, reg2)
            }
            Self::SNE_REGISTER_REGISTER(Register(reg), Register(reg2)) => {
                write!(f, "SNE V{:X} V{:X}", reg, reg2)
            }
            Self::LD_IREGISTER_ADDR(Addr(addr)) => write!(f, "LD I {:#03X}", addr),
            Self::JP_V0_ADDR(Addr(addr)) => write!(f, "JP V0 {:#03X}", addr),
            Self::RND_REGISTER_BYTE(Register(reg), Byte(byte)) => {
                write!(f, "RND V{:X} #{:02X}", reg, byte)
            }
            Self::DRW_REGISTER_REGISTER_NIBBLE(Register(reg), Register(reg2), Byte(byte)) => {
                write!(f, "DRW V{:X} V{:X} #{:02X}", reg, reg2, byte)
            }
            Self::SKP_REGISTER(Register(reg)) => write!(f, "SKP V{:X}", reg),
            Self::SKNP_REGISTER(Register(reg)) => write!(f, "SKNP V{:X}", reg),
            Self::LD_REGISTER_TIMER(Register(reg)) => write!(f, "LD V{:X} DT", reg),
            Self::LD_REGISTER_KEY(Register(reg)) => write!(f, "LD V{:X} K", reg),
            Self::LD_TIMER_REGISTER(Register(reg)) => write!(f, "LD DT V{:X}", reg),
            Self::LD_SOUND_REGISTER(Register(reg)) => write!(f, "LD ST V{:X}", reg),
            Self::ADD_IREGISTER_REGISTER(Register(reg)) => write!(f, "ADD I V{:X}", reg),
            Self::LD_REGISTER_SPRITE(Register(reg)) => write!(f, "LD F V{:X}", reg),
            Self::LD_BCD_REGISTER_IREGISTER(Register(reg)) => write!(f, "LD B V{:X}", reg),
            Self::LD_IREGISTER_REGISTER(Register(reg)) => write!(f, "LD I V{:X}", reg),
            Self::LD_REGISTER_IREGISTER(Register(reg)) => write!(f, "LD V{:X} I", reg),
        }
    }
}
#[derive(Debug, PartialEq, Eq)]
pub struct Addr(pub u16);
#[derive(Debug, PartialEq, Eq)]
pub struct Register(pub u8);
#[derive(Debug, PartialEq, Eq)]
pub struct Byte(pub u8);
