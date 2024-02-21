use clap::{arg, command, Parser};

mod make_link;

#[derive(Parser)] // requires `derive` feature
#[command(name = "tools", bin_name = "tools", version = "0.1.0", about = "Piggsoft的工具包")]
enum ToolsCli {
    #[command(name = "makeLink", about = "将<source>文件迁移到<target>，并将<source>变为软链接")]
    MakeLink(MakeLinkArgs),  //subcommand

    #[command(name = "Host", about = "打印host")]
    Host(MakeLinkArgs),  //subcommand
}


#[derive(clap::Args)]
struct MakeLinkArgs {
    #[arg(required = true, long, short = 's', help = "原始文件或者目录")]
    source: String,
    #[arg(required = false, long, short = 't', default_value = "", help = "迁移的目标文件或者目录，保证在迁移前无该文件或目录", long_help = "不传时，将使用<source_path>，并将其开头的<C:>改成<D:>")]
    target: String,
}


fn main() {
    match ToolsCli::parse() {
        ToolsCli::MakeLink(args) => make_link::make_link(args.source.as_str(), args.target.as_str()),
        _ => println!("不支持的命令"),
    }
}
