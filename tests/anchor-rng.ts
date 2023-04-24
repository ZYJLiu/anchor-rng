import * as anchor from "@coral-xyz/anchor"
import { Program } from "@coral-xyz/anchor"
import { AnchorRng } from "../target/types/anchor_rng"

describe("anchor-rng", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env())

  const program = anchor.workspace.AnchorRng as Program<AnchorRng>

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.xorshift(new anchor.BN(100)).rpc()
    console.log(`https://explorer.solana.com/tx/${tx}?cluster=devnet`)
  })

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.sha256(new anchor.BN(100)).rpc()
    console.log(`https://explorer.solana.com/tx/${tx}?cluster=devnet`)
  })

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.sha256Multiple(new anchor.BN(100)).rpc()
    console.log(`https://explorer.solana.com/tx/${tx}?cluster=devnet`)
  })

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.murmur(new anchor.BN(100)).rpc()
    console.log(`https://explorer.solana.com/tx/${tx}?cluster=devnet`)
  })

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.murmurMultiple(new anchor.BN(100)).rpc()
    console.log(`https://explorer.solana.com/tx/${tx}?cluster=devnet`)
  })

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.rng(new anchor.BN(100)).rpc()
    console.log(`https://explorer.solana.com/tx/${tx}?cluster=devnet`)
  })
})
