use super::{errors::WgseEngineError, types::ControlFrame};
use anyhow::{anyhow, Result};
use itertools::Itertools;
use wgse_kernel::types::common::Value;
use wgse_utils::{Deref, DerefMut};

#[derive(Clone, Debug, Default, Deref, DerefMut, PartialEq)]
pub struct OperandStack {
    slots: Vec<Value>,
}

impl OperandStack {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn try_pop(&mut self) -> Result<Value> {
        self.pop().ok_or(anyhow!(WgseEngineError::StackEmpty))
    }

    /// The last `n` elements from a vector, or returns an error if the vector does not have enough elements.
    ///
    /// # Arguments
    ///
    /// * `n`: The parameter `n` represents the number of elements to be popped from the `slots`
    /// vector.
    ///
    /// # Returns
    ///
    /// The `pop_n` function returns a `Result` containing a `Vec<Value>`.
    ///
    /// # Note
    ///
    /// The poped values are sorted as `FIFO`, which means, the lastest value is at the back of returned vec.
    ///
    /// # Example
    /// ```rust
    /// # use wgse_kernel::types::common::{Value, Integer};
    /// # use wgse_engine::stacks::OperandStack;
    /// # use anyhow::Result;
    /// # fn test() -> Result<()> {
    ///
    /// # let mut operand_stack = OperandStack::new();
    /// # let mut values = vec![Value::Integer(Integer(1)), Value::Integer(Integer(2)), Value::Integer(Integer(3))];
    /// # operand_stack.append(&mut values);
    /// assert_eq!(*operand_stack, vec![Value::Integer(Integer(1)), Value::Integer(Integer(2)), Value::Integer(Integer(3))]);
    ///
    /// let poped_value = operand_stack.try_pop_n(2)?;
    /// assert_eq!(*operand_stack, vec![Value::Integer(Integer(1))]);
    /// assert_eq!(poped_value, vec![Value::Integer(Integer(2)), Value::Integer(Integer(3))]);
    /// # Ok(())
    /// # }
    /// ```
    pub fn try_pop_n(&mut self, n: usize) -> Result<Vec<Value>> {
        match self.len() {
            len @ _ if len < n => Err(anyhow!(WgseEngineError::StackReverseIndexExceeded(n, len))),
            len @ _ => Ok(self.split_off(len - n)),
        }
    }

    pub fn try_drop(&mut self) -> Result<()> {
        match self.len() {
            0 => Err(anyhow!(WgseEngineError::StackEmpty)),
            _ => {
                let _ = self.pop();
                Ok(())
            }
        }
    }

    pub fn try_drop_n(&mut self, n: usize) -> Result<()> {
        match self.len() {
            len @ _ if len < n => Err(anyhow!(WgseEngineError::StackReverseIndexExceeded(n, len))),
            _ => {
                self.slots = self.slots.clone().into_iter().dropping_back(n).collect();
                Ok(())
            }
        }
    }
}

#[derive(Clone, Debug, Default, Deref, DerefMut)]
pub struct ControlStack {
    slots: Vec<ControlFrame>,
}

impl ControlStack {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn try_top(&self) -> Result<&ControlFrame> {
        self.last().ok_or(anyhow!(WgseEngineError::StackEmpty))
    }

    pub fn try_top_mut(&mut self) -> Result<&mut ControlFrame> {
        self.last_mut().ok_or(anyhow!(WgseEngineError::StackEmpty))
    }

    pub fn try_pop(&mut self) -> Result<ControlFrame> {
        self.pop().ok_or(anyhow!(WgseEngineError::StackEmpty))
    }

    /// Tries to retrieve the top control frame from the stack and returns it as a result.
    ///
    /// # Returns
    ///
    /// A `Result` containing a reference to a `ControlFrame`.
    ///
    /// # Note
    ///
    /// All non-function-call control frame will be ejected before the first function-call control frame
    /// been poped.
    pub fn try_pop_callframe(&mut self) -> Result<&ControlFrame> {
        let pos = self.last_callframe_pos();
        self.slots = self.slots.clone().into_iter().dropping_back(pos).collect();
        // if no callframe in control stack, the stack will be cleared after calling
        // this method then raise an error.
        self.slots
            .last()
            .ok_or(anyhow!(WgseEngineError::MismatchedTarget(
                "FunctionCallFrame"
            )))
    }

    pub fn last_callframe_pos(&self) -> usize {
        self.iter()
            .rev()
            .take_while(|cf| !cf.is_function_call())
            .count()
    }
}
