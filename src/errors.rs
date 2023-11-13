use thiserror::Error;

type Message = String;
type Index = usize;

#[derive(Debug, Error)]
pub enum WgseEngineError {
    #[error("inconsistent type, expect `{expect}`, found `{found}`")]
    InconsistentTypes { expect: String, found: String },
    #[error("target {0} not found")]
    MismatchedTarget(Message),
    #[error("try to access an empty stack")]
    StackEmpty,
    #[error("try to access the `{expect}`th element while stack size is only `{size}`")]
    StackIndexExceeded { expect: Index, size: Index },
    #[error("try to access the last `{expect}`th element while stack size is only `{size}`")]
    StackReverseIndexExceeded { expect: Index, size: Index },
}
