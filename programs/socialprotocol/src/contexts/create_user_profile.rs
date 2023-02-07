use crate::*;

#[derive(Accounts)]
pub struct CreateUserProfile<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    // retrieve spling account to update number of users
    #[account(mut, seeds = [b"spling"], bump = spling.bump)]
    pub spling: Account<'info, Spling>,
    // create new user profile account, using the user id as seed
    #[account(init, payer = user, space = 8 + mem::size_of::<UserProfile>(), seeds = [b"user_profile".as_ref(), user.key().as_ref()], bump)]
    pub user_profile: Account<'info, UserProfile>,
    #[account(mut)]
    pub b: Account<'info, B>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

impl<'info> CreateUserProfile<'_> {
    pub fn process(&mut self, shdw: Pubkey, amount: Option<u64>, bump: u8) -> Result<()> {
        let Self {
            user_profile,
            spling,
            user,
            b,
            ..
        } = self;

        // load the clock to create a creation date timestamp (ts)
        let clock: Clock = Clock::get().unwrap();
        user_profile.ts = clock.unix_timestamp;

        // store the signers public key as user
        user_profile.user = *user.key;

        // take the uid from the UserId PDA and store it in this UserProfilePDA
        user_profile.uid = &spling.users + 1;
        spling.users += 1;

        // status (st) is standard 1, can have future utility for moderation purposes
        user_profile.st = 1;

        // public key of user's Shadow Drive storage account is stored here
        user_profile.shdw = shdw;

        // UserProfile is a PDA, so here we store the bump
        user_profile.bump = bump;


        Ok(())
    }
}
