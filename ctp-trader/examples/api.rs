extern crate ctp_trader;

use std::ffi::{ CStr, CString };
use ctp_trader::*;

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
    fn on_rsp_error(&mut self, result: RspResult, request_id: i32, is_last: bool) {
        println!("on_rsp_error: {}, {:?}, {:?}", result.err().unwrap().msg, request_id, is_last);
    }
}

fn main() {
    let flow_path = ::std::ffi::CString::new("").unwrap();
    let mut trader_api = TraderApi::new(flow_path);
    trader_api.register_spi(Box::new(Spi));
    trader_api.register_front(std::ffi::CString::new("tcp://180.168.146.187:10030").unwrap());
    trader_api.init();
    std::thread::sleep(std::time::Duration::from_secs(1));
    match trader_api.req_user_login(&Default::default(), 1) {
        Ok(()) => println!("req_user_login ok"),
        Err(err) => println!("req_user_login err: {:?}", err),
    };
    std::thread::sleep(std::time::Duration::from_secs(1));
    std::thread::sleep(std::time::Duration::from_secs(5));
    /*
    match trader_api.req_user_logout(&Default::default(), 2) {
        Ok(()) => println!("req_user_logout ok"),
        Err(err) => println!("req_user_logout err: {:?}", err),
    };
    std::thread::sleep(std::time::Duration::from_secs(1));
    */
}
