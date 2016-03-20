#![feature(ptr_as_ref)]
extern crate ctp_common;

use std::ffi::{ CStr, CString };
use std::mem::transmute;
use std::os::raw::{ c_void, c_char, c_int };

#[allow(non_camel_case_types)]
type c_bool = std::os::raw::c_uchar;

pub use ctp_common::*;

#[link(name = "thostmduserapi")]
extern "C" {
    fn _ZN15CThostFtdcMdApi15CreateFtdcMdApiEPKcbb(pszFlowPath: *const c_char, bIsUsingUdp: c_bool, bIsMulticast: c_bool) -> *mut c_void;
    fn _ZN14CFtdcMdApiImpl7ReleaseEv(api: *mut c_void);
    fn _ZN14CFtdcMdApiImpl4InitEv(api: *mut c_void);
    fn _ZN14CFtdcMdApiImpl4JoinEv(api: *mut c_void) -> c_int;
    fn _ZN14CFtdcMdApiImpl13GetTradingDayEv(api: *mut c_void) -> *const c_char;
    fn _ZN14CFtdcMdApiImpl13RegisterFrontEPc(api: *mut c_void, pszFrontAddress: *const c_char);
    fn _ZN14CFtdcMdApiImpl18RegisterNameServerEPc(api: *mut c_void, pszNsAddress: *const c_char);
    fn _ZN14CFtdcMdApiImpl20RegisterFensUserInfoEP27CThostFtdcFensUserInfoField(api: *mut c_void, pFensUserInfo: *const Struct_CThostFtdcFensUserInfoField);
    fn _ZN14CFtdcMdApiImpl11RegisterSpiEP15CThostFtdcMdSpi(api: *mut c_void, pSpi: *mut c_void);
    fn _ZN14CFtdcMdApiImpl19SubscribeMarketDataEPPci(api: *mut c_void, ppInstrumentID: *const *const c_char, nCount: c_int) -> c_int;
    fn _ZN14CFtdcMdApiImpl21UnSubscribeMarketDataEPPci(api: *mut c_void, ppInstrumentID: *const *const c_char, nCount: c_int) -> c_int;
    fn _ZN14CFtdcMdApiImpl20SubscribeForQuoteRspEPPci(api: *mut c_void, ppInstrumentID: *const *const c_char, nCount: c_int) -> c_int;
    fn _ZN14CFtdcMdApiImpl22UnSubscribeForQuoteRspEPPci(api: *mut c_void, ppInstrumentID: *const *const c_char, nCount: c_int) -> c_int;
    fn _ZN14CFtdcMdApiImpl12ReqUserLoginEP27CThostFtdcReqUserLoginFieldi(api: *mut c_void, pReqUserLoginField: *const Struct_CThostFtdcReqUserLoginField, nRequestID: c_int) -> c_int;
    fn _ZN14CFtdcMdApiImpl13ReqUserLogoutEP25CThostFtdcUserLogoutFieldi(api: *mut c_void, pUserLogoutField: *const Struct_CThostFtdcUserLogoutField, nRequestID: c_int) -> c_int;
}

pub struct MdApi {
    md_api_ptr: *mut c_void,
    registered_spi: Option<*mut Struct_CThostFtdcMdSpi>,
}

unsafe impl Send for MdApi {}

impl MdApi {
    pub fn new(flow_path: CString, use_udp: bool, use_multicast: bool) -> MdApi {
        let flow_path_ptr = flow_path.into_raw();
        let api = unsafe { _ZN15CThostFtdcMdApi15CreateFtdcMdApiEPKcbb(flow_path_ptr, use_udp as c_bool, use_multicast as c_bool) };
        let flow_path = unsafe { CString::from_raw(flow_path_ptr) };
        drop(flow_path);
        MdApi{ md_api_ptr: api, registered_spi: None }
    }

    pub fn init(&mut self) {
        unsafe { _ZN14CFtdcMdApiImpl4InitEv(self.md_api_ptr) };
    }

