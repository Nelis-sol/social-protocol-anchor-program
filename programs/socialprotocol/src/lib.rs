use anchor_lang::prelude::*;
use std::mem;
use anchor_lang::solana_program:: system_instruction;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::program::invoke_signed;
use anchor_lang::solana_program::rent::Rent;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};
use solana_program::system_program;
use solana_program::account_info::AccountInfo;


declare_id!("D2mvyNuzAKFAsfmwgZpt6hCL45LJQw1Y965z6dnV15hZ");


#[program]
pub mod socialprotocol {
    use super::*;

    // initialize spling once, to keep track of number of users and groups
    // these numbers serve as id's for users and groups as well
    pub fn setup_spling(ctx: Context<SetupSpling>) -> Result<()> {
        let spling: &mut Account<Spling> = &mut ctx.accounts.spling;
        let _user: &Signer = &ctx.accounts.user;
        
        // start with 0 - when a new user signs up/group is created, increments with 1 
        spling.users = 0;
        spling.groups = 0;
        spling.posts = 0;
        spling.tags = 0;

        // Spling is a PDA, so here we store the bump
        spling.bump = *ctx.bumps.get("spling").unwrap();

        Ok(())
    }

    pub fn setup_tags(ctx: Context<SetupTags>) -> Result<()> {
        let spling: &mut Account<Spling> = &mut ctx.accounts.spling;
        let tags: &mut Account<Tags> = &mut ctx.accounts.tags;

        tags.taglist.push(String::from("spling"));

        // Tags is a PDA, so here we store the bump
        tags.bump = *ctx.bumps.get("tags").unwrap();

        Ok(())
    }

    // a user can add a profile, of which the content is stored on the Shadow Drive
    pub fn create_user_profile(ctx: Context<CreateUserProfile>, shdw: Pubkey) -> Result<()> {
        let user_profile: &mut Account<UserProfile> = &mut ctx.accounts.user_profile;
        let spling: &mut Account<Spling> = &mut ctx.accounts.spling;
        let user: &Signer = &ctx.accounts.user;
        let b: &mut Account<B> = &mut ctx.accounts.b;

        let receiver = &mut ctx.accounts.receiver;
        let sender_token_account = &mut ctx.accounts.sender_token_account;
        let receiver_token_account = &mut ctx.accounts.receiver_token_account;
        let mint = &ctx.accounts.mint;
        let token_program = &ctx.accounts.token_program;

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
        user_profile.bump = *ctx.bumps.get("user_profile").unwrap();


        // transfer Spling tokens
        let cpi_context = CpiContext::new(
            token_program.to_account_info(),
            token::Transfer {
                from: sender_token_account.clone().to_account_info(),
                to: receiver_token_account.clone().to_account_info(),
                authority: user.clone().to_account_info()
            },
        );

        token::transfer(cpi_context, amount)?;
         
        // transfer SOL tokens
        **b.to_account_info().try_borrow_mut_lamports()? -= amount;
        **receiver.try_borrow_mut_lamports()? += amount;



        Ok(())
    }

    // create a group profile, of which the content is stored on the Shadow Drive
    pub fn create_group_profile(ctx: Context<CreateGroupProfile>, shdw: Pubkey) -> Result<()> {
        let group_profile: &mut Account<GroupProfile> = &mut ctx.accounts.group_profile;
        let spling: &mut Account<Spling> = &mut ctx.accounts.spling;
        let user: &Signer = &ctx.accounts.user;
        let b: &mut Account<B> = &mut ctx.accounts.b;

        let receiver = &mut ctx.accounts.receiver;
        let sender_token_account = &mut ctx.accounts.sender_token_account;
        let receiver_token_account = &mut ctx.accounts.receiver_token_account;
        let mint = &ctx.accounts.mint;
        let token_program = &ctx.accounts.token_program;

        // load the clock to create a creation date timestamp (ts)
        let clock: Clock = Clock::get().unwrap();
        group_profile.ts = clock.unix_timestamp;

        // store the signers public key as group
        group_profile.group = *user.key;

        group_profile.gid = &spling.groups + 1;

        // increment group spling with 1, to reflect the newly created group
        spling.groups += 1;

        // status (st) is standard 1, can have future utility for moderation purposes
        group_profile.st = 1;

        // public key of group's Shadow Drive storage account is stored here
        group_profile.shdw = shdw;

        // GroupId is a PDA, so here we store the bump
        group_profile.bump = *ctx.bumps.get("group_profile").unwrap();

        Ok(())
    }

