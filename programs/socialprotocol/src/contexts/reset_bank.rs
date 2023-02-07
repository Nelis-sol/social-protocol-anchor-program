use crate::*;

#[derive(Accounts)]
pub struct ResetBank<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut, seeds = [b"spling"], bump = spling.bump)]
    pub spling: Account<'info, Spling>,
    #[account(mut, seeds = [b"bank".as_ref()], bump = bank.bump)]
    pub bank: Account<'info, Bank>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

impl<'info> ResetBank<'_> {
    pub fn process(&mut self) -> Result<()> {
        let Self { bank, .. } = self;
        bank.size = 9900;

        Ok(())
    }
}
