fn main() {
    let git = env!("SMOL_VERGEN_GIT_BRANCH");
    println!("git branch: {}", git);
}
