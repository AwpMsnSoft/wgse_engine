use super::{
    memory::Memory,
    stacks::{ControlStack, OperandStack},
    types::{ControlBlockSignature, ControlFrame, Registers},
};
use anyhow::Result;
use wgse_kernel::types::common::Instruction;

#[derive(Debug, Default)]
pub struct Kernel {
    pub operand_stack: OperandStack,
    pub control_stack: ControlStack,
    pub registers: Registers,
    pub rdata: Memory,
}

impl Kernel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn try_enter_control_block(
        &mut self,
        signature: ControlBlockSignature,
        instructions: Vec<Instruction>,
    ) -> Result<()> {
        *self.registers.sp = (self.operand_stack.len() - signature.args_ty.len()) as u64;

        let mut cf = ControlFrame {
            signature,
            instructions,
            registers: self.registers,
        };
        if cf.is_function_call() {
            self.registers.bp = self.registers.sp;
            cf.registers.bp = self.registers.sp;
        }
        self.control_stack.push(cf);

        Ok(())
    }

    pub fn try_exit_control_block(&mut self) -> Result<()> {
        let cf = self.control_stack.try_pop()?;
        self.try_clear_control_block(&cf)
    }

    pub fn try_clear_control_block(&mut self, cf: &ControlFrame) -> Result<()> {
        let mut results = self.operand_stack.try_pop_n(cf.signature.rets_ty.len())?;
        self.operand_stack
            .try_drop_n(self.operand_stack.len() - *cf.registers.sp as usize)?;
        self.operand_stack.append(&mut results);

        if cf.is_function_call() && self.control_stack.len() > 0 {
            let last_call = self.control_stack.try_pop_callframe()?;
            self.registers.bp = last_call.registers.sp;
        }

        Ok(())
    }

    pub fn try_reset_control_block(&mut self, cf: &ControlFrame) -> Result<()> {
        let mut args = self.operand_stack.try_pop_n(cf.signature.args_ty.len())?;
        self.operand_stack
            .try_drop_n(self.operand_stack.len() - *cf.registers.sp as usize)?;
        self.operand_stack.append(&mut args);

        Ok(())
    }

    #[allow(unused_variables)]
    pub fn try_execute(&mut self, instruction: Instruction) -> Result<()> {
        todo!()
    }

    pub fn try_loop(&mut self) -> Result<()> {
        let depth = self.control_stack.len();

        while self.control_stack.len() >= depth {
            let cf = self.control_stack.try_top_mut()?;
            if *cf.registers.pc == cf.instructions.len() as u64 {
                self.try_exit_control_block()?;
            } else {
                let instruction = cf.instructions[*cf.registers.pc as usize].clone();
                *cf.registers.pc += 1;
                self.try_execute(instruction)?;
            }
        }

        Ok(())
    }
}
