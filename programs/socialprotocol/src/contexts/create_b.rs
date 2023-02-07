use crate::*;

#[derive(Accounts)]
pub struct CreateB<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut, seeds = [b"spling"], bump = spling.bump)]
    pub spling: Account<'info, Spling>,
    #[account(init, payer = user, space = 8, seeds = [b"b".as_ref()], bump)]
    pub b: Account<'info, B>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

impl<'info> CreateB<'_> {
    pub fn process(&mut self) -> Result<()> {
        Ok(())
    }
}