    pub fn join(&mut self) -> ApiResult {
        from_api_return_to_api_result(unsafe { _ZN14CFtdcMdApiImpl4JoinEv(self.md_api_ptr) })
    }

    pub fn get_trading_day<'a>(&mut self) -> &'a CStr {
        let trading_day_cstr = unsafe { _ZN14CFtdcMdApiImpl13GetTradingDayEv(self.md_api_ptr) };
        let trading_day = unsafe { CStr::from_ptr(trading_day_cstr) };
        trading_day
    }

    pub fn register_front(&mut self, front_socket_address: CString) {
        let front_socket_address_ptr = front_socket_address.into_raw();
        unsafe { _ZN14CFtdcMdApiImpl13RegisterFrontEPc(self.md_api_ptr, front_socket_address_ptr) };
        let front_socket_address = unsafe { CString::from_raw(front_socket_address_ptr) };
        drop(front_socket_address);
    }

    pub fn register_name_server(&mut self, name_server: CString) {
        let name_server_ptr = name_server.into_raw();
        unsafe { _ZN14CFtdcMdApiImpl18RegisterNameServerEPc(self.md_api_ptr, name_server_ptr) };
        let name_server = unsafe { CString::from_raw(name_server_ptr) };
        drop(name_server);
    }

    pub fn register_fens_user_info(&mut self, fens_user_info: &Struct_CThostFtdcFensUserInfoField) {
        unsafe { _ZN14CFtdcMdApiImpl20RegisterFensUserInfoEP27CThostFtdcFensUserInfoField(self.md_api_ptr, fens_user_info) };
    }

    pub fn register_spi(&mut self, md_spi: Box<MdSpi>) {
        let last_registered_spi_ptr = self.registered_spi.take();
        let md_spi_ptr = Box::into_raw(md_spi);
        let spi_ptr = Box::into_raw(Box::new(new_spi(md_spi_ptr)));
        unsafe { _ZN14CFtdcMdApiImpl11RegisterSpiEP15CThostFtdcMdSpi(self.md_api_ptr, spi_ptr as *mut c_void) };
        self.registered_spi = Some(spi_ptr);
        match last_registered_spi_ptr {
            Some(last_registered_spi_ptr) => {
                unsafe {
                    let last_registered_spi = Box::from_raw(last_registered_spi_ptr);
                    drop(last_registered_spi.md_spi_ptr);
                    drop(last_registered_spi);
                }
            },
            None => (),
        };
    }

    pub fn subscribe_market_data(&mut self, instrument_ids: Vec<CString>) -> ApiResult {
        let v = cstring_vec_to_char_star_vec(&instrument_ids);
        from_api_return_to_api_result(unsafe { _ZN14CFtdcMdApiImpl19SubscribeMarketDataEPPci(self.md_api_ptr, v.as_ptr(), v.len() as c_int) })
    }

    pub fn unsubscribe_market_data(&mut self, instrument_ids: Vec<CString>) -> ApiResult {
        let v = cstring_vec_to_char_star_vec(&instrument_ids);
        from_api_return_to_api_result(unsafe { _ZN14CFtdcMdApiImpl21UnSubscribeMarketDataEPPci(self.md_api_ptr, v.as_ptr(), v.len() as c_int) })
    }

    pub fn subscribe_for_quote_rsp(&mut self, instrument_ids: Vec<CString>) -> ApiResult {
        let v = cstring_vec_to_char_star_vec(&instrument_ids);
        from_api_return_to_api_result(unsafe { _ZN14CFtdcMdApiImpl20SubscribeForQuoteRspEPPci(self.md_api_ptr, v.as_ptr(), v.len() as c_int) })
    }