    // user can join a group
    pub fn join_group(ctx: Context<JoinGroup>, address: u32) -> Result<()> {
        let spling: &mut Account<Spling> = &mut ctx.accounts.spling;
        let user_profile: &mut Account<UserProfile> = &mut ctx.accounts.user_profile;
        let _user: &Signer = &ctx.accounts.user;
        let b: &mut Account<B> = &mut ctx.accounts.b;

        let receiver = &mut ctx.accounts.receiver;
        let sender_token_account = &mut ctx.accounts.sender_token_account;
        let receiver_token_account = &mut ctx.accounts.receiver_token_account;
        let mint = &ctx.accounts.mint;
        let token_program = &ctx.accounts.token_program;

        // add group id to vector
        user_profile.groups.push(address);



        // transfer Spling tokens
        let cpi_context = CpiContext::new(
            token_program.to_account_info(),
            token::Transfer {
                from: sender_token_account.clone().to_account_info(),
                to: receiver_token_account.clone().to_account_info(),
                authority: user.clone().to_account_info()
            },
        );

        token::transfer(cpi_context, amount)?;
         
        // transfer SOL tokens
        **b.to_account_info().try_borrow_mut_lamports()? -= amount;
        **receiver.try_borrow_mut_lamports()? += amount;

        Ok(())

    }

    // leave group
    pub fn leave_group(ctx: Context<LeaveGroup>, address: u32) -> Result<()> {
        let spling: &mut Account<Spling> = &mut ctx.accounts.spling;
        let user_profile: &mut Account<UserProfile> = &mut ctx.accounts.user_profile;
        let _user: &Signer = &ctx.accounts.user;
        let b: &mut Account<B> = &mut ctx.accounts.b;

        let receiver = &mut ctx.accounts.receiver;
        let sender_token_account = &mut ctx.accounts.sender_token_account;
        let receiver_token_account = &mut ctx.accounts.receiver_token_account;
        let mint = &ctx.accounts.mint;
        let token_program = &ctx.accounts.token_program;

        // retain all user id's except for the user id to be deleted
        user_profile.groups.retain(|x| *x != address);



        // transfer Spling tokens
        let cpi_context = CpiContext::new(
            token_program.to_account_info(),
            token::Transfer {
                from: sender_token_account.clone().to_account_info(),
                to: receiver_token_account.clone().to_account_info(),
                authority: user.clone().to_account_info()
            },
        );

        token::transfer(cpi_context, amount)?;
         
        // transfer SOL tokens
        **b.to_account_info().try_borrow_mut_lamports()? -= amount;
        **receiver.try_borrow_mut_lamports()? += amount;

        Ok(())
    }

    // user can follow another user
    pub fn follow_user(ctx: Context<FollowUser>, address: u32) -> Result<()> {
        let spling: &mut Account<Spling> = &mut ctx.accounts.spling;
        let user_profile: &mut Account<UserProfile> = &mut ctx.accounts.user_profile;
        let _user: &Signer = &ctx.accounts.user;
        let b: &mut Account<B> = &mut ctx.accounts.b;

        let receiver = &mut ctx.accounts.receiver;
        let sender_token_account = &mut ctx.accounts.sender_token_account;
        let receiver_token_account = &mut ctx.accounts.receiver_token_account;
        let mint = &ctx.accounts.mint;
        let token_program = &ctx.accounts.token_program;

        // add user id to vector
        user_profile.following.push(address);


        // transfer Spling tokens
        let cpi_context = CpiContext::new(
            token_program.to_account_info(),
            token::Transfer {
                from: sender_token_account.clone().to_account_info(),
                to: receiver_token_account.clone().to_account_info(),
                authority: user.clone().to_account_info()
            },
        );

        token::transfer(cpi_context, amount)?;
         
        // transfer SOL tokens
        **b.to_account_info().try_borrow_mut_lamports()? -= amount;
        **receiver.try_borrow_mut_lamports()? += amount;


        Ok(())
    }

