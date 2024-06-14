use std::sync::Arc;
use crate::compiler::ByteCode;

pub struct VM {
    byte_code: Arc<ByteCode>,
}

impl VM {
    pub fn new(byte_code: Arc<ByteCode>) -> Self {
        VM {
            byte_code
        }
    }

    pub fn run(&self) -> () {
        todo!()
    }
}