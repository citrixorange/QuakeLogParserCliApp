extern crate quake_log_parser_lib;

use quake_log_parser_lib::{
    config::config::{CONFIG_FILE_PATH},
    implementation::log_parser::LOG_FILE_PATH,
    lib::factory
};

use clap::Parser;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    config_file: String,

    #[arg(short, long)]
    log_file: String,
}

#[tokio::main]
async fn main() {
    
    let args = Args::parse();

    CONFIG_FILE_PATH.with(|config_file_path_handler| {
        *config_file_path_handler.borrow_mut() = Some(args.config_file);
    });

    LOG_FILE_PATH.with(|log_file_path_handler| {
        *log_file_path_handler.borrow_mut() = Some(args.log_file);
    });
    
    let mut log_parser = factory();

    if let Ok(value) = log_parser.parse_file().await {
        println!("{}", value);
    } else {
        println!("Error Parsing Log File");
    }


}
