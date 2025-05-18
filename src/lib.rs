use sp1_core_machine::riscv::RiscvAir;
use sp1_stark::{
    BabyBearPoseidon2Inner as SC, ShardProof, StarkGenericConfig, StarkVerifyingKey, Verifier,
};
use wasm_bindgen::prelude::*;

type Air = RiscvAir<<SC as StarkGenericConfig>::Val>;

#[wasm_bindgen]
pub fn verify_stark(proof_bytes: &[u8], vk_bytes: &[u8]) -> bool {
    // 1. decode
    let proof: ShardProof<SC> = bincode::deserialize(proof_bytes).unwrap();
    let vk: StarkVerifyingKey<SC> = bincode::deserialize(vk_bytes).unwrap();

    // 2. config + challenger
    let cfg = SC::new();
    let mut fs = cfg.challenger();

    // 3. verify (single shard; empty chip slice is fine for core proofs)
    Verifier::<SC, Air>::verify_shard(&cfg, &vk, &[], &mut fs, &proof).is_ok()
}
