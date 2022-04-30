use crate::error::*;
use std::str;
use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};

struct BytecodeReader {
    buf: Vec<u8>,
    needle: usize,
    size_t: usize
}

impl BytecodeReader {
    pub fn new(buf: Vec<u8>) -> BytecodeReader {
        return BytecodeReader { buf, needle: 0, size_t: 4};
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
        let mut bytes = Cursor::new(self.read(4));
        return bytes.read_u32::<BigEndian>().expect("couldn't convert bytes to u32");
    }

    /// Register the size of size_t
    pub fn set_size_t(&mut self, size_t: usize) {
        self.size_t = size_t;
    }

    /// Reads a size_t and returns a u32
    pub fn read_size_t(&mut self) -> u32 {
        let mut size_t_vec = Cursor::new(self.read(self.size_t));
        return size_t_vec.read_u32::<BigEndian>().expect("couldn't convert size_t to u32")
    }

    /// Reads a String inside of the bytecode, and if it's length is more than 0, it returns Some(String)
    /// Strings which are size_t of 0 are returned as None
    pub fn read_string(&mut self) -> Option<String> {
        let size_string = self.read_size_t() as usize;
        if size_string == 0 {
            return None;
        } else {
            match String::from_utf8(self.read(size_string)) {
                Ok(interpreted) => {
                    return Some(interpreted);
                }
                Err(_) => ErrorKind::invalid_string()
            }

        }

        // We should never get here
        return None;
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
    bytecode.set_size_t(size_size_t as usize);
    // Size of Instruction (in bytes)
    let size_instruction = bytecode.read(1)[0];
    // Size of lua_Number (in bytes)
    let size_lua_number = bytecode.read(1)[0];
    // Integral flag | 0=floating-point, 1=integral number type
    let integral_flag = bytecode.read(1)[0];
    if integral_flag > 1 {
        ErrorKind::invalid_header("integral flag can only be 0 or 1".to_owned());
    }

    // Now, we move onto the function block


}  