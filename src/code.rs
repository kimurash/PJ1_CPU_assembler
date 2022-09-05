/* codeモジュール */
use crate::instruction::*;

use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::io::{BufWriter, Write};

enum OperandA {
    ACC, // アキュムレータ
    IX,  // インデックスレジスタ
}

#[allow(non_camel_case_types)]
enum OperandB {
    ACC,         // アキュレータ
    IX,          // インデックスレジスタ
    IMMD(u8),    // 即値
    ABS_PRG(u8), // 絶対アドレス(プログラム領域)
    ABS_DT(u8),  // 絶対アドレス(データ領域)
    IX_PRG(u8),  // インデックス修飾(プログラム領域)
    IX_DT(u8),   // インデックス修飾(データ領域)
}

pub fn assemble(fname: &str, mut prog: Vec<Instrct>) -> Result<(), std::io::Error> {
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
                buf_writer.write(form_branch(instrct).as_bytes())?;
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
    if let OperandA::IX = discern_opr_a(instrct.opr_a.as_str()) {
        base += 0x08;
    }

    // オペランドBに応じて加算
    let opr_b_kind = discern_opr_b(instrct.opr_b.as_str());
    match opr_b_kind {
        OperandB::ACC => {}
        OperandB::IX => {
            base += 0x01;
        }
        OperandB::IMMD(_) => {
            base += 0x02;
        }
        OperandB::ABS_PRG(_) => {
            base += 0x04;
        }
        OperandB::ABS_DT(_) => {
            base += 0x05;
        }
        OperandB::IX_PRG(_) => {
            base += 0x06;
        }
        OperandB::IX_DT(_) => {
            base += 0x07;
        }
    }

    match opr_b_kind {
        OperandB::ACC | OperandB::IX => {
            format!("{:2X}\n", base)
        }
        OperandB::IMMD(d)
        | OperandB::ABS_PRG(d)
        | OperandB::ABS_DT(d)
        | OperandB::IX_PRG(d)
        | OperandB::IX_DT(d) => {
            format!("{:02X} {:02X}\n", base, d)
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
    if let OperandA::IX = discern_opr_a(instrct.opr_a.as_str()) {
        base += 0x08;
    }

    format!("{:02X}\n", base)
}

// 分岐命令の命令語を形成する
fn form_branch(instrct: &mut Instruction) -> String {
    let mut base = 0x30u8;

    // 分岐条件によって加算
    base += BRANCH_COND.get(&(instrct.inst[1..].to_string())).unwrap();

    // 分岐アドレスが1桁だった場合は先頭に0を付与
    if instrct.opr_a.len() < 2 {
        instrct.opr_a.insert(0, '0');
    }
    
    format!("{:02X} {}\n", base, instrct.opr_a)
}

// オペランドAを識別する
fn discern_opr_a(opr_a: &str) -> OperandA {
    match opr_a {
        "ACC" => OperandA::ACC,
        "IX" => OperandA::IX,
        _ => unreachable!("Invalid Operand A"),
    }
}

// オペランドBを識別する
fn discern_opr_b(opr_b: &str) -> OperandB {
    // 即値アドレス
    let re_immd = Regex::new(r"[0-9a-fA-F]{1,2}").unwrap();
    // 絶対アドレス(プログラム領域)
    let re_abs_prg = Regex::new(r"\[([0-9a-fA-F]{1,2})\]").unwrap();
    // 絶対アドレス(データ領域)
    let re_abs_dt = Regex::new(r"\(([0-9a-fA-F]{1,2})\)").unwrap();
    // インデックス修飾(プログラム領域)
    let re_idx_prg = Regex::new(r"\[IX\+([0-9a-fA-F]{1,2})\]").unwrap();
    // インデックス修飾(データ領域)
    let re_idx_dt = Regex::new(r"\(IX\+([0-9a-fA-F]{1,2})\)").unwrap();

    match opr_b {
        "ACC" => OperandB::ACC,
        "IX" => OperandB::IX,
        _ => {
            if re_idx_prg.is_match(opr_b) {
                let caps = re_idx_prg.captures(opr_b).unwrap();
                let offset = u8::from_str_radix(&caps[1], 16).unwrap();
                OperandB::IX_PRG(offset)
            } else if re_idx_dt.is_match(opr_b) {
                let caps = re_idx_dt.captures(opr_b).unwrap();
                let offset = u8::from_str_radix(&caps[1], 16).unwrap();
                OperandB::IX_DT(offset)
            } else if re_abs_prg.is_match(opr_b) {
                let caps = re_abs_prg.captures(opr_b).unwrap();
                let addr = u8::from_str_radix(&caps[1], 16).unwrap();
                OperandB::ABS_PRG(addr)
            } else if re_abs_dt.is_match(opr_b) {
                let caps = re_abs_dt.captures(opr_b).unwrap();
                let addr = u8::from_str_radix(&caps[1], 16).unwrap();
                OperandB::ABS_DT(addr)
            } else if re_immd.is_match(opr_b) {
                let caps = re_immd.captures(opr_b).unwrap();
                let data = u8::from_str_radix(&caps[0], 16).unwrap();
                OperandB::IMMD(data)
            } else {
                unreachable!("Invalid Operand B")
            }
        }
    }
}
