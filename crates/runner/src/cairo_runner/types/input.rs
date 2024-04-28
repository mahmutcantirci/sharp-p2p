use serde::{Deserialize, Serialize};
use sharp_p2p_common::job::Job;

#[derive(Serialize, Deserialize)]
pub struct SimpleBootloaderInput {
    pub public_key: Vec<u8>,
    pub job: Job,
    pub single_page: bool,
}
