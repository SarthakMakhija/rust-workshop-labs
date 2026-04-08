use std::error::Error;
use std::fmt::{Display, Formatter};

// ❓ Error Handling in Concurrent Systems.
// 🤔 Questions:
// - In earlier stages, we used 'unwrap()' or 'expect()'. Why move to 'Result' now?
// - What should a caller do if they receive a 'ShuttingDown' error?
// - How does a structured error API improve the "UX" for other developers 
//   using this crate?
#[derive(Debug, PartialEq)]
pub enum CacheError {
    // 💡 The Lifecycle Signal: Notifying callers that the party is over.
    ShuttingDown,
}

impl Display for CacheError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", self)
    }
}

impl Error for CacheError {}