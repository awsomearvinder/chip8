pub mod OpCodes;
#[cfg(test)]
mod tests;

use OpCodes::OpCode;

pub fn parse(input: &[u8]) -> Result<Vec<OpCodes::OpCode>, ParseError> {
    if input.len() % 2 != 0 {
        return Err(ParseError::WrongOpCodeCount);
    }
    let mut codes = vec![];
    for i in input.chunks(2) {
        codes.push(split_to_code((i[0], i[1]))?);
    }
    Ok(codes)
}

fn split_to_code(input: (u8, u8)) -> Result<OpCode, ParseError> {
    let operation = ((input.0 as u16) << 8) + input.1 as u16;
    Ok(match operation {
        0x00E0 => OpCode::CLS,
        0x00EE => OpCode::RET,
        0x0000..=0x0FFF => OpCode::SYS_ADDR(OpCodes::Addr(operation)),
        0x1000..=0x1FFF => OpCode::JP_ADDR(OpCodes::Addr(operation & 0x0FFF)),
        0x2000..=0x2FFF => OpCode::CALL_ADDR(OpCodes::Addr(operation & 0x0FFF)),
        0x3000..=0x3FFF => OpCode::SE_REGISTER_BYTE(
            OpCodes::Register(((operation & 0x0F00) >> 8) as u8),
            OpCodes::Byte((operation & 0x00FF) as u8),
        ),
        0x4000..=0x4FFF => OpCode::SNE_REGISTER_BYTE(
            OpCodes::Register(((operation & 0x0F00) >> 8) as u8),
            OpCodes::Byte((operation & 0x00FF) as u8),
        ),
        0x5000..=0x5FFF if operation << 12 == 0x0000 => OpCode::SE_REGISTER_REGISTER(
            OpCodes::Register(((operation & 0x0F00) >> 8) as u8),
            OpCodes::Register(((operation & 0x00F0) >> 4) as u8),
        ),
        0x6000..=0x6FFF => OpCode::LD_REGISTER_BYTE(
            OpCodes::Register(((operation & 0x0F00) >> 8) as u8),
            OpCodes::Byte((operation & 0x00FF) as u8),
        ),
        0x7000..=0x7FFF => OpCode::ADD_REGISTER_BYTE(
            OpCodes::Register(((operation & 0x0F00) >> 8) as u8),
            OpCodes::Byte((operation & 0x00FF) as u8),
        ),
        0x8000..=0x8FFF => match operation << 12 {
            0x0000 => OpCode::LD_REGISTER_REGISTER(
                OpCodes::Register(((operation & 0x0F00) >> 8) as u8),
                OpCodes::Register(((operation & 0x00F0) >> 4) as u8),
            ),
            0x1000 => OpCode::OR_REGISTER_REGISTER(
                OpCodes::Register(((operation & 0x0F00) >> 8) as u8),
                OpCodes::Register(((operation & 0x00F0) >> 4) as u8),
            ),
            0x2000 => OpCode::AND_REGISTER_REGISTER(
                OpCodes::Register(((operation & 0x0F00) >> 8) as u8),
                OpCodes::Register(((operation & 0x00F0) >> 4) as u8),
            ),
            0x3000 => OpCode::XOR_REGISTER_REGISTER(
                OpCodes::Register(((operation & 0x0F00) >> 8) as u8),
                OpCodes::Register(((operation & 0x00F0) >> 4) as u8),
            ),
            0x4000 => OpCode::ADD_REGISTER_REGISTER(
                OpCodes::Register(((operation & 0x0F00) >> 8) as u8),
                OpCodes::Register(((operation & 0x00F0) >> 4) as u8),
            ),
            0x5000 => OpCode::SUB_REGISTER_REGISTER(
                OpCodes::Register(((operation & 0x0F00) >> 8) as u8),
                OpCodes::Register(((operation & 0x00F0) >> 4) as u8),
            ),
            0x6000 => OpCode::SHR_REGISTER_REGISTER(
                OpCodes::Register(((operation & 0x0F00) >> 8) as u8),
                OpCodes::Register(((operation & 0x00F0) >> 4) as u8),
            ),
            0x7000 => OpCode::SUBN_REGISTER_REGISTER(
                OpCodes::Register(((operation & 0x0F00) >> 8) as u8),
                OpCodes::Register(((operation & 0x00F0) >> 4) as u8),
            ),
            0xE000 => OpCode::SHL_REGISTER_REGISTER(
                OpCodes::Register(((operation & 0x0F00) >> 8) as u8),
                OpCodes::Register(((operation & 0x00F0) >> 4) as u8),
            ),
            _ => return Err(ParseError::NoFoundOpCode(operation)),
        },
        0x9000..=0x9FFF if operation << 12 == 0 => OpCode::SNE_REGISTER_REGISTER(
            OpCodes::Register(((operation & 0x0F00) >> 8) as u8),
            OpCodes::Register(((operation & 0x00F0) >> 4) as u8),
        ),
        0xA000..=0xAFFF => OpCode::LD_IREGISTER_ADDR(OpCodes::Addr(operation & 0x0FFF)),
        0xB000..=0xBFFF => OpCode::JP_V0_ADDR(OpCodes::Addr(operation & 0x0FFF)),
        0xC000..=0xCFFF => OpCode::RND_REGISTER_BYTE(
            OpCodes::Register(((operation & 0x0F00) >> 8) as u8),
            OpCodes::Byte((operation & 0x00FF) as u8),
        ),
        0xD000..=0xDFFF => OpCode::DRW_REGISTER_REGISTER_NIBBLE(
            OpCodes::Register(((operation & 0x0F00) >> 8) as u8),
            OpCodes::Register(((operation & 0x00F0) >> 4) as u8),
            OpCodes::Byte((operation & 0x000F) as u8),
        ),
        0xE000..=0xEFFF if operation & 0x00FF == 0x009E => {
            OpCode::SKP_REGISTER(OpCodes::Register(((operation & 0x0F00) >> 8) as u8))
        }
        0xE000..=0xEFFF if operation & 0x00FF == 0x00A1 => {
            OpCode::SKNP_REGISTER(OpCodes::Register(((operation & 0x0F00) >> 8) as u8))
        }
        0xF000..=0xFFFF if operation & 0x00FF == 0x0007 => {
            OpCode::LD_REGISTER_TIMER(OpCodes::Register(((operation & 0x0F00) >> 8) as u8))
        }
        0xF000..=0xFFFF if operation & 0x00FF == 0x000A => {
            OpCode::LD_REGISTER_KEY(OpCodes::Register(((operation & 0x0F00) >> 8) as u8))
        }
        0xF000..=0xFFFF if operation & 0x00FF == 0x0015 => {
            OpCode::LD_TIMER_REGISTER(OpCodes::Register(((operation & 0x0F00) >> 8) as u8))
        }
        0xF000..=0xFFFF if operation & 0x00FF == 0x0018 => {
            OpCode::LD_SOUND_REGISTER(OpCodes::Register(((operation & 0x0F00) >> 8) as u8))
        }
        0xF000..=0xFFFF if operation & 0x00FF == 0x001E => {
            OpCode::ADD_IREGISTER_REGISTER(OpCodes::Register(((operation & 0x0F00) >> 8) as u8))
        }
        0xF000..=0xFFFF if operation & 0x00FF == 0x0029 => {
            OpCode::LD_REGISTER_SPRITE(OpCodes::Register(((operation & 0x0F00) >> 8) as u8))
        }
        0xF000..=0xFFFF if operation & 0x00FF == 0x0033 => {
            OpCode::LD_BCD_REGISTER_IREGISTER(OpCodes::Register(((operation & 0x0F00) >> 8) as u8))
        }
        0xF000..=0xFFFF if operation & 0x00FF == 0x0055 => {
            OpCode::LD_IREGISTER_REGISTER(OpCodes::Register(((operation & 0x0F00) >> 8) as u8))
        }
        0xF000..=0xFFFF if operation & 0x00FF == 0x0065 => {
            OpCode::LD_REGISTER_IREGISTER(OpCodes::Register(((operation & 0x0F00) >> 8) as u8))
        }
        val => return Err(ParseError::NoFoundOpCode(val)),
    })
}

#[derive(Debug)]
pub enum ParseError {
    NoFoundOpCode(u16),
    WrongOpCodeCount,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::NoFoundOpCode(val) => write!(f, "Could not find valid opcode for {:x?}", val),
            Self::WrongOpCodeCount => write!(
                f,
                "OpCodes are always 2 bytes, you sent an odd number of bytes."
            ),
        }
    }
}
impl std::error::Error for ParseError {}
