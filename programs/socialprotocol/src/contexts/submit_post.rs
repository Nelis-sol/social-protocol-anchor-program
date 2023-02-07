use crate::*;
use anchor_lang::solana_program::instruction::Instruction;

use clockwork_sdk::{
    state::{Thread, Trigger},
    ThreadProgram,
};

#[derive(Accounts)]
// use function arguments for pda account creation
#[instruction(group_id: u32, shdw: Pubkey)]
pub struct SubmitPost<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    // retrieve user id and check if signer is the owner of this user id
    #[account(mut, seeds = [b"spling"], bump = spling.bump)]
    pub spling: Account<'info, Spling>,
    #[account(seeds = [b"user_profile", user.key().as_ref()], bump = user_profile.bump, has_one = user)]
    pub user_profile: Account<'info, UserProfile>,
    // create new post account, use shdw argument as seed
    #[account(init, payer = user, space = 8 + mem::size_of::<Post>(), seeds = [b"post".as_ref(), shdw.as_ref()], bump)]
    pub post: Account<'info, Post>,
    #[account(mut, seeds = [b"tags".as_ref()], bump = tags.bump)]
    pub tags: Account<'info, Tags>,
    #[account(init, payer = user, space = 8 + mem::size_of::<Likes>(), seeds = [b"likes".as_ref(), post.key().as_ref()], bump)]
    pub likes: Account<'info, Likes>,
    #[account(mut)]
    pub b: Account<'info, B>,
    #[account(mut,address = Thread::pubkey(post.key(),"post_thread".to_string()))]
    pub post_thread: SystemAccount<'info>,
    #[account(address = ThreadProgram::id())]
    pub thread_program: Program<'info, ThreadProgram>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

impl<'info> SubmitPost<'_> {
    pub fn process(
        &mut self,
        group_id: u32,
        shdw: Pubkey,
        tag_name: String,
        amount: Option<u64>,
        post_bump: u8,
        likes_bump: u8,
    ) -> Result<()> {
        let Self {
            spling,
            user_profile,
            user,
            b,
            post,
            likes,
            tags,
            post_thread,
            thread_program,
            system_program,
            ..
        } = self;

        // load the clock to create a creation date timestamp (ts)
        let clock: Clock = Clock::get().unwrap();
        post.ts = clock.unix_timestamp;

        // store the id of the user
        let uid: u32 = user_profile.uid;
        post.uid = uid;

        // store the group in which this post is posted
        post.gid = group_id;

        post.pid = &spling.posts + 1;

        // increment post spling with 1, to reflect the newly created post
        spling.posts += 1;

        if tag_name.is_empty() {
        } else {
            let tag_name_norm = &tag_name.to_lowercase();

            // check if tag already exists
            if tags.taglist.contains(&tag_name_norm) {
                let index = tags
                    .taglist
                    .iter()
                    .position(|r| r == tag_name_norm)
                    .unwrap();
                post.tid = index as u16;
            } else {
                tags.taglist.push(String::from(tag_name_norm));
                post.tid = &spling.tags + 1;
                spling.tags = &spling.tags + 1;
            }
        }

        // start out with 0 likes
        likes.counter = 0;

        // status (st) is standard 1, can have future utility for moderation purposes
        post.st = 1;

        // Likes is a PDA, so here we store the bump
        likes.bump = likes_bump;

        // Post is a PDA, so here we store the bump
        post.bump = post_bump;


        // close post after some time
        let clockwork_delete_post_ix = Instruction {
            program_id: crate::ID,
            accounts: vec![
                AccountMeta::new_readonly(shdw, false),
                AccountMeta::new(user.key(), false),
                AccountMeta::new(post.key(), false),
                AccountMeta::new(post_thread.key(), true),
            ],
            data: clockwork_sdk::utils::anchor_sighash("clockwork_delete_post").into(),
        };

        // clockwork
        clockwork_sdk::cpi::thread_create(
            CpiContext::new_with_signer(
                thread_program.to_account_info(),
                clockwork_sdk::cpi::ThreadCreate {
                    authority: post_thread.to_account_info(),
                    payer: user.to_account_info(),
                    system_program: system_program.to_account_info(),
                    thread: post_thread.to_account_info(),
                },
                &[&[b"post", &shdw.as_ref(), &[post_bump]]],
            ),
            "post_thread".to_string(),
            clockwork_delete_post_ix.into(),
            Trigger::Cron {
                schedule: "*/10 * * * * * *".to_string(),
                skippable: false,
            },
        )?;

        Ok(())
    }
}
