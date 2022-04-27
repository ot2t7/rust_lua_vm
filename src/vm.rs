use crate::error::*;
use std::convert::TryInto;

struct BytecodeReader {
    buf: Vec<u8>,
    needle: usize
}

impl BytecodeReader {
    pub fn new(buf: Vec<u8>) -> BytecodeReader {
        return BytecodeReader { buf, needle: 0};
    }

    /// Reads the specified amount of bytes from the bytecode, and "pop"s it out
    pub fn read(&mut self, size: usize) -> Vec<u8> {
        if self.buf.len() < self.needle + size {
            panic!("tried to read {} bytes, but only {} bytes remain", size, self.buf.len() - self.needle);
        }
        let snippet = &self.buf[self.needle..self.needle+size];
        self.needle += size; 
        return snippet.to_vec();
    }

    /// Reads 4 big-endian bytes, and returns a u32
    pub fn read_u32(&mut self) -> u32 {
        let snippet: &[u8 ; 4] = &self.buf[self.needle..self.needle+4].try_into().expect("read a u32, but it was not 4 bytes");
        self.needle += 4;
        return u32::from_be_bytes(*snippet);
    }
}

pub fn from_bytecode(buf: Vec<u8>) {
    // The bytecode can't be too short, have to have bytes for header and stuff
    if buf.len() < 12 {
        ErrorKind::too_short()
    }

    let mut bytecode = BytecodeReader::new(buf);

    // The bytecode must begin with 0x1B4C7561, or "ESC Lua"
    if bytecode.read_u32() != 457995617 {
        ErrorKind::invalid_header("no Lua signature".to_owned());
    }
    // The version number must be 0x51, standing for Lua.51
    if bytecode.read(1)[0] != 81 {
        ErrorKind::invalid_header("not lua 5.1 bytecode".to_owned());
    }
    // Format version, no clue what this is so we skip it
    bytecode.read(1);
    // Endianness flag, this library completely ignores this flag 
    bytecode.read(1);
    // Size of int (in bytes)
    let size_int = bytecode.read(1)[0];
    // Size of size_t (in bytes)
    let size_size_t = bytecode.read(1)[0];
    // Size of Instruction (in bytes)
    let size_instruction = bytecode.read(1)[0];
    // Size of lua_Number (in bytes)
    let size_lua_number = bytecode.read(1)[0];
    // Integral flag | 0=floating-point, 1=integral number type
    let integral_flag = bytecode.read(1)[0];
    if integral_flag > 1 {
        ErrorKind::invalid_header("integral flag can only be 0 or 1".to_owned());
    }
} 