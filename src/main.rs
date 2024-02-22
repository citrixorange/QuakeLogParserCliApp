extern crate quake_log_parser_lib;

use quake_log_parser_lib::{
    errors::LogParserError,
    config::config::{ConfigValue},
    config::dynamic_config::{CONFIG_FILE_PATH, CONFIG, ConfigParameter},
    interface::CallbackPayload,
    lib::factory
};

use clap::Parser;
use log::{info, warn};
use env_logger::Env;
use serde_json::Value;
use chrono::Utc;


#[derive(Parser, Debug)]
#[command(author="citrixorange", version, about="A cli app for parsing local Quake log files", long_about = None)]
struct Args {
    #[arg(short, long, help="path for config .json file")]
    config_file: String,

    #[arg(short, long, help="path for log .log file")]
    log_file: String,

    #[arg(short, long, help="include kill means in match stats")]
    enable_kill_by_means: Option<bool>,

    #[arg(short, long, help="if true, self kill increases player kill score")]
    self_kill_score_increase: Option<bool>,

    #[arg(short, long, help="if true, being killed decreases player kill score")]
    being_killed_decreases_score: Option<bool>,
}

pub async fn success_callback(success_data: Option<Value>) -> Result<(), LogParserError> {
    if let Some(data) = success_data {
        let payload_result: Result<CallbackPayload, _> = serde_json::from_value(data);
        if let Ok(payload) = payload_result {
            if let Some (parsed_log) = payload.data {
                let formatted_parsed_log = format!("parsed_log: {}", parsed_log);
                info!("{}", formatted_parsed_log);
                return Ok(());
            } else {
                return Err(LogParserError::UnexpectedError);
            }
        } else {
            return Err(LogParserError::UnexpectedError);
        }
    } else {
        return Err(LogParserError::UnexpectedError);
    }
}

pub async fn warning_callback(warning_data: Option<Value>) -> Result<(), LogParserError> {
    if let Some(data) = warning_data {
        
        let mut warn_log = String::from("");

        let ts = format!("{} | ", Utc::now());

        warn_log.push_str(&ts);

        let payload_result: Result<CallbackPayload, _> = serde_json::from_value(data);

        if let Ok(payload) = payload_result {

            if let Some (error) = payload.error {
                let error_field = format!("{} | ", error);
                warn_log.push_str(&error_field);
            } else {
                let error_field = format!("ErrorNotProvided | ");
                warn_log.push_str(&error_field);
            }
    
            if let Some (line) = payload.data {
                let line_field = format!("log_line: {}", line);
                warn_log.push_str(&line_field);
            } else {
                let line_field = format!("log_line not provided");
                warn_log.push_str(&line_field);
            }
    
            warn!("{}", warn_log);
        }
    }

    return Ok(());
}

#[tokio::main]
async fn main() {

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    
    let args = Args::parse();

    CONFIG_FILE_PATH.with(|config_file_path_handler| {
        *config_file_path_handler.borrow_mut() = Some(args.config_file);
    });

    CONFIG.with(|config| {

        config.borrow_mut().set_parameter(ConfigParameter::LogFilePath, ConfigValue::Str(args.log_file));

        if let Some(show_kill_by_means) = args.enable_kill_by_means {
            config.borrow_mut().set_parameter(ConfigParameter::ShowDeathCauses, ConfigValue::Bool(show_kill_by_means));
        }
    
        if let Some(self_kill_score_increase) = args.self_kill_score_increase {
            config.borrow_mut().set_parameter(ConfigParameter::KillYourselfIncreasesScore, ConfigValue::Bool(self_kill_score_increase));
        }
    
        if let Some(being_killed_decreases_score) = args.being_killed_decreases_score {
            config.borrow_mut().set_parameter(ConfigParameter::BeingKilledDecreasesScore, ConfigValue::Bool(being_killed_decreases_score));
        }

    });
    
    let mut log_parser = factory();

    log_parser.register_success_callback(Box::new(|payload| Box::pin(success_callback(payload))));
    log_parser.register_warning_callback(Box::new(|payload| Box::pin(warning_callback(payload))));

    let _res = log_parser.parse_file().await;

}
