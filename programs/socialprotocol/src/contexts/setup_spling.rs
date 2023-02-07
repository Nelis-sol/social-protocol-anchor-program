use crate::*;

#[derive(Accounts)]
pub struct SetupSpling<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init, payer = user, space = 8 + mem::size_of::<Spling>(), seeds = [b"spling"], bump)]
    pub spling: Account<'info, Spling>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

impl<'info> SetupSpling<'_> {
    pub fn process(&mut self, bump: u8) -> Result<()> {
        let Self { spling, user, .. } = self;

        // start with 0 - when a new user signs up/group is created, increments with 1
        spling.users = 0;
        spling.groups = 0;
        spling.posts = 0;
        spling.tags = 0;

        // Spling is a PDA, so here we store the bump
        spling.bump = bump;
        Ok(())
    }
}
