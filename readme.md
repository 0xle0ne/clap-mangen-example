# Auto-generate man pages for a clap-derive-based CLI (with nested subcommands)

Define your CLI once with `clap` derive, share it with `build.rs` via `include!`, and let `clap_mangen` emit a man page per command/subcommand into `target/man/` on every build.

<img src="./static/image.png" alt="clap_mangen logo" />

## Why this is neat

- Single source of truth: the derive types drive runtime parsing and documentation.
- No stale docs: man pages regenerate on build when the CLI changes.
- Great for packaging: ship `target/man/*.1` or install them into `/usr/share/man`.
- Works with nested subcommands out of the box.

## The CLI definition (derive)

We define the entire CLI under `src/cli.rs` using `clap` derive. Both the binary and the build script reuse these types.

```rust
// in src/cli.rs
use clap::{Command, CommandFactory, Parser};

// Longer description used for the top-level man page section.
const LONG_ABOUT: &str = r#"
mycli is a tiny example CLI demonstrating auto-generated man pages with clap and clap_mangen.

It showcases:
    - Nested subcommands (e.g., `config get`, `config set`)
    - Rich help/usage text derived from a single source of truth
    - Build-time man page generation to `target/man/`

Top-level commands:
    - config: manage configuration values (get/set)
    - server: run a demo server (addr/port/verbosity)
    - remote: add or remove a remote by name
"#;

/// mycli — a tiny example CLI used to demonstrate auto-generated man pages.
#[derive(Debug, Parser)]
#[command(
    name = "mycli",
    about = "Example CLI with nested subcommands and man page generation",
    long_about = LONG_ABOUT,
    version
)]
pub struct Cli {
    /// Top-level subcommand to execute
    #[command(subcommand)]
    pub command: Commands,
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
clap_mangen::generate_to(&mut cmd, &out_dir)?; // writes mycli.1, mycli-config.1, mycli-config-get.1, etc.
```

The build script prints a cargo:warning showing where files were written.

## Try it locally

```bash
cargo build
ls target/man
man -l target/man/mycli.1          # root command
man -l target/man/mycli-config-get.1  # nested subcommand
```

## Caveats & tradeoffs

- `build.rs` runs on every build; heavy generation can slow iterative cycles. For bigger doc pipelines, consider a `cargo xtask` instead and run it on demand or in CI.
- Some doc hosts (e.g., docs.rs) restrict or ignore build script side effects. Keep it lightweight.
- If you need custom filenames/sections, use `clap_mangen::Man::new(cmd).render(&mut writer)` and recurse yourself.

## Next steps

- Package installers can place pages under `/usr/share/man/man1/`.
- Homebrew formulae can install `target/man/*.1` during `brew install`.
- Explore docs: [`clap` on docs.rs](https://docs.rs/clap) and [`clap_mangen` on docs.rs](https://docs.rs/clap_mangen) for advanced options.
