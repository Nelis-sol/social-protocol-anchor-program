use crate::*;
use clockwork_sdk::{state::Thread, ThreadProgram};

#[derive(Accounts)]
pub struct ClockworkDeletePost<'info> {
    pub shdw: SystemAccount<'info>,
    #[account(mut, seeds = [b"spling"], bump = spling.bump)]
    pub spling: Account<'info, Spling>,
    #[account(mut,close=spling, seeds = [b"post".as_ref(), shdw.key().as_ref()], bump)]
    pub post: Account<'info, Post>,
    #[account(mut,signer,address = Thread::pubkey(post.key(),"post_thread".to_string()))]
    pub post_thread: Box<Account<'info, Thread>>,
    #[account(address = ThreadProgram::id())]
    pub thread_program: Program<'info, ThreadProgram>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

impl<'info> ClockworkDeletePost<'_> {
    pub fn process(&mut self) -> Result<()> {
        Ok(())
    }
}
