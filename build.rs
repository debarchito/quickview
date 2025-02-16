fn main() -> Result<(), Box<dyn std::error::Error>> {
  println!("cargo:rerun-if-changed=i18n");

  let mut vergen = vergen::EmitBuilder::builder();

  println!("cargo:rerun-if-env-changed=VERGEN_GIT_SHA");
  if std::env::var_os("VERGEN_GIT_SHA").is_none() {
    vergen.git_sha(false);
  }

  vergen.fail_on_error().emit()?;

  Ok(())
}
