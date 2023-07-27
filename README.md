# The Repography analyser CLI

This is the CLI option on [repography.com](https://repography.com), to analyse a local Git repo and submit the metadata to Repography. This lets you create Repography posters without using the GitHub app.

## Building and running the analyser from source

The command on the site to "copy and paste into your terminal" downloads and runs [./run.sh](run.sh), a small Bash script which itself downloads and runs the correct analyser binary depending on your platform and the availability of the `git` CLI.

If you'd prefer to clone this repo and run the analyser yourself, that works fine!

First of all you'll need Rust installed (see [Install Rust](https://www.rust-lang.org/tools/install)).

If you have the `git` CLI installed already, then you can simply use `cargo run`:

```sh
cargo run --release -- --target /path/to/your/repo
```

If you don't have the `git` CLI then we can use [libgit2](https://github.com/libgit2/libgit2), it's just slower:

```sh
cargo run --release --features=git-libgit2 -- --target /path/to/your/repo
```
