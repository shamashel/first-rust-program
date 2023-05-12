# first-rust-program

The first program I've created in rust. After my experience with it, hopefully this is the first of many.

To keep things short, this is a CLI interface for the JSON Placeholder `Posts` API.

## Why I wrote this

For anyone that stumbles upon this page, my basic workflow for learning a language is to:
1. Build a simple project with it, relying entirely on googling to hack something together
2. Rewrite the earlier parts of the code to match whatever I've learned by the end
3. Read the official manual/book/tutorial

What you're seeing here is parts 1-2 of that flow. The reason why I do things this way is because I've found that there are certain nuances in documentation that you can't really pick up on until you've actually tried to do *something* with the language.

As an example, a couple things that stood out to me while writing this were the reliance on `match` statements to handle `Result` or `Option` types (really hoping there's a better way to handle this beyond just bubbling errors up to `main`), the weirdness of using `&` or `*` to declare when I wanted a reference or copy of something, and the general weirdness of "borrowing" something resulting in a lot of code similar to `"...".to_string()`. Because of this experience, I'll probably pay more attention to those things when going through the actual docs to figure out why things work that way.