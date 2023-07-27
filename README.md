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
## Inspecting the analyser

If you're here because you want to ensure that the analyser isn't doing anything nefarious, here are some pointers to the most relevant bits:

* [src/main.rs](./src/main.rs): The entrypoint which shows each step of the process.
* [src/analysis.rs](./src/analysis.rs): This contains the Analysis structure which is uploaded to Repography for visualization.
* [src/git/cli.rs](./src/git/cli.rs): The actual `git` commands which are used ([src/git/git2.rs](./src/git/git2.rs) is the same, if using libgit2 instead of `git`).
* [src/api.rs](./src/api.rs): Interactions with the Repography API.
* [src/encode.rs](./src/encode.rs): Encodes the analysis for upload (N.B. it is encrypted at this point with a per-repo key and will stay encrypted at rest).