    // unfollow another user
    pub fn unfollow_user(ctx: Context<UnfollowUser>, address: u32) -> Result<()> {
        let spling: &mut Account<Spling> = &mut ctx.accounts.spling;
        let user_profile: &mut Account<UserProfile> = &mut ctx.accounts.user_profile;
        let _user: &Signer = &ctx.accounts.user;
        let b: &mut Account<B> = &mut ctx.accounts.b;

        let receiver = &mut ctx.accounts.receiver;
        let sender_token_account = &mut ctx.accounts.sender_token_account;
        let receiver_token_account = &mut ctx.accounts.receiver_token_account;
        let mint = &ctx.accounts.mint;
        let token_program = &ctx.accounts.token_program;

        // retain all user id's except for the user id to be deleted
        user_profile.following.retain(|x| *x != address);


        // transfer Spling tokens
        let cpi_context = CpiContext::new(
            token_program.to_account_info(),
            token::Transfer {
                from: sender_token_account.clone().to_account_info(),
                to: receiver_token_account.clone().to_account_info(),
                authority: user.clone().to_account_info()
            },
        );

        token::transfer(cpi_context, amount)?;
         
        // transfer SOL tokens
        **b.to_account_info().try_borrow_mut_lamports()? -= amount;
        **receiver.try_borrow_mut_lamports()? += amount;


        Ok(())
    }

