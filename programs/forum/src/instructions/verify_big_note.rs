use anchor_lang::prelude::*;

use crate::state::{BigNote, Forum, UserProfile};
use prog_common::{now_ts, errors::ErrorCode};

#[derive(Accounts)]
#[instruction(bump_moderator_profile: u8, bump_user_profile: u8, bump_big_note: u8)]
pub struct VerifyBigNote<'info> {

    // Forum
    pub forum: Box<Account<'info, Forum>>,

    pub moderator: Signer<'info>,

    // The moderator profile
    #[account(mut, seeds = [b"user_profile".as_ref(), forum.key().as_ref(), moderator.key().as_ref()],
              bump = bump_moderator_profile, has_one = forum, constraint = moderator_profile.profile_owner == moderator.key())]
    pub moderator_profile: Box<Account<'info, UserProfile>>,

    /// CHECK: Used for seed verification of user profile pda account
    pub profile_owner: AccountInfo<'info>,

    // The user profile
    #[account(seeds = [b"user_profile".as_ref(), forum.key().as_ref(), profile_owner.key().as_ref()],
              bump = bump_user_profile, has_one = forum, has_one = profile_owner)]
    pub user_profile: Box<Account<'info, UserProfile>>,

    // Big Note PDA account and seed
    #[account(mut, seeds = [b"big_note".as_ref(), forum.key().as_ref(), user_profile.key().as_ref(), big_note_seed.key().as_ref()],
              bump = bump_big_note, has_one = forum, has_one = user_profile, has_one = big_note_seed)]
    pub big_note: Box<Account<'info, BigNote>>,

    /// CHECK: The seed address used for initialization of the big note PDA
    pub big_note_seed: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<VerifyBigNote>) -> Result<()> {

    let now_ts: u64 = now_ts()?;

    let moderator_profile = &ctx.accounts.moderator_profile;

    if !moderator_profile.is_moderator {
        return Err(error!(ErrorCode::ProfileIsNotModerator));
    }

    // Update big note account's most recent engagement timestamp and flip is verified boolean
    let big_note = &mut ctx.accounts.big_note;
    big_note.most_recent_engagement_ts = now_ts;
    big_note.is_verified = true;

    // Update moderator profile's most recent engagement
    let moderator_profile = &mut ctx.accounts.moderator_profile;
    moderator_profile.most_recent_engagement_ts = now_ts;

    msg!("Big Note PDA account with address {} now verified", ctx.accounts.big_note.key());
    Ok(())
}
