use anchor_lang::prelude::*;
use murmur3::murmur3_x64_128;
use sha2::{Digest, Sha256};
use solana_program::{clock::Clock, sysvar::Sysvar};
use std::io::Cursor;

declare_id!("FaxBGrEXfDT7USLAwNDQCtXvAJW4W44VdLFa5YVb6p3G");

#[program]
pub mod anchor_rng {
    use super::*;

    pub fn xorshift(_ctx: Context<Rng>, max_value: u64) -> Result<()> {
        let slot = Clock::get()?.slot;
        let xorshift_output = xorshift64star(slot);
        let random_number = xorshift_output % max_value;
        msg!("Slot: {}", slot);
        msg!("Xorshift: {}", xorshift_output);
        msg!("Random Number: {}", random_number);

        Ok(())
    }

    pub fn sha256(_ctx: Context<Rng>, max_value: u64) -> Result<()> {
        let slot = Clock::get()?.slot;
        let mut hasher = Sha256::new();
        hasher.update(slot.to_be_bytes());
        let hash = hasher.finalize();
        let random_number = u64::from_be_bytes(hash[..8].try_into().unwrap()) % max_value;
        msg!("Slot: {}", slot);
        msg!("SHA256 Hash: {:?}", hash);
        msg!("Random Number: {}", random_number);

        Ok(())
    }

    pub fn sha256_multiple(_ctx: Context<Rng>, max_value: u64) -> Result<()> {
        let slot = Clock::get()?.slot;
        let mut hasher = Sha256::new();
        hasher.update(slot.to_be_bytes());
        let hash = hasher.finalize();

        let mut random_numbers: [u64; 4] = [0; 4];
        for i in 0..4 {
            let hash_slice = &hash[i * 8..(i + 1) * 8];
            random_numbers[i] = u64::from_be_bytes(hash_slice.try_into().unwrap()) % max_value;
        }

        msg!("Slot: {}", slot);
        msg!("SHA256 Hash: {:?}", hash);
        for (index, number) in random_numbers.iter().enumerate() {
            msg!("Random Number {}: {}", index + 1, number);
        }

        Ok(())
    }

    pub fn murmur(_ctx: Context<Rng>, max_value: u64) -> Result<()> {
        let slot = Clock::get()?.slot;
        let slot_bytes = slot.to_be_bytes();
        let mut cursor = Cursor::new(&slot_bytes);
        let murmur_output = murmur3_x64_128(&mut cursor, 0).unwrap();
        let random_number = (murmur_output % (max_value as u128)) as u64;

        msg!("Slot: {}", slot);
        msg!("Murmur: {}", murmur_output);
        msg!("Random Number: {}", random_number);

        Ok(())
    }

    pub fn murmur_multiple(_ctx: Context<Rng>, max_value: u64) -> Result<()> {
        let slot = Clock::get()?.slot;
        let slot_bytes = slot.to_be_bytes();
        let mut cursor = Cursor::new(&slot_bytes);
        let murmur_output = murmur3_x64_128(&mut cursor, 0).unwrap();

        let lower = (murmur_output & 0xFFFFFFFFFFFFFFFF) as u64 % max_value;
        let upper = ((murmur_output >> 64) & 0xFFFFFFFFFFFFFFFF) as u64 % max_value;

        msg!("Slot: {}", slot);
        msg!("Murmur: {}", murmur_output);
        msg!("Random Number 1: {}", lower);
        msg!("Random Number 2: {}", upper);

        Ok(())
    }

    pub fn rng(_ctx: Context<Rng>, max_value: u64) -> Result<()> {
        let slot = Clock::get()?.slot;
        msg!("Slot: {}", slot);

        let xorshift_output = xorshift64star(slot);
        let random_number = xorshift_output % max_value;
        msg!("Xorshift: {}", xorshift_output);
        msg!("Random Number: {}", random_number);

        let mut hasher = Sha256::new();
        hasher.update(slot.to_be_bytes());
        let hash = hasher.finalize();
        let random_number = u64::from_be_bytes(hash[..8].try_into().unwrap()) % max_value;
        msg!("SHA256 Hash: {:?}", hash);
        msg!("Random Number: {}", random_number);

        let slot_bytes = slot.to_be_bytes();
        let mut cursor = Cursor::new(&slot_bytes);
        let murmur_output = murmur3_x64_128(&mut cursor, 0).unwrap();
        let random_number = (murmur_output % (max_value as u128)) as u64;
        msg!("Murmur: {}", murmur_output);
        msg!("Random Number: {}", random_number);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Rng {}

pub fn xorshift64star(seed: u64) -> u64 {
    let mut x = seed;
    x ^= x << 12;
    x ^= x >> 25;
    x ^= x << 27;
    x = (x as u128 * 0x2545F4914F6CDD1D) as u64;
    x
}
