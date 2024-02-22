use std::process;
use urlencoding;

#[derive(clap::Args, Debug)]
pub(crate) struct UrlEncodeArgs {
    url: String,
    #[arg(short, long, action = clap::ArgAction::Count, help = "加密模式")]
    encode: u8,
    #[arg(short, long, action = clap::ArgAction::Count, help = "解密模式")]
    decode: u8,
}

pub(crate) fn process_auto(url_encode_args: &UrlEncodeArgs) {
    if url_encode_args.encode > 0 && url_encode_args.decode > 0 {
        println!("加密模式(encode)和解密模式(decode)不能同时开启");
        process::exit(-1);
    } else if url_encode_args.encode == 0 && url_encode_args.decode ==0 {
        println!("需要指定为加密模式(encode)或者解密模式(decode)");
        process::exit(-1);
    }

    if url_encode_args.encode > 0 {
        process_encode(url_encode_args);
    } else if url_encode_args.decode > 0 {
        process_decode(url_encode_args);
    }
}

fn process_decode(args: &UrlEncodeArgs) {
    let result = urlencoding::encode(&args.url);
    println!("{}", result);
}

fn process_encode(args: &UrlEncodeArgs) {
    let result = urlencoding::decode(&args.url);
    match result {
        Ok(str) => { println!("{}", str) }
        Err(err) => { eprintln!("{}", err) }
    }
}