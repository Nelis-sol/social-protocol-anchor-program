use crate::*;
#[derive(Accounts)]
pub struct JoinGroup<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut, seeds = [b"spling"], bump = spling.bump)]
    pub spling: Account<'info, Spling>,
    // increase user profile account size, with 4 (u32) to accomodate adding the group id to the user's joined groups
    #[account(
        mut, 
        seeds = [b"user_profile", user.key().as_ref()],
        bump = user_profile.bump,
        has_one = user)]
    pub user_profile: Account<'info, UserProfile>,
    #[account(mut)]
    pub b: Account<'info, B>,
    #[account(mut)]
    /// CHECK: receiving account, not dangerous
    pub receiver: AccountInfo<'info>,
    #[account(mut)]
    pub sender_token_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub receiver_token_account: Box<Account<'info, TokenAccount>>,
    #[account()]
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

impl<'info> JoinGroup<'_> {
    pub fn process(&mut self,address:u32,amount:Option<u64>) -> Result<()> {
        let Self {spling,user_profile,user,b,receiver,sender_token_account,receiver_token_account,mint,token_program,..} = self;

        // add group id to vector
        user_profile.groups.push(address);

        match amount {
            None => (),
            Some(am) => {

                // transfer Spling tokens
                let cpi_context = CpiContext::new(
                    token_program.to_account_info(),
                    token::Transfer {
                        from: sender_token_account.clone().to_account_info(),
                        to: receiver_token_account.clone().to_account_info(),
                        authority: user.clone().to_account_info()
                    },
                );

                token::transfer(cpi_context, am)?;
                
                // transfer SOL tokens
                **b.to_account_info().try_borrow_mut_lamports()? -= am;
                **receiver.try_borrow_mut_lamports()? += am;
            }
        }
        Ok(())
    }

}