    pub fn unsubscribe_for_quote_rsp(&mut self, instrument_ids: Vec<CString>) -> ApiResult {
        let v = cstring_vec_to_char_star_vec(&instrument_ids);
        from_api_return_to_api_result(unsafe { _ZN14CFtdcMdApiImpl22UnSubscribeForQuoteRspEPPci(self.md_api_ptr, v.as_ptr(), v.len() as c_int) })
    }

    pub fn req_user_login(&mut self, req_user_login: &Struct_CThostFtdcReqUserLoginField, request_id: i32) -> ApiResult {
        from_api_return_to_api_result(unsafe { _ZN14CFtdcMdApiImpl12ReqUserLoginEP27CThostFtdcReqUserLoginFieldi(self.md_api_ptr, req_user_login, request_id) })
    }

    pub fn req_user_logout(&mut self, req_user_logout: &Struct_CThostFtdcUserLogoutField, request_id: i32) -> ApiResult {
        from_api_return_to_api_result(unsafe { _ZN14CFtdcMdApiImpl13ReqUserLogoutEP25CThostFtdcUserLogoutFieldi(self.md_api_ptr, req_user_logout, request_id) })
    }
}

impl Drop for MdApi {
    fn drop(&mut self) {
        let last_registered_spi_ptr = self.registered_spi.take();
        match last_registered_spi_ptr {
            Some(last_registered_spi_ptr) => {
                unsafe {
                    _ZN14CFtdcMdApiImpl11RegisterSpiEP15CThostFtdcMdSpi(self.md_api_ptr, ::std::ptr::null_mut::<c_void>());
                    let last_registered_spi = Box::from_raw(last_registered_spi_ptr);
                    drop(last_registered_spi.md_spi_ptr);
                    drop(last_registered_spi);
                }
            },
            None => (),
        };
        unsafe { _ZN14CFtdcMdApiImpl7ReleaseEv(self.md_api_ptr) };
    }
}

fn cstring_vec_to_char_star_vec(cstring_vec: &Vec<CString>) -> Vec<*const c_char> {
    cstring_vec.iter().map(|cstring| cstring.as_ptr()).collect()
}

pub fn specific_instrument_to_cstr(i: &Struct_CThostFtdcSpecificInstrumentField) -> &CStr {
    unsafe { CStr::from_ptr(&i.InstrumentID as *const u8 as *const c_char) }
}

pub trait MdSpi {
    fn on_front_connected(&mut self) {
        println!("on_front_connected");
    }

    fn on_front_disconnected(&mut self, reason: DisconnectionReason) {
        println!("on_front_disconnected: {:?}", reason);
    }

    #[allow(unused_variables)]
    fn on_rsp_user_login(&mut self, rsp_user_login: Option<&Struct_CThostFtdcRspUserLoginField>, result: RspResult, request_id: i32, is_last: bool) {
        println!("on_rsp_user_login: {:?}, {}, {:?}, {:?}", rsp_user_login, from_rsp_result_to_string(result), request_id, is_last);
    }

    #[allow(unused_variables)]
    fn on_rsp_user_logout(&mut self, rsp_user_logout: Option<&Struct_CThostFtdcUserLogoutField>, result: RspResult, request_id: i32, is_last: bool) {
        println!("on_rsp_user_logout: {:?}, {}, {:?}, {:?}", rsp_user_logout, from_rsp_result_to_string(result), request_id, is_last);
    }

    #[allow(unused_variables)]
    fn on_rsp_error(&mut self, result: RspResult, request_id: i32, is_last: bool) {
        println!("on_rsp_error: {}, {:?}, {:?}", from_rsp_result_to_string(result), request_id, is_last);
    }

    #[allow(unused_variables)]
    fn on_rsp_sub_market_data(&mut self, specific_instrument: Option<&Struct_CThostFtdcSpecificInstrumentField>, result: RspResult, request_id: i32, is_last: bool) {
        println!("on_rsp_sub_market_data: {:?}, {}, {:?}, {:?}", specific_instrument, from_rsp_result_to_string(result), request_id, is_last);
    }

