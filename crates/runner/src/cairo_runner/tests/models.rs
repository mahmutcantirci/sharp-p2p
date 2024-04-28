use rand::{thread_rng, Rng};
use sharp_p2p_common::job::{Job, JobData};

use starknet_crypto::FieldElement;
use std::{env, fs, path::PathBuf};

pub struct TestFixture {
    pub job: Job,
    pub program_path: PathBuf,
}

pub fn fixture() -> TestFixture {
    let mut rng = thread_rng();
    let ws_root =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR env not present"))
            .join("../../");
    let cairo_pie_path = ws_root.join("crates/tests/cairo/fibonacci_pie.zip");
    let program_path = ws_root.join("target/bootloader.json");

    TestFixture {
        job: Job::from_job_data(
            JobData::new(
                rng.gen(),
                rng.gen(),
                fs::read(cairo_pie_path).unwrap(),
                FieldElement::ZERO,
            ),
            &libp2p::identity::ecdsa::Keypair::generate(),
        ),
        program_path,
    }
}
