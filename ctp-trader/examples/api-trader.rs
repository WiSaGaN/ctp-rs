use std::io::Write;

use ctp_trader::*;

const TRADER_FRONT: &'static str = "tcp://180.168.146.187:10030";
const BROKER_ID: &'static str = "9999";
struct Spi;
impl TraderSpi for Spi {
}

fn new_login(user_id: &str, password: &str) -> CThostFtdcReqUserLoginField {
    let mut f: CThostFtdcReqUserLoginField = Default::default();
    set_cstr_from_str_truncate(&mut f.BrokerID, BROKER_ID);
    set_cstr_from_str_truncate(&mut f.UserID, user_id);
    set_cstr_from_str_truncate(&mut f.Password, password);
    f
}

fn new_qry_settlement_info(user_id: &str) -> CThostFtdcQrySettlementInfoField {
    let mut f: CThostFtdcQrySettlementInfoField = Default::default();
    set_cstr_from_str_truncate(&mut f.BrokerID, BROKER_ID);
    set_cstr_from_str_truncate(&mut f.InvestorID, user_id);
    f
}

fn new_settlement_info_confirm(user_id: &str) -> CThostFtdcSettlementInfoConfirmField {
    let mut f: CThostFtdcSettlementInfoConfirmField = Default::default();
    set_cstr_from_str_truncate(&mut f.BrokerID, BROKER_ID);
    set_cstr_from_str_truncate(&mut f.InvestorID, user_id);
    f
}

fn new_qry_settlement_info_confirm(user_id: &str) -> CThostFtdcQrySettlementInfoConfirmField {
    let mut f: CThostFtdcQrySettlementInfoConfirmField = Default::default();
    set_cstr_from_str_truncate(&mut f.BrokerID, BROKER_ID);
    set_cstr_from_str_truncate(&mut f.InvestorID, user_id);
    f
}

fn new_qry_instrument(pattern: &str) -> CThostFtdcQryInstrumentField {
    let mut f: CThostFtdcQryInstrumentField = Default::default();
    set_cstr_from_str_truncate(&mut f.InstrumentID, pattern);
    f
}

fn new_qry_exchange(pattern: &str) -> CThostFtdcQryExchangeField {
    let mut f: CThostFtdcQryExchangeField = Default::default();
    set_cstr_from_str_truncate(&mut f.ExchangeID, pattern);
    f
}

fn new_qry_product(pattern: &str) -> CThostFtdcQryProductField {
    let mut f: CThostFtdcQryProductField = Default::default();
    set_cstr_from_str_truncate(&mut f.ProductID, pattern);
    f
}

fn new_qry_order(user_id: &str) -> CThostFtdcQryOrderField {
    let mut f: CThostFtdcQryOrderField = Default::default();
    set_cstr_from_str_truncate(&mut f.BrokerID, BROKER_ID);
    set_cstr_from_str_truncate(&mut f.InvestorID, user_id);
    f
}

fn new_qry_trade(user_id: &str) -> CThostFtdcQryTradeField {
    let mut f: CThostFtdcQryTradeField = Default::default();
    set_cstr_from_str_truncate(&mut f.BrokerID, BROKER_ID);
    set_cstr_from_str_truncate(&mut f.InvestorID, user_id);
    f
}

fn new_qry_investor_position(user_id: &str) -> CThostFtdcQryInvestorPositionField {
    let mut f: CThostFtdcQryInvestorPositionField = Default::default();
    set_cstr_from_str_truncate(&mut f.BrokerID, BROKER_ID);
    set_cstr_from_str_truncate(&mut f.InvestorID, user_id);
    f
}

fn new_qry_trading_account(user_id: &str) -> CThostFtdcQryTradingAccountField {
    let mut f: CThostFtdcQryTradingAccountField = Default::default();
    set_cstr_from_str_truncate(&mut f.BrokerID, BROKER_ID);
    set_cstr_from_str_truncate(&mut f.InvestorID, user_id);
    f
}

fn new_qry_investor(user_id: &str) -> CThostFtdcQryInvestorField {
    let mut f: CThostFtdcQryInvestorField = Default::default();
    set_cstr_from_str_truncate(&mut f.BrokerID, BROKER_ID);
    set_cstr_from_str_truncate(&mut f.InvestorID, user_id);
    f
}

fn new_qry_trading_code(user_id: &str) -> CThostFtdcQryTradingCodeField {
    let mut f: CThostFtdcQryTradingCodeField = Default::default();
    set_cstr_from_str_truncate(&mut f.BrokerID, BROKER_ID);
    set_cstr_from_str_truncate(&mut f.InvestorID, user_id);
    f
}

fn new_input_order(user_id: &str) -> CThostFtdcInputOrderField {
    let mut f: CThostFtdcInputOrderField = Default::default();
    set_cstr_from_str_truncate(&mut f.BrokerID, BROKER_ID);
    set_cstr_from_str_truncate(&mut f.InvestorID, user_id);
    set_cstr_from_str_truncate(&mut f.InstrumentID, "IF1703");
    set_cstr_from_str_truncate(&mut f.UserID, user_id);
    f.Direction = THOST_FTDC_D_Buy;
    f.OrderPriceType = THOST_FTDC_OPT_LimitPrice;
    f.LimitPrice = 1f64;
    f.VolumeTotalOriginal = 1;
    f.CombOffsetFlag[0] = THOST_FTDC_OF_Open;
    f.CombHedgeFlag[0] = THOST_FTDC_HF_Speculation;
    f.TimeCondition = THOST_FTDC_TC_GFD;
    f.VolumeCondition = THOST_FTDC_VC_AV;
    f.MinVolume = 1;
    f.ContingentCondition = THOST_FTDC_CC_Immediately;
    f.ForceCloseReason = THOST_FTDC_FCC_NotForceClose;
    f.RequestID = 20;
    f
}

fn new_input_order_action() -> CThostFtdcInputOrderActionField {
    let mut f: CThostFtdcInputOrderActionField = Default::default();
    f.ActionFlag = THOST_FTDC_AF_Delete;
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
    user_id = user_id.trim_end().to_string();
    let mut password = String::new();
    print!("password: ");
    std::io::stdout().flush().unwrap();
    match std::io::stdin().read_line(&mut password) {
        Ok(_) => (),
        Err(e) => {
            println!("invalid password, {}", e);
        },
    }
    password = password.trim_end().to_string();
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
    match trader_api.req_order_insert(&new_input_order(&user_id), last_request_id) {
        Ok(()) => println!("req_order_insert ok"),
        Err(err) => println!("req_order_insert err: {:?}", err),
    };
    std::thread::sleep(std::time::Duration::from_secs(1));
    last_request_id += 1;
    match trader_api.req_order_action(&new_input_order_action(), last_request_id) {
        Ok(()) => println!("req_order_action ok"),
        Err(err) => println!("req_order_action err: {:?}", err),
    };
    std::thread::sleep(std::time::Duration::from_secs(1));
}