    #[allow(unused_variables)]
    fn on_rsp_un_sub_market_data(&mut self, specific_instrument: Option<&Struct_CThostFtdcSpecificInstrumentField>, result: RspResult, request_id: i32, is_last: bool) {
        println!("on_rsp_un_sub_market_data: {:?}, {}, {:?}, {:?}", specific_instrument, from_rsp_result_to_string(result), request_id, is_last);
    }

    #[allow(unused_variables)]
    fn on_rsp_sub_for_quote_rsp(&mut self, specific_instrument: Option<&Struct_CThostFtdcSpecificInstrumentField>, result: RspResult, request_id: i32, is_last: bool) {
        println!("on_rsp_sub_for_quote_rsp: {:?}, {}, {:?}, {:?}", specific_instrument, from_rsp_result_to_string(result), request_id, is_last);
    }

    #[allow(unused_variables)]
    fn on_rsp_un_sub_for_quote_rsp(&mut self, specific_instrument: Option<&Struct_CThostFtdcSpecificInstrumentField>, result: RspResult, request_id: i32, is_last: bool) {
        println!("on_rsp_un_sub_for_quote_rsp: {:?}, {}, {:?}, {:?}", specific_instrument, from_rsp_result_to_string(result), request_id, is_last);
    }

    #[allow(unused_variables)]
    fn on_rtn_depth_market_data(&mut self, depth_market_data: Option<&Struct_CThostFtdcDepthMarketDataField>) {
        println!("on_rtn_depth_market_data: {:?}", depth_market_data);
    }

    #[allow(unused_variables)]
    fn on_rtn_for_quote_rsp(&mut self, for_quote_rsp: Option<&Struct_CThostFtdcForQuoteRspField>) {
        println!("on_rtn_for_quote_rsp: {:?}", for_quote_rsp);
    }
}


#[allow(non_snake_case)]
extern "C" fn spi_on_front_connected(spi: *mut Struct_CThostFtdcMdSpi) {
    unsafe { transmute::<*mut MdSpi, &mut MdSpi>(transmute::<*mut Struct_CThostFtdcMdSpi, &mut Struct_CThostFtdcMdSpi>(spi).md_spi_ptr).on_front_connected() };
}

#[allow(non_snake_case)]
extern "C" fn spi_on_front_disconnected(spi: *mut Struct_CThostFtdcMdSpi, nReason: c_int) {
    let reason = std::convert::From::from(nReason);
    unsafe { transmute::<*mut MdSpi, &mut MdSpi>(transmute::<*mut Struct_CThostFtdcMdSpi, &mut Struct_CThostFtdcMdSpi>(spi).md_spi_ptr).on_front_disconnected(reason) };
}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_heart_beat_warning(spi: *mut Struct_CThostFtdcMdSpi, nTimeLapse: c_int) {
    // CTP API specification shows this will never be called
    unreachable!();
}

