use anchor_lang::prelude::*;

use crate::{ constants::MESSAGE_SEED, errors::CustomError, states::MessageAccount };

#[derive(Accounts)]
pub struct InitializeMessage<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = 8 + MessageAccount::MESSAGE_ACCOUNT_SIZE,
        seeds = [MESSAGE_SEED],
        bump
    )]
    pub message_account: Account<'info, MessageAccount>,

    pub system_program: Program<'info, System>,
}

pub fn initialize_message(ctx: Context<InitializeMessage>, message: String) -> Result<()> {
    require!(message.len() <= crate::constants::MAX_MESSAGE_BYTES, CustomError::MessageTooLong);

    let message_account = &mut ctx.accounts.message_account;

    message_account.message = message;
    msg!("Message stored: {}", message_account.message);

    Ok(())
}
