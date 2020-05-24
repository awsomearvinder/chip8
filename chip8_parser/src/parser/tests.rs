use super::*;

#[test]
fn test_parse() {
    assert_eq!(parse(&[0, 0xE0]).unwrap(), vec![OpCode::CLS])
}
#[test]
fn test_parse_multiple() {
    assert_eq!(
        parse(&[0, 0xE0, 0x1, 0xE0]).unwrap(),
        vec![OpCode::CLS, OpCode::SYS_ADDR(OpCodes::Addr(0x1E0))]
    )
}
#[test]
fn test_CLS() {
    assert_eq!(split_to_code((0, 0xE0)).unwrap(), OpCode::CLS);
}
#[test]
fn test_RET() {
    assert_eq!(split_to_code((0, 0xEE)).unwrap(), OpCode::RET);
}
#[test]
fn test_SYS_ADDR() {
    assert_eq!(
        split_to_code((0x1, 0x11)).unwrap(),
        OpCode::SYS_ADDR(OpCodes::Addr(0x111))
    )
}
#[test]
fn test_JP_ADDR() {
    assert_eq!(
        split_to_code((0x10, 0x10)).unwrap(),
        OpCode::JP_ADDR(OpCodes::Addr(0x10))
    );
}
#[test]
fn test_CALL_ADDR() {
    assert_eq!(
        split_to_code((0x20, 0xFF)).unwrap(),
        OpCode::CALL_ADDR(OpCodes::Addr(0xFF))
    );
}
#[test]
fn test_SE_REGISTER_BYTE() {
    assert_eq!(
        split_to_code((0x3F, 0x3A)).unwrap(),
        OpCode::SE_REGISTER_BYTE(OpCodes::Register(0xF), OpCodes::Byte(0x3A))
    )
}
#[test]
fn test_SNE_REGISTER_BYTE() {
    assert_eq!(
        split_to_code((0x4E, 0x3A)).unwrap(),
        OpCode::SNE_REGISTER_BYTE(OpCodes::Register(0xE), OpCodes::Byte(0x3A))
    )
}
#[test]
fn test_SE_REGISTER_REGISTER() {
    assert_eq!(
        split_to_code((0x5A, 0x40)).unwrap(),
        OpCode::SE_REGISTER_REGISTER(OpCodes::Register(0xA), OpCodes::Register(0x4))
    );
}
#[test]
fn test_LD_REGISTER_BYTE() {
    assert_eq!(
        split_to_code((0x6D, 0x40)).unwrap(),
        OpCode::LD_REGISTER_BYTE(OpCodes::Register(0xD), OpCodes::Byte(0x40))
    );
}
#[test]
fn test_ADD_REGISTER_BYTE() {
    assert_eq!(
        split_to_code((0x73, 0x39)).unwrap(),
        OpCode::ADD_REGISTER_BYTE(OpCodes::Register(0x3), OpCodes::Byte(0x39))
    );
}
#[test]
fn test_LD_REGISTER_REGISTER() {
    assert_eq!(
        split_to_code((0x85, 0x40)).unwrap(),
        OpCode::LD_REGISTER_REGISTER(OpCodes::Register(0x5), OpCodes::Register(0x4))
    )
}
#[test]
fn test_OR_REGISTER_REGISTER() {
    assert_eq!(
        split_to_code((0x85, 0x41)).unwrap(),
        OpCode::OR_REGISTER_REGISTER(OpCodes::Register(0x5), OpCodes::Register(0x4))
    )
}
#[test]
fn test_AND_REGISTER_REGISTER() {
    assert_eq!(
        split_to_code((0x85, 0x42)).unwrap(),
        OpCode::AND_REGISTER_REGISTER(OpCodes::Register(0x5), OpCodes::Register(0x4))
    )
}
#[test]
fn test_XOR_REGISTER_REGISTER() {
    assert_eq!(
        split_to_code((0x85, 0x43)).unwrap(),
        OpCode::XOR_REGISTER_REGISTER(OpCodes::Register(0x5), OpCodes::Register(0x4))
    )
}
#[test]
fn test_ADD_REGISTER_REGISTER() {
    assert_eq!(
        split_to_code((0x85, 0x44)).unwrap(),
        OpCode::ADD_REGISTER_REGISTER(OpCodes::Register(0x5), OpCodes::Register(0x4))
    )
}
#[test]
fn test_SUB_REGISTER_REGISTER() {
    assert_eq!(
        split_to_code((0x85, 0x45)).unwrap(),
        OpCode::SUB_REGISTER_REGISTER(OpCodes::Register(0x5), OpCodes::Register(0x4))
    )
}
#[test]
fn test_SHR_REGISTER_REGISTER() {
    assert_eq!(
        split_to_code((0x85, 0x46)).unwrap(),
        OpCode::SHR_REGISTER_REGISTER(OpCodes::Register(0x5), OpCodes::Register(0x4))
    )
}
#[test]
fn test_SUBN_REGISTER_REGISTER() {
    assert_eq!(
        split_to_code((0x85, 0x47)).unwrap(),
        OpCode::SUBN_REGISTER_REGISTER(OpCodes::Register(0x5), OpCodes::Register(0x4))
    )
}
#[test]
fn test_SHL_REGISTER_REGISTER() {
    assert_eq!(
        split_to_code((0x85, 0x4E)).unwrap(),
        OpCode::SHL_REGISTER_REGISTER(OpCodes::Register(0x5), OpCodes::Register(0x4))
    )
}
#[test]
fn test_SNE_REGISTER_REGISTER() {
    assert_eq!(
        split_to_code((0x95, 0x40)).unwrap(),
        OpCode::SNE_REGISTER_REGISTER(OpCodes::Register(0x5), OpCodes::Register(0x4))
    )
}
#[test]
fn test_LD_IREGISTER_ADDR() {
    assert_eq!(
        split_to_code((0xAF, 0xA0)).unwrap(),
        OpCode::LD_IREGISTER_ADDR(OpCodes::Addr(0xFA0))
    )
}
#[test]
fn test_JP_V0_ADDR() {
    assert_eq!(
        split_to_code((0xBF, 0xA0)).unwrap(),
        OpCode::JP_V0_ADDR(OpCodes::Addr(0xFA0))
    )
}
#[test]
fn test_RND_REGISTER_BYTE() {
    assert_eq!(
        split_to_code((0xCF, 0xA0)).unwrap(),
        OpCode::RND_REGISTER_BYTE(OpCodes::Register(0xF), OpCodes::Byte(0xA0))
    )
}
#[test]
fn test_DRW_REGISTER_REGISTER_NIBBLE() {
    assert_eq!(
        split_to_code((0xDD, 0xAF)).unwrap(),
        OpCode::DRW_REGISTER_REGISTER_NIBBLE(
            OpCodes::Register(0xD),
            OpCodes::Register(0xA),
            OpCodes::Byte(0xF)
        )
    )
}
#[test]
fn test_SKP_REGISTER() {
    assert_eq!(
        split_to_code((0xEE, 0x9E)).unwrap(),
        OpCode::SKP_REGISTER(OpCodes::Register(0xE))
    )
}
#[test]
fn test_SKNP_REGISTER() {
    assert_eq!(
        split_to_code((0xEE, 0xA1)).unwrap(),
        OpCode::SKNP_REGISTER(OpCodes::Register(0xE))
    )
}
#[test]
fn test_LD_REGISTER_TIMER() {
    assert_eq!(
        split_to_code((0xFA, 0x07)).unwrap(),
        OpCode::LD_REGISTER_TIMER(OpCodes::Register(0xA))
    )
}
#[test]
fn test_LD_REGISTER_KEY() {
    assert_eq!(
        split_to_code((0xFA, 0x0A)).unwrap(),
        OpCode::LD_REGISTER_KEY(OpCodes::Register(0xA))
    )
}
#[test]
fn test_LD_TIMER_REGISTER() {
    assert_eq!(
        split_to_code((0xFA, 0x15)).unwrap(),
        OpCode::LD_TIMER_REGISTER(OpCodes::Register(0xA))
    )
}
#[test]
fn test_LD_SOUNDTIMER_REGISTER() {
    assert_eq!(
        split_to_code((0xFA, 0x18)).unwrap(),
        OpCode::LD_SOUND_REGISTER(OpCodes::Register(0xA))
    )
}
#[test]
fn test_ADD_IREGISTER_REGISTER() {
    assert_eq!(
        split_to_code((0xFA, 0x1E)).unwrap(),
        OpCode::ADD_IREGISTER_REGISTER(OpCodes::Register(0xA))
    )
}
#[test]
fn test_LD_SPRITE_REGISTER() {
    assert_eq!(
        split_to_code((0xFA, 0x29)).unwrap(),
        OpCode::LD_REGISTER_SPRITE(OpCodes::Register(0xA))
    )
}
#[test]
fn test_LD_BCD_REGISTER() {
    assert_eq!(
        split_to_code((0xFA, 0x33)).unwrap(),
        OpCode::LD_BCD_REGISTER_IREGISTER(OpCodes::Register(0xA))
    )
}
#[test]
fn test_LD_IREGISTER_REGISTER() {
    assert_eq!(
        split_to_code((0xFA, 0x55)).unwrap(),
        OpCode::LD_IREGISTER_REGISTER(OpCodes::Register(0xA))
    )
}
#[test]
fn test_LD_REGISTER_IREGISTER() {
    assert_eq!(
        split_to_code((0xFA, 0x65)).unwrap(),
        OpCode::LD_REGISTER_IREGISTER(OpCodes::Register(0xA))
    )
}
