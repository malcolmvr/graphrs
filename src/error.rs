#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub message: String,
}

#[derive(Debug)]
pub enum ErrorKind {
    NodeMissing,
}
