use crate::*;

#[derive(Accounts)]
pub struct SetupTags<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut, seeds = [b"spling"], bump = spling.bump)]
    pub spling: Account<'info, Spling>,
    #[account(init, payer = user, space = 9000, seeds = [b"tags"], bump)]
    pub tags: Account<'info, Tags>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

impl<'info> SetupTags<'_> {
    pub fn process(&mut self, bump: u8) -> Result<()> {
        let Self { spling, tags, .. } = self;

        tags.taglist.push(String::from("spling"));

        // Tags is a PDA, so here we store the bump
        tags.bump = bump;
        Ok(())
    }
}
