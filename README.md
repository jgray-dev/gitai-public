# Gitai


This is my first ever Rust project, so its horrible, and 90% of it is AI with human refactoring and bug fixing LOL


## Usage

**Instructions for Ubuntu**

Currently im not exactly sure how to properly release rust projects, but this should get you started:
- Clone the repo
- Modify `src/anthropic.rs` to include your own Anthropic API key
- run `cargo build --release`
- run `sudo mv target/release/gitai /usr/local/bin/gitai`
 
#### Thats it!

From any git repo, run the command `gitai` and it will use `git diff` and Claude 3 to automatically commit your changes with the appropriate message


*Again, this is my **first** ever Rust project, and its hardly mine other than the idea (Thanks Claude <3) if you encounter any issues, feel free to reach out to me. I'd love to really really learn and understand Rust, so im completely open to suggestions, improvements, and help!*


- [Linkedin](https://www.linkedin.com/in/jackson--gray/)
- [Personal Website](https://jgray.cc/home)
