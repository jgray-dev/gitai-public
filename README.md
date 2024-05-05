# Gitai


This is my first ever Rust project, so its not exactly great, but it was done in a couple hours off a random thought, and I'm happy with how it turned out.


## Usage

**Instructions for Ubuntu**

Currently im not exactly sure how to properly release rust projects, but this should get you started:
- Clone the repo
- Modify `src/anthropic.rs` to include your own Anthropic API key
- run `cargo build --release`
- run `sudo mv target/release/gitai /usr/local/bin/gitai`
 
#### Thats it!

From any git repo, run the command `gitai` and it will use `git diff` and Claude 3 to automatically commit your changes with the appropriate message


*Again, this is my **first** ever Rust project, and a lot of it was just thrown together.. if you encounter any issues, feel free to reach out to me. I'd love to really really learn and understand Rust, so im completely open to suggestions, improvements, and help!*


- [Linkedin](https://www.linkedin.com/in/jackson--gray/)
- [Personal Website](https://jgray.cc/home)
