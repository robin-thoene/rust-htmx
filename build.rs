use std::process::Command;

fn main() {
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
