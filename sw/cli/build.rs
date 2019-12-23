use std::process::Command;

fn main() {
    println!("Running `yarn install` for the Web UI application.");
    Command::new("yarn")
        .args(&["--cwd", "src/ui/static", "install"])
        .status()
        .unwrap();

    println!("Running `yarn build` for the Web UI application.");
    Command::new("yarn")
        .args(&["--cwd", "src/ui/static", "build"])
        .status()
        .unwrap();

    println!("Web UI application has been successfully bootstrapped and built.");

    println!("cargo:rerun-if-changed=src/ui/static/src/index.html");
    println!("cargo:rerun-if-changed=src/ui/static/src/index.tsx");
    println!("cargo:rerun-if-changed=src/ui/static/package.json");
}
