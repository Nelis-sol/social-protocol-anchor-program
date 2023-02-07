import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Socialprotocol } from "../target/types/socialprotocol";
import { PublicKey, ComputeBudgetProgram } from "@solana/web3.js";

describe("socialprotocol", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  let shdw = anchor.web3.Keypair.generate();

  const program = anchor.workspace.Socialprotocol as Program<Socialprotocol>;

  it("Sets up spling", async () => {
    const [SplingPDA] = await PublicKey.findProgramAddress(
      [anchor.utils.bytes.utf8.encode("spling")],
      program.programId
    );

    await program.methods
      .setupSpling()
      .accounts({
        user: provider.wallet.publicKey,
        spling: SplingPDA,
      })
      .rpc();
  });

  it("Creates a Bank", async () => {
    const [SplingPDA] = await PublicKey.findProgramAddress(
      [anchor.utils.bytes.utf8.encode("spling")],
      program.programId
    );

    const [BankPDA] = await PublicKey.findProgramAddress(
      [anchor.utils.bytes.utf8.encode("b")],
      program.programId
    );

    await program.methods
      .createB()
      .accounts({
        user: provider.wallet.publicKey,
        b: BankPDA,
        spling: SplingPDA,
      })
      .rpc();
  });

  it("Creates User Profile", async () => {
    const [SplingPDA] = await PublicKey.findProgramAddress(
      [anchor.utils.bytes.utf8.encode("spling")],
      program.programId
    );

    const [BankPDA] = await PublicKey.findProgramAddress(
      [anchor.utils.bytes.utf8.encode("b")],
      program.programId
    );

    const [UserProfilePDA] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("user_profile"),
        provider.wallet.publicKey.toBuffer(),
      ],
      program.programId
    );

    let shdw_keypair = anchor.web3.Keypair.generate();
    let shdw_public = shdw_keypair.publicKey;

    let ta = new PublicKey("EwwRs2bCnStnB21QqHbBYMAusCTJ75o2Mepq9RJCEtos");

    await program.methods
      .createUserProfile(shdw_public, null)
      .accounts({
        user: provider.wallet.publicKey,
        spling: SplingPDA,
        userProfile: UserProfilePDA,
        b: BankPDA,
        // receiver: shdw_public,
        // senderTokenAccount: ta,
        // receiverTokenAccount: ta,
        // mint: ta,
        // token_program: shdw_public,
      })
      .rpc();
  });

  it("Sets up tags", async () => {
    const [TagsPDA] = await PublicKey.findProgramAddress(
      [anchor.utils.bytes.utf8.encode("tags")],
      program.programId
    );

    const [SplingPDA] = await PublicKey.findProgramAddress(
      [anchor.utils.bytes.utf8.encode("spling")],
      program.programId
    );

    await program.methods
      .setupTags()
      .accounts({
        user: provider.wallet.publicKey,
        spling: SplingPDA,
        tags: TagsPDA,
      })
      .rpc();
  });

  it("Submits a post", async () => {
    const [SplingPDA] = await PublicKey.findProgramAddress(
      [anchor.utils.bytes.utf8.encode("spling")],
      program.programId
    );

    const [TagsPDA] = await PublicKey.findProgramAddress(
      [anchor.utils.bytes.utf8.encode("tags")],
      program.programId
    );

    const [BankPDA] = await PublicKey.findProgramAddress(
      [anchor.utils.bytes.utf8.encode("b")],
      program.programId
    );

    const [UserProfilePDA] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("user_profile"),
        provider.wallet.publicKey.toBuffer(),
      ],
      program.programId
    );

    const [PostPDA] = await PublicKey.findProgramAddress(
      [anchor.utils.bytes.utf8.encode("post"), shdw.publicKey.toBuffer()],
      program.programId
    );

    const [LikesPDA] = await PublicKey.findProgramAddress(
      [anchor.utils.bytes.utf8.encode("likes"), PostPDA.toBuffer()],
      program.programId
    );

    const postThread = PublicKey.findProgramAddressSync(
      [Buffer.from("thread"), PostPDA.toBuffer(), Buffer.from("post_thread")],
      new PublicKey("3XXuUFfweXBwFgFfYaejLvZE4cGZiHgKiGfMtdxNzYmv")
    )[0];

    await program.methods
      .submitPost(1, shdw.publicKey, "hello", null)
      .accounts({
        user: provider.wallet.publicKey,
        spling: SplingPDA,
        userProfile: UserProfilePDA,
        post: PostPDA,
        tags: TagsPDA,
        likes: LikesPDA,
        b: BankPDA,
        postThread,
        threadProgram: new PublicKey(
          "3XXuUFfweXBwFgFfYaejLvZE4cGZiHgKiGfMtdxNzYmv"
        ),

        // receiver: shdw_public,
        // senderTokenAccount: ta,
        // receiverTokenAccount: ta,
        // mint: ta,
        // token_program: shdw_public,
      })
      .rpc();

    console.log(postThread.toBase58());
  });
});
