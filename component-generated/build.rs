use anyhow::Result;
use vergen_gix::{Build, Cargo, Emitter, Gix};

fn main() -> Result<()> {
    let build = Build::all_build();
    let gix = Gix::all_git();
    let cargo = Cargo::all_cargo();
    Emitter::default()
        .add_instructions(&build)?
        .add_instructions(&gix)?
        .add_instructions(&cargo)?
        .emit()
}
