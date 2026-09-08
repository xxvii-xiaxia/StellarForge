//! Contract invocation argument values.
//!
//! A deliberately small subset of Soroban's `ScVal` type — enough to describe a contract call
//! (e.g. a SEP-41 `transfer`) before it is encoded to XDR. Full `ScVal` encoding/decoding is
//! left to the Soroban module (`project.md` issues 011-012).

/// A single contract-call argument.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScArg {
    /// A Stellar address argument (account or contract), passed as its strkey string.
    Address(String),
    /// A 128-bit signed integer, used for token amounts (e.g. SEP-41 `i128` amounts).
    I128(i128),
    /// A 64-bit unsigned integer.
    U64(u64),
    /// A short symbol, such as a function selector argument or enum tag.
    Symbol(String),
}
