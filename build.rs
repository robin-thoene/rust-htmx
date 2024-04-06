use std::{env, process::Command};

fn main() {
    if let Ok(profile) = env::var("PROFILE") {
        if profile == "release" {
            Command::new("npx")
                .args([
                    "tailwindcss",
                    "-i",
                    "./style/tailwind.css",
                    "-o",
                    "./static/global.css",
                    "--minify",
                ])
                .status()
                .expect("Expect tailwind css to build successful.");
        }
    }
}
