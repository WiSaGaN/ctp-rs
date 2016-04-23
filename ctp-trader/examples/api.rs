extern crate ctp_trader;

use ctp_trader::*;
use std::io::Write;

const TRADER_FRONT: &'static str = "tcp://180.168.146.187:10030";
const BROKER_ID: &'static str = "9999";
struct Spi;
impl TraderSpi for Spi {
}

fn new_login(user_id: &str, password: &str) -> Struct_CThostFtdcReqUserLoginField {
    let mut f: Struct_CThostFtdcReqUserLoginField = Default::default();
    set_cstr_from_str_truncate(&mut f.BrokerID, BROKER_ID);
    set_cstr_from_str_truncate(&mut f.UserID, user_id);
    set_cstr_from_str_truncate(&mut f.Password, password);
    f
}

fn new_qry_settlement_info(user_id: &str) -> Struct_CThostFtdcQrySettlementInfoField {
    let mut f: Struct_CThostFtdcQrySettlementInfoField = Default::default();
    set_cstr_from_str_truncate(&mut f.BrokerID, BROKER_ID);
    set_cstr_from_str_truncate(&mut f.InvestorID, user_id);
    f
}

fn new_settlement_info_confirm(user_id: &str) -> Struct_CThostFtdcSettlementInfoConfirmField {
    let mut f: Struct_CThostFtdcSettlementInfoConfirmField = Default::default();
    set_cstr_from_str_truncate(&mut f.BrokerID, BROKER_ID);
    set_cstr_from_str_truncate(&mut f.InvestorID, user_id);
    f
}

fn new_qry_settlement_info_confirm(user_id: &str) -> Struct_CThostFtdcQrySettlementInfoConfirmField {
    let mut f: Struct_CThostFtdcQrySettlementInfoConfirmField = Default::default();
    set_cstr_from_str_truncate(&mut f.BrokerID, BROKER_ID);
    set_cstr_from_str_truncate(&mut f.InvestorID, user_id);
    f
}

fn new_qry_instrument(pattern: &str) -> Struct_CThostFtdcQryInstrumentField {
    let mut f: Struct_CThostFtdcQryInstrumentField = Default::default();
    set_cstr_from_str_truncate(&mut f.InstrumentID, pattern);
    f
}

fn new_qry_exchange(pattern: &str) -> Struct_CThostFtdcQryExchangeField {
    let mut f: Struct_CThostFtdcQryExchangeField = Default::default();
    set_cstr_from_str_truncate(&mut f.ExchangeID, pattern);
    f
}

fn new_qry_product(pattern: &str) -> Struct_CThostFtdcQryProductField {
    let mut f: Struct_CThostFtdcQryProductField = Default::default();
    set_cstr_from_str_truncate(&mut f.ProductID, pattern);
    f
}

fn new_qry_order(user_id: &str) -> Struct_CThostFtdcQryOrderField {
    let mut f: Struct_CThostFtdcQryOrderField = Default::default();
    set_cstr_from_str_truncate(&mut f.BrokerID, BROKER_ID);
    set_cstr_from_str_truncate(&mut f.InvestorID, user_id);
    f
}

fn new_qry_trade(user_id: &str) -> Struct_CThostFtdcQryTradeField {
    let mut f: Struct_CThostFtdcQryTradeField = Default::default();
    set_cstr_from_str_truncate(&mut f.BrokerID, BROKER_ID);
    set_cstr_from_str_truncate(&mut f.InvestorID, user_id);
    f
}

fn new_qry_investor_position(user_id: &str) -> Struct_CThostFtdcQryInvestorPositionField {
    let mut f: Struct_CThostFtdcQryInvestorPositionField = Default::default();
    set_cstr_from_str_truncate(&mut f.BrokerID, BROKER_ID);
    set_cstr_from_str_truncate(&mut f.InvestorID, user_id);
    f
}

