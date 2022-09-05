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
    pub kind: InstructionKind, // 命令の種類
    pub inst: String,          // 命令
    pub opr_a: String,         // オペランドA
    pub opr_b: String,         // オペランドB
}

pub type Instrct = Instruction;

impl Instruction {
    pub fn new(
        kind: InstructionKind,
        inst: impl Into<String>,
        opr_a: impl Into<String>,
        opr_b: impl Into<String>,
    ) -> Instruction {
        Instruction {
            kind: kind,
            inst: inst.into(),
            opr_a: opr_a.into(),
            opr_b: opr_b.into(),
        }
    }
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.inst, self.opr_a, self.opr_b)
    }
}
