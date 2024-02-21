use std::{io, process};

pub(crate) fn make_link(source: &str, target: &str) {
    let source_path = &source[..];
    let binding = source
        .replace("C:\\", "D:\\")
        .replace("c:\\", "d:\\")
        .replace("C:/", "D:/")
        .replace("c:/", "d:/");
    let mut target_path = &binding[..];
    if target != "" {
        target_path = &target[..];
    }
    println!("原始目录为：{source_path}");
    println!("软连接迁移后的目录为：{target_path}");
    println!("请确认是否将原始目录: {source_path}，迁移到：{target_path}，并将原始目录删除后建立软连接？ Y or N ？");

    let mut user_input: String = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("读取输入出错！");


    if user_input.trim().to_ascii_uppercase().as_str() != "Y" {
        println!("程序正在退出！");
        return;
    }

    println!("---------------开始目录拷贝---------------");
    let result = copy_dir::copy_dir(source_path, target_path);
    match result {
        Ok(errors) => {
            if !errors.is_empty() {
                errors.iter().for_each(|err| { eprintln!("目录拷贝出错: {}", err) });
                process::exit(-1);
            }
        },
        Err(err) => {
            eprintln!("目录拷贝出错: {:?}", err);
            process::exit(-1);
        },
    }
    println!("---------------结束目录拷贝---------------");

    println!("---------------开始原始目录删除---------------");
    let result = rm_rf::ensure_removed(source_path);
    if let Err(err) = result {
        eprintln!("目录删除出错: {}", err);
        process::exit(-1);
    }
    println!("---------------结束原始目录删除---------------");

    println!("---------------开始建立软链接---------------");
    let result = symlink::symlink_auto(target_path, source_path);
    if let Err(err) = result {
        eprintln!("建立软链接出错: {}", err);
        process::exit(-1);
    }
    println!("---------------结束建立软链接---------------");

    println!("---------------Make Link Done!---------------");
}

// fn cmd() -> clap::Command {
//     command!() // requires `cargo` feature
//         .subcommand_required(true)
//         .subcommand(
//             command!(MAKE_LINK)
//                 .about("将<source>文件迁移到<target>，并将<source>变为软链接")
//                 .arg(
//                     arg!(-s --source <source_path>)
//                         .required(true)
//                         .help("原始文件或者目录")
//                         .action(ArgAction::Set)
//                 ).arg(
//                 arg!(-t --target <target_path>)
//                     .required(false)
//                     .help("迁移的目标文件或者目录，保证在迁移前无该文件或目录")
//                     .action(ArgAction::Set)
//             )
//         )
// }