#[allow(non_snake_case)]
extern "C" fn spi_on_rsp_user_login(spi: *mut Struct_CThostFtdcMdSpi, pRspUserLogin: *const Struct_CThostFtdcRspUserLoginField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut MdSpi, &mut MdSpi>(transmute::<*mut Struct_CThostFtdcMdSpi, &mut Struct_CThostFtdcMdSpi>(spi).md_spi_ptr).on_rsp_user_login(pRspUserLogin.as_ref(), rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case)]
extern "C" fn spi_on_rsp_user_logout(spi: *mut Struct_CThostFtdcMdSpi, pUserLogout: *const Struct_CThostFtdcUserLogoutField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut MdSpi, &mut MdSpi>(transmute::<*mut Struct_CThostFtdcMdSpi, &mut Struct_CThostFtdcMdSpi>(spi).md_spi_ptr).on_rsp_user_logout(pUserLogout.as_ref(), rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case)]
extern "C" fn spi_on_rsp_error(spi: *mut Struct_CThostFtdcMdSpi, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut MdSpi, &mut MdSpi>(transmute::<*mut Struct_CThostFtdcMdSpi, &mut Struct_CThostFtdcMdSpi>(spi).md_spi_ptr).on_rsp_error(rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case)]
extern "C" fn spi_on_rsp_sub_market_data(spi: *mut Struct_CThostFtdcMdSpi, pSpecificInstrument: *const Struct_CThostFtdcSpecificInstrumentField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut MdSpi, &mut MdSpi>(transmute::<*mut Struct_CThostFtdcMdSpi, &mut Struct_CThostFtdcMdSpi>(spi).md_spi_ptr).on_rsp_sub_market_data(pSpecificInstrument.as_ref(), rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case)]
extern "C" fn spi_on_rsp_un_sub_market_data(spi: *mut Struct_CThostFtdcMdSpi, pSpecificInstrument: *const Struct_CThostFtdcSpecificInstrumentField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut MdSpi, &mut MdSpi>(transmute::<*mut Struct_CThostFtdcMdSpi, &mut Struct_CThostFtdcMdSpi>(spi).md_spi_ptr).on_rsp_un_sub_market_data(pSpecificInstrument.as_ref(), rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case)]
extern "C" fn spi_on_rsp_sub_for_quote_rsp(spi: *mut Struct_CThostFtdcMdSpi, pSpecificInstrument: *const Struct_CThostFtdcSpecificInstrumentField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut MdSpi, &mut MdSpi>(transmute::<*mut Struct_CThostFtdcMdSpi, &mut Struct_CThostFtdcMdSpi>(spi).md_spi_ptr).on_rsp_sub_for_quote_rsp(pSpecificInstrument.as_ref(), rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case)]
extern "C" fn spi_on_rsp_un_sub_for_quote_rsp(spi: *mut Struct_CThostFtdcMdSpi, pSpecificInstrument: *const Struct_CThostFtdcSpecificInstrumentField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut MdSpi, &mut MdSpi>(transmute::<*mut Struct_CThostFtdcMdSpi, &mut Struct_CThostFtdcMdSpi>(spi).md_spi_ptr).on_rsp_un_sub_for_quote_rsp(pSpecificInstrument.as_ref(), rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case)]
extern "C" fn spi_on_rtn_depth_market_data(spi: *mut Struct_CThostFtdcMdSpi, pDepthMarketData: *const Struct_CThostFtdcDepthMarketDataField ) {
    unsafe { transmute::<*mut MdSpi, &mut MdSpi>(transmute::<*mut Struct_CThostFtdcMdSpi, &mut Struct_CThostFtdcMdSpi>(spi).md_spi_ptr).on_rtn_depth_market_data(pDepthMarketData.as_ref()) };
}

#[allow(non_snake_case)]
extern "C" fn spi_on_rtn_for_quote_rsp(spi: *mut Struct_CThostFtdcMdSpi, pForQuoteRsp: *const Struct_CThostFtdcForQuoteRspField ) {
    unsafe { transmute::<*mut MdSpi, &mut MdSpi>(transmute::<*mut Struct_CThostFtdcMdSpi, &mut Struct_CThostFtdcMdSpi>(spi).md_spi_ptr).on_rtn_for_quote_rsp(pForQuoteRsp.as_ref()) };
}

