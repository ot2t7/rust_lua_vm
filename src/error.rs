pub enum ErrorKind {
    CouldntReadFile,
    InvalidHeader(String),
    TooShort
}

pub fn err(kind: ErrorKind) {
    match kind {
        ErrorKind::CouldntReadFile => panic!("couldn't read the provided file"),
        ErrorKind::InvalidHeader(v) => panic!("lua header is malformed ({})", v),
        ErrorKind::TooShort => panic!("bytecode length too short")
    }
}