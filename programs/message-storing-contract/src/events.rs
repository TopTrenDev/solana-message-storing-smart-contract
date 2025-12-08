use anchor_lang::prelude::*;

#[event]
pub struct MessageUpdated {
    pub new_message: String,
}
