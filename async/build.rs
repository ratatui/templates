use std::process::Command;

fn main() {
  let output =
    Command::new(if cfg!(windows) { "git.exe" } else { "git" }).args(["describe", "--always"]).output().unwrap();
  let git_info = String::from_utf8(output.stdout).unwrap().trim().to_string();

  println!("cargo:rustc-env=RATATUI_TEMPLATE_GIT_INFO={git_info}");
}
