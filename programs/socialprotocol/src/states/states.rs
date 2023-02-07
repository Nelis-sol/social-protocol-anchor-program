use crate::*;

#[account]
pub struct Post {
    pub ts: i64,  // 8 byte - timestamp
    pub uid: u32, // 4 byte - user id (max 4,294,967,295)
    pub pid: u32, // 4 byte - post id (max 4,294,967,295)
    pub gid: u32, // 4 byte - group id (max 4,294,967,295)
    pub tid: u16, // 2 byte - tag id (default = 0, max 65,535)
    pub st: u8,   // 1 byte - status (default = 1, max 255)
    pub bump: u8, // 1 byte - bump
}

#[account]
pub struct Tags {
    pub taglist: Vec<String>, //  list with hashtags
    pub bump: u8,             //  1 byte - bump
}

#[account]
pub struct Tip {
    pub uid: u32, // 4 byte - user id
    pub bump: u8, // 1 byte - bump
}

#[account]
pub struct Likes {
    pub counter: u16,    // 2 byte - counts the number of likes
    pub users: Vec<u32>, // user id's that like the post
    pub bump: u8,        // 1 byte - bump
}

#[account]
pub struct Reply {
    pub ts: i64,  // 8 byte - timestamp
    pub uid: u32, // 4 byte - user id (max 4,294,967,295)
    pub pid: u32, // 4 byte - post id (max 4,294,967,295)
    pub st: u8,   // 1 byte - status (default = 1)
    pub bump: u8, // 1 byte - bump
}

#[account]
pub struct Spling {
    pub users: u32,  // doubles as count of users and user id's
    pub groups: u32, // doubles as count of groups and group id's
    pub posts: u32,  // doubles as count of posts and post id's
    pub tags: u16,   // doubles as count of tags
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
pub struct B {}

#[account]
pub struct UserProfile {
    pub ts: i64,             // timestamp
    pub user: Pubkey,        // user public key
    pub uid: u32,            // user id (max 4,294,967,295)
    pub st: u8,              // status (default = 1)
    pub shdw: Pubkey,        // public key of user's shadow storage account
    pub groups: Vec<u32>,    // group id's the user is member of
    pub following: Vec<u32>, // user id's the user is following
    pub bump: u8,
}

#[account]
pub struct GroupProfile {
    pub ts: i64,       // timestamp
    pub group: Pubkey, // user public key
    pub gid: u32,      // group id (max 4,294,967,295)
    pub st: u8,        // status (default = 1)
    pub shdw: Pubkey,  // public key of group's shadow storage account
    pub bump: u8,
}
