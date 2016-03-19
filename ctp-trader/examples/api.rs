extern crate ctp_trader;

use ctp_trader::*;
use std::ffi::CStr;
use std::os::raw::c_char;

struct Spi;
impl TraderSpi for Spi {
    #[allow(unused_variables)]
    fn on_rsp_qry_instrument(&mut self, instrument: Option<&Struct_CThostFtdcInstrumentField>, result: RspResult, request_id: i32, is_last: bool) {
        let inst = instrument.unwrap();
        unsafe {
            println!("on_rsp_qry_instrument: {}, {}, {}, {}, {}, {:?}, {:?}",
                 to_str(&inst.InstrumentID[..]),
                 to_str(&inst.ExchangeID[..]),
                 gb18030_cstr_to_string(CStr::from_ptr(inst.InstrumentName[..].as_ptr() as *const c_char)),
                 to_str(&inst.ExchangeInstID[..]),
                 to_str(&inst.ProductID[..]),
                 request_id,
                 is_last);
        }
    }
}

fn fill_cstr_array(array: &mut [u8], content: &str) {
    for (place, data) in array.split_last_mut().unwrap().1.iter_mut().zip(content.as_bytes().iter()) {
        *place = *data;
    }
}

unsafe fn to_str(array: &[u8]) -> &str {
    CStr::from_ptr(array.as_ptr() as *const c_char).to_str().unwrap()
}

fn new_login(broker_id: &str, user_id: &str, password: &str) -> Struct_CThostFtdcReqUserLoginField {
    let mut login: Struct_CThostFtdcReqUserLoginField = Default::default();
    fill_cstr_array(&mut login.BrokerID, broker_id);
    fill_cstr_array(&mut login.UserID, user_id);
    fill_cstr_array(&mut login.Password, password);
    login
}

fn new_logout(broker_id: &str, user_id: &str) -> Struct_CThostFtdcUserLogoutField {
    let mut logout: Struct_CThostFtdcUserLogoutField = Default::default();
    fill_cstr_array(&mut logout.BrokerID, broker_id);
    fill_cstr_array(&mut logout.UserID, user_id);
    logout
}

fn new_qry_settlement_info(broker_id: &str, investor_id: &str) -> Struct_CThostFtdcQrySettlementInfoField {
    let mut f: Struct_CThostFtdcQrySettlementInfoField = Default::default();
    fill_cstr_array(&mut f.BrokerID, broker_id);
    fill_cstr_array(&mut f.InvestorID, investor_id);
    f
}

fn new_settlement_info_confirm(broker_id: &str, investor_id: &str) -> Struct_CThostFtdcSettlementInfoConfirmField {
    let mut f: Struct_CThostFtdcSettlementInfoConfirmField = Default::default();
    fill_cstr_array(&mut f.BrokerID, broker_id);
    fill_cstr_array(&mut f.InvestorID, investor_id);
    f
}

fn new_qry_settlement_info_confirm(broker_id: &str, investor_id: &str) -> Struct_CThostFtdcQrySettlementInfoConfirmField {
    let mut f: Struct_CThostFtdcQrySettlementInfoConfirmField = Default::default();
    fill_cstr_array(&mut f.BrokerID, broker_id);
    fill_cstr_array(&mut f.InvestorID, investor_id);
    f
}

fn new_qry_instrument(pattern: &str) -> Struct_CThostFtdcQryInstrumentField {
    let mut f: Struct_CThostFtdcQryInstrumentField = Default::default();
    fill_cstr_array(&mut f.InstrumentID, pattern);
    f
}

fn new_qry_order(pattern: &str) -> Struct_CThostFtdcQryOrderField {
    let mut f: Struct_CThostFtdcQryOrderField = Default::default();
    fill_cstr_array(&mut f.InvestorID, pattern);
    f
}

fn new_qry_trade(pattern: &str) -> Struct_CThostFtdcQryTradeField {
    let mut f: Struct_CThostFtdcQryTradeField = Default::default();
    fill_cstr_array(&mut f.InvestorID, pattern);
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
    let mut last_request_id = 0;
    let flow_path = ::std::ffi::CString::new("").unwrap();
    let mut trader_api = TraderApi::new(flow_path);
    trader_api.register_spi(Box::new(Spi));
    trader_api.register_front(std::ffi::CString::new("tcp://180.168.146.187:10030").unwrap());
    trader_api.subscribe_private_topic(ResumeType::Quick);
    trader_api.subscribe_public_topic(ResumeType::Quick);
    trader_api.init();
    std::thread::sleep(std::time::Duration::from_secs(1));
    last_request_id += 1;
    match trader_api.req_user_login(&new_login("9999", "036954", "lourlair"), last_request_id) {
        Ok(()) => println!("req_user_login ok"),
        Err(err) => println!("req_user_login err: {:?}", err),
    };
    std::thread::sleep(std::time::Duration::from_secs(1));
    last_request_id += 1;
    match trader_api.req_qry_instrument(&new_qry_instrument(""), last_request_id) {
        Ok(()) => println!("req_qry_instrument ok"),
        Err(err) => println!("req_qry_instrument err: {:?}", err),
    };
    std::thread::sleep(std::time::Duration::from_secs(1));
    last_request_id += 1;
    match trader_api.req_qry_settlement_info(&new_qry_settlement_info("9999", "036954"), last_request_id) {
        Ok(()) => println!("req_qry_settlement_info ok"),
        Err(err) => println!("req_qry_settlement_info err: {:?}", err),
    };
    std::thread::sleep(std::time::Duration::from_secs(1));
    last_request_id += 1;
    match trader_api.req_qry_settlement_info_confirm(&new_qry_settlement_info_confirm("9999", "036954"), last_request_id) {
        Ok(()) => println!("req_qry_settlement_info_confirm ok"),
        Err(err) => println!("req_qry_settlement_info_confirm err: {:?}", err),
    };
    std::thread::sleep(std::time::Duration::from_secs(1));
    last_request_id += 1;
    match trader_api.req_settlement_info_confirm(&new_settlement_info_confirm("9999", "036954"), last_request_id) {
        Ok(()) => println!("req_settlement_info_confirm ok"),
        Err(err) => println!("req_settlement_info_confirm err: {:?}", err),
    };
    std::thread::sleep(std::time::Duration::from_secs(1));
    last_request_id += 1;
    match trader_api.req_qry_order(&new_qry_order(""), last_request_id) {
        Ok(()) => println!("req_qry_order ok"),
        Err(err) => println!("req_qry_order err: {:?}", err),
    };
    std::thread::sleep(std::time::Duration::from_secs(1));
    last_request_id += 1;
    match trader_api.req_qry_trade(&new_qry_trade(""), last_request_id) {
        Ok(()) => println!("req_qry_trade ok"),
        Err(err) => println!("req_qry_trade err: {:?}", err),
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
    /*
    match trader_api.req_user_logout(&new_logout("9999", "036954"), 99) {
        Ok(()) => println!("req_user_logout ok"),
        Err(err) => println!("req_user_logout err: {:?}", err),
    };
    std::thread::sleep(std::time::Duration::from_secs(1));
    */
}