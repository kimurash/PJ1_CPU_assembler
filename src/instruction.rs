/* instructionモジュール */

// Debugトレイトを導出すると{:?}で表示できる
// PartialEqトレイトによって==演算子が使えるようになる
#[derive(Debug, PartialEq)]
pub enum InstructionKind {
    MAL,     // (データ転送/算術演算/論理演算)命令
    Ctrl,    // 制御命令
    Shift,   // シフト命令
    Branch,  // 分岐命令
    Unknown, // 不明
}

pub type InstrctKind = InstructionKind;

#[derive(Debug)]
pub struct Instruction {
    pub kind: InstructionKind,   // 命令の種類
    pub inst: String,            // 命令
    pub opr_a: Option<OperandA>, // オペランドA
    pub opr_b: Option<OperandB>, // オペランドB
}

pub type Instrct = Instruction;

impl Instruction {
    pub fn new(
        kind: InstructionKind,
        inst: impl Into<String>,
        opr_a: Option<OperandA>,
        opr_b: Option<OperandB>,
    ) -> Instruction {
        Instruction {
            kind: kind,
            inst: inst.into(),
            opr_a: opr_a,
            opr_b: opr_b,
        }
    }
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(opr_a) = &self.opr_a {
            if let Some(opr_b) = &self.opr_b {
                write!(f, "{}\t{:?}\t{:?}", self.inst, opr_a, opr_b)
            } else {
                write!(f, "{}\t{:?}\t{:?}", self.inst, opr_a, self.opr_b)
            }
        } else {
            if let Some(opr_b) = &self.opr_b {
                write!(f, "{}\t{:?}\t{:?}", self.inst, self.opr_a, opr_b)
            } else {
                write!(f, "{}\t{:?}\t{:?}", self.inst, self.opr_a, self.opr_b)
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum OperandA {
    ACC,            // アキュムレータ
    IX,             // インデックスレジスタ
    Dest(u8),       // 分岐先アドレス
    Symbol(String), // 記号アドレス
    /*
        分岐先アドレスと記号アドレスはオペランドAに分類されないが
        便宜上OperandAのバリアントとする
    */
}

#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum OperandB {
    ACC,         // アキュレータ
    IX,          // インデックスレジスタ
    IMMD(u8),    // 即値
    ABS_PRG(u8), // 絶対アドレス(プログラム領域)
    ABS_DT(u8),  // 絶対アドレス(データ領域)
    IX_PRG(u8),  // インデックス修飾(プログラム領域)
    IX_DT(u8),   // インデックス修飾(データ領域)
}
