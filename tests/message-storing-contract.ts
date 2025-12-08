import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Connection, sendAndConfirmTransaction, Transaction } from "@solana/web3.js";
import { MessageStoringContract } from "../target/types/message_storing_contract";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";

let cluster = "devnet";

const connection =
  cluster == "localnet"
    ? new Connection("http://localhost:8899", "confirmed")
    : new Connection("https://api.devnet.solana.com", "confirmed");

anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.messageStoringContract as Program<MessageStoringContract>;

const admin = anchor.web3.Keypair.fromSecretKey(
  bs58.decode(
    process.env.ADMIN_SECRET_KEY
  )
);

const user = anchor.web3.Keypair.fromSecretKey(
  bs58.decode(
    process.env.USER_SECRET_KEY
  )
);

describe("message-storing-contract", () => {

  it("Message is initialized!", async () => {
    const tx = new Transaction()

    const ix = await program.methods
      .initializeMessage("Hello, Solana!")
      .accounts({
        admin: admin.publicKey,
      })
      .instruction();

    tx.add(ix);
    tx.feePayer = admin.publicKey;
    tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;
    tx.sign(admin);

    console.log(await connection.simulateTransaction(tx));

    const signature = await sendAndConfirmTransaction(connection, tx, [admin]);
    console.log(
      "Initialize message transaction signature:",
      `https://solscan.io/tx/${signature}?cluster=devnet`
    );
  });

  it("Message is updated!", async () => {
    const tx = new Transaction();

    const ix = await program.methods
      .updateMessage("Solana is the fastest Blockchain!")
      .accounts({
        user: user.publicKey,
      })
      .instruction();

    tx.add(ix);
    tx.feePayer = user.publicKey;
    tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;
    tx.sign(user);

    console.log(await connection.simulateTransaction(tx));

    const signature = await sendAndConfirmTransaction(connection, tx, [user]);
    console.log(
      "Update message transaction signature:",
      `https://solscan.io/tx/${signature}?cluster=devnet`
    );

    let [messagePda] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("message_seed")],
      program.programId
    );

    const messageAccount = await program.account.messageAccount.fetch(messagePda);

    console.log("Current On-chain Stored Message:", messageAccount.message);
  })
});
