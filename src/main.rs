use assembler::parser;
use assembler::code;

use std::collections::HashMap;

fn main() {
    let mut symbol_table: HashMap<String, u8> = HashMap::new();
    match parser::parse("./src/testsuite/phase2/mult.txt", &mut symbol_table) {
        Ok(prog) => {
            // プログラムのダンプ
            // for inst in prog.iter() {
            //     println!("{}", inst);
            // }

            if let Err(err) = code::assemble("./src/output.txt", prog, &symbol_table) {
                println!("{}", err);
            }
        },
        Err(err) => {
            println!("{}", err)
        }
    }
}
