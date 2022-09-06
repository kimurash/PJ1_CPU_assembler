/* codeモジュール */
use crate::instruction::*;

use std::collections::HashMap;
use std::fs;
use std::io::{BufWriter, Write};

/*
    fname: 命令語を書き込むファイルへのパス
    prog: 命令語が格納されたベクタ
    symbol_table: 記号アドレスとアドレスの対応表
*/
pub fn assemble(
    fname: &str,
    mut prog: Vec<Instrct>,
    symbol_table: &HashMap<String, u8>,
) -> Result<(), std::io::Error> {
    let file = fs::File::create(fname).unwrap();
    let mut buf_writer = BufWriter::new(file);

    for instrct in prog.iter_mut() {
        match instrct.kind {
            InstrctKind::MAL => {
                buf_writer.write(form_mal(instrct).as_bytes())?;
            }
            InstrctKind::Ctrl => {
                buf_writer.write(form_ctrl(instrct).as_bytes())?;
            }
            InstrctKind::Shift => {
                buf_writer.write(form_shift(instrct).as_bytes())?;
            }
            InstrctKind::Branch => {
                buf_writer.write(form_branch(instrct, symbol_table).as_bytes())?;
            }
            InstrctKind::Unknown => {
                unreachable!("Unknown Instruction")
            }
        }
    }

    Ok(())
}

/*
    各種命令と命令語との対応をHashMapのグローバル変数として
    用意しようと試みたが,HashMapはコンパイル時に値が確定しないため不可
    - https://qiita.com/tatsuya6502/items/bed3702517b36afbdbca
    - https://qnighy.hatenablog.com/entry/2018/06/17/190000
*/
use lazy_static::lazy_static;

lazy_static! {
    // (データ移動/算術演算/論理演算)命令と命令後の対応
    static ref CODE_MAL: HashMap<String, u8> = vec![
        ("LD".to_owned(), 0x60),
        ("ST".to_owned(), 0x70),
        ("ADD".to_owned(), 0xB0),
        ("ADC".to_owned(), 0x90),
        ("SUB".to_owned(), 0xA0),
        ("SBC".to_owned(), 0x80),
        ("CMP".to_owned(), 0xF0),
        ("AND".to_owned(), 0xE0),
        ("OR".to_owned(), 0xD0),
        ("EOR".to_owned(), 0xC0),
    ].into_iter().collect::<HashMap<_, _>>();

    // 制御命令と命令語の対応
    static ref CODE_CTRL: HashMap<String, u8> = vec![
        ("NOP".to_owned(), 0x00),
        ("HLT".to_owned(), 0x0F),
        ("OUT".to_owned(), 0x10),
        ("IN".to_owned(), 0x1F),
        ("RCF".to_owned(), 0x20),
        ("SCF".to_owned(), 0x2F),
    ].into_iter().collect::<HashMap<_, _>>();

    // シフトモードと命令語の対応
    static ref SHIFT_MODE: HashMap<String, u8> = vec![
        ("RA".to_owned(), 0x00),
        ("LA".to_owned(), 0x01),
        ("RL".to_owned(), 0x02),
        ("LL".to_owned(), 0x03),
    ].into_iter().collect::<HashMap<_, _>>();

    // 分岐条件と命令語の対応
    static ref BRANCH_COND: HashMap<String, u8> = vec![
        ("A".to_owned(), 0x00),
        ("VF".to_owned(), 0x08),
        ("NZ".to_owned(), 0x01),
        ("Z".to_owned(), 0x09),
        ("ZP".to_owned(), 0x02),
        ("N".to_owned(), 0x0A),
        ("P".to_owned(), 0x03),
        ("ZN".to_owned(), 0x0B),
        ("NI".to_owned(), 0x04),
        ("NO".to_owned(), 0x0C),
        ("NC".to_owned(), 0x05),
        ("C".to_owned(), 0x0D),
        ("GE".to_owned(), 0x06),
        ("LT".to_owned(), 0x0E),
        ("GT".to_owned(), 0x07),
        ("LE".to_owned(), 0x0F),
    ].into_iter().collect::<HashMap<_, _>>();
}

// (データ移動/算術演算/論理演算)命令を形成する
fn form_mal(instrct: &Instruction) -> String {
    let mut base = CODE_MAL.get(&instrct.inst).unwrap().clone();

    // オペランドAがIXであれば加算
    if let Some(OperandA::IX) = instrct.opr_a {
        base += 0x08;
    }

    // オペランドBに応じて加算
    match instrct.opr_b {
        Some(OperandB::ACC) => {
            format!("{:2X}\n", base)
        }
        Some(OperandB::IX) => {
            base += 0x01;
            format!("{:2X}\n", base)
        }
        Some(OperandB::IMMD(d)) => {
            base += 0x02;
            format!("{:02X} {:02X}\n", base, d)
        }
        Some(OperandB::ABS_PRG(d)) => {
            base += 0x04;
            format!("{:02X} {:02X}\n", base, d)
        }
        Some(OperandB::ABS_DT(d)) => {
            base += 0x05;
            format!("{:02X} {:02X}\n", base, d)
        }
        Some(OperandB::IX_PRG(d)) => {
            base += 0x06;
            format!("{:02X} {:02X}\n", base, d)
        }
        Some(OperandB::IX_DT(d)) => {
            base += 0x07;
            format!("{:02X} {:02X}\n", base, d)
        }
        None => {
            unreachable!("MAL must have operand B")
        }
    }
}

// 制御命令の命令語を形成する
fn form_ctrl(instrct: &Instruction) -> String {
    format!("{:02X}\n", CODE_CTRL.get(&instrct.inst).unwrap())
}

// シフト命令の命令語を形成する
fn form_shift(instrct: &Instruction) -> String {
    let mut base = 0x40u8;

    // rotate命令であれば4加算
    let init = instrct.inst.chars().nth(0).unwrap();
    if init == 'R' {
        base += 0x04;
    }

    // シフトモードに応じて加算
    base += SHIFT_MODE.get(&(instrct.inst[1..3].to_string())).unwrap();

    // オペランドAがIXであれば8加算
    if let Some(OperandA::IX) = instrct.opr_a {
        base += 0x08;
    }

    format!("{:02X}\n", base)
}

// 分岐命令の命令語を形成する
fn form_branch(
    instrct: &mut Instruction, 
    symbol_table: &HashMap<String, u8>
) -> String {
    let mut base = 0x30u8;

    // 分岐条件によって加算
    base += BRANCH_COND.get(&(instrct.inst[1..].to_string())).unwrap();

    /* &をつけなかった場合 --> E0507
    (1) StringにはCopyトレイトが実装されていないため,
        パターンマッチには所有権のムーブが伴う.
    (2) instrctは可変の参照であるため,Symbolのアーム内で
        Stringの所有権をムーブできない.
    */
    match &instrct.opr_a {
        Some(OperandA::DEST(addr)) => {
            format!("{:02X} {:02X}\n", base, addr)
        }
        Some(OperandA::Symbol(symbol)) => {
            let addr = symbol_table.get(symbol).unwrap();
            format!("{:02X} {:02X}\n", base, addr)
        }
        _ => {
            unreachable!("Branch must have destination address")
        }
    }
}
