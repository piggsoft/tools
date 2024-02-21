use std::io;
use copy_dir::copy_dir;
use rm_rf;
use symlink;
use clap::{arg, ArgAction, command, Parser};

const MAKE_LINK: &'static str = "makeLink";

#[derive(Parser)] // requires `derive` feature
#[command(name = "tools", bin_name = "tools", version = "1.0", about = "Piggsoft的工具包")]
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
    #[arg(required = false, long, short = 't', default_value = "", help = "迁移的目标文件或者目录，保证在迁移前无该文件或目录", long_help="不传时，将使用<source_path>，并将其开头的<C:>改成<D:>")]
    target: Option<String>,
}


fn main() {
    //println!("this {:?}", make_link_1);

    match ToolsCli::parse() {
        ToolsCli::MakeLink(args) => make_link(args.source.as_str(), args.target),
        _ => println!("不支持的命令"),
    }


    // let pattern = std::env::args().nth(1).expect("没找到pattern参数，请确认命令未：tools {pattern} {args}。");
    // let args = std::env::args().nth(2).expect("没找到args参数，请确认命令未：tools {pattern} {args}。");
    //
    // match pattern.as_str() {
    //     "make_link" => make_link(args.as_str()),
    //     _ => println!("不支持的pattern。"),
    // }
}


fn make_link(source: &str, target: Option<String>) {
    let source_path = source;
    let binding = source
        .replace("C:\\", "D:\\")
        .replace("c:\\", "d:\\")
        .replace("C:/", "D:/")
        .replace("c:/", "d:/");
    let mut target_path = binding
        .as_str();
    if (target.is_some()) {
        target_path = target.unwrap().as_str();
    }
    println!("原始目录为：{source_path}");
    println!("软连接迁移后的目录为：{target_path}");
    println!("请确认是否将原始目录: {source_path}，迁移到：{target_path}，并将原始目录删除后建立软连接？ Y or N ？");

    let mut user_input: String = Default::default();
    io::stdin()
        .read_line(&mut user_input)
        .expect("读取输入出错！");

    if user_input.to_ascii_uppercase() == "N" {
        println!("程序正在退出！");
        return;
    }

    let _ = copy_dir(source_path, target_path);

    let _ = rm_rf::ensure_removed(source_path);

    let _ = symlink::symlink_auto(target_path, source_path);
}

fn cmd() -> clap::Command {
    command!() // requires `cargo` feature
        .subcommand_required(true)
        .subcommand(
            command!(MAKE_LINK)
                .about("将<source>文件迁移到<target>，并将<source>变为软链接")
                .arg(
                    arg!(-s --source <source_path>)
                        .required(true)
                        .help("原始文件或者目录")
                        .action(ArgAction::Set)
                ).arg(
                arg!(-t --target <target_path>)
                    .required(false)
                    .help("迁移的目标文件或者目录，保证在迁移前无该文件或目录")
                    .action(ArgAction::Set)
            )
        )
}