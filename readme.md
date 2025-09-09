# Auto-generate man pages for a clap-derive-based CLI (with nested subcommands)

TL;DR: Define your CLI once with `clap` derive, share it with `build.rs` via `include!`, and let `clap_mangen` emit a man page per command/subcommand into `target/man/` on every build.

<img src="./static/image.png" alt="clap_mangen logo" />

<div style="text-align: center;">

![](https://img.shields.io/badge/clap-4.4.0-blue.svg) ![](https://img.shields.io/badge/clap_mangen-0.2.5-blue.svg) ![](https://img.shields.io/badge/rust-1.70+-orange.svg)

</div>

## Why this is neat

- Single source of truth: the derive types drive runtime parsing and documentation.
- No stale docs: man pages regenerate on build when the CLI changes.
- Great for packaging: ship `target/man/*.1` or install them into `/usr/share/man`.
- Works with nested subcommands out of the box.

## The CLI definition (derive)

We define the entire CLI under `src/cli.rs` using `clap` derive. Both the binary and the build script reuse these types.

Key trick: to obtain the programmatic `clap::Command`, call the factory methods the derive gives you:

```rust
// in src/cli.rs
use clap::{Command, CommandFactory, Parser};

#[derive(Debug, Parser)]
#[command(name = "myapp", about = "Example CLI with nested subcommands and man page generation", version)]
pub struct Cli {
	#[command(subcommand)]
	pub command: Commands,
}

pub fn build_cli() -> Command {
	<Cli as CommandFactory>::command()
}
```

That same `build_cli()` is used in runtime and from `build.rs`.

## Sharing CLI with `build.rs`

`build.rs` compiles as a separate crate, so we include the same `src/cli.rs` (and ensure `clap` is available in `[build-dependencies]` with `features=["derive"]`).

```rust
// in build.rs
mod cli {
	include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/cli.rs"));
}

let mut cmd = <cli::Cli as clap::CommandFactory>::command();
```

## Generating man pages recursively

`clap_mangen` can generate man pages for the root command and all nested subcommands with a single call:

```rust
let out_dir = manifest_dir.join("target").join("man");
std::fs::create_dir_all(&out_dir)?;
let mut cmd = <cli::Cli as clap::CommandFactory>::command();
clap_mangen::generate_to(&mut cmd, &out_dir)?; // writes myapp.1, myapp-config.1, myapp-config-get.1, etc.
```

The build script prints a cargo:warning showing where files were written.

## Try it locally

```bash
cargo build
ls target/man
man -l target/man/myapp.1          # root command
man -l target/man/myapp-config-get.1  # nested subcommand
```

## Caveats & tradeoffs

- `build.rs` runs on every build; heavy generation can slow iterative cycles. For bigger doc pipelines, consider a `cargo xtask` instead and run it on demand or in CI.
- Some doc hosts (e.g., docs.rs) restrict or ignore build script side effects. Keep it lightweight.
- If you need custom filenames/sections, use `clap_mangen::Man::new(cmd).render(&mut writer)` and recurse yourself.

## Next steps

- Package installers can place pages under `/usr/share/man/man1/`.
- Homebrew formulae can install `target/man/*.1` during `brew install`.
- Explore docs: [`clap` on docs.rs](https://docs.rs/clap) and [`clap_mangen` on docs.rs](https://docs.rs/clap_mangen) for advanced options.
