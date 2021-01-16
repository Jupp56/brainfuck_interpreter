pub type Program = Vec<Instruction>;

#[derive(Clone, Debug, PartialEq)]
/// All instructions possible, with optimizations
pub enum Instruction {
    IncPtr(usize),
    DecPtr(usize),
    IncCell(u32),
    DecCell(u32),
    Output,
    Input,
    JumpForward(usize),
    JumpBackward(usize),
    ZeroCell,
    FindZeroRight(usize),
    FindZeroLeft(usize),
}