use crate::error::*;
use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt, LittleEndian};

struct BytecodeReader {
    buf: Vec<u8>,
    needle: usize,
    size_t: usize,
    int: usize,
    endianness: u8
}

impl BytecodeReader {
    pub fn new(buf: Vec<u8>) -> BytecodeReader {
        return BytecodeReader { 
            buf, 
            needle: 0, 
            size_t: 4, 
            int: 4,
            endianness: 1
        };
    }

    /// Prints some current information
    pub fn debug(&self) {
        println!("Size of size_t: {} bytes.", self.size_t);
        println!("Size of integer: {} bytes.", self.int);
    }

    /// There are cases where a buffer of bytes is needed to be turned into some kind of number,
    /// but it isn't long enough to be parsed. This function takes a buffer, and adds 0's in
    /// order to make it long enough to be parsed.
    fn fill_buffer(&mut self, buf: &mut Vec<u8>, desired_length: usize) {
        println!("buf len {}", buf.len());
        if buf.len() < desired_length {
            let to_add = desired_length - buf.len();
            if self.endianness == 1 {
                //println!("Hey im in statement 1!");
                buf.append(&mut vec![0 ; to_add]);
            } else {
                //println!("Hey im in statement 2!");
                let mut empty = vec![0 ; to_add];
                empty.append(buf);
                *buf = empty;
            }
        }
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

    /// Reads a size_t and returns a usize
    pub fn read_size_t(&mut self) -> usize {
        let mut size_t_vec = self.read(self.size_t);
        //self.fill_buffer(&mut size_t_vec, 8);
        let mut bytes = Cursor::new(size_t_vec);
        if self.endianness == 1 {
            return bytes.read_u64::<LittleEndian>().expect("couldn't convert size_t to usize") as usize;
        } else {
            return bytes.read_u64::<BigEndian>().expect("couldn't convert size_t to usize") as usize;
        }
    }

    /// Reads a String inside of the bytecode, and if it's length is more than 0, it returns Some(String)
    /// Strings which are size_t of 0 are returned as None
    pub fn read_string(&mut self) -> Option<String> {
        let size_string = self.read_size_t() as usize;
        if size_string == 0 {
            return None;
        } else {
            match String::from_utf8(self.read(size_string - 1)) { // We read 1 less than the string size to not include the NUL (ASCII 0) at the end.
                Ok(interpreted) => {
                    return Some(interpreted);
                }
                Err(_) => ErrorKind::invalid_string()
            }

        }

        // We should never get here
        return None;
    }

    /// Sets the size of an integer
    pub fn set_int(&mut self, size: usize) {
        self.int = size;
    }

    /// Reads an integer from the bytecode
    pub fn read_int(&mut self) -> i128 {
        let mut int_vec = self.read(self.int);
        self.fill_buffer(&mut int_vec, 16);
        let mut bytes = Cursor::new(int_vec);
        if self.endianness == 1 {
            return bytes.read_i128::<LittleEndian>().expect("couldn't convert int to i128");
        } else {
            return bytes.read_i128::<BigEndian>().expect("couldn't convert int to i128");
        }
    }

    /// Sets the endianness flag
    pub fn set_endian(&mut self, flag: u8) {
        self.endianness = flag;
    }
}

fn function_block(bytecode: &mut BytecodeReader) {
    // The source name of the function
    let source_name = bytecode.read_string();
    // The line defined, and the last line defined
    let line_defined = bytecode.read_int();
    let last_time_defined = bytecode.read_int();
    // Number of upvalues and parameters
    let num_upvalues = bytecode.read(1)[0];
    let num_params = bytecode.read(1)[0];
    // is_vararg flag, 1=VARARG_HASARG, 2=VARARG_ISVARARG, 4=VARARG_NEEDSARG
    let is_vararg = bytecode.read(1)[0];
    // Maximum stack size, or the number of registers used
    let max_stack = bytecode.read(1)[0];

    println!("Line defined: {}, Last line defined: {}, Upvalues: {}, Params: {}", line_defined, last_time_defined, num_upvalues, num_params);
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
        ErrorKind::invalid_header("not lua 5.1 bytecode".to_string());
    }
    // Format version, no clue what this is so we skip it
    bytecode.read(1);
    // Endianness flag
    let endianness = bytecode.read(1)[0];
    if endianness > 1 {
        ErrorKind::invalid_header("endianness flag can only be 0 or 1".to_string());
    }
    bytecode.set_endian(endianness);
    // Size of int (in bytes)
    let size_int = bytecode.read(1)[0];
    if size_int > 16 {
        ErrorKind::invalid_header("int size cannot be bigger than 16 bytes".to_string())
    }
    bytecode.set_int(size_int as usize);
    // Size of size_t (in bytes)
    let size_size_t = bytecode.read(1)[0];
    if size_size_t > 8 {
        ErrorKind::invalid_header("size_t cannot be bigger than 8 bytes".to_string())
    }
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

    bytecode.debug();

    // Now, we move onto the function block

    function_block(&mut bytecode);
}  