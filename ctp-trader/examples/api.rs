extern crate ctp_trader;

use ctp_trader::*;
use std::ffi::CStr;
use std::os::raw::c_char;

struct Spi;
impl TraderSpi for Spi {
    fn on_front_connected(&mut self) {
        println!("on_front_connected");
    }

    fn on_front_disconnected(&mut self, reason: DisconnectionReason) {
        println!("on_front_disconnected: {:?}", reason);
    }

    #[allow(unused_variables)]
    fn on_rsp_user_login(&mut self, rsp_user_login: Option<&Struct_CThostFtdcRspUserLoginField>, result: RspResult, request_id: i32, is_last: bool) {
        println!("on_rsp_user_login: .., {:?}, {:?}", request_id, is_last);
    }

    #[allow(unused_variables)]
    fn on_rsp_user_logout(&mut self, rsp_user_logout: Option<&Struct_CThostFtdcUserLogoutField>, result: RspResult, request_id: i32, is_last: bool) {
        println!("on_rsp_user_logout: .., {:?}, {:?}", request_id, is_last);
    }

    #[allow(unused_variables)]
    fn on_rsp_qry_instrument(&mut self, instrument: Option<&Struct_CThostFtdcInstrumentField>, result: RspResult, request_id: i32, is_last: bool) {
        println!("on_rsp_qry_instrument: {}, {:?}, {:?}", unsafe { CStr::from_ptr(instrument.unwrap().InstrumentID[..].as_ptr() as *const c_char) }.to_str().unwrap(), request_id, is_last);
    }

    #[allow(unused_variables)]
    fn on_rsp_error(&mut self, result: RspResult, request_id: i32, is_last: bool) {
        println!("on_rsp_error: {}, {:?}, {:?}", result.err().unwrap().msg, request_id, is_last);
    }
}

fn fill_cstr_array(array: &mut [u8], content: &str) {
    for (place, data) in array.split_last_mut().unwrap().1.iter_mut().zip(content.as_bytes().iter()) {
        *place = *data;
    }
}

fn new_login(broker_id: &str, user_id: &str, password: &str) -> Struct_CThostFtdcReqUserLoginField {
    let mut login: Struct_CThostFtdcReqUserLoginField = Default::default();
    fill_cstr_array(&mut login.BrokerID, broker_id);
    fill_cstr_array(&mut login.UserID, user_id);
    fill_cstr_array(&mut login.Password, password);
    login
}

fn new_qry_instrument(pattern: &str) -> Struct_CThostFtdcQryInstrumentField {
    let mut qry_instrument: Struct_CThostFtdcQryInstrumentField = Default::default();
    fill_cstr_array(&mut qry_instrument.InstrumentID, pattern);
    qry_instrument
}

fn main() {
    let flow_path = ::std::ffi::CString::new("").unwrap();
    let mut trader_api = TraderApi::new(flow_path);
    trader_api.register_spi(Box::new(Spi));
    trader_api.register_front(std::ffi::CString::new("tcp://180.168.146.187:10030").unwrap());
    trader_api.subscribe_private_topic(ResumeType::Quick);
    trader_api.subscribe_public_topic(ResumeType::Quick);
    trader_api.init();
    std::thread::sleep(std::time::Duration::from_secs(1));
    match trader_api.req_user_login(&new_login("9999", "036954", "lourlair"), 1) {
        Ok(()) => println!("req_user_login ok"),
        Err(err) => println!("req_user_login err: {:?}", err),
    };
    std::thread::sleep(std::time::Duration::from_secs(1));
    match trader_api.req_qry_instrument(&new_qry_instrument(""), 2) {
        Ok(()) => println!("req_qry_instrument ok"),
        Err(err) => println!("req_qry_instrument err: {:?}", err),
    };
    std::thread::sleep(std::time::Duration::from_secs(5));
    match trader_api.req_user_logout(&Default::default(), 3) {
        Ok(()) => println!("req_user_logout ok"),
        Err(err) => println!("req_user_logout err: {:?}", err),
    };
    std::thread::sleep(std::time::Duration::from_secs(1));
}
