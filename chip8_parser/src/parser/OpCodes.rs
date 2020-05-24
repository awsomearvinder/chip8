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
#[derive(Debug, PartialEq, Eq)]
pub struct Addr(pub u16);
#[derive(Debug, PartialEq, Eq)]
pub struct Register(pub u8);
#[derive(Debug, PartialEq, Eq)]
pub struct Byte(pub u8);
