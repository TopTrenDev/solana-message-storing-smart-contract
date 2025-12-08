use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Message exceeds maximum length")]
    MessageTooLong,
}
