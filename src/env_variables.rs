use std::env;

pub fn load_env() {
    env::set_var("CRAB_ROCKET_WORKING_DIR", env!("CARGO_MANIFEST_DIR"));
}
