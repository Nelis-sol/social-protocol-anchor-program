use crate::*;
#[derive(Accounts)]
pub struct CreateBank<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut, seeds = [b"spling"], bump = spling.bump)]
    pub spling: Account<'info, Spling>,
    #[account(init, payer = user, space = 9900, seeds = [b"bank".as_ref()], bump)]
    pub bank: Account<'info, Bank>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

impl<'info> CreateBank<'_> {
    pub fn process(&mut self, bump: u8) -> Result<()> {
        let Self { user, bank, .. } = self;

        // Pb is a PDA, so here we store the bump
        bank.size = 9900;
        bank.bump = bump;

        Ok(())
    }
}
