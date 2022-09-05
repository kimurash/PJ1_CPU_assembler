use assembler::parser;
use assembler::code;

use std::collections::HashMap;

fn main() {
    let mut symbol_table: HashMap<String, u8> = HashMap::new();
    match parser::parse("./src/testsuite/eor.txt", symbol_table) {
        Ok(prog) => {
            // プログラムのダンプ
            // for inst in prog.iter() {
            //     println!("{}", inst);
            // }

            match code::assemble("./src/output.txt", prog) {
                Ok(_) => {},
                Err(err) => {
                    println!("{}", err);
                } 
            }
        },
        Err(err) => {
            println!("{}", err)
        }
    }
}
