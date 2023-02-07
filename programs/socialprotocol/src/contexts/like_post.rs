use crate::*;

#[derive(Accounts)]
// use function arguments to determine which post to like
pub struct LikePost<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut, seeds = [b"spling"], bump = spling.bump)]
    pub spling: Account<'info, Spling>,
    #[account(seeds = [b"user_profile", user.key().as_ref()], bump = user_profile.bump, has_one = user)]
    pub user_profile: Account<'info, UserProfile>,
    // get a post account, no way to add constraint because shdw (hash) is unknown
    #[account()]
    pub post: Account<'info, Post>,
    #[account(mut, seeds = [b"likes".as_ref(), post.key().as_ref()], bump = likes.bump)]
    pub likes: Account<'info, Likes>,
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

impl<'info> LikePost<'_> {
    pub fn process(&mut self, amount: Option<u64>) -> Result<()> {
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
            likes,
            ..
        } = self;

        // check if user liked the post already
        if likes.users.contains(&user_profile.uid) {
            // retain all user id's except for the user id to be deleted
            likes.users.retain(|x| *x != user_profile.uid);
            // increment like counter lower
            likes.counter -= 1;
        } else {
            // add user id to vector
            likes.users.push(user_profile.uid);
            // increment like counter higher
            likes.counter += 1;
        }

        match amount {
            None => (),
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
