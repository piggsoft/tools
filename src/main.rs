use std::path::PathBuf;
use clap::{arg, command, Parser, Subcommand};

mod make_link;


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
    MakeLink(MakeLinkArgs),
}


#[derive(clap::Args, Debug)]
struct MakeLinkArgs {
    #[arg(required = true, long, short = 's', help = "原始文件或者目录")]
    source: String,
    #[arg(required = false, long, short = 't', help = "迁移的目标文件或者目录，保证在迁移前无该文件或目录", long_help = "不传时，将使用<source_path>，并将其开头的<C:>改成<D:>")]
    target: Option<String>,
}


fn main() {
    // match ToolsCli::parse() {
    //     ToolsCli::MakeLink(args) => make_link::make_link(args.source.as_str(), args.target.as_str()),
    //     _ => println!("不支持的命令"),
    // }

    let cli = ToolsCli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::MakeLink(args)) => {
            make_link::make_link(&args.source, &args.target);
        },
        None => {}
    }
}
