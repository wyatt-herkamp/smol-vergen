use smol_vergen::{SmolVergenBuilder, SmolVergenResult};
use smol_vergen_git::GitPlugin;

fn main() -> SmolVergenResult {
    let mut smol_vergen = SmolVergenBuilder::default()
        .add_plugin(GitPlugin {
            check_parents: true,
        })
        .build()?;
    smol_vergen.run_on_env()?;
    smol_vergen.context.iter().for_each(|(k, v)| {
        println!("cargo:warning={}: {:?}", k, v);
    });
    Ok(())
}