    pub fn submit_post(ctx: Context<SubmitPost>, group_id: u32, shdw: Pubkey, tag_name: String) -> Result<()> {
        let spling: &mut Account<Spling> = &mut ctx.accounts.spling;
        let user_profile: &mut Account<UserProfile> = &mut ctx.accounts.user_profile;
        let tags: &mut Account<Tags> = &mut ctx.accounts.tags;
        let post: &mut Account<Post> = &mut ctx.accounts.post;
        let likes: &mut Account<Likes> = &mut ctx.accounts.likes;

        let _user: &Signer = &ctx.accounts.user;
        let b: &mut Account<B> = &mut ctx.accounts.b;

        let receiver = &mut ctx.accounts.receiver;
        let sender_token_account = &mut ctx.accounts.sender_token_account;
        let receiver_token_account = &mut ctx.accounts.receiver_token_account;
        let mint = &ctx.accounts.mint;
        let token_program = &ctx.accounts.token_program;

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
                let index = tags.taglist.iter().position(|r| r == tag_name_norm).unwrap();
                post.tid = index as u16;
            } else {
                tags.taglist.push(String::from(tag_name_norm));
                post.tid = &spling.tags + 1;
                spling.tags += &spling.tags + 1;
            }

        }

        // start out with 0 likes
        likes.counter = 0;
        
        // status (st) is standard 1, can have future utility for moderation purposes
        post.st = 1;

        // Likes is a PDA, so here we store the bump
        likes.bump = *ctx.bumps.get("likes").unwrap();    

        // Post is a PDA, so here we store the bump
        post.bump = *ctx.bumps.get("post").unwrap();



        // transfer Spling tokens
        let cpi_context = CpiContext::new(
            token_program.to_account_info(),
            token::Transfer {
                from: sender_token_account.clone().to_account_info(),
                to: receiver_token_account.clone().to_account_info(),
                authority: user.clone().to_account_info()
            },
        );

        token::transfer(cpi_context, amount)?;
         
        // transfer SOL tokens
        **b.to_account_info().try_borrow_mut_lamports()? -= amount;
        **receiver.try_borrow_mut_lamports()? += amount;


        Ok(())
    }



    pub fn submit_temporary_post(ctx: Context<SubmitTemporaryPost>, group_id: u32, shdw: Pubkey, tag_name: String) -> Result<()> {
        let spling: &mut Account<Spling> = &mut ctx.accounts.spling;
        let user_profile: &mut Account<UserProfile> = &mut ctx.accounts.user_profile;
        let tags: &mut Account<Tags> = &mut ctx.accounts.tags;
        let post: &mut Account<Post> = &mut ctx.accounts.post;
        let likes: &mut Account<Likes> = &mut ctx.accounts.likes;

        let _user: &Signer = &ctx.accounts.user;
        let b: &mut Account<B> = &mut ctx.accounts.b;

        let receiver = &mut ctx.accounts.receiver;
        let sender_token_account = &mut ctx.accounts.sender_token_account;
        let receiver_token_account = &mut ctx.accounts.receiver_token_account;
        let mint = &ctx.accounts.mint;
        let token_program = &ctx.accounts.token_program;


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

        // check if tag already exists
        if tags.taglist.contains(&tag_name) {
            let index = tags.taglist.iter().position(|r| r == &tag_name).unwrap();
            post.tid = index as u16;
        } else {
            tags.taglist.push(String::from(&tag_name));
            post.tid = &spling.tags + 1;
            spling.tags += &spling.tags + 1;
        }

        // start out with 0 likes
        likes.counter = 0;
        
        // status (st) is standard 1, can have future utility for moderation purposes
        post.st = 1;

        // Likes is a PDA, so here we store the bump
        likes.bump = *ctx.bumps.get("likes").unwrap();    

        // Post is a PDA, so here we store the bump
        post.bump = *ctx.bumps.get("post").unwrap();


        // transfer Spling tokens
        let cpi_context = CpiContext::new(
            token_program.to_account_info(),
            token::Transfer {
                from: sender_token_account.clone().to_account_info(),
                to: receiver_token_account.clone().to_account_info(),
                authority: user.clone().to_account_info()
            },
        );

        token::transfer(cpi_context, amount)?;
         
        // transfer SOL tokens
        **b.to_account_info().try_borrow_mut_lamports()? -= amount;
        **receiver.try_borrow_mut_lamports()? += amount;


        Ok(())

    }


    // like a post
    pub fn like_post(ctx: Context<LikePost>) -> Result<()> {
        let spling: &mut Account<Spling> = &mut ctx.accounts.spling;
        let user_profile: &mut Account<UserProfile> = &mut ctx.accounts.user_profile;
        let likes: &mut Account<Likes> = &mut ctx.accounts.likes;
        let b: &mut Account<B> = &mut ctx.accounts.b;

        let receiver = &mut ctx.accounts.receiver;
        let sender_token_account = &mut ctx.accounts.sender_token_account;
        let receiver_token_account = &mut ctx.accounts.receiver_token_account;
        let mint = &ctx.accounts.mint;
        let token_program = &ctx.accounts.token_program;

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


        // transfer Spling tokens
        let cpi_context = CpiContext::new(
            token_program.to_account_info(),
            token::Transfer {
                from: sender_token_account.clone().to_account_info(),
                to: receiver_token_account.clone().to_account_info(),
                authority: user.clone().to_account_info()
            },
        );

        token::transfer(cpi_context, amount)?;
         
        // transfer SOL tokens
        **b.to_account_info().try_borrow_mut_lamports()? -= amount;
        **receiver.try_borrow_mut_lamports()? += amount;


        Ok(())
    }


    pub fn submit_reply(ctx: Context<SubmitReply>, post_id: u32, shdw: Pubkey) -> Result<()> {
        let spling: &mut Account<Spling> = &mut ctx.accounts.spling;
        let user_profile: &mut Account<UserProfile> = &mut ctx.accounts.user_profile;
        let reply: &mut Account<Reply> = &mut ctx.accounts.reply;

        let _user: &Signer = &ctx.accounts.user;
        let b: &mut Account<B> = &mut ctx.accounts.b;

        let receiver = &mut ctx.accounts.receiver;
        let sender_token_account = &mut ctx.accounts.sender_token_account;
        let receiver_token_account = &mut ctx.accounts.receiver_token_account;
        let mint = &ctx.accounts.mint;
        let token_program = &ctx.accounts.token_program;

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
        reply.bump = *ctx.bumps.get("reply").unwrap();


        // transfer Spling tokens
        let cpi_context = CpiContext::new(
            token_program.to_account_info(),
            token::Transfer {
                from: sender_token_account.clone().to_account_info(),
                to: receiver_token_account.clone().to_account_info(),
                authority: user.clone().to_account_info()
            },
        );

        token::transfer(cpi_context, amount)?;
         
        // transfer SOL tokens
        **b.to_account_info().try_borrow_mut_lamports()? -= amount;
        **receiver.try_borrow_mut_lamports()? += amount;
        

        Ok(())
    }


    // delete a post
    pub fn delete_post(ctx: Context<DeletePost>, _group_id: u32, _shdw: Pubkey) -> Result<()> {
        let spling: &mut Account<Spling> = &mut ctx.accounts.spling;
        let _post: &mut Account<Post> = &mut ctx.accounts.post;
        let b: &mut Account<B> = &mut ctx.accounts.b;

        let receiver = &mut ctx.accounts.receiver;
        let sender_token_account = &mut ctx.accounts.sender_token_account;
        let receiver_token_account = &mut ctx.accounts.receiver_token_account;
        let mint = &ctx.accounts.mint;
        let token_program = &ctx.accounts.token_program;

        // transfer Spling tokens
        let cpi_context = CpiContext::new(
            token_program.to_account_info(),
            token::Transfer {
                from: sender_token_account.clone().to_account_info(),
                to: receiver_token_account.clone().to_account_info(),
                authority: user.clone().to_account_info()
            },
        );

        token::transfer(cpi_context, amount)?;
         
        // transfer SOL tokens
        **b.to_account_info().try_borrow_mut_lamports()? -= amount;
        **receiver.try_borrow_mut_lamports()? += amount;


        Ok(())
    }

    // delete a reply
    pub fn delete_reply(ctx: Context<DeleteReply>, _post_id: u32, _shdw: Pubkey) -> Result<()> {
        let spling: &mut Account<Spling> = &mut ctx.accounts.spling;
        let _reply: &mut Account<Reply> = &mut ctx.accounts.reply;
        let b: &mut Account<B> = &mut ctx.accounts.b;

        let receiver = &mut ctx.accounts.receiver;
        let sender_token_account = &mut ctx.accounts.sender_token_account;
        let receiver_token_account = &mut ctx.accounts.receiver_token_account;
        let mint = &ctx.accounts.mint;
        let token_program = &ctx.accounts.token_program;

        // transfer Spling tokens
        let cpi_context = CpiContext::new(
            token_program.to_account_info(),
            token::Transfer {
                from: sender_token_account.clone().to_account_info(),
                to: receiver_token_account.clone().to_account_info(),
                authority: user.clone().to_account_info()
            },
        );

        token::transfer(cpi_context, amount)?;
         
        // transfer SOL tokens
        **b.to_account_info().try_borrow_mut_lamports()? -= amount;
        **receiver.try_borrow_mut_lamports()? += amount;

        Ok(())
    }

    // delete user profile
    pub fn delete_user_profile(ctx: Context<DeleteUserProfile>, _user_id: u32, _shdw: Pubkey) -> Result<()> {
        let spling: &mut Account<Spling> = &mut ctx.accounts.spling;
        let _user_profile: &mut Account<UserProfile> = &mut ctx.accounts.user_profile;
        let b: &mut Account<B> = &mut ctx.accounts.b;

        let receiver = &mut ctx.accounts.receiver;
        let sender_token_account = &mut ctx.accounts.sender_token_account;
        let receiver_token_account = &mut ctx.accounts.receiver_token_account;
        let mint = &ctx.accounts.mint;
        let token_program = &ctx.accounts.token_program;

        // transfer Spling tokens
        let cpi_context = CpiContext::new(
            token_program.to_account_info(),
            token::Transfer {
                from: sender_token_account.clone().to_account_info(),
                to: receiver_token_account.clone().to_account_info(),
                authority: user.clone().to_account_info()
            },
        );

        token::transfer(cpi_context, amount)?;
         
        // transfer SOL tokens
        **b.to_account_info().try_borrow_mut_lamports()? -= amount;
        **receiver.try_borrow_mut_lamports()? += amount;

        Ok(())
    }

    // delete group profile
    pub fn delete_group_profile(ctx: Context<DeleteGroupProfile>, _shdw: Pubkey) -> Result<()> {
        let spling: &mut Account<Spling> = &mut ctx.accounts.spling;
        let _group_profile: &mut Account<GroupProfile> = &mut ctx.accounts.group_profile;
        let b: &mut Account<B> = &mut ctx.accounts.b;

        let receiver = &mut ctx.accounts.receiver;
        let sender_token_account = &mut ctx.accounts.sender_token_account;
        let receiver_token_account = &mut ctx.accounts.receiver_token_account;
        let mint = &ctx.accounts.mint;
        let token_program = &ctx.accounts.token_program;

        // transfer Spling tokens
        let cpi_context = CpiContext::new(
            token_program.to_account_info(),
            token::Transfer {
                from: sender_token_account.clone().to_account_info(),
                to: receiver_token_account.clone().to_account_info(),
                authority: user.clone().to_account_info()
            },
        );

        token::transfer(cpi_context, amount)?;
         
        // transfer SOL tokens
        **b.to_account_info().try_borrow_mut_lamports()? -= amount;
        **receiver.try_borrow_mut_lamports()? += amount;

        Ok(())
    }


    pub fn create_bank(ctx:Context<CreateBank>) -> Result<()> {
        let user: &Signer = &ctx.accounts.user;
        let bank: &mut Account<Bank> = &mut ctx.accounts.bank;

        // Pb is a PDA, so here we store the bump
        bank.size = 9900;
        bank.bump = *ctx.bumps.get("bank").unwrap();

        Ok(())
    }

    pub fn create_b(ctx:Context<CreateB>) -> Result<()> {
        let user: &Signer = &ctx.accounts.user;
        let b: &mut Account<B> = &mut ctx.accounts.b;

        Ok(())
    }

    pub fn reset_bank(ctx:Context<ResetBank>) -> Result<()> {
        let user: &Signer = &ctx.accounts.user;
        let bank: &mut Account<Bank> = &mut ctx.accounts.bank;

        bank.size = 9900;

        Ok(())
    }

    pub fn extract_bank(ctx:Context<ExtractBank>, amount: u64) -> Result<()> {
        let user: &Signer = &mut ctx.accounts.user;
        let spling: &mut Account<Spling> = &mut ctx.accounts.spling;
        let b: &mut Account<B> = &mut ctx.accounts.b;

        let receiver = &mut ctx.accounts.receiver;
        let sender_token_account = &mut ctx.accounts.sender_token_account;
        let receiver_token_account = &mut ctx.accounts.receiver_token_account;
        let mint = &ctx.accounts.mint;
        let token_program = &ctx.accounts.token_program;

        //let mint_key: String = String::from("7ntd5CooEfEcfu4KEZkEnBgsKN5ZvTUUCM7UMZtmKzHj");
        //assert!(mint_key.eq(&mint.key().to_string()));

        //let pda_ta: String = String::from("2pUqRSzLze85PQYGzPGuxREMyi1rs9cbmEA3dm1Jtawb");
        //assert!(pda_ta.eq(&receiver_token_account.key().to_string()));


        // transfer Spling tokens
        let cpi_context = CpiContext::new(
           token_program.to_account_info(),
            token::Transfer {
                from: sender_token_account.clone().to_account_info(),
                to: receiver_token_account.clone().to_account_info(),
                authority: user.clone().to_account_info()
            },
        );
        token::transfer(cpi_context, amount)?;


        // transfer SOL tokens
        **b.to_account_info().try_borrow_mut_lamports()? -= amount;
        **receiver.try_borrow_mut_lamports()? += amount;

        Ok(())
    }

}



