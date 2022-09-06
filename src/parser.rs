/* parserモジュール */
use crate::instruction::*;

use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};

/*
    fname: アセンブラプログラムが記述されたファイルへのパス
    symbol_table: 記号アドレスとアドレスの対応表
    -> 命令を格納したベクタ
*/
pub fn parse(
    fname: &str,
    symbol_table: &mut HashMap<String, u8>,
) -> Result<Vec<Instrct>, std::io::Error> {
    let re_symbol_dec = Regex::new(r"^([A-Z]+):$").unwrap(); // 記号アドレス
    let mut prog: Vec<Instrct> = Vec::new(); // 命令を格納するベクタ
    let mut pc = 0u8; // プログラムカウンタ

    match fs::File::open(fname) {
        Ok(file) => {
            let buf_reader = BufReader::new(file);
            for line in buf_reader.lines() {
                match line {
                    Ok(line) => {
                        if re_symbol_dec.is_match(line.as_str()) {  // 記号アドレス
                            let caps = re_symbol_dec.captures(line.as_str()).unwrap();
                            symbol_table.insert((&caps[1]).to_string(), pc);
                        } else {    // 記号アドレス以外
                            prog.push(discern(line.as_str(), &mut pc));
                        }
                    }
                    // 1行の読み込みに失敗
                    Err(err) => return Err(err),
                }
            }
        }
        // ファイルオープンに失敗
        Err(err) => return Err(err),
    }

    Ok(prog)
}

use lazy_static::lazy_static;

lazy_static! {
    // 制御命令
    static ref RE_CTRL: Regex = Regex::new(r"^[A-Z]{2,3}$").unwrap();
    // シフト命令
    static ref RE_SHIFT: Regex = Regex::new(r"^((S|R)(RA|LA|RL|LL)) (ACC|IX)$").unwrap();
    // 分岐命令
    static ref RE_BRANCH: Regex = Regex::new(r"^(B[A-Z]{1,2}) ([0-9a-fA-F]{1,2}|[A-Z]+)$").unwrap();
    // (データ転送/算術演算/論理演算)命令
    static ref RE_MAL: Regex = Regex::new(
        "^([A-Z]{2,3}) (ACC|IX) \
        (ACC|IX|[0-9a-fA-F]{1,2}\
        |\\((IX\\+)?[0-9a-fA-F]{1,2}\\)\
        |\\[(IX\\+)?[0-9a-fA-F]{1,2}\\])$",
    )
    .unwrap();
}

// 1行分の命令を識別する
fn discern(line: &str, pc: &mut u8) -> Instrct {
    if RE_MAL.is_match(line) {
        /*
            オペランドBによって命令語長が変化するため
            pcの更新はdiscern_opr_b()に委託する
        */
        let caps = RE_MAL.captures(line).unwrap();
        Instrct::new(
            InstrctKind::MAL,
            &caps[1],
            Some(discern_opr_a(&caps[2])),
            Some(discern_opr_b(&caps[3], pc)),
        )
    } else if RE_BRANCH.is_match(line) {
        *pc += 2;

        let caps = RE_BRANCH.captures(line).unwrap();
        Instrct::new(
            InstrctKind::Branch,
            &caps[1],
            // [WARN]: 3桁以上の16進数は上2桁で打ち切られる
            Some(discern_opr_a(&caps[2])),
            None,
        )
    } else if RE_SHIFT.is_match(line) {
        *pc += 1;

        let caps = RE_SHIFT.captures(line).unwrap();
        Instrct::new(
            InstrctKind::Shift,
            &caps[1],
            Some(discern_opr_a(&caps[4])),
            None,
        )
    } else if RE_CTRL.is_match(line) {
        *pc += 1;

        let caps = RE_CTRL.captures(line).unwrap();
        Instrct::new(InstrctKind::Ctrl, &caps[0], None, None)
    } else {
        unreachable!("Unknown Instruction")
    }
}

lazy_static! {
    // 分岐先アドレス
    static ref RE_ADDR: Regex = Regex::new(r"^[0-9a-fA-F]{1,2}$").unwrap();
    // 記号アドレス
    static ref RE_SYMBOL: Regex = Regex::new(r"^[A-Z]+$").unwrap();
}

// オペランドAを識別する
fn discern_opr_a(opr_a: &str) -> OperandA {
    match opr_a {
        "ACC" => OperandA::ACC,
        "IX" => OperandA::IX,
        _ => {
            if RE_ADDR.is_match(opr_a) {
                OperandA::Dest(u8::from_str_radix(opr_a, 16).unwrap())
            } else if RE_SYMBOL.is_match(opr_a) {
                /* [HACK]: 無駄な変換
                    この関数に渡すときStringを&strへ変換し,
                    ここで&strからStringへ変換している
                */
                OperandA::Symbol(opr_a.to_string())
            } else{
                unreachable!("Invalid Operand A")
            }
        }
    }
}

lazy_static! {
    // 即値アドレス
    static ref RE_IMMD: Regex = Regex::new(r"^[0-9a-fA-F]{1,2}$").unwrap();
    // 絶対アドレス(プログラム領域)
    static ref RE_ABS_PRG: Regex = Regex::new(r"^\[([0-9a-fA-F]{1,2})\]$").unwrap();
    // 絶対アドレス(データ領域)
    static ref RE_ABS_DT: Regex = Regex::new(r"^\(([0-9a-fA-F]{1,2})\)$").unwrap();
    // インデックス修飾(プログラム領域)
    static ref RE_IX_PRG: Regex = Regex::new(r"^\[IX\+([0-9a-fA-F]{1,2})\]$").unwrap();
    // インデックス修飾(データ領域)
    static ref RE_IX_DT: Regex = Regex::new(r"^\(IX\+([0-9a-fA-F]{1,2})\)$").unwrap();
}

// オペランドBを識別する
fn discern_opr_b(opr_b: &str, pc: &mut u8) -> OperandB {
    match opr_b {
        "ACC" => {
            *pc += 1;
            OperandB::ACC
        }
        "IX" => {
            *pc += 1;
            OperandB::IX
        }
        _ => {
            if RE_IX_PRG.is_match(opr_b) {
                *pc += 2;
                let caps = RE_IX_PRG.captures(opr_b).unwrap();
                let offset = u8::from_str_radix(&caps[1], 16).unwrap();
                OperandB::IX_PRG(offset)
            } else if RE_IX_DT.is_match(opr_b) {
                *pc += 2;
                let caps = RE_IX_DT.captures(opr_b).unwrap();
                let offset = u8::from_str_radix(&caps[1], 16).unwrap();
                OperandB::IX_DT(offset)
            } else if RE_ABS_PRG.is_match(opr_b) {
                *pc += 2;
                let caps = RE_ABS_PRG.captures(opr_b).unwrap();
                let addr = u8::from_str_radix(&caps[1], 16).unwrap();
                OperandB::ABS_PRG(addr)
            } else if RE_ABS_DT.is_match(opr_b) {
                *pc += 2;
                let caps = RE_ABS_DT.captures(opr_b).unwrap();
                let addr = u8::from_str_radix(&caps[1], 16).unwrap();
                OperandB::ABS_DT(addr)
            } else if RE_IMMD.is_match(opr_b) {
                *pc += 2;
                let caps = RE_IMMD.captures(opr_b).unwrap();
                let data = u8::from_str_radix(&caps[0], 16).unwrap();
                OperandB::IMMD(data)
            } else {
                unreachable!("Invalid Operand B")
            }
        }
    }
}