fn new_qry_trading_account(user_id: &str) -> Struct_CThostFtdcQryTradingAccountField {
    let mut f: Struct_CThostFtdcQryTradingAccountField = Default::default();
    set_cstr_from_str_truncate(&mut f.BrokerID, BROKER_ID);
    set_cstr_from_str_truncate(&mut f.InvestorID, user_id);
    f
}

fn new_qry_investor(user_id: &str) -> Struct_CThostFtdcQryInvestorField {
    let mut f: Struct_CThostFtdcQryInvestorField = Default::default();
    set_cstr_from_str_truncate(&mut f.BrokerID, BROKER_ID);
    set_cstr_from_str_truncate(&mut f.InvestorID, user_id);
    f
}

fn new_qry_trading_code(user_id: &str) -> Struct_CThostFtdcQryTradingCodeField {
    let mut f: Struct_CThostFtdcQryTradingCodeField = Default::default();
    set_cstr_from_str_truncate(&mut f.BrokerID, BROKER_ID);
    set_cstr_from_str_truncate(&mut f.InvestorID, user_id);
    f
}

fn new_input_order(pattern: &str) -> Struct_CThostFtdcInputOrderField {
    let mut f: Struct_CThostFtdcInputOrderField = Default::default();
    f.TimeCondition = THOST_FTDC_TC_GFD;
    f.OrderPriceType = THOST_FTDC_OPT_LIMITPRICE;
    f.LimitPrice = 1f64;
    f
}

fn new_input_order_action(pattern: &str) -> Struct_CThostFtdcInputOrderActionField {
    let mut f: Struct_CThostFtdcInputOrderActionField = Default::default();
    f.ActionFlag = THOST_FTDC_AF_DELETE;
    f
}

