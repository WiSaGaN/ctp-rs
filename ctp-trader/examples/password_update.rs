use ctp_trader::*;
use serde::Deserialize;
use simple_error::{try_with, SimpleResult};

#[derive(Deserialize)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Config {
    pub trader_front: String,
    pub broker_id: String,
    pub user_id: String,
    pub old_password: String,
    pub new_password: String,
    pub app_id: String,
    pub auth_code: String,
}

struct Spi;
impl TraderSpi for Spi {
}

fn new_authenticate(config: &Config) -> CThostFtdcReqAuthenticateField {
    let mut f: CThostFtdcReqAuthenticateField  = Default::default();
    set_cstr_from_str_truncate(&mut f.BrokerID, &config.broker_id);
    set_cstr_from_str_truncate(&mut f.UserID, &config.user_id);
    set_cstr_from_str_truncate(&mut f.AuthCode, &config.auth_code);
    set_cstr_from_str_truncate(&mut f.AppID, &config.app_id);
    f
}

fn new_login(config: &Config) -> CThostFtdcReqUserLoginField {
    let mut f: CThostFtdcReqUserLoginField = Default::default();
    set_cstr_from_str_truncate(&mut f.BrokerID, &config.broker_id);
    set_cstr_from_str_truncate(&mut f.UserID, &config.user_id);
    set_cstr_from_str_truncate(&mut f.Password, &config.old_password);
    f
}

fn new_user_password_update(config: &Config) -> CThostFtdcUserPasswordUpdateField {
    let mut f: CThostFtdcUserPasswordUpdateField = Default::default();
    set_cstr_from_str_truncate(&mut f.BrokerID, &config.broker_id);
    set_cstr_from_str_truncate(&mut f.UserID, &config.user_id);
    set_cstr_from_str_truncate(&mut f.OldPassword, &config.old_password);
    set_cstr_from_str_truncate(&mut f.NewPassword, &config.new_password);
    f
}

fn main() -> SimpleResult<()> {
    let yaml_str = try_with!(std::fs::read_to_string("config.password_update.yml"), "cannot read config.password_update.yml");
    let config: Config = try_with!(serde_yaml::from_str(&yaml_str), "cannot parse config");

    let mut last_request_id = 0;
    let flow_path = ::std::ffi::CString::new("").unwrap();
    let mut trader_api = TraderApi::new(flow_path);
    trader_api.register_spi(Box::new(Spi));
    trader_api.register_front(std::ffi::CString::new(config.trader_front.clone()).unwrap());
    trader_api.subscribe_private_topic(ResumeType::Quick);
    trader_api.subscribe_public_topic(ResumeType::Quick);
    trader_api.init();
    std::thread::sleep(std::time::Duration::from_secs(2));
    last_request_id += 1;
    match trader_api.req_authenticate(&new_authenticate(&config), last_request_id) {
        Ok(()) => println!("req_authenticate ok"),
        Err(err) => println!("req_authenticate err: {:?}", err),
    };
    std::thread::sleep(std::time::Duration::from_secs(2));
    last_request_id += 1;
    match trader_api.req_user_login(&new_login(&config), last_request_id) {
        Ok(()) => println!("req_user_login ok"),
        Err(err) => println!("req_user_login err: {:?}", err),
    };
    std::thread::sleep(std::time::Duration::from_secs(2));

    last_request_id += 1;
    let f = new_user_password_update(&config);
    match trader_api.req_user_password_update(&f, last_request_id) {
        Ok(()) => println!("req_user_password_update ok"),
        Err(err) => println!("req_user_password_update err: {:?}", err),
    };
    std::thread::sleep(std::time::Duration::from_secs(10));
    Ok(())
}
