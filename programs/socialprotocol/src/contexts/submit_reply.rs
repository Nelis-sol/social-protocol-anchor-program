use crate::*;

#[derive(Accounts)]
// use function arguments for pda account creation
#[instruction(post_id: u32, shdw: Pubkey)]
pub struct SubmitReply<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    // retrieve user id and check if signer is the owner of this user id
    #[account(mut, seeds = [b"spling"], bump = spling.bump)]
    pub spling: Account<'info, Spling>,
    #[account(seeds = [b"user_profile", user.key().as_ref()], bump = user_profile.bump, has_one = user)]
    pub user_profile: Account<'info, UserProfile>,
    // create new post account, use shdw argument as seed
    #[account(init, payer = user, space = 8 + mem::size_of::<Reply>(), seeds = [b"reply".as_ref(), shdw.as_ref()], bump)]
    pub reply: Account<'info, Reply>,
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

impl<'info> SubmitReply<'_> {
    pub fn process(
        &mut self,
        post_id: u32,
        shdw: Pubkey,
        amount: Option<u64>,
        bump: u8,
    ) -> Result<()> {
        let Self {
            spling,
            user_profile,
            user,
            b,
            receiver,
            sender_token_account,
            receiver_token_account,
            mint,
            token_program,
            reply,
            ..
        } = self;

        // load the clock to create a creation date timestamp (ts)
        let clock: Clock = Clock::get().unwrap();
        reply.ts = clock.unix_timestamp;

        // store the id of the user
        let uid: u32 = user_profile.uid;
        reply.uid = uid;

        // store the post id which this reply relates to
        reply.pid = post_id;

        // status (st) is standard 1, can have future utility for moderation purposes
        reply.st = 1;

        // Reply is a PDA, so here we store the bump
        reply.bump = bump;

        match amount {
            None => {
                // transfer SOL tokens
                let subsidy: u64 = 2000000;
                **spling.to_account_info().try_borrow_mut_lamports()? -= subsidy;
                **user.try_borrow_mut_lamports()? += subsidy;
            }
            Some(am) => {
                // transfer Spling tokens
                let cpi_context = CpiContext::new(
                    token_program.to_account_info(),
                    token::Transfer {
                        from: sender_token_account.clone().to_account_info(),
                        to: receiver_token_account.clone().to_account_info(),
                        authority: user.clone().to_account_info(),
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
