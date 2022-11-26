use std::{fmt::Error, process::ExitCode};

fn main() -> Result<ExitCode, Error> {
    mindustry_rogic_hardening::test();
    mindustry_rogic_hardening::r#loop();
    return Ok(ExitCode::SUCCESS);
}
