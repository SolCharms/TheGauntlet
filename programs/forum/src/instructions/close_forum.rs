use anchor_lang::prelude::*;

use crate::state::{Forum};
use prog_common::errors::ErrorCode;
use prog_common::{close_account};

#[derive(Accounts)]
#[instruction(bump_treasury: u8)]
pub struct CloseForum<'info> {

    // Forum and Forum Manager
    #[account(mut, has_one = forum_manager, has_one = forum_treasury)]
    pub forum: Box<Account<'info, Forum>>,
    pub forum_manager: Signer<'info>,

    /// CHECK:
    #[account(mut, seeds = [b"treasury".as_ref(), forum.key().as_ref()], bump = bump_treasury)]
    pub forum_treasury: AccountInfo<'info>,

    /// CHECK:
    #[account(mut)]
    pub receiver: AccountInfo<'info>,

    // misc
    pub system_program: Program<'info, System>,

}

pub fn handler(ctx: Context<CloseForum>) -> Result<()> {

    let forum = &mut ctx.accounts.forum;

    // Ensure count PDAs associated to forum have already been closed
    if (forum.forum_profile_count > 0) || (forum.forum_question_count > 0) || (forum.forum_answer_count > 0) || (forum.forum_comment_count > 0) {
        return Err(error!(ErrorCode::NotAllForumPDAsClosed));
    }

    // Set the receiver of the lamports to be reclaimed from the rent of the accounts to be closed
    let receiver = &mut ctx.accounts.receiver;

    // Close the forum treasury state account
    let treasury_account_info = &mut ctx.accounts.forum_treasury.to_account_info();
    close_account(treasury_account_info, receiver)?;

    // Close the forum state account
    let forum_account_info = &mut (*ctx.accounts.forum).to_account_info();
    close_account(forum_account_info, receiver)?;

    msg!("forum account with pubkey {} now closed", ctx.accounts.forum.key());

    Ok(())
}
