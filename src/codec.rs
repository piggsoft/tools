use std::process;
use clap::Subcommand;

#[derive(clap::Args, Debug)]
pub(crate) struct CodecArgs {
    #[command(subcommand)]
    pub method: CodecCommand,
}

#[derive(Subcommand)]
#[derive(Debug)]
pub(crate) enum CodecCommand {
    #[command(name = "base64", version = "0.1.0", about = "进行Base64加解密")]
    Base64(CodecImplArgs),
    #[command(name = "aes", version = "0.1.0", about = "进行AES加解密")]
    AES(CodecImplArgs),
    #[command(name = "des", version = "0.1.0", about = "进行DES加解密")]
    DES(CodecImplArgs),
}

#[derive(clap::Args, Debug)]
pub(crate) struct CodecImplArgs {
    input: String,
    #[arg(short, long, action = clap::ArgAction::Count, help = "加密模式")]
    encode: u8,
    #[arg(short, long, action = clap::ArgAction::Count, help = "解密模式")]
    decode: u8,
}

pub(crate) fn codec_auto(codec_args: &CodecArgs) {
    match &codec_args.method {
        CodecCommand::Base64(args) => { println!("{:?}", args) }
        CodecCommand::AES(args) => {}
        CodecCommand::DES(args) => {}
    }
}

fn check_codec_type(codec_impl_args: &CodecImplArgs) {
    if codec_impl_args.encode > 0 && codec_impl_args.decode > 0 {
        println!("加密模式(encode)和解密模式(decode)不能同时开启");
        process::exit(-1);
    } else if codec_impl_args.encode == 0 && codec_impl_args.decode ==0 {
        println!("需要指定为加密模式(encode)或者解密模式(decode)");
        process::exit(-1);
    }
}

fn codec_base64_auto(codec_impl_args: &CodecImplArgs) {
    check_codec_type(codec_impl_args);
    if codec_impl_args.encode > 0 {

    } else if codec_impl_args.decode > 0 {

    }
}