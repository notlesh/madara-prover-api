use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CachedLdeConfig {
    pub store_full_lde: bool,
    pub use_fft_for_eval: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProverConfig {
    pub cached_lde_config: CachedLdeConfig,
    pub constraint_polynomial_task_size: i32,
    pub n_out_of_memory_merkle_layers: i32,
    pub table_prover_n_tasks_per_segment: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FriParameters {
    fri_step_list: Vec<u32>,
    last_layer_degree_bound: u32,
    n_queries: u32,
    proof_of_work_bits: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StarkParameters {
    fri: FriParameters,
    log_n_cosets: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProverParameters {
    pub field: String,
    pub stark: StarkParameters,
    pub use_extension_field: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PrivateInput {
    pub memory_path: PathBuf,
    pub trace_path: PathBuf,
    // TODO: the types for the 3 fields below are not clear, ask for a spec.
    pub pedersen: Vec<u32>,
    pub range_check: Vec<u32>,
    pub ecdsa: Vec<u32>,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
pub enum Layout {
    #[serde(rename = "plain")]
    Plain,
    #[serde(rename = "small")]
    Small,
    #[serde(rename = "dex")]
    Dex,
    #[serde(rename = "recursive")]
    Recursive,
    #[serde(rename = "starknet")]
    Starknet,
    #[serde(rename = "recursive_large_output")]
    RecursiveLargeOutput,
    #[serde(rename = "all_solidity")]
    AllSolidity,
    #[serde(rename = "starknet_with_keccak")]
    StarknetWithKeccak,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MemorySegment {
    pub begin_addr: u32,
    pub stop_ptr: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MemorySegments {
    pub program: MemorySegment,
    pub execution: MemorySegment,
    pub output: MemorySegment,
    pub pedersen: MemorySegment,
    pub range_check: MemorySegment,
    pub ecdsa: MemorySegment,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MemorySlot {
    pub address: u32,
    pub value: String,
    pub page: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicInput {
    pub layout: Layout,
    pub rc_min: u32,
    pub rc_max: u32,
    pub n_steps: u32,
    pub memory_segments: MemorySegments,
    pub public_memory: Vec<MemorySlot>,
    pub dynamic_params: Option<HashMap<String, u32>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Proof {
    // Note: we only map output fields for now
    pub proof_hex: String,
}

#[cfg(test)]
mod tests {
    use crate::toolkit::load_fixture;
    use std::path::Path;

    use super::*;

    /// Sanity check: verify that we can deserialize a private input JSON file.
    #[test]
    fn deserialize_private_input() {
        let private_input_str = load_fixture("fibonacci/fibonacci_private_input.json");
        let private_input: PrivateInput = serde_json::from_str(&private_input_str)
            .expect("Failed to deserialize private input fixture");

        assert_eq!(
            private_input.memory_path,
            Path::new("/home/root/fibonacci_memory.json")
        );
        assert_eq!(
            private_input.trace_path,
            Path::new("/home/root/fibonacci_trace.json")
        );
        assert_eq!(private_input.pedersen, Vec::<u32>::new());
        assert_eq!(private_input.range_check, Vec::<u32>::new());
        assert_eq!(private_input.ecdsa, Vec::<u32>::new());
    }

    /// Sanity check: verify that we can deserialize a public input JSON file.
    #[test]
    fn deserialize_public_input() {
        let public_input_str = load_fixture("fibonacci/fibonacci_public_input.json");
        let public_input: PublicInput = serde_json::from_str(&public_input_str)
            .expect("Failed to deserialize public input fixture");

        // We don't check all fields, just ensure that we can deserialize the fixture
        assert_eq!(public_input.layout, Layout::Small);
        assert_eq!(public_input.n_steps, 512);
        assert_eq!(public_input.dynamic_params, None);
    }

    #[test]
    fn deserialize_solver_parameters() {
        let parameters_str = load_fixture("fibonacci/cpu_air_params.json");
        let parameters: ProverParameters = serde_json::from_str(&parameters_str)
            .expect("Failed to deserialize prover parameters fixture");

        // We don't check all fields, just ensure that we can deserialize the fixture
        assert!(!parameters.use_extension_field);
    }
}