use anyhow::Result;
use vergen_gix::{BuildBuilder, CargoBuilder, Emitter, GixBuilder};

fn main() -> Result<()> {
    let build = Build::all_build();
    let gix = Gix::all()
        {% if repository != "" -%}
        .remote_url("{{repository}}") // Enter the link to your repo for the version message to work in `cargo install`
        {%- endif %}
        .build();
    let cargo = Cargo::all_cargo();
    Emitter::default()
        .add_instructions(&build)?
        .add_instructions(&gix)?
        .add_instructions(&cargo)?
        .emit()
}
