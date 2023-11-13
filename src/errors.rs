use thiserror::Error;

type Message = &'static str;
type Index = usize;

#[derive(Debug, Error)]
pub enum WgseEngineError {
    #[error("target {0} not found")]
    MismatchedTarget(Message),
    #[error("try to access an empty stack")]
    StackEmpty,
    #[error("try to access the `{0}`th element while stack size is only `{1}`")]
    StackIndexExceeded(Index, Index),
    #[error("try to access the last `{0}`th element while stack size is only `{1}`")]
    StackReverseIndexExceeded(Index, Index),
}
