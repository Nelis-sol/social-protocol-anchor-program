use anchor_lang::prelude::*;
use anchor_lang::solana_program::rent::Rent;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use spl_token::solana_program::account_info::AccountInfo;
use spl_token::solana_program::system_program;
use std::mem;

declare_id!("BfZEDfZLyTkNgdotvwokkayzHxCYJZQqFQM8BMc9kSza");

pub mod contexts;
pub mod states;

pub use contexts::*;
pub use states::*;

#[program]
pub mod socialprotocol {
    use super::*;

    // initialize spling once, to keep track of number of users and groups
    // these numbers serve as id's for users and groups as well
    pub fn setup_spling(ctx: Context<SetupSpling>) -> Result<()> {
        let bump = *ctx.bumps.get("spling").unwrap();
        ctx.accounts.process(bump)
    }

    pub fn setup_tags(ctx: Context<SetupTags>) -> Result<()> {
        let bump = *ctx.bumps.get("tags").unwrap();
        ctx.accounts.process(bump)
    }

    // a user can add a profile, of which the content is stored on the Shadow Drive
    pub fn create_user_profile(
        ctx: Context<CreateUserProfile>,
        shdw: Pubkey,
        amount: Option<u64>,
    ) -> Result<()> {
        let bump = *ctx.bumps.get("user_profile").unwrap();
        ctx.accounts.process(shdw, amount, bump)
    }

    // create a group profile, of which the content is stored on the Shadow Drive
    pub fn create_group_profile(
        ctx: Context<CreateGroupProfile>,
        shdw: Pubkey,
        amount: Option<u64>,
    ) -> Result<()> {
        let bump = *ctx.bumps.get("group_profile").unwrap();
        ctx.accounts.process(shdw, amount, bump)
    }

    // user can join a group
    pub fn join_group(ctx: Context<JoinGroup>, address: u32, amount: Option<u64>) -> Result<()> {
        ctx.accounts.process(address, amount)
    }

    // leave group
    pub fn leave_group(ctx: Context<LeaveGroup>, address: u32, amount: Option<u64>) -> Result<()> {
        ctx.accounts.process(address, amount)
    }

    // user can follow another user
    pub fn follow_user(ctx: Context<FollowUser>, address: u32, amount: Option<u64>) -> Result<()> {
        ctx.accounts.process(address, amount)
    }

    // unfollow another user
    pub fn unfollow_user(
        ctx: Context<UnfollowUser>,
        address: u32,
        amount: Option<u64>,
    ) -> Result<()> {
        ctx.accounts.process(address, amount)
    }

    pub fn submit_post(
        ctx: Context<SubmitPost>,
        group_id: u32,
        shdw: Pubkey,
        tag_name: String,
        amount: Option<u64>,
        schedule: String,
    ) -> Result<()> {
        let post_bump = *ctx.bumps.get("post").unwrap();
        let likes_bump = *ctx.bumps.get("likes").unwrap();
        ctx.accounts.process(
            group_id, shdw, tag_name, amount, post_bump, likes_bump, schedule,
        )
    }

    // like a post
    pub fn like_post(ctx: Context<LikePost>, amount: Option<u64>) -> Result<()> {
        ctx.accounts.process(amount)
    }

    pub fn submit_reply(
        ctx: Context<SubmitReply>,
        post_id: u32,
        shdw: Pubkey,
        amount: Option<u64>,
    ) -> Result<()> {
        let bump = *ctx.bumps.get("reply").unwrap();
        ctx.accounts.process(post_id, shdw, amount, bump)
    }

    // delete a post
    pub fn delete_post(
        ctx: Context<DeletePost>,
        _group_id: u32,
        _shdw: Pubkey,
        amount: Option<u64>,
    ) -> Result<()> {
        ctx.accounts.process(amount)
    }

    // delete a reply
    pub fn delete_reply(
        ctx: Context<DeleteReply>,
        _post_id: u32,
        _shdw: Pubkey,
        amount: Option<u64>,
    ) -> Result<()> {
        ctx.accounts.process(amount)
    }

    // delete user profile
    pub fn delete_user_profile(
        ctx: Context<DeleteUserProfile>,
        _user_id: u32,
        _shdw: Pubkey,
        amount: Option<u64>,
    ) -> Result<()> {
        ctx.accounts.process(amount)
    }

    // delete group profile
    pub fn delete_group_profile(
        ctx: Context<DeleteGroupProfile>,
        _shdw: Pubkey,
        amount: Option<u64>,
    ) -> Result<()> {
        ctx.accounts.process(amount)
    }

    pub fn create_bank(ctx: Context<CreateBank>) -> Result<()> {
        let bump = *ctx.bumps.get("bank").unwrap();
        ctx.accounts.process(bump)
    }

    pub fn create_b(ctx: Context<CreateB>) -> Result<()> {
        ctx.accounts.process()
    }

    pub fn reset_bank(ctx: Context<ResetBank>) -> Result<()> {
        ctx.accounts.process()
    }

    pub fn extract_bank(ctx: Context<ExtractBank>, amount: Option<u64>) -> Result<()> {
        ctx.accounts.process(amount)
    }

    pub fn clockwork_delete_post(ctx: Context<ClockworkDeletePost>) -> Result<()> {
        ctx.accounts.process()
    }
}
