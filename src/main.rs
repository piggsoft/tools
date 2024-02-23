use std::path::PathBuf;

use clap::{arg, command, Parser, Subcommand};

use toolslib::*;

#[derive(Parser)] // requires `derive` feature
#[command(name = "tools", bin_name = "tools", version = "0.1.0", about = "Piggsoft的工具包")]
struct ToolsCli {
    name: Option<String>,
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "makeLink", version = "0.1.0", about = "将<source>文件迁移到<target>，并将<source>变为软链接")]
    MakeLink(make_link::MakeLinkArgs),
    // #[command(name = "urlencode", version = "0.1.0", about = "对url进行encode和decode,字符编码为utf8")]
    // UrlEncode(url_encoding::UrlEncodeArgs),
    #[command(name = "codec", version = "0.1.0", about = "加解密工具集合")]
    Codec(codec::CodecArgs),
}

fn main() {
    let cli = ToolsCli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::MakeLink(args)) => {
            make_link::make_link(args);
        }
        // Some(Commands::UrlEncode(args)) => {
        //     url_encoding::process_auto(args);
        // }
        Some(Commands::Codec(args)) => {
            codec::codec_auto(args);
        }

        None => {}
    }
}
