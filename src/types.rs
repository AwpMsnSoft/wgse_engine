use wgse_kernel::types::{common::Instruction, wrapper::Register};

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Registers {
    // stack pointer
    pub sp: Register,
    // base pointer, used for local variable indexing
    pub bp: Register,
    // program counter
    pub pc: Register,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum ValueType {
    #[default]
    Integer,
    Real,
    Address,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum ControlBlockType {
    #[default]
    StructureBlock,
    ConditionalBlock,
    LoopBlock,
    FunctionCall,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ControlBlockSignature {
    pub ty: ControlBlockType,
    pub args_ty: Vec<ValueType>,
    pub rets_ty: Vec<ValueType>,
}

#[derive(Clone, Debug, Default)]
pub struct ControlFrame {
    pub signature: ControlBlockSignature,
    pub instructions: Vec<Instruction>,
    pub registers: Registers,
}

impl ControlFrame {
    pub fn is_conditional_block(&self) -> bool {
        self.signature.ty == ControlBlockType::ConditionalBlock
    }

    pub fn is_loop_block(&self) -> bool {
        self.signature.ty == ControlBlockType::LoopBlock
    }

    pub fn is_function_call(&self) -> bool {
        self.signature.ty == ControlBlockType::FunctionCall
    }
}
