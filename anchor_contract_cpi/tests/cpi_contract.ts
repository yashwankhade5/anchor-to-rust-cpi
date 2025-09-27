import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CpiContract } from "../target/types/cpi_contract";
import { Keypair, PublicKey, TransactionInstruction, } from "@solana/web3.js";
import { it } from "mocha";
import { assert, expect } from "chai";

describe("cpi_contract", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const nativeProgramPubkey = new PublicKey("94iiU3ikgHSxHAs7HSw8ACw1PCmB24krxSzweKVSjT7M");
  const program = anchor.workspace.cpiContract as Program<CpiContract>;
  const newAccount = new Keypair()
  it("Is initialized!", async () => {
    const tx = await program.methods.initialize()
      .accounts({
        signer: anchor.getProvider().wallet.publicKey,
        account: newAccount.publicKey,
        cpiprogram: nativeProgramPubkey
      })
      .signers([newAccount])
      .rpc();
    console.log("Your transaction signature", tx.toString());
    await program.provider.connection.confirmTransaction(tx, "confirmed")
    const accountInfo = await program.provider.connection.getAccountInfo(newAccount.publicKey, "confirmed");
    console.log("Account info:", accountInfo.data.readBigUint64LE(0));
  });

  it("double", async () => {
    async function double() {
      const tx = await program.methods.double()
        .accounts({
          signer: anchor.getProvider().wallet.publicKey,
          account: newAccount.publicKey,
          cpiProgram: nativeProgramPubkey,
        })
        .signers([newAccount])
        .rpc();
      const cx = await program.provider.connection.confirmTransaction(tx, "confirmed");
    }
    await double()
    await double()
    await double()
    const accountInfo1 = await program.provider.connection.getAccountInfo(newAccount.publicKey, "confirmed");
    console.log("Account info:", Number(accountInfo1.data.readBigUInt64LE(0)));
    expect(Number(accountInfo1.data.readBigUInt64LE(0))).to.equal(8)
    await double()
    const accountInfo2 = await program.provider.connection.getAccountInfo(newAccount.publicKey, "confirmed");
    expect(Number(accountInfo2.data.readBigUInt64LE(0))).to.equal(16)

  })
  it("halve", async () => {
    async function halve() {
      const tx = await program.methods.halve()
        .accounts({
          signer: anchor.getProvider().wallet.publicKey,
          account: newAccount.publicKey,
          cpiProgram: nativeProgramPubkey,
        })
        .signers([newAccount])
        .rpc()
      const cx = await program.provider.connection.confirmTransaction(tx, "confirmed");
    }
    await halve()

    const accountInfo1 = await program.provider.connection.getAccountInfo(newAccount.publicKey, "confirmed");
    console.log("Account info:", Number(accountInfo1.data.readBigUInt64LE(0)));
    expect(Number(accountInfo1.data.readBigUInt64LE(0))).to.equal(8)
    await halve()
    const accountInfo2 = await program.provider.connection.getAccountInfo(newAccount.publicKey, "confirmed");
    console.log("Account info:", Number(accountInfo2.data.readBigUInt64LE(0)));
    expect(Number(accountInfo2.data.readBigUInt64LE(0))).to.equal(4)
    await halve()
    const accountInfo3 = await program.provider.connection.getAccountInfo(newAccount.publicKey, "confirmed");
    console.log("Account info:", Number(accountInfo3.data.readBigUInt64LE(0)));
    expect(Number(accountInfo3.data.readBigUInt64LE(0))).to.equal(2)
    await halve()
    const accountInfo4 = await program.provider.connection.getAccountInfo(newAccount.publicKey, "confirmed");
    console.log("Account info:", Number(accountInfo4.data.readBigUInt64LE(0)));
    expect(Number(accountInfo4.data.readBigUInt64LE(0))).to.equal(1)


  })
});
