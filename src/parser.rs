/* parserモジュール */
use crate::instruction::*;

use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};

// 命令を格納したベクタを返すパーサ
// [TODO]: 記号表の作成
pub fn parse(fname: &str, mut symbol_table: HashMap<String, u8>)
    -> Result<Vec<Instrct>, std::io::Error> {
    // 命令を格納するベクタ
    let mut prog: Vec<Instrct> = Vec::new();

    // 命令後の総数を管理するカウンタ
    // 上のインクリメント処理

    match fs::File::open(fname) {
        Ok(file) => {
            let buf_reader = BufReader::new(file);
            for line in buf_reader.lines() {
                match line {
                    Ok(line) => {
                        // [TODO]: 記号アドレスはこの中でマッチしてしまう
                        // if let式が使えそう
                        prog.push(discern(line.as_str()));
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

fn discern(line: &str) -> Instrct {
    // 制御命令
    let re_ctrl = Regex::new(r"[A-Z]+").unwrap();
    // シフト命令
    let re_shift = Regex::new(r"((S|R)(RA|LA|RL|LL)) (ACC|IX)").unwrap();
    // 分岐命令
    let re_branch = Regex::new(r"(B[A-Z]{1,2}) ([0-9a-fA-F]{1,2}|[A-Z]+)").unwrap();
    // (データ転送/算術演算/論理演算)命令
    let re_mal = Regex::new(
    "([A-Z]{2,3}) (ACC|IX) \
        (ACC|IX|[0-9a-fA-F]{1,2}\
        |\\((IX\\+)?[0-9a-fA-F]{1,2}\\)\
        |\\[(IX\\+)?[0-9a-fA-F]{1,2}\\])",
    )
    .unwrap();

    if re_mal.is_match(line) {
        // println!("MAL");

        let caps = re_mal.captures(line).unwrap();
        Instrct::new(InstrctKind::MAL, &caps[1], &caps[2], &caps[3])
    } else if re_branch.is_match(line) {
        // println!("branch");

        let caps = re_branch.captures(line).unwrap();
        Instrct::new(InstrctKind::Branch, &caps[1], &caps[2], "")
        // [FIXME]: 3桁以上の16進数は上2桁で打ち切られる
        /*  分岐命令の次にキャプチャされる分岐先アドレスはオペランドAではないが
            便宜上opr_aに格納する */
    } else if re_shift.is_match(line) {
        // println!("shift");

        let caps = re_shift.captures(line).unwrap();
        Instrct::new(InstrctKind::Shift, &caps[1], &caps[4], "")
    } else if re_ctrl.is_match(line) {
        // println!("ctrl");

        let caps = re_ctrl.captures(line).unwrap();
        Instrct::new(InstrctKind::Ctrl, &caps[0], "", "")
    } else {
        unreachable!("Unknown Instruction")
    }
}

/*
    parseの時点でオペランドA/Bを識別して
    Instruction構造体のメンバとする
    - 関数の移植
    - codeモジュールからの参照
*/
