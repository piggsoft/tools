use urlencoding;

#[derive(clap::Args, Debug)]
pub(crate) struct UrlEncodeArgs {
    url: String,
    #[arg(required = false, long = "type", short = 't', default_value = "encode", help = "encode代表加密，decode代表节目，默认不传为：decode")]
    en_type: String,
}

pub(crate) fn process_auto(args: &UrlEncodeArgs) {
    if args.en_type == "encode" {
        process_encode(args);
    } else if args.en_type == "decode" {
        process_decode(args);
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