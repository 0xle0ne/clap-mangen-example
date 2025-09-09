mod cli;

use clap::Parser;

fn main() {
    let opts = cli::Cli::parse();
    match opts.command {
        cli::Commands::Server(s) => {
            println!(
                "server start on {}:{} (verbosity: {})",
                s.addr, s.port, s.verbose
            );
        }
        cli::Commands::Remote(r) => {
            if r.remove {
                println!("remote removed: {}", r.name);
            } else if let Some(url) = r.url {
                println!("remote added: {} -> {}", r.name, url);
            } else {
                println!("remote info requested: {}", r.name);
            }
        }
        cli::Commands::Config(cfg) => match cfg.action {
            cli::ConfigAction::Get(g) => {
                println!("config get {} (format: {:?})", g.key, g.format);
            }
            cli::ConfigAction::Set(s) => {
                println!(
                    "config set {}={} (global: {})",
                    s.key, s.value, s.global
                );
            }
        },
    }
}