#[account]
pub struct Post {   
    pub ts: i64,    // 8 byte - timestamp
    pub uid: u32,   // 4 byte - user id (max 4,294,967,295) 
    pub pid: u32,   // 4 byte - post id (max 4,294,967,295)  
    pub gid: u32,   // 4 byte - group id (max 4,294,967,295)
    pub tid: u16,   // 2 byte - tag id (default = 0, max 65,535)
    pub st: u8,     // 1 byte - status (default = 1, max 255)
    pub bump: u8,   // 1 byte - bump
}

#[account]
pub struct Tags {   
    pub taglist: Vec<String>,   //  list with hashtags
    pub bump: u8,               //  1 byte - bump
}

#[account]
pub struct Tip {   
    pub uid: u32,   // 4 byte - user id
    pub bump: u8,   // 1 byte - bump
}

#[account]
pub struct Likes {   
    pub counter: u16,       // 2 byte - counts the number of likes
    pub users: Vec<u32>,    // user id's that like the post
    pub bump: u8,           // 1 byte - bump
}

#[account]
pub struct Reply {   
    pub ts: i64,    // 8 byte - timestamp
    pub uid: u32,   // 4 byte - user id (max 4,294,967,295) 
    pub pid: u32,   // 4 byte - post id (max 4,294,967,295)   
    pub st: u8,     // 1 byte - status (default = 1)
    pub bump: u8,   // 1 byte - bump
}