fn main() {
    println!("Going to connect to simnow {} with broker_id {}", TRADER_FRONT, BROKER_ID);
    let mut user_id = String::new();
    print!("user_id: ");
    std::io::stdout().flush().unwrap();
    match std::io::stdin().read_line(&mut user_id) {
        Ok(_) => (),
        Err(e) => {
            println!("invalid user_id, {}", e);
        },
    }
    user_id = user_id.trim_right().to_string();
    let mut password = String::new();
    print!("password: ");
    std::io::stdout().flush().unwrap();
    match std::io::stdin().read_line(&mut password) {
        Ok(_) => (),
        Err(e) => {
            println!("invalid password, {}", e);
        },
    }
    password = password.trim_right().to_string();
    let mut last_request_id = 0;
    let flow_path = ::std::ffi::CString::new("").unwrap();
    let mut trader_api = TraderApi::new(flow_path);
    trader_api.register_spi(Box::new(Spi));
    trader_api.register_front(std::ffi::CString::new(TRADER_FRONT).unwrap());
    trader_api.subscribe_private_topic(ResumeType::Quick);
    trader_api.subscribe_public_topic(ResumeType::Quick);
    trader_api.init();
    std::thread::sleep(std::time::Duration::from_secs(1));
    last_request_id += 1;
    match trader_api.req_user_login(&new_login(&user_id, &password), last_request_id) {
        Ok(()) => println!("req_user_login ok"),
        Err(err) => println!("req_user_login err: {:?}", err),
    };
    std::thread::sleep(std::time::Duration::from_secs(1));
    last_request_id += 1;
    match trader_api.req_qry_instrument(&new_qry_instrument(""), last_request_id) {
        Ok(()) => println!("req_qry_instrument ok"),
        Err(err) => println!("req_qry_instrument err: {:?}", err),
    };
    std::thread::sleep(std::time::Duration::from_secs(2));
    last_request_id += 1;
    match trader_api.req_qry_product(&new_qry_product(""), last_request_id) {
        Ok(()) => println!("req_qry_product ok"),
        Err(err) => println!("req_qry_product err: {:?}", err),
    };
    std::thread::sleep(std::time::Duration::from_secs(1));
    last_request_id += 1;
    match trader_api.req_qry_exchange(&new_qry_exchange(""), last_request_id) {
        Ok(()) => println!("req_qry_exchange ok"),
        Err(err) => println!("req_qry_exchange err: {:?}", err),
    };
    std::thread::sleep(std::time::Duration::from_secs(1));
    last_request_id += 1;
    match trader_api.req_qry_settlement_info(&new_qry_settlement_info(&user_id), last_request_id) {
        Ok(()) => println!("req_qry_settlement_info ok"),
        Err(err) => println!("req_qry_settlement_info err: {:?}", err),
    };
    std::thread::sleep(std::time::Duration::from_secs(1));
    last_request_id += 1;
    match trader_api.req_qry_settlement_info_confirm(&new_qry_settlement_info_confirm(&user_id), last_request_id) {
        Ok(()) => println!("req_qry_settlement_info_confirm ok"),
        Err(err) => println!("req_qry_settlement_info_confirm err: {:?}", err),
    };
    std::thread::sleep(std::time::Duration::from_secs(1));
    last_request_id += 1;
    match trader_api.req_settlement_info_confirm(&new_settlement_info_confirm(&user_id), last_request_id) {
        Ok(()) => println!("req_settlement_info_confirm ok"),
        Err(err) => println!("req_settlement_info_confirm err: {:?}", err),
    };
    std::thread::sleep(std::time::Duration::from_secs(1));
    last_request_id += 1;
    match trader_api.req_qry_order(&new_qry_order(&user_id), last_request_id) {
        Ok(()) => println!("req_qry_order ok"),
        Err(err) => println!("req_qry_order err: {:?}", err),
    };
    std::thread::sleep(std::time::Duration::from_secs(1));
    last_request_id += 1;
    match trader_api.req_qry_trade(&new_qry_trade(&user_id), last_request_id) {
        Ok(()) => println!("req_qry_trade ok"),
        Err(err) => println!("req_qry_trade err: {:?}", err),
    };
    std::thread::sleep(std::time::Duration::from_secs(1));
    last_request_id += 1;
    match trader_api.req_qry_investor_position(&new_qry_investor_position(&user_id), last_request_id) {
        Ok(()) => println!("req_qry_investor_position ok"),
        Err(err) => println!("req_qry_investor_position err: {:?}", err),
    };
    std::thread::sleep(std::time::Duration::from_secs(1));
    last_request_id += 1;
    match trader_api.req_qry_trading_account(&new_qry_trading_account(&user_id), last_request_id) {
        Ok(()) => println!("req_qry_trading_account ok"),
        Err(err) => println!("req_qry_trading_account err: {:?}", err),
    };
    std::thread::sleep(std::time::Duration::from_secs(1));
    last_request_id += 1;
    match trader_api.req_qry_investor(&new_qry_investor(&user_id), last_request_id) {
        Ok(()) => println!("req_qry_investor ok"),
        Err(err) => println!("req_qry_investor err: {:?}", err),
    };
    std::thread::sleep(std::time::Duration::from_secs(1));
    last_request_id += 1;
    match trader_api.req_qry_trading_code(&new_qry_trading_code(&user_id), last_request_id) {
        Ok(()) => println!("req_qry_trading_code ok"),
        Err(err) => println!("req_qry_trading_code err: {:?}", err),
    };
    std::thread::sleep(std::time::Duration::from_secs(1));
    last_request_id += 1;
    match trader_api.req_order_insert(&new_input_order(""), last_request_id) {
        Ok(()) => println!("req_order_insert ok"),
        Err(err) => println!("req_order_insert err: {:?}", err),
    };
    std::thread::sleep(std::time::Duration::from_secs(1));
    last_request_id += 1;
    match trader_api.req_order_action(&new_input_order_action(""), last_request_id) {
        Ok(()) => println!("req_order_action ok"),
        Err(err) => println!("req_order_action err: {:?}", err),
    };
    std::thread::sleep(std::time::Duration::from_secs(1));
}
