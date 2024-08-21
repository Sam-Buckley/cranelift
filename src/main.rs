#![allow(dead_code, unused)]
mod code;
use code::Operation;
mod compile;
use compile::Context;
use cranelift_module::Module;

fn main() {
    let code = String::from(">>><<");
    let operations: Vec<Operation> = code::lex(&code);
    let mut x = Context::new();
    let (id, mut ctx) = x.define_entrypoint(None);
    x.objmodule.define_function(id, &mut ctx);
    let product = x.objmodule.finish();

    //write to output.o
    std::fs::write("output.o", product.emit().unwrap()).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex() {
        let code = String::from("+-[.,]");
        let operations: Vec<Operation> = code::lex(&code);
        assert_eq!(
            operations,
            vec![
                Operation::Add,
                Operation::Sub,
                Operation::LoopStart,
                Operation::Output,
                Operation::Input,
                Operation::LoopEnd,
            ]
        );
    }
    #[test]
    fn test_bad_char() {
        let code = String::from("+-[.,]x");
        let operations: Vec<Operation> = code::lex(&code);
        assert_eq!(
            operations,
            vec![
                Operation::Add,
                Operation::Sub,
                Operation::LoopStart,
                Operation::Output,
                Operation::Input,
                Operation::LoopEnd,
            ]
        );
    }
}