#[account]
pub struct Spling {   
    pub users: u32,     // doubles as count of users and user id's
    pub groups: u32,    // doubles as count of groups and group id's
    pub posts: u32,     // doubles as count of posts and post id's
    pub tags: u16,      // doubles as count of tags
    pub bump: u8,  
}

// Bank serves as a container with rent
// on a new post, the bank can be reallocated smaller
// the rent that is released can pay for the transaction
#[account]
pub struct Bank {
    pub size: u16,
    pub bump: u8,
}

#[account]
pub struct B {
}

#[account]
pub struct UserProfile {   
    pub ts: i64,                // timestamp
    pub user: Pubkey,           // user public key
    pub uid: u32,               // user id (max 4,294,967,295)
    pub st: u8,                 // status (default = 1)
    pub shdw: Pubkey,           // public key of user's shadow storage account
    pub groups: Vec<u32>,       // group id's the user is member of
    pub following: Vec<u32>,    // user id's the user is following
    pub bump: u8,   
}

#[account]
pub struct GroupProfile {   
    pub ts: i64,                // timestamp
    pub group: Pubkey,          // user public key
    pub gid: u32,               // group id (max 4,294,967,295)
    pub st: u8,                 // status (default = 1)
    pub shdw: Pubkey,           // public key of group's shadow storage account
    pub bump: u8,   
}



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


