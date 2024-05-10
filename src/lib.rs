#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![doc = include_str!("../README.md")]
#![allow(non_upper_case_globals)]
#![allow(clippy::needless_doctest_main)]
#![warn(missing_docs)]
#![allow(clippy::type_complexity)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::arc_with_non_send_sync)]

mod lib_tests;
pub mod rs_box_log;
use std::fs;
use toml::Value;

#[derive(Clone, Copy, Debug,PartialEq)]
pub enum RunMode {
    RunModeUnknown,
    RunModeDebug,
    RunModeRelease,
    RunModeTest,
}
impl RunMode {
    pub fn to_string(&self) -> &'static str {
        match *self {
            RunMode::RunModeDebug => "Debug",
            RunMode::RunModeRelease => "Release",
            RunMode::RunModeTest => "Test",
            RunMode::RunModeUnknown => "Unknown",
        }
    }
}

static mut CURRENT_RUN_MODE: RunMode = RunMode::RunModeUnknown;

pub fn get_current_run_mode() -> RunMode {
    unsafe { CURRENT_RUN_MODE }
}

pub fn rs_box_setup(project_name: &str, run_mode: RunMode, product_log_dir: &str, log_max_save_days: u64, http_request_timeout: u64) {

    let enable_save_log_file = match run_mode {
        RunMode::RunModeRelease | RunMode::RunModeTest => true,
        _ => false,
    };

    let log_level = match run_mode {
        RunMode::RunModeDebug | RunMode::RunModeTest => crate::rs_box_log::rs_box_log::LogLevel::LogLevelDebug,
        RunMode::RunModeRelease => crate::rs_box_log::rs_box_log::LogLevel::LogLevelInfo,
        _ => crate::rs_box_log::rs_box_log::LogLevel::LogLevelDebug,
    };

    crate::rs_box_log::rs_box_log::setup_log_tools("test_project",enable_save_log_file,product_log_dir,rs_box_log::rs_box_log::LogLevel::LogLevelDebug,7,rs_box_log::rs_box_log::LogFileSaveType::LogFileSaveTypeDays);

    unsafe {
        CURRENT_RUN_MODE = run_mode;
    }

    println!("Rust Developer Tool Box Setup End");
    println!("project_name=[{}]", project_name);
    println!("RunMode=[{}]", run_mode.to_string());
    println!("LogLevel=[{:?}]", log_level);
    println!("main_log_dir=[{}]", product_log_dir.to_string());
    println!("log_save_days_max=[{}]", log_max_save_days);
    println!("Http_request_timeout=[{} Second]", http_request_timeout);
}

pub fn get_version() -> Result<String, Box<dyn std::error::Error>>{
    // 读取当前目录下的 Cargo.toml 文件
    let contents = fs::read_to_string("Cargo.toml")?;

    // 解析 TOML 内容
    let parsed = contents.parse::<Value>()?;

    // 尝试从解析后的 TOML 数据中获取版本号
    let version = parsed
        .get("package")
        .and_then(|pkg| pkg.get("version"))
        .and_then(|v| v.as_str())
        .ok_or("Version not found or is not a string")?
        .to_string();

    Ok(version)
}




