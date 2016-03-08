extern crate ctp_md;

use std::ffi::{ CStr, CString };
use ctp_md::*;

struct Spi;
impl MdSpi for Spi {
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

    #[allow(unused_variables)]
    fn on_rsp_sub_market_data(&mut self, specific_instrument: Option<&Struct_CThostFtdcSpecificInstrumentField>, result: RspResult, request_id: i32, is_last: bool) {
        println!("on_rsp_sub_market_data: {}, {:?}, {:?}", specific_instrument_to_cstr(specific_instrument.unwrap()).to_str().expect("CThostFtdcSpecificInstrumentField has invalid UTF8 in InstrumentID"), request_id, is_last);
    }

    #[allow(unused_variables)]
    fn on_rsp_un_sub_market_data(&mut self, specific_instrument: Option<&Struct_CThostFtdcSpecificInstrumentField>, result: RspResult, request_id: i32, is_last: bool) {
        println!("on_rsp_un_sub_market_data: {}, {:?}, {:?}", specific_instrument_to_cstr(specific_instrument.unwrap()).to_str().expect("CThostFtdcSpecificInstrumentField has invalid UTF8 in InstrumentID"), request_id, is_last);
    }

    #[allow(unused_variables)]
    fn on_rsp_sub_for_quote_rsp(&mut self, specific_instrument: Option<&Struct_CThostFtdcSpecificInstrumentField>, result: RspResult, request_id: i32, is_last: bool) {
        println!("on_rsp_sub_for_quote_rsp: {}, {:?}, {:?}", specific_instrument_to_cstr(specific_instrument.unwrap()).to_str().expect("CThostFtdcSpecificInstrumentField has invalid UTF8 in InstrumentID"), request_id, is_last);
    }

    #[allow(unused_variables)]
    fn on_rsp_un_sub_for_quote_rsp(&mut self, specific_instrument: Option<&Struct_CThostFtdcSpecificInstrumentField>, result: RspResult, request_id: i32, is_last: bool) {
        println!("on_rsp_un_sub_for_quote_rsp: {}, {:?}, {:?}", specific_instrument_to_cstr(specific_instrument.unwrap()).to_str().expect("CThostFtdcSpecificInstrumentField has invalid UTF8 in InstrumentID"), request_id, is_last);
    }

    #[allow(unused_variables)]
    fn on_rtn_depth_market_data(&mut self, depth_market_data: Option<&Struct_CThostFtdcDepthMarketDataField>) {
        let md = depth_market_data.unwrap();
        let instrument_id = unsafe { CStr::from_ptr(&md.InstrumentID as *const u8 as *const i8).to_str().unwrap() };
        println!("on_rtn_depth_market_data: {:?}, {:?}, {:?}, {:?}, {:?}", instrument_id, md.BidPrice1, md.BidVolume1, md.AskPrice1, md.AskVolume1);
    }

    #[allow(unused_variables)]
    fn on_rtn_for_quote_rsp(&mut self, for_quote_rsp: Option<&Struct_CThostFtdcForQuoteRspField>) {
        println!("on_rtn_for_quote_rsp");
    }
}

fn main() {
    let flow_path = ::std::ffi::CString::new("").unwrap();
    let mut md_api = MdApi::new(flow_path, false, false);
    md_api.register_spi(Box::new(Spi));
    md_api.register_front(std::ffi::CString::new("tcp://180.168.146.187:10031").unwrap());
    md_api.init();
    std::thread::sleep(std::time::Duration::from_secs(1));
    match md_api.req_user_login(&Default::default(), 1) {
        Ok(()) => println!("req_user_login ok"),
        Err(err) => println!("req_user_login err: {:?}", err),
    };
    std::thread::sleep(std::time::Duration::from_secs(1));
    let instrument_ids = vec!(CString::new("IF1603").unwrap(),
                              CString::new("au1612").unwrap(),
                              CString::new("m1609").unwrap(),
                              CString::new("CF609").unwrap());
    match md_api.subscribe_market_data(instrument_ids.clone()) {
        Ok(()) => println!("subscribe_market_data ok"),
        Err(err) => println!("subscribe_market_data err: {:?}", err),
    };
    std::thread::sleep(std::time::Duration::from_secs(1));
    match md_api.subscribe_for_quote_rsp(instrument_ids) {
        Ok(()) => println!("subscribe_for_quote_rsp ok"),
        Err(err) => println!("subscribe_for_quote_rsp err: {:?}", err),
    };
    std::thread::sleep(std::time::Duration::from_secs(5));
    /*
    match md_api.req_user_logout(&Default::default(), 2) {
        Ok(()) => println!("req_user_logout ok"),
        Err(err) => println!("req_user_logout err: {:?}", err),
    };
    std::thread::sleep(std::time::Duration::from_secs(1));
    */
}
