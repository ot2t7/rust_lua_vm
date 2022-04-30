pub struct ErrorKind;

impl ErrorKind {
    pub fn couldnt_read_file() { panic!("couldn't read the provided file") }
    pub fn invalid_header(desc: String) { panic!("lua header is malformed ({})", desc) }
    pub fn too_short() { panic!("bytecode length too short") }
    pub fn invalid_string() { panic!("a malformed utf-8 string was recieved") }
}