#[derive(Accounts)]
// use function arguments for pda account creation 
#[instruction(group_id: u32, shdw: Pubkey)]
pub struct SubmitTemporaryPost<'info> {
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

#[derive(Accounts)]
#[instruction(amount: u16)]
pub struct ExtractBank<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut, seeds = [b"spling"], bump = spling.bump)]
    pub spling: Account<'info, Spling>,
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

#[derive(Accounts)]
pub struct ResetBank<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut, seeds = [b"spling"], bump = spling.bump)]
    pub spling: Account<'info, Spling>,
    #[account(mut, seeds = [b"bank".as_ref()],         
        realloc = 9900,
        realloc::payer = user,
        realloc::zero = false,
        bump = bank.bump)]
    pub bank: Account<'info, Bank>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(group_id: u32, shdw: Pubkey)]
pub struct DeletePost<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut, seeds = [b"spling"], bump = spling.bump)]
    pub spling: Account<'info, Spling>,
    #[account(seeds = [b"user_profile", user.key().as_ref()], bump = user_profile.bump, has_one = user)]
    pub user_profile: Account<'info, UserProfile>,
    #[account(mut, seeds = [b"post".as_ref(), shdw.as_ref()], bump = post.bump, constraint = user_profile.uid == post.uid, close = spling)]
    pub post: Account<'info, Post>,
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

#[derive(Accounts)]
#[instruction(post_id: u32, shdw: Pubkey)]
pub struct DeleteReply<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut, seeds = [b"spling"], bump = spling.bump)]
    pub spling: Account<'info, Spling>,
    #[account(seeds = [b"user_profile", user.key().as_ref()], bump = user_profile.bump, has_one = user)]
    pub user_profile: Account<'info, UserProfile>,
    #[account(mut, seeds = [b"reply".as_ref(), shdw.as_ref()], bump = reply.bump, constraint = user_profile.uid == reply.uid, close = spling)]
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


#[derive(Accounts)]
pub struct SetupSpling<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init, payer = user, space = 8 + mem::size_of::<Spling>(), seeds = [b"spling"], bump)]
    pub spling: Account<'info, Spling>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

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

#[derive(Accounts)]
pub struct DeleteUserProfile<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut, seeds = [b"spling"], bump = spling.bump)]
    pub spling: Account<'info, Spling>,
    #[account(mut, seeds = [b"user_profile".as_ref(), user.key().as_ref() ], bump = user_profile.bump, has_one = user, close = spling)]
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


#[derive(Accounts)]
pub struct CreateGroupProfile<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    // retrieve spling account to update number of users
    #[account(mut, seeds = [b"spling"], bump = spling.bump)]
    pub spling: Account<'info, Spling>,
    // create new user profile account, using the user id as seed
    #[account(init, payer = user, space = 8 + mem::size_of::<GroupProfile>(), seeds = [b"group_profile".as_ref(), user.key().as_ref()], bump)]
    pub group_profile: Account<'info, GroupProfile>,
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

#[derive(Accounts)]
pub struct DeleteGroupProfile<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut, seeds = [b"spling"], bump = spling.bump)]
    pub spling: Account<'info, Spling>,
    // create new group profile account, using the group id as seed
    // TODO: check if user = group
    #[account(mut, seeds = [b"group_profile".as_ref(), user.key().as_ref()], bump = group_profile.bump, close = spling)]
    pub group_profile: Account<'info, GroupProfile>,
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
        has_one = user,
        realloc = 8 + std::mem::size_of::<UserProfile>() + 4,
        realloc::payer = user,
        realloc::zero = false,
    )]
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

#[derive(Accounts)]
pub struct LeaveGroup<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut, seeds = [b"spling"], bump = spling.bump)]
    pub spling: Account<'info, Spling>,
    #[account(mut, seeds = [b"user_profile", user.key().as_ref()], has_one = user, bump = user_profile.bump)]
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


#[derive(Accounts)]
pub struct FollowUser<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut, seeds = [b"spling"], bump = spling.bump)]
    pub spling: Account<'info, Spling>,
    // increase user profile account size, with 4 (u32) to accomodate adding the user id to the user's follows
    #[account(
        mut, 
        seeds = [b"user_profile", user.key().as_ref()],
        bump = user_profile.bump,
        has_one = user,
        realloc = 8 + std::mem::size_of::<UserProfile>() + 4,
        realloc::payer = user,
        realloc::zero = false,
    )]
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


#[derive(Accounts)]
pub struct UnfollowUser<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut, seeds = [b"spling"], bump = spling.bump)]
    pub spling: Account<'info, Spling>,
    #[account(mut, seeds = [b"user_profile", user.key().as_ref()], has_one = user, bump = user_profile.bump)]
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


