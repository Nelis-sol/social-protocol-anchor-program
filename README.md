# About the project
![alt text](https://github.com/spling-labs/social-protocol-anchor-program/blob/main/spling-banner.png "Spling banner")
<br /><br />

## Spling social protocol

With Spling social protocol, developers could build their own social app on top of the Solana blockchain. They would benefit from the composability blockchains bring, with a fraction of the cost normally associated with storing data on blockchains. Spling pioneered a early version of compression on Solana. Users of apps built on Spling retain full control over their data - they own their data and only allow social apps to read data (e.g. user profile, user posts) for displaying in the social app. 

**Functionalities**:
  * Create a user profile
  * Create a group (e.g. a community or app-specific feed)
  * Join a group
  * Follow a user
  * Create a post
  * Create reply to a post
  * Like a post (or reply)

Two special features:
  * Actions are free for a user, they are paid by the internal program bank account (no SOL required)
  * Content is compressed; a hash is taking from the content (e.g. user post) and used to generate the PDA address that stored metadata onchain

<br />

## Built with

- [x] **Rust**
- [x] **Solana**  
- [x] **Anchor**

<br />

____

<br />

## Install & run

### 1. Install Rust, Cargo
```
$ curl https://sh.rustup.rs -sSf | sh
$ source $HOME/.cargo/env
```

If you have Rust already installed, make sure to update to the latest stable version.
```
$ rustup update
```
<br />

### 2. Install Anchor, AVM
```
$ cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
$ avm install 0.26.0
$ avm use 0.26.0
```
<br />

### 3. Deploy program on devnet
Update the program id if necessary (in the `lib.rs` and `anchor.toml` files).

```
$ anchor build
$ anchor deploy
```
<br />

### 4. Result: composable social protocol

Any app can integrate the social protocol. Posts, follows, likes are owned by the user and apps serve as a client to show the data.
Apps can decide what groups/feeds they show, and thus decide on the degree of composability.

<br />

# Contact
Contact me via email (nelis.sol@protonmail.com) or on X (@nelis-sol)

<br /><br />