#[repr(C)]
#[derive(Debug)]
struct SpiVTable {
    on_front_connected: extern "C" fn(spi: *mut Struct_CThostFtdcMdSpi),
    on_front_disconnected: extern "C" fn(spi: *mut Struct_CThostFtdcMdSpi, nReason: c_int),
    on_heart_beat_warning: extern "C" fn(spi: *mut Struct_CThostFtdcMdSpi, nTimeLapse: c_int),
    on_rsp_user_login: extern "C" fn(spi: *mut Struct_CThostFtdcMdSpi, pRspUserLogin: *const Struct_CThostFtdcRspUserLoginField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    on_rsp_user_logout: extern "C" fn(spi: *mut Struct_CThostFtdcMdSpi, pUserLogout: *const Struct_CThostFtdcUserLogoutField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    on_rsp_error: extern "C" fn(spi: *mut Struct_CThostFtdcMdSpi, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    on_rsp_sub_market_data: extern "C" fn(spi: *mut Struct_CThostFtdcMdSpi, pSpecificInstrument: *const Struct_CThostFtdcSpecificInstrumentField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    on_rsp_un_sub_market_data: extern "C" fn(spi: *mut Struct_CThostFtdcMdSpi, pSpecificInstrument: *const Struct_CThostFtdcSpecificInstrumentField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    on_rsp_sub_for_quote_rsp: extern "C" fn(spi: *mut Struct_CThostFtdcMdSpi, pSpecificInstrument: *const Struct_CThostFtdcSpecificInstrumentField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    on_rsp_un_sub_for_quote_rsp: extern "C" fn(spi: *mut Struct_CThostFtdcMdSpi, pSpecificInstrument: *const Struct_CThostFtdcSpecificInstrumentField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    on_rtn_depth_market_data: extern "C" fn(spi: *mut Struct_CThostFtdcMdSpi, pDepthMarketData: *const Struct_CThostFtdcDepthMarketDataField ),
    on_rtn_for_quote_rsp: extern "C" fn(spi: *mut Struct_CThostFtdcMdSpi, pForQuoteRsp: *const Struct_CThostFtdcForQuoteRspField ),
}

static SPI_VTABLE: SpiVTable = SpiVTable{
    on_front_connected: spi_on_front_connected,
    on_front_disconnected: spi_on_front_disconnected,
    on_heart_beat_warning: spi_on_heart_beat_warning,
    on_rsp_user_login: spi_on_rsp_user_login,
    on_rsp_user_logout: spi_on_rsp_user_logout,
    on_rsp_error: spi_on_rsp_error,
    on_rsp_sub_market_data: spi_on_rsp_sub_market_data,
    on_rsp_un_sub_market_data: spi_on_rsp_un_sub_market_data,
    on_rsp_sub_for_quote_rsp: spi_on_rsp_sub_for_quote_rsp,
    on_rsp_un_sub_for_quote_rsp: spi_on_rsp_un_sub_for_quote_rsp,
    on_rtn_depth_market_data: spi_on_rtn_depth_market_data,
    on_rtn_for_quote_rsp: spi_on_rtn_for_quote_rsp,
};

#[repr(C)]
pub struct Struct_CThostFtdcMdSpi {
    vtable: *const SpiVTable,
    pub md_spi_ptr: *mut MdSpi
}

fn new_spi(md_spi: *mut MdSpi) -> Struct_CThostFtdcMdSpi {
    Struct_CThostFtdcMdSpi{ vtable: &SPI_VTABLE, md_spi_ptr: md_spi }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    #[test]
    fn create_release() {
        let flow_path = CString::new("").unwrap();
        let md_api = MdApi::new(flow_path, false, false);
        drop(md_api);
        assert!(true);
    }

    #[test]
    fn get_trading_day() {
        let flow_path = CString::new("").unwrap();
        let mut md_api = MdApi::new(flow_path, false, false);
        let trading_day = md_api.get_trading_day();
        assert_eq!(b"19700101".len(), trading_day.to_bytes().len());
        let year = ::std::str::from_utf8(&trading_day.to_bytes()[0..4]).unwrap().parse::<i32>().unwrap();
        assert!(year > 1970 && year < 2038);
        let month = ::std::str::from_utf8(&trading_day.to_bytes()[4..6]).unwrap().parse::<i32>().unwrap();
        assert!(month > 0 && month < 13);
        let day = ::std::str::from_utf8(&trading_day.to_bytes()[6..8]).unwrap().parse::<i32>().unwrap();
        assert!(day > 0 && day < 32);
    }
}
