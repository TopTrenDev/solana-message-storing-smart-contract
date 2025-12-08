use anchor_lang::prelude::*;

declare_id!("Bi7UTYuqUUS81gitNW4AgY315bgmFbiRpfj98h9UfaN7");

pub mod states;
pub mod events;
pub mod constants;
pub mod instructions;
pub mod errors;

use instructions::*;

#[program]
pub mod message_storing_contract {
    use super::*;

    pub fn initialize_message(ctx: Context<InitializeMessage>, message: String) -> Result<()> {
        initialize_message::initialize_message(ctx, message)?;
        Ok(())
    }

    pub fn update_message(ctx: Context<UpdateMessage>, new_message: String) -> Result<()> {
        update_message::update_message(ctx, new_message)?;
        Ok(())
    }
}
