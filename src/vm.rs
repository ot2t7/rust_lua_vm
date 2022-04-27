use crate::error;

fn pop(needle: &mut usize, buf: &Vec<u8>, to_pop: usize) {
    let snippet: Vec<u8> = buf[*needle..*needle+to_pop].try_into().unwrap().to_vec();
}

pub fn run_vm(buf: Vec<u8>) {
    // The bytecode can't be too short, have to have bytes for header and stuff
    if buf.len() < 12 {
        error::err(error::ErrorKind::TooShort);
    }


    let mut needle: usize = 0;
    
    fn read(size: usize) {
        let snippet = buf[needle..needle+size].try_into().unwrap();
    }

    // The bytecode must start with 0x1B4C7561, or "Lua"
    let sig: &[u8 ; 4] = buf[needle..needle+4].try_into().unwrap();
    if u32::from_be_bytes(*sig) != 457995617 {
        error::err(error::ErrorKind::InvalidHeader("no Lua signature".to_string()))
    }

    needle += 4;

    // The version number of lua, this only supports 5.1, or 0x51
    let ver: &[u8 ; 1] = buf[needle..needle+1].try_into().unwrap();
    if u8::from_be_bytes(*ver) != 81 {
        error::err(error::ErrorKind::InvalidHeader("this library only supports 5.1".to_string()))
    }

    needle += 1;

    // Format version, we don't need to worry about this byte, so we skip it
    needle += 1;

    // Endianness flag, I'm not sure how endianness works in rust, so we skip it
    needle += 1;


} 