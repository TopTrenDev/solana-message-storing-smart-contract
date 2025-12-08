use anchor_lang::prelude::*;

use crate::constants::MAX_MESSAGE_BYTES;

#[account]
pub struct MessageAccount {
    pub message: String,
}

impl MessageAccount {
    pub const MESSAGE_ACCOUNT_SIZE: usize = 8 + 4 + MAX_MESSAGE_BYTES;
}
