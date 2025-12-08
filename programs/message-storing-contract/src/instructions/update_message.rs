use anchor_lang::prelude::*;

use crate::{
    constants::MESSAGE_SEED,
    errors::CustomError,
    events::MessageUpdated,
    states::MessageAccount,
};

#[derive(Accounts)]
pub struct UpdateMessage<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [MESSAGE_SEED],
        bump
    )]
    pub message_account: Account<'info, MessageAccount>,

    pub system_program: Program<'info, System>,
}

pub fn update_message(ctx: Context<UpdateMessage>, new_message: String) -> Result<()> {
    require!(new_message.len() <= crate::constants::MAX_MESSAGE_BYTES, CustomError::MessageTooLong);

    let message_account = &mut ctx.accounts.message_account;

    message_account.message = new_message;
    msg!("Message updated to: {}", message_account.message);

    emit!(MessageUpdated {
        new_message: message_account.message.clone(),
    });

    Ok(())
}
