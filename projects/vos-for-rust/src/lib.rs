trait ArithmeticRPC {
    fn add(&mut self, a: i32, b: i32) -> i32 {
        a + b
    }
    fn sub(&mut self, a: i32, b: i32) -> i32 {
        a - b
    }
    fn mul(&mut self, a: i32, b: i32) -> i32 {
        a * b
    }
    fn div(&mut self, a: i32, b: i32) -> i32 {
        a / b
    }
}

pub struct StateProvider;

impl ArithmeticRPC for StateProvider {}

fn main() {
    let mut io = jsonrpc_core::IoHandler::new();
    io.extend_with(StateProvider.to_delegate())
}
