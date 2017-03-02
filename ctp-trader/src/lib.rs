extern crate ctp_common;

use std::ffi::{ CStr, CString };
use std::mem::transmute;
use std::os::raw::{ c_void, c_char, c_int };

#[allow(non_camel_case_types)]
type c_bool = std::os::raw::c_uchar;

pub use ctp_common::*;

#[link(name = "thosttraderapi")]
extern "C" {
    fn _ZN19CThostFtdcTraderApi19CreateFtdcTraderApiEPKc(pszFlowPath: *const c_char) -> *mut c_void;
    fn _ZN18CFtdcTraderApiImpl7ReleaseEv(api: *mut c_void);
    fn _ZN18CFtdcTraderApiImpl4InitEv(api: *mut c_void);
    fn _ZN18CFtdcTraderApiImpl4JoinEv(api: *mut c_void) -> c_int;
    fn _ZN18CFtdcTraderApiImpl13GetTradingDayEv(api: *mut c_void) -> *const c_char;
    fn _ZN18CFtdcTraderApiImpl13RegisterFrontEPc(api: *mut c_void, pszFrontAddress: *const c_char);
    fn _ZN18CFtdcTraderApiImpl18RegisterNameServerEPc(api: *mut c_void, pszNsAddress: *const c_char);
    fn _ZN18CFtdcTraderApiImpl20RegisterFensUserInfoEP27CThostFtdcFensUserInfoField(api: *mut c_void, pFensUserInfo: *const Struct_CThostFtdcFensUserInfoField);
    fn _ZN18CFtdcTraderApiImpl11RegisterSpiEP19CThostFtdcTraderSpi(api: *mut c_void, pSpi: *mut c_void);
    fn _ZN18CFtdcTraderApiImpl21SubscribePrivateTopicE20THOST_TE_RESUME_TYPE(api: *mut c_void, nResumeType: Enum_THOST_TE_RESUME_TYPE);
    fn _ZN18CFtdcTraderApiImpl20SubscribePublicTopicE20THOST_TE_RESUME_TYPE(api: *mut c_void, nResumeType: Enum_THOST_TE_RESUME_TYPE);
    fn _ZN18CFtdcTraderApiImpl15ReqAuthenticateEP30CThostFtdcReqAuthenticateFieldi(api: *mut c_void, pReqAuthenticateField: *const Struct_CThostFtdcReqAuthenticateField, nRequestID: c_int) -> c_int;
    fn _ZN18CFtdcTraderApiImpl12ReqUserLoginEP27CThostFtdcReqUserLoginFieldi(api: *mut c_void, pReqUserLoginField: *const Struct_CThostFtdcReqUserLoginField, nRequestID: c_int) -> c_int;
    fn _ZN18CFtdcTraderApiImpl13ReqUserLogoutEP25CThostFtdcUserLogoutFieldi(api: *mut c_void, pUserLogoutField: *const Struct_CThostFtdcUserLogoutField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl21ReqUserPasswordUpdateEP33CThostFtdcUserPasswordUpdateFieldi(api: *mut c_void, pUserPasswordUpdate: *const Struct_CThostFtdcUserPasswordUpdateField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl31ReqTradingAccountPasswordUpdateEP43CThostFtdcTradingAccountPasswordUpdateFieldi(api: *mut c_void, pTradingAccountPasswordUpdate: *const Struct_CThostFtdcTradingAccountPasswordUpdateField, nRequestID: c_int) -> c_int;
    fn _ZN18CFtdcTraderApiImpl14ReqOrderInsertEP25CThostFtdcInputOrderFieldi(api: *mut c_void, pInputOrder: *const Struct_CThostFtdcInputOrderField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl20ReqParkedOrderInsertEP26CThostFtdcParkedOrderFieldi(api: *mut c_void, pParkedOrder: *const Struct_CThostFtdcParkedOrderField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl20ReqParkedOrderActionEP32CThostFtdcParkedOrderActionFieldi(api: *mut c_void, pParkedOrderAction: *const Struct_CThostFtdcParkedOrderActionField, nRequestID: c_int) -> c_int;
    fn _ZN18CFtdcTraderApiImpl14ReqOrderActionEP31CThostFtdcInputOrderActionFieldi(api: *mut c_void, pInputOrderAction: *const Struct_CThostFtdcInputOrderActionField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl22ReqQueryMaxOrderVolumeEP34CThostFtdcQueryMaxOrderVolumeFieldi(api: *mut c_void, pQueryMaxOrderVolume: *const Struct_CThostFtdcQueryMaxOrderVolumeField, nRequestID: c_int) -> c_int;
    fn _ZN18CFtdcTraderApiImpl24ReqSettlementInfoConfirmEP36CThostFtdcSettlementInfoConfirmFieldi(api: *mut c_void, pSettlementInfoConfirm: *const Struct_CThostFtdcSettlementInfoConfirmField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl20ReqRemoveParkedOrderEP32CThostFtdcRemoveParkedOrderFieldi(api: *mut c_void, pRemoveParkedOrder: *const Struct_CThostFtdcRemoveParkedOrderField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl26ReqRemoveParkedOrderActionEP38CThostFtdcRemoveParkedOrderActionFieldi(api: *mut c_void, pRemoveParkedOrderAction: *const Struct_CThostFtdcRemoveParkedOrderActionField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl18ReqExecOrderInsertEP29CThostFtdcInputExecOrderFieldi(api: *mut c_void, pInputExecOrder: *const Struct_CThostFtdcInputExecOrderField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl18ReqExecOrderActionEP35CThostFtdcInputExecOrderActionFieldi(api: *mut c_void, pInputExecOrderAction: *const Struct_CThostFtdcInputExecOrderActionField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl17ReqForQuoteInsertEP28CThostFtdcInputForQuoteFieldi(api: *mut c_void, pInputForQuote: *const Struct_CThostFtdcInputForQuoteField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl14ReqQuoteInsertEP25CThostFtdcInputQuoteFieldi(api: *mut c_void, pInputQuote: *const Struct_CThostFtdcInputQuoteField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl14ReqQuoteActionEP31CThostFtdcInputQuoteActionFieldi(api: *mut c_void, pInputQuoteAction: *const Struct_CThostFtdcInputQuoteActionField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl19ReqCombActionInsertEP30CThostFtdcInputCombActionFieldi(api: *mut c_void, pInputCombAction: *const Struct_CThostFtdcInputCombActionField, nRequestID: c_int) -> c_int;
    fn _ZN18CFtdcTraderApiImpl11ReqQryOrderEP23CThostFtdcQryOrderFieldi(api: *mut c_void, pQryOrder: *const Struct_CThostFtdcQryOrderField, nRequestID: c_int) -> c_int;
    fn _ZN18CFtdcTraderApiImpl11ReqQryTradeEP23CThostFtdcQryTradeFieldi(api: *mut c_void, pQryTrade: *const Struct_CThostFtdcQryTradeField, nRequestID: c_int) -> c_int;
    fn _ZN18CFtdcTraderApiImpl22ReqQryInvestorPositionEP34CThostFtdcQryInvestorPositionFieldi(api: *mut c_void, pQryInvestorPosition: *const Struct_CThostFtdcQryInvestorPositionField, nRequestID: c_int) -> c_int;
    fn _ZN18CFtdcTraderApiImpl20ReqQryTradingAccountEP32CThostFtdcQryTradingAccountFieldi(api: *mut c_void, pQryTradingAccount: *const Struct_CThostFtdcQryTradingAccountField, nRequestID: c_int) -> c_int;
    fn _ZN18CFtdcTraderApiImpl14ReqQryInvestorEP26CThostFtdcQryInvestorFieldi(api: *mut c_void, pQryInvestor: *const Struct_CThostFtdcQryInvestorField, nRequestID: c_int) -> c_int;
    fn _ZN18CFtdcTraderApiImpl17ReqQryTradingCodeEP29CThostFtdcQryTradingCodeFieldi(api: *mut c_void, pQryTradingCode: *const Struct_CThostFtdcQryTradingCodeField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl26ReqQryInstrumentMarginRateEP38CThostFtdcQryInstrumentMarginRateFieldi(api: *mut c_void, pQryInstrumentMarginRate: *const Struct_CThostFtdcQryInstrumentMarginRateField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl30ReqQryInstrumentCommissionRateEP42CThostFtdcQryInstrumentCommissionRateFieldi(api: *mut c_void, pQryInstrumentCommissionRate: *const Struct_CThostFtdcQryInstrumentCommissionRateField, nRequestID: c_int) -> c_int;
    fn _ZN18CFtdcTraderApiImpl14ReqQryExchangeEP26CThostFtdcQryExchangeFieldi(api: *mut c_void, pQryExchange: *const Struct_CThostFtdcQryExchangeField, nRequestID: c_int) -> c_int;
    fn _ZN18CFtdcTraderApiImpl13ReqQryProductEP25CThostFtdcQryProductFieldi(api: *mut c_void, pQryProduct: *const Struct_CThostFtdcQryProductField, nRequestID: c_int) -> c_int;
    fn _ZN18CFtdcTraderApiImpl16ReqQryInstrumentEP28CThostFtdcQryInstrumentFieldi(api: *mut c_void, pQryInstrument: *const Struct_CThostFtdcQryInstrumentField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl21ReqQryDepthMarketDataEP33CThostFtdcQryDepthMarketDataFieldi(api: *mut c_void, pQryDepthMarketData: *const Struct_CThostFtdcQryDepthMarketDataField, nRequestID: c_int) -> c_int;
    fn _ZN18CFtdcTraderApiImpl20ReqQrySettlementInfoEP32CThostFtdcQrySettlementInfoFieldi(api: *mut c_void, pQrySettlementInfo: *const Struct_CThostFtdcQrySettlementInfoField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl18ReqQryTransferBankEP30CThostFtdcQryTransferBankFieldi(api: *mut c_void, pQryTransferBank: *const Struct_CThostFtdcQryTransferBankField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl28ReqQryInvestorPositionDetailEP40CThostFtdcQryInvestorPositionDetailFieldi(api: *mut c_void, pQryInvestorPositionDetail: *const Struct_CThostFtdcQryInvestorPositionDetailField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl12ReqQryNoticeEP24CThostFtdcQryNoticeFieldi(api: *mut c_void, pQryNotice: *const Struct_CThostFtdcQryNoticeField, nRequestID: c_int) -> c_int;
    fn _ZN18CFtdcTraderApiImpl27ReqQrySettlementInfoConfirmEP39CThostFtdcQrySettlementInfoConfirmFieldi(api: *mut c_void, pQrySettlementInfoConfirm: *const Struct_CThostFtdcQrySettlementInfoConfirmField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl35ReqQryInvestorPositionCombineDetailEP47CThostFtdcQryInvestorPositionCombineDetailFieldi(api: *mut c_void, pQryInvestorPositionCombineDetail: *const Struct_CThostFtdcQryInvestorPositionCombineDetailField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl28ReqQryCFMMCTradingAccountKeyEP40CThostFtdcQryCFMMCTradingAccountKeyFieldi(api: *mut c_void, pQryCFMMCTradingAccountKey: *const Struct_CThostFtdcQryCFMMCTradingAccountKeyField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl20ReqQryEWarrantOffsetEP32CThostFtdcQryEWarrantOffsetFieldi(api: *mut c_void, pQryEWarrantOffset: *const Struct_CThostFtdcQryEWarrantOffsetField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl32ReqQryInvestorProductGroupMarginEP44CThostFtdcQryInvestorProductGroupMarginFieldi(api: *mut c_void, pQryInvestorProductGroupMargin: *const Struct_CThostFtdcQryInvestorProductGroupMarginField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl24ReqQryExchangeMarginRateEP36CThostFtdcQryExchangeMarginRateFieldi(api: *mut c_void, pQryExchangeMarginRate: *const Struct_CThostFtdcQryExchangeMarginRateField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl30ReqQryExchangeMarginRateAdjustEP42CThostFtdcQryExchangeMarginRateAdjustFieldi(api: *mut c_void, pQryExchangeMarginRateAdjust: *const Struct_CThostFtdcQryExchangeMarginRateAdjustField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl18ReqQryExchangeRateEP30CThostFtdcQryExchangeRateFieldi(api: *mut c_void, pQryExchangeRate: *const Struct_CThostFtdcQryExchangeRateField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl21ReqQrySecAgentACIDMapEP33CThostFtdcQrySecAgentACIDMapFieldi(api: *mut c_void, pQrySecAgentACIDMap: *const Struct_CThostFtdcQrySecAgentACIDMapField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl18ReqQryProductGroupEP30CThostFtdcQryProductGroupFieldi(api: *mut c_void, pQryProductGroup: *const Struct_CThostFtdcQryProductGroupField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl29ReqQryInstrumentOrderCommRateEP41CThostFtdcQryInstrumentOrderCommRateFieldi(api: *mut c_void, pQryInstrumentOrderCommRate: *const Struct_CThostFtdcQryInstrumentOrderCommRateField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl26ReqQryOptionInstrTradeCostEP38CThostFtdcQryOptionInstrTradeCostFieldi(api: *mut c_void, pQryOptionInstrTradeCost: *const Struct_CThostFtdcQryOptionInstrTradeCostField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl25ReqQryOptionInstrCommRateEP37CThostFtdcQryOptionInstrCommRateFieldi(api: *mut c_void, pQryOptionInstrCommRate: *const Struct_CThostFtdcQryOptionInstrCommRateField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl15ReqQryExecOrderEP27CThostFtdcQryExecOrderFieldi(api: *mut c_void, pQryExecOrder: *const Struct_CThostFtdcQryExecOrderField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl14ReqQryForQuoteEP26CThostFtdcQryForQuoteFieldi(api: *mut c_void, pQryForQuote: *const Struct_CThostFtdcQryForQuoteField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl11ReqQryQuoteEP23CThostFtdcQryQuoteFieldi(api: *mut c_void, pQryQuote: *const Struct_CThostFtdcQryQuoteField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl25ReqQryCombInstrumentGuardEP37CThostFtdcQryCombInstrumentGuardFieldi(api: *mut c_void, pQryCombInstrumentGuard: *const Struct_CThostFtdcQryCombInstrumentGuardField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl16ReqQryCombActionEP28CThostFtdcQryCombActionFieldi(api: *mut c_void, pQryCombAction: *const Struct_CThostFtdcQryCombActionField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl20ReqQryTransferSerialEP32CThostFtdcQryTransferSerialFieldi(api: *mut c_void, pQryTransferSerial: *const Struct_CThostFtdcQryTransferSerialField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl21ReqQryAccountregisterEP33CThostFtdcQryAccountregisterFieldi(api: *mut c_void, pQryAccountregister: *const Struct_CThostFtdcQryAccountregisterField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl18ReqQryContractBankEP30CThostFtdcQryContractBankFieldi(api: *mut c_void, pQryContractBank: *const Struct_CThostFtdcQryContractBankField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl17ReqQryParkedOrderEP29CThostFtdcQryParkedOrderFieldi(api: *mut c_void, pQryParkedOrder: *const Struct_CThostFtdcQryParkedOrderField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl23ReqQryParkedOrderActionEP35CThostFtdcQryParkedOrderActionFieldi(api: *mut c_void, pQryParkedOrderAction: *const Struct_CThostFtdcQryParkedOrderActionField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl19ReqQryTradingNoticeEP31CThostFtdcQryTradingNoticeFieldi(api: *mut c_void, pQryTradingNotice: *const Struct_CThostFtdcQryTradingNoticeField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl25ReqQryBrokerTradingParamsEP37CThostFtdcQryBrokerTradingParamsFieldi(api: *mut c_void, pQryBrokerTradingParams: *const Struct_CThostFtdcQryBrokerTradingParamsField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl24ReqQryBrokerTradingAlgosEP36CThostFtdcQryBrokerTradingAlgosFieldi(api: *mut c_void, pQryBrokerTradingAlgos: *const Struct_CThostFtdcQryBrokerTradingAlgosField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl32ReqQueryCFMMCTradingAccountTokenEP44CThostFtdcQueryCFMMCTradingAccountTokenFieldi(api: *mut c_void, pQueryCFMMCTradingAccountToken: *const Struct_CThostFtdcQueryCFMMCTradingAccountTokenField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl27ReqFromBankToFutureByFutureEP26CThostFtdcReqTransferFieldi(api: *mut c_void, pReqTransfer: *const Struct_CThostFtdcReqTransferField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl27ReqFromFutureToBankByFutureEP26CThostFtdcReqTransferFieldi(api: *mut c_void, pReqTransfer: *const Struct_CThostFtdcReqTransferField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl32ReqQueryBankAccountMoneyByFutureEP30CThostFtdcReqQueryAccountFieldi(api: *mut c_void, pReqQueryAccount: *const Struct_CThostFtdcReqQueryAccountField, nRequestID: c_int) -> c_int;
}

pub struct TraderApi {
    trader_api_ptr: *mut c_void,
    registered_spi: Option<*mut Struct_CThostFtdcTraderSpi>,
}

unsafe impl Send for TraderApi {}

impl TraderApi {
    pub fn new(flow_path: CString) -> TraderApi {
        let flow_path_ptr = flow_path.into_raw();
        let api = unsafe { _ZN19CThostFtdcTraderApi19CreateFtdcTraderApiEPKc(flow_path_ptr) };
        let flow_path = unsafe { CString::from_raw(flow_path_ptr) };
        drop(flow_path);
        TraderApi{ trader_api_ptr: api, registered_spi: None }
    }

    pub fn init(&mut self) {
        unsafe { _ZN18CFtdcTraderApiImpl4InitEv(self.trader_api_ptr) };
    }

    pub fn join(&mut self) -> ApiResult {
        from_api_return_to_api_result(unsafe { _ZN18CFtdcTraderApiImpl4JoinEv(self.trader_api_ptr) })
    }

    pub fn get_trading_day<'a>(&mut self) -> &'a CStr {
        let trading_day_cstr = unsafe { _ZN18CFtdcTraderApiImpl13GetTradingDayEv(self.trader_api_ptr) };
        unsafe { CStr::from_ptr(trading_day_cstr) }
    }

    pub fn register_front(&mut self, front_socket_address: CString) {
        let front_socket_address_ptr = front_socket_address.into_raw();
        unsafe { _ZN18CFtdcTraderApiImpl13RegisterFrontEPc(self.trader_api_ptr, front_socket_address_ptr) };
        let front_socket_address = unsafe { CString::from_raw(front_socket_address_ptr) };
        drop(front_socket_address);
    }

    pub fn register_name_server(&mut self, name_server: CString) {
        let name_server_ptr = name_server.into_raw();
        unsafe { _ZN18CFtdcTraderApiImpl18RegisterNameServerEPc(self.trader_api_ptr, name_server_ptr) };
        let name_server = unsafe { CString::from_raw(name_server_ptr) };
        drop(name_server);
    }

    pub fn register_fens_user_info(&mut self, fens_user_info: &Struct_CThostFtdcFensUserInfoField) {
        unsafe { _ZN18CFtdcTraderApiImpl20RegisterFensUserInfoEP27CThostFtdcFensUserInfoField(self.trader_api_ptr, fens_user_info) };
    }

    pub fn register_spi(&mut self, trader_spi: Box<TraderSpi>) {
        let last_registered_spi_ptr = self.registered_spi.take();
        let trader_spi_ptr = Box::into_raw(trader_spi);
        let spi_ptr = Box::into_raw(Box::new(new_spi(trader_spi_ptr)));
        unsafe { _ZN18CFtdcTraderApiImpl11RegisterSpiEP19CThostFtdcTraderSpi(self.trader_api_ptr, spi_ptr as *mut c_void) };
        self.registered_spi = Some(spi_ptr);
        if let Some(last_registered_spi_ptr) = last_registered_spi_ptr {
            unsafe {
                let last_registered_spi = Box::from_raw(last_registered_spi_ptr);
                drop(last_registered_spi.trader_spi_ptr);
                drop(last_registered_spi);
            }
        };
    }

    pub fn subscribe_private_topic(&mut self, resume_type: ResumeType) {
        unsafe { _ZN18CFtdcTraderApiImpl21SubscribePrivateTopicE20THOST_TE_RESUME_TYPE(self.trader_api_ptr, resume_type.into()) };
    }

    pub fn subscribe_public_topic(&mut self, resume_type: ResumeType) {
        unsafe { _ZN18CFtdcTraderApiImpl20SubscribePublicTopicE20THOST_TE_RESUME_TYPE(self.trader_api_ptr, resume_type.into()) };
    }

    pub fn req_authenticate(&mut self, req_authenticate: &Struct_CThostFtdcReqAuthenticateField, request_id: i32) -> ApiResult {
        from_api_return_to_api_result(unsafe { _ZN18CFtdcTraderApiImpl15ReqAuthenticateEP30CThostFtdcReqAuthenticateFieldi(self.trader_api_ptr, req_authenticate, request_id) })
    }

    pub fn req_user_login(&mut self, req_user_login: &Struct_CThostFtdcReqUserLoginField, request_id: i32) -> ApiResult {
        from_api_return_to_api_result(unsafe { _ZN18CFtdcTraderApiImpl12ReqUserLoginEP27CThostFtdcReqUserLoginFieldi(self.trader_api_ptr, req_user_login, request_id) })
    }

    pub fn req_user_logout(&mut self, req_user_logout: &Struct_CThostFtdcUserLogoutField, request_id: i32) -> ApiResult {
        from_api_return_to_api_result(unsafe { _ZN18CFtdcTraderApiImpl13ReqUserLogoutEP25CThostFtdcUserLogoutFieldi(self.trader_api_ptr, req_user_logout, request_id) })
    }

    pub fn req_order_insert(&mut self, input_order: &Struct_CThostFtdcInputOrderField, request_id: i32) -> ApiResult {
        from_api_return_to_api_result(unsafe { _ZN18CFtdcTraderApiImpl14ReqOrderInsertEP25CThostFtdcInputOrderFieldi(self.trader_api_ptr, input_order, request_id) })
    }

    pub fn req_order_action(&mut self, input_order_action: &Struct_CThostFtdcInputOrderActionField, request_id: i32) -> ApiResult {
        from_api_return_to_api_result(unsafe { _ZN18CFtdcTraderApiImpl14ReqOrderActionEP31CThostFtdcInputOrderActionFieldi(self.trader_api_ptr, input_order_action, request_id) })
    }

    pub fn req_settlement_info_confirm(&mut self, settlement_info_confirm: &Struct_CThostFtdcSettlementInfoConfirmField, request_id: i32) -> ApiResult {
        from_api_return_to_api_result(unsafe { _ZN18CFtdcTraderApiImpl24ReqSettlementInfoConfirmEP36CThostFtdcSettlementInfoConfirmFieldi(self.trader_api_ptr, settlement_info_confirm, request_id) })
    }

    pub fn req_qry_order(&mut self, qry_order: &Struct_CThostFtdcQryOrderField, request_id: i32) -> ApiResult {
        from_api_return_to_api_result(unsafe { _ZN18CFtdcTraderApiImpl11ReqQryOrderEP23CThostFtdcQryOrderFieldi(self.trader_api_ptr, qry_order, request_id) })
    }

    pub fn req_qry_trade(&mut self, qry_trade: &Struct_CThostFtdcQryTradeField, request_id: i32) -> ApiResult {
        from_api_return_to_api_result(unsafe { _ZN18CFtdcTraderApiImpl11ReqQryTradeEP23CThostFtdcQryTradeFieldi(self.trader_api_ptr, qry_trade, request_id) })
    }

    pub fn req_qry_investor_position(&mut self, qry_investor_position: &Struct_CThostFtdcQryInvestorPositionField, request_id: i32) -> ApiResult {
        from_api_return_to_api_result(unsafe { _ZN18CFtdcTraderApiImpl22ReqQryInvestorPositionEP34CThostFtdcQryInvestorPositionFieldi(self.trader_api_ptr, qry_investor_position, request_id) })
    }

    pub fn req_qry_trading_account(&mut self, qry_trading_account: &Struct_CThostFtdcQryTradingAccountField, request_id: i32) -> ApiResult {
        from_api_return_to_api_result(unsafe { _ZN18CFtdcTraderApiImpl20ReqQryTradingAccountEP32CThostFtdcQryTradingAccountFieldi(self.trader_api_ptr, qry_trading_account, request_id) })
    }

    pub fn req_qry_investor(&mut self, qry_investor: &Struct_CThostFtdcQryInvestorField, request_id: i32) -> ApiResult {
        from_api_return_to_api_result(unsafe { _ZN18CFtdcTraderApiImpl14ReqQryInvestorEP26CThostFtdcQryInvestorFieldi(self.trader_api_ptr, qry_investor, request_id) })
    }

    pub fn req_qry_trading_code(&mut self, qry_trading_code: &Struct_CThostFtdcQryTradingCodeField, request_id: i32) -> ApiResult {
        from_api_return_to_api_result(unsafe { _ZN18CFtdcTraderApiImpl17ReqQryTradingCodeEP29CThostFtdcQryTradingCodeFieldi(self.trader_api_ptr, qry_trading_code, request_id) })
    }

    pub fn req_qry_instrument_margin_rate(&mut self, qry_instrument_margin_rate: &Struct_CThostFtdcQryInstrumentMarginRateField, request_id: i32) -> ApiResult {
        from_api_return_to_api_result(unsafe { _ZN18CFtdcTraderApiImpl26ReqQryInstrumentMarginRateEP38CThostFtdcQryInstrumentMarginRateFieldi(self.trader_api_ptr, qry_instrument_margin_rate, request_id) })
    }

    pub fn req_qry_instrument_commission_rate(&mut self, qry_instrument_commission_rate: &Struct_CThostFtdcQryInstrumentCommissionRateField, request_id: i32) -> ApiResult {
        from_api_return_to_api_result(unsafe { _ZN18CFtdcTraderApiImpl30ReqQryInstrumentCommissionRateEP42CThostFtdcQryInstrumentCommissionRateFieldi(self.trader_api_ptr, qry_instrument_commission_rate, request_id) })
    }

    pub fn req_qry_exchange(&mut self, qry_exchange: &Struct_CThostFtdcQryExchangeField, request_id: i32) -> ApiResult {
        from_api_return_to_api_result(unsafe { _ZN18CFtdcTraderApiImpl14ReqQryExchangeEP26CThostFtdcQryExchangeFieldi(self.trader_api_ptr, qry_exchange, request_id) })
    }

    pub fn req_qry_product(&mut self, qry_product: &Struct_CThostFtdcQryProductField, request_id: i32) -> ApiResult {
        from_api_return_to_api_result(unsafe { _ZN18CFtdcTraderApiImpl13ReqQryProductEP25CThostFtdcQryProductFieldi(self.trader_api_ptr, qry_product, request_id) })
    }

    pub fn req_qry_instrument(&mut self, qry_instrument: &Struct_CThostFtdcQryInstrumentField, request_id: i32) -> ApiResult {
        from_api_return_to_api_result(unsafe { _ZN18CFtdcTraderApiImpl16ReqQryInstrumentEP28CThostFtdcQryInstrumentFieldi(self.trader_api_ptr, qry_instrument, request_id) })
    }

    pub fn req_qry_settlement_info(&mut self, qry_settlement_info: &Struct_CThostFtdcQrySettlementInfoField, request_id: i32) -> ApiResult {
        from_api_return_to_api_result(unsafe { _ZN18CFtdcTraderApiImpl20ReqQrySettlementInfoEP32CThostFtdcQrySettlementInfoFieldi(self.trader_api_ptr, qry_settlement_info, request_id) })
    }

    pub fn req_qry_settlement_info_confirm(&mut self, qry_settlement_info_confirm: &Struct_CThostFtdcQrySettlementInfoConfirmField, request_id: i32) -> ApiResult {
        from_api_return_to_api_result(unsafe { _ZN18CFtdcTraderApiImpl27ReqQrySettlementInfoConfirmEP39CThostFtdcQrySettlementInfoConfirmFieldi(self.trader_api_ptr, qry_settlement_info_confirm, request_id) })
    }

    pub fn req_qry_exchange_margin_rate(&mut self, qry_exchange_margin_rate: &Struct_CThostFtdcQryExchangeMarginRateField, request_id: i32) -> ApiResult {
        from_api_return_to_api_result(unsafe { _ZN18CFtdcTraderApiImpl24ReqQryExchangeMarginRateEP36CThostFtdcQryExchangeMarginRateFieldi(self.trader_api_ptr, qry_exchange_margin_rate, request_id) })
    }

    pub fn req_qry_exchange_margin_rate_adjust(&mut self, qry_exchange_margin_rate_adjust: &Struct_CThostFtdcQryExchangeMarginRateAdjustField, request_id: i32) -> ApiResult {
        from_api_return_to_api_result(unsafe { _ZN18CFtdcTraderApiImpl30ReqQryExchangeMarginRateAdjustEP42CThostFtdcQryExchangeMarginRateAdjustFieldi(self.trader_api_ptr, qry_exchange_margin_rate_adjust, request_id) })
    }

    pub fn req_qry_exchange_rate(&mut self, qry_exchange_rate: &Struct_CThostFtdcQryExchangeRateField, request_id: i32) -> ApiResult {
        from_api_return_to_api_result(unsafe { _ZN18CFtdcTraderApiImpl18ReqQryExchangeRateEP30CThostFtdcQryExchangeRateFieldi(self.trader_api_ptr, qry_exchange_rate, request_id) })
    }
}

impl Drop for TraderApi {
    fn drop(&mut self) {
        let last_registered_spi_ptr = self.registered_spi.take();
        if let Some(last_registered_spi_ptr) = last_registered_spi_ptr {
            unsafe {
                _ZN18CFtdcTraderApiImpl11RegisterSpiEP19CThostFtdcTraderSpi(self.trader_api_ptr, ::std::ptr::null_mut::<c_void>());
                let last_registered_spi = Box::from_raw(last_registered_spi_ptr);
                drop(last_registered_spi.trader_spi_ptr);
                drop(last_registered_spi);
            }
        };
        unsafe { _ZN18CFtdcTraderApiImpl7ReleaseEv(self.trader_api_ptr) };
    }
}

pub trait TraderSpi {
    fn on_front_connected(&mut self) {
        println!("on_front_connected");
    }

    fn on_front_disconnected(&mut self, reason: DisconnectionReason) {
        println!("on_front_disconnected: {:?}", reason);
    }

    fn on_rsp_authenticate(&mut self, rsp_authenticate: Option<&Struct_CThostFtdcRspAuthenticateField>, result: RspResult, request_id: i32, is_last: bool) {
        println!("on_rsp_authenticate: {:?}, {}, {:?}, {:?}", rsp_authenticate, from_rsp_result_to_string(&result), request_id, is_last);
    }

    fn on_rsp_user_login(&mut self, rsp_user_login: Option<&Struct_CThostFtdcRspUserLoginField>, result: RspResult, request_id: i32, is_last: bool) {
        println!("on_rsp_user_login: {:?}, {}, {:?}, {:?}", rsp_user_login, from_rsp_result_to_string(&result), request_id, is_last);
    }

    fn on_rsp_user_logout(&mut self, rsp_user_logout: Option<&Struct_CThostFtdcUserLogoutField>, result: RspResult, request_id: i32, is_last: bool) {
        println!("on_rsp_user_logout: {:?}, {}, {:?}, {:?}", rsp_user_logout, from_rsp_result_to_string(&result), request_id, is_last);
    }

    fn on_rsp_order_insert(&mut self, input_order: Option<&Struct_CThostFtdcInputOrderField>, result: RspResult, request_id: i32, is_last: bool) {
        println!("on_rsp_order_insert: {:?}, {}, {:?}, {:?}", input_order, from_rsp_result_to_string(&result), request_id, is_last);
    }

    fn on_rsp_order_action(&mut self, input_order_action: Option<&Struct_CThostFtdcInputOrderActionField>, result: RspResult, request_id: i32, is_last: bool) {
        println!("on_rsp_order_action: {:?}, {}, {:?}, {:?}", input_order_action, from_rsp_result_to_string(&result), request_id, is_last);
    }

    fn on_rsp_settlement_info_confirm(&mut self, settlement_info_confirm: Option<&Struct_CThostFtdcSettlementInfoConfirmField>, result: RspResult, request_id: i32, is_last: bool) {
        println!("on_rsp_settlement_info_confirm: {:?}, {}, {:?}, {:?}", settlement_info_confirm, from_rsp_result_to_string(&result), request_id, is_last);
    }

    fn on_rsp_qry_order(&mut self, order: Option<&Struct_CThostFtdcOrderField>, result: RspResult, request_id: i32, is_last: bool) {
        println!("on_rsp_qry_order: {:?}, {}, {:?}, {:?}", order, from_rsp_result_to_string(&result), request_id, is_last);
    }

    fn on_rsp_qry_trade(&mut self, trade: Option<&Struct_CThostFtdcTradeField>, result: RspResult, request_id: i32, is_last: bool) {
        println!("on_rsp_qry_trade: {:?}, {}, {:?}, {:?}", trade, from_rsp_result_to_string(&result), request_id, is_last);
    }

    fn on_rsp_qry_investor_position(&mut self, investor_position: Option<&Struct_CThostFtdcInvestorPositionField>, result: RspResult, request_id: i32, is_last: bool) {
        println!("on_rsp_qry_investor_position: {:?}, {}, {:?}, {:?}", investor_position, from_rsp_result_to_string(&result), request_id, is_last);
    }

    fn on_rsp_qry_trading_account(&mut self, trading_account: Option<&Struct_CThostFtdcTradingAccountField>, result: RspResult, request_id: i32, is_last: bool) {
        println!("on_rsp_qry_trading_account: {:?}, {}, {:?}, {:?}", trading_account, from_rsp_result_to_string(&result), request_id, is_last);
    }

    fn on_rsp_qry_investor(&mut self, investor: Option<&Struct_CThostFtdcInvestorField>, result: RspResult, request_id: i32, is_last: bool) {
        println!("on_rsp_qry_investor: {:?}, {}, {:?}, {:?}", investor, from_rsp_result_to_string(&result), request_id, is_last);
    }

    fn on_rsp_qry_trading_code(&mut self, trading_code: Option<&Struct_CThostFtdcTradingCodeField>, result: RspResult, request_id: i32, is_last: bool) {
        println!("on_rsp_qry_trading_code: {:?}, {}, {:?}, {:?}", trading_code, from_rsp_result_to_string(&result), request_id, is_last);
    }

	fn on_rsp_qry_instrument_margin_rate(&mut self, instrument_margin_rate: Option<&Struct_CThostFtdcInstrumentMarginRateField>, result: RspResult, request_id: i32, is_last: bool) {
        println!("on_rsp_qry_instrument_margin_rate: {:?}, {}, {:?}, {:?}", instrument_margin_rate, from_rsp_result_to_string(&result), request_id, is_last);
    }

	fn on_rsp_qry_instrument_commission_rate(&mut self, instrument_commission_rate: Option<&Struct_CThostFtdcInstrumentCommissionRateField>, result: RspResult, request_id: i32, is_last: bool) {
        println!("on_rsp_qry_instrument_commission_rate: {:?}, {}, {:?}, {:?}", instrument_commission_rate, from_rsp_result_to_string(&result), request_id, is_last);
    }

    fn on_rsp_qry_exchange(&mut self, exchange: Option<&Struct_CThostFtdcExchangeField>, result: RspResult, request_id: i32, is_last: bool) {
        println!("on_rsp_qry_exchange: {:?}, {}, {:?}, {:?}", exchange, from_rsp_result_to_string(&result), request_id, is_last);
    }

    fn on_rsp_qry_product(&mut self, product: Option<&Struct_CThostFtdcProductField>, result: RspResult, request_id: i32, is_last: bool) {
        println!("on_rsp_qry_product: {:?}, {}, {:?}, {:?}", product, from_rsp_result_to_string(&result), request_id, is_last);
    }

    fn on_rsp_qry_instrument(&mut self, instrument: Option<&Struct_CThostFtdcInstrumentField>, result: RspResult, request_id: i32, is_last: bool) {
        println!("on_rsp_qry_instrument: {:?}, {}, {:?}, {:?}", instrument, from_rsp_result_to_string(&result), request_id, is_last);
    }

    fn on_rsp_qry_settlement_info(&mut self, settlement_info: Option<&Struct_CThostFtdcSettlementInfoField>, result: RspResult, request_id: i32, is_last: bool) {
        println!("on_rsp_qry_settlement_info: {:?}, {}, {:?}, {:?}", settlement_info, from_rsp_result_to_string(&result), request_id, is_last);
    }

    fn on_rsp_qry_settlement_info_confirm(&mut self, settlement_info_confirm: Option<&Struct_CThostFtdcSettlementInfoConfirmField>, result: RspResult, request_id: i32, is_last: bool) {
        println!("on_rsp_qry_settlement_info_confirm: {:?}, {}, {:?}, {:?}", settlement_info_confirm, from_rsp_result_to_string(&result), request_id, is_last);
    }

	fn on_rsp_qry_exchange_margin_rate(&mut self, exchange_margin_rate: Option<&Struct_CThostFtdcExchangeMarginRateField>, result: RspResult, request_id: i32, is_last: bool) {
        println!("on_rsp_qry_exchange_margin_rate: {:?}, {}, {:?}, {:?}", exchange_margin_rate, from_rsp_result_to_string(&result), request_id, is_last);
    }

	fn on_rsp_qry_exchange_margin_rate_adjust(&mut self, exchange_margin_rate_adjust: Option<&Struct_CThostFtdcExchangeMarginRateAdjustField>, result: RspResult, request_id: i32, is_last: bool) {
        println!("on_rsp_qry_exchange_margin_rate_adjust: {:?}, {}, {:?}, {:?}", exchange_margin_rate_adjust, from_rsp_result_to_string(&result), request_id, is_last);
    }

	fn on_rsp_qry_exchange_rate(&mut self, exchange_rate: Option<&Struct_CThostFtdcExchangeRateField>, result: RspResult, request_id: i32, is_last: bool) {
        println!("on_rsp_qry_exchange_rate: {:?}, {}, {:?}, {:?}", exchange_rate, from_rsp_result_to_string(&result), request_id, is_last);
    }

    fn on_rsp_error(&mut self, result: RspResult, request_id: i32, is_last: bool) {
        println!("on_rsp_error: {}, {:?}, {:?}", from_rsp_result_to_string(&result), request_id, is_last);
    }

    fn on_rtn_order(&mut self, order: Option<&Struct_CThostFtdcOrderField>) {
        println!("on_rtn_order: {:?}", order);
    }

    fn on_rtn_trade(&mut self, trade: Option<&Struct_CThostFtdcTradeField>) {
        println!("on_rtn_trade: {:?}", trade);
    }

    fn on_err_rtn_order_insert(&mut self, input_order: Option<&Struct_CThostFtdcInputOrderField>, result: RspResult) {
        println!("on_err_rtn_order_insert: {:?}, {}", input_order, from_rsp_result_to_string(&result));
    }

    fn on_err_rtn_order_action(&mut self, order_action: Option<&Struct_CThostFtdcOrderActionField>, result: RspResult) {
        println!("on_err_rtn_order_action: {:?}, {}", order_action, from_rsp_result_to_string(&result));
    }

    fn on_rtn_instrument_status(&mut self, instrument_status: Option<&Struct_CThostFtdcInstrumentStatusField>) {
        println!("on_rtn_instrument_status: {:?}", instrument_status);
    }

    fn on_rtn_trading_notice(&mut self, trading_notice_info: Option<&Struct_CThostFtdcTradingNoticeInfoField>) {
        println!("on_rtn_trading_notice: {:?}", trading_notice_info);
    }
}

#[allow(non_snake_case)]
extern "C" fn spi_on_front_connected(spi: *mut Struct_CThostFtdcTraderSpi) {
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut Struct_CThostFtdcTraderSpi, &mut Struct_CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_front_connected() };
}

#[allow(non_snake_case)]
extern "C" fn spi_on_front_disconnected(spi: *mut Struct_CThostFtdcTraderSpi, nReason: c_int) {
    let reason = std::convert::From::from(nReason);
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut Struct_CThostFtdcTraderSpi, &mut Struct_CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_front_disconnected(reason) };
}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_heart_beat_warning(spi: *mut Struct_CThostFtdcTraderSpi, nTimeLapse: c_int) {
    // CTP API specification shows this will never be called
    unreachable!();
}

#[allow(non_snake_case)]
extern "C" fn spi_on_rsp_authenticate(spi: *mut Struct_CThostFtdcTraderSpi, pRspAuthenticateField: *const Struct_CThostFtdcRspAuthenticateField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut Struct_CThostFtdcTraderSpi, &mut Struct_CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_rsp_authenticate(pRspAuthenticateField.as_ref(), rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case)]
extern "C" fn spi_on_rsp_user_login(spi: *mut Struct_CThostFtdcTraderSpi, pRspUserLogin: *const Struct_CThostFtdcRspUserLoginField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut Struct_CThostFtdcTraderSpi, &mut Struct_CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_rsp_user_login(pRspUserLogin.as_ref(), rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case)]
extern "C" fn spi_on_rsp_user_logout(spi: *mut Struct_CThostFtdcTraderSpi, pUserLogout: *const Struct_CThostFtdcUserLogoutField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut Struct_CThostFtdcTraderSpi, &mut Struct_CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_rsp_user_logout(pUserLogout.as_ref(), rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_user_password_update(spi: *mut Struct_CThostFtdcTraderSpi, pUserPasswordUpdate: *const Struct_CThostFtdcUserPasswordUpdateField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_trading_account_password_update(spi: *mut Struct_CThostFtdcTraderSpi, pTradingAccountPasswordUpdate: *const Struct_CThostFtdcTradingAccountPasswordUpdateField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case)]
extern "C" fn spi_on_rsp_order_insert(spi: *mut Struct_CThostFtdcTraderSpi, pInputOrder: *const Struct_CThostFtdcInputOrderField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut Struct_CThostFtdcTraderSpi, &mut Struct_CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_rsp_order_insert(pInputOrder.as_ref(), rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_parked_order_insert(spi: *mut Struct_CThostFtdcTraderSpi, pParkedOrder: *const Struct_CThostFtdcParkedOrderField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_parked_order_action(spi: *mut Struct_CThostFtdcTraderSpi, pParkedOrderAction: *const Struct_CThostFtdcParkedOrderActionField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case)]
extern "C" fn spi_on_rsp_order_action(spi: *mut Struct_CThostFtdcTraderSpi, pInputOrderAction: *const Struct_CThostFtdcInputOrderActionField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut Struct_CThostFtdcTraderSpi, &mut Struct_CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_rsp_order_action(pInputOrderAction.as_ref(), rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_query_max_order_volume(spi: *mut Struct_CThostFtdcTraderSpi, pQueryMaxOrderVolume: *const Struct_CThostFtdcQueryMaxOrderVolumeField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case)]
extern "C" fn spi_on_rsp_settlement_info_confirm(spi: *mut Struct_CThostFtdcTraderSpi, pSettlementInfoConfirm: *const Struct_CThostFtdcSettlementInfoConfirmField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut Struct_CThostFtdcTraderSpi, &mut Struct_CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_rsp_settlement_info_confirm(pSettlementInfoConfirm.as_ref(), rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_remove_parked_order(spi: *mut Struct_CThostFtdcTraderSpi, pRemoveParkedOrder: *const Struct_CThostFtdcRemoveParkedOrderField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_remove_parked_order_action(spi: *mut Struct_CThostFtdcTraderSpi, pRemoveParkedOrderAction: *const Struct_CThostFtdcRemoveParkedOrderActionField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_exec_order_insert(spi: *mut Struct_CThostFtdcTraderSpi, pInputExecOrder: *const Struct_CThostFtdcInputExecOrderField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_exec_order_action(spi: *mut Struct_CThostFtdcTraderSpi, pInputExecOrderAction: *const Struct_CThostFtdcInputExecOrderActionField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_for_quote_insert(spi: *mut Struct_CThostFtdcTraderSpi, pInputForQuote: *const Struct_CThostFtdcInputForQuoteField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_quote_insert(spi: *mut Struct_CThostFtdcTraderSpi, pInputQuote: *const Struct_CThostFtdcInputQuoteField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_quote_action(spi: *mut Struct_CThostFtdcTraderSpi, pInputQuoteAction: *const Struct_CThostFtdcInputQuoteActionField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_comb_action_insert(spi: *mut Struct_CThostFtdcTraderSpi, pInputCombAction: *const Struct_CThostFtdcInputCombActionField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case)]
extern "C" fn spi_on_rsp_qry_order(spi: *mut Struct_CThostFtdcTraderSpi, pOrder: *const Struct_CThostFtdcOrderField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut Struct_CThostFtdcTraderSpi, &mut Struct_CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_rsp_qry_order(pOrder.as_ref(), rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case)]
extern "C" fn spi_on_rsp_qry_trade(spi: *mut Struct_CThostFtdcTraderSpi, pTrade: *const Struct_CThostFtdcTradeField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut Struct_CThostFtdcTraderSpi, &mut Struct_CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_rsp_qry_trade(pTrade.as_ref(), rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case)]
extern "C" fn spi_on_rsp_qry_investor_position(spi: *mut Struct_CThostFtdcTraderSpi, pInvestorPosition: *const Struct_CThostFtdcInvestorPositionField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut Struct_CThostFtdcTraderSpi, &mut Struct_CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_rsp_qry_investor_position(pInvestorPosition.as_ref(), rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case)]
extern "C" fn spi_on_rsp_qry_trading_account(spi: *mut Struct_CThostFtdcTraderSpi, pTradingAccount: *const Struct_CThostFtdcTradingAccountField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut Struct_CThostFtdcTraderSpi, &mut Struct_CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_rsp_qry_trading_account(pTradingAccount.as_ref(), rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case)]
extern "C" fn spi_on_rsp_qry_investor(spi: *mut Struct_CThostFtdcTraderSpi, pInvestor: *const Struct_CThostFtdcInvestorField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut Struct_CThostFtdcTraderSpi, &mut Struct_CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_rsp_qry_investor(pInvestor.as_ref(), rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case)]
extern "C" fn spi_on_rsp_qry_trading_code(spi: *mut Struct_CThostFtdcTraderSpi, pTradingCode: *const Struct_CThostFtdcTradingCodeField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut Struct_CThostFtdcTraderSpi, &mut Struct_CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_rsp_qry_trading_code(pTradingCode.as_ref(), rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_instrument_margin_rate(spi: *mut Struct_CThostFtdcTraderSpi, pInstrumentMarginRate: *const Struct_CThostFtdcInstrumentMarginRateField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut Struct_CThostFtdcTraderSpi, &mut Struct_CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_rsp_qry_instrument_margin_rate(pInstrumentMarginRate.as_ref(), rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_instrument_commission_rate(spi: *mut Struct_CThostFtdcTraderSpi, pInstrumentCommissionRate: *const Struct_CThostFtdcInstrumentCommissionRateField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut Struct_CThostFtdcTraderSpi, &mut Struct_CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_rsp_qry_instrument_commission_rate(pInstrumentCommissionRate.as_ref(), rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_exchange(spi: *mut Struct_CThostFtdcTraderSpi, pExchange: *const Struct_CThostFtdcExchangeField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut Struct_CThostFtdcTraderSpi, &mut Struct_CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_rsp_qry_exchange(pExchange.as_ref(), rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_product(spi: *mut Struct_CThostFtdcTraderSpi, pProduct: *const Struct_CThostFtdcProductField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut Struct_CThostFtdcTraderSpi, &mut Struct_CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_rsp_qry_product(pProduct.as_ref(), rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_instrument(spi: *mut Struct_CThostFtdcTraderSpi, pInstrument: *const Struct_CThostFtdcInstrumentField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut Struct_CThostFtdcTraderSpi, &mut Struct_CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_rsp_qry_instrument(pInstrument.as_ref(), rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_depth_market_data(spi: *mut Struct_CThostFtdcTraderSpi, pDepthMarketData: *const Struct_CThostFtdcDepthMarketDataField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_settlement_info(spi: *mut Struct_CThostFtdcTraderSpi, pSettlementInfo: *const Struct_CThostFtdcSettlementInfoField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut Struct_CThostFtdcTraderSpi, &mut Struct_CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_rsp_qry_settlement_info(pSettlementInfo.as_ref(), rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_transfer_bank(spi: *mut Struct_CThostFtdcTraderSpi, pTransferBank: *const Struct_CThostFtdcTransferBankField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_investor_position_detail(spi: *mut Struct_CThostFtdcTraderSpi, pInvestorPositionDetail: *const Struct_CThostFtdcInvestorPositionDetailField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_notice(spi: *mut Struct_CThostFtdcTraderSpi, pNotice: *const Struct_CThostFtdcNoticeField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_settlement_info_confirm(spi: *mut Struct_CThostFtdcTraderSpi, pSettlementInfoConfirm: *const Struct_CThostFtdcSettlementInfoConfirmField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut Struct_CThostFtdcTraderSpi, &mut Struct_CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_rsp_qry_settlement_info_confirm(pSettlementInfoConfirm.as_ref(), rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_investor_position_combine_detail(spi: *mut Struct_CThostFtdcTraderSpi, pInvestorPositionCombineDetail: *const Struct_CThostFtdcInvestorPositionCombineDetailField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_cfmmc_trading_account_key(spi: *mut Struct_CThostFtdcTraderSpi, pCFMMCTradingAccountKey: *const Struct_CThostFtdcCFMMCTradingAccountKeyField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_e_warrant_offset(spi: *mut Struct_CThostFtdcTraderSpi, pEWarrantOffset: *const Struct_CThostFtdcEWarrantOffsetField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_investor_product_group_margin(spi: *mut Struct_CThostFtdcTraderSpi, pInvestorProductGroupMargin: *const Struct_CThostFtdcInvestorProductGroupMarginField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_exchange_margin_rate(spi: *mut Struct_CThostFtdcTraderSpi, pExchangeMarginRate: *const Struct_CThostFtdcExchangeMarginRateField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut Struct_CThostFtdcTraderSpi, &mut Struct_CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_rsp_qry_exchange_margin_rate(pExchangeMarginRate.as_ref(), rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_exchange_margin_rate_adjust(spi: *mut Struct_CThostFtdcTraderSpi, pExchangeMarginRateAdjust: *const Struct_CThostFtdcExchangeMarginRateAdjustField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut Struct_CThostFtdcTraderSpi, &mut Struct_CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_rsp_qry_exchange_margin_rate_adjust(pExchangeMarginRateAdjust.as_ref(), rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_exchange_rate(spi: *mut Struct_CThostFtdcTraderSpi, pExchangeRate: *const Struct_CThostFtdcExchangeRateField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut Struct_CThostFtdcTraderSpi, &mut Struct_CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_rsp_qry_exchange_rate(pExchangeRate.as_ref(), rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_sec_agent_acid_map(spi: *mut Struct_CThostFtdcTraderSpi, pSecAgentACIDMap: *const Struct_CThostFtdcSecAgentACIDMapField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_product_group(spi: *mut Struct_CThostFtdcTraderSpi, pProductGroup: *const Struct_CThostFtdcProductGroupField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_instrument_order_comm_rate(spi: *mut Struct_CThostFtdcTraderSpi, pInstrumentOrderCommRate: *const Struct_CThostFtdcInstrumentOrderCommRateField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_option_instr_trade_cost(spi: *mut Struct_CThostFtdcTraderSpi, pOptionInstrTradeCost: *const Struct_CThostFtdcOptionInstrTradeCostField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_option_instr_comm_rate(spi: *mut Struct_CThostFtdcTraderSpi, pOptionInstrCommRate: *const Struct_CThostFtdcOptionInstrCommRateField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_exec_order(spi: *mut Struct_CThostFtdcTraderSpi, pExecOrder: *const Struct_CThostFtdcExecOrderField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_for_quote(spi: *mut Struct_CThostFtdcTraderSpi, pForQuote: *const Struct_CThostFtdcForQuoteField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_quote(spi: *mut Struct_CThostFtdcTraderSpi, pQuote: *const Struct_CThostFtdcQuoteField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_comb_instrument_guard(spi: *mut Struct_CThostFtdcTraderSpi, pCombInstrumentGuard: *const Struct_CThostFtdcCombInstrumentGuardField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_comb_action(spi: *mut Struct_CThostFtdcTraderSpi, pCombAction: *const Struct_CThostFtdcCombActionField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_transfer_serial(spi: *mut Struct_CThostFtdcTraderSpi, pTransferSerial: *const Struct_CThostFtdcTransferSerialField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_accountregister(spi: *mut Struct_CThostFtdcTraderSpi, pAccountregister: *const Struct_CThostFtdcAccountregisterField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case)]
extern "C" fn spi_on_rsp_error(spi: *mut Struct_CThostFtdcTraderSpi, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut Struct_CThostFtdcTraderSpi, &mut Struct_CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_rsp_error(rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case)]
extern "C" fn spi_on_rtn_order(spi: *mut Struct_CThostFtdcTraderSpi, pOrder: *const Struct_CThostFtdcOrderField) {
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut Struct_CThostFtdcTraderSpi, &mut Struct_CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_rtn_order(pOrder.as_ref()) };
}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rtn_trade(spi: *mut Struct_CThostFtdcTraderSpi, pTrade: *const Struct_CThostFtdcTradeField) {
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut Struct_CThostFtdcTraderSpi, &mut Struct_CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_rtn_trade(pTrade.as_ref()) };
}

#[allow(non_snake_case)]
extern "C" fn spi_on_err_rtn_order_insert(spi: *mut Struct_CThostFtdcTraderSpi, pInputOrder: *const Struct_CThostFtdcInputOrderField, pRspInfo: *const Struct_CThostFtdcRspInfoField) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut Struct_CThostFtdcTraderSpi, &mut Struct_CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_err_rtn_order_insert(pInputOrder.as_ref(), rsp_info) };
}

#[allow(non_snake_case)]
extern "C" fn spi_on_err_rtn_order_action(spi: *mut Struct_CThostFtdcTraderSpi, pOrderAction: *const Struct_CThostFtdcOrderActionField, pRspInfo: *const Struct_CThostFtdcRspInfoField) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut Struct_CThostFtdcTraderSpi, &mut Struct_CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_err_rtn_order_action(pOrderAction.as_ref(), rsp_info) };
}

#[allow(non_snake_case)]
extern "C" fn spi_on_rtn_instrument_status(spi: *mut Struct_CThostFtdcTraderSpi, pInstrumentStatus: *const Struct_CThostFtdcInstrumentStatusField) {
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut Struct_CThostFtdcTraderSpi, &mut Struct_CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_rtn_instrument_status(pInstrumentStatus.as_ref()) };
}

#[allow(non_snake_case)]
extern "C" fn spi_on_rtn_trading_notice(spi: *mut Struct_CThostFtdcTraderSpi, pTradingNoticeInfo: *const Struct_CThostFtdcTradingNoticeInfoField) {
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut Struct_CThostFtdcTraderSpi, &mut Struct_CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_rtn_trading_notice(pTradingNoticeInfo.as_ref()) };
}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rtn_error_conditional_order(spi: *mut Struct_CThostFtdcTraderSpi, pErrorConditionalOrder: *const Struct_CThostFtdcErrorConditionalOrderField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rtn_exec_order(spi: *mut Struct_CThostFtdcTraderSpi, pExecOrder: *const Struct_CThostFtdcExecOrderField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_err_rtn_exec_order_insert(spi: *mut Struct_CThostFtdcTraderSpi, pInputExecOrder: *const Struct_CThostFtdcInputExecOrderField, pRspInfo: *const Struct_CThostFtdcRspInfoField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_err_rtn_exec_order_action(spi: *mut Struct_CThostFtdcTraderSpi, pExecOrderAction: *const Struct_CThostFtdcExecOrderActionField, pRspInfo: *const Struct_CThostFtdcRspInfoField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_err_rtn_for_quote_insert(spi: *mut Struct_CThostFtdcTraderSpi, pInputForQuote: *const Struct_CThostFtdcInputForQuoteField, pRspInfo: *const Struct_CThostFtdcRspInfoField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rtn_quote(spi: *mut Struct_CThostFtdcTraderSpi, pQuote: *const Struct_CThostFtdcQuoteField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_err_rtn_quote_insert(spi: *mut Struct_CThostFtdcTraderSpi, pInputQuote: *const Struct_CThostFtdcInputQuoteField, pRspInfo: *const Struct_CThostFtdcRspInfoField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_err_rtn_quote_action(spi: *mut Struct_CThostFtdcTraderSpi, pQuoteAction: *const Struct_CThostFtdcQuoteActionField, pRspInfo: *const Struct_CThostFtdcRspInfoField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rtn_for_quote_rsp(spi: *mut Struct_CThostFtdcTraderSpi, pForQuoteRsp: *const Struct_CThostFtdcForQuoteRspField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rtn_cfmmc_trading_account_token(spi: *mut Struct_CThostFtdcTraderSpi, pCFMMCTradingAccountToken: *const Struct_CThostFtdcCFMMCTradingAccountTokenField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rtn_comb_action(spi: *mut Struct_CThostFtdcTraderSpi, pCombAction: *const Struct_CThostFtdcCombActionField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_err_rtn_comb_action_insert(spi: *mut Struct_CThostFtdcTraderSpi, pInputCombAction: *const Struct_CThostFtdcInputCombActionField, pRspInfo: *const Struct_CThostFtdcRspInfoField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_contract_bank(spi: *mut Struct_CThostFtdcTraderSpi, pContractBank: *const Struct_CThostFtdcContractBankField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_parked_order(spi: *mut Struct_CThostFtdcTraderSpi, pParkedOrder: *const Struct_CThostFtdcParkedOrderField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_parked_order_action(spi: *mut Struct_CThostFtdcTraderSpi, pParkedOrderAction: *const Struct_CThostFtdcParkedOrderActionField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_trading_notice(spi: *mut Struct_CThostFtdcTraderSpi, pTradingNotice: *const Struct_CThostFtdcTradingNoticeField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_broker_trading_params(spi: *mut Struct_CThostFtdcTraderSpi, pBrokerTradingParams: *const Struct_CThostFtdcBrokerTradingParamsField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_broker_trading_algos(spi: *mut Struct_CThostFtdcTraderSpi, pBrokerTradingAlgos: *const Struct_CThostFtdcBrokerTradingAlgosField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_query_cfmmc_trading_account_token(spi: *mut Struct_CThostFtdcTraderSpi, pQueryCFMMCTradingAccountToken: *const Struct_CThostFtdcQueryCFMMCTradingAccountTokenField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rtn_from_bank_to_future_by_bank(spi: *mut Struct_CThostFtdcTraderSpi, pRspTransfer: *const Struct_CThostFtdcRspTransferField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rtn_from_future_to_bank_by_bank(spi: *mut Struct_CThostFtdcTraderSpi, pRspTransfer: *const Struct_CThostFtdcRspTransferField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rtn_repeal_from_bank_to_future_by_bank(spi: *mut Struct_CThostFtdcTraderSpi, pRspRepeal: *const Struct_CThostFtdcRspRepealField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rtn_repeal_from_future_to_bank_by_bank(spi: *mut Struct_CThostFtdcTraderSpi, pRspRepeal: *const Struct_CThostFtdcRspRepealField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rtn_from_bank_to_future_by_future(spi: *mut Struct_CThostFtdcTraderSpi, pRspTransfer: *const Struct_CThostFtdcRspTransferField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rtn_from_future_to_bank_by_future(spi: *mut Struct_CThostFtdcTraderSpi, pRspTransfer: *const Struct_CThostFtdcRspTransferField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rtn_repeal_from_bank_to_future_by_future_manual(spi: *mut Struct_CThostFtdcTraderSpi, pRspRepeal: *const Struct_CThostFtdcRspRepealField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rtn_repeal_from_future_to_bank_by_future_manual(spi: *mut Struct_CThostFtdcTraderSpi, pRspRepeal: *const Struct_CThostFtdcRspRepealField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rtn_query_bank_balance_by_future(spi: *mut Struct_CThostFtdcTraderSpi, pNotifyQueryAccount: *const Struct_CThostFtdcNotifyQueryAccountField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_err_rtn_bank_to_future_by_future(spi: *mut Struct_CThostFtdcTraderSpi, pReqTransfer: *const Struct_CThostFtdcReqTransferField, pRspInfo: *const Struct_CThostFtdcRspInfoField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_err_rtn_future_to_bank_by_future(spi: *mut Struct_CThostFtdcTraderSpi, pReqTransfer: *const Struct_CThostFtdcReqTransferField, pRspInfo: *const Struct_CThostFtdcRspInfoField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_err_rtn_repeal_bank_to_future_by_future_manual(spi: *mut Struct_CThostFtdcTraderSpi, pReqRepeal: *const Struct_CThostFtdcReqRepealField, pRspInfo: *const Struct_CThostFtdcRspInfoField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_err_rtn_repeal_future_to_bank_by_future_manual(spi: *mut Struct_CThostFtdcTraderSpi, pReqRepeal: *const Struct_CThostFtdcReqRepealField, pRspInfo: *const Struct_CThostFtdcRspInfoField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_err_rtn_query_bank_balance_by_future(spi: *mut Struct_CThostFtdcTraderSpi, pReqQueryAccount: *const Struct_CThostFtdcReqQueryAccountField, pRspInfo: *const Struct_CThostFtdcRspInfoField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rtn_repeal_from_bank_to_future_by_future(spi: *mut Struct_CThostFtdcTraderSpi, pRspRepeal: *const Struct_CThostFtdcRspRepealField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rtn_repeal_from_future_to_bank_by_future(spi: *mut Struct_CThostFtdcTraderSpi, pRspRepeal: *const Struct_CThostFtdcRspRepealField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_from_bank_to_future_by_future(spi: *mut Struct_CThostFtdcTraderSpi, pReqTransfer: *const Struct_CThostFtdcReqTransferField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_from_future_to_bank_by_future(spi: *mut Struct_CThostFtdcTraderSpi, pReqTransfer: *const Struct_CThostFtdcReqTransferField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_query_bank_account_money_by_future(spi: *mut Struct_CThostFtdcTraderSpi, pReqQueryAccount: *const Struct_CThostFtdcReqQueryAccountField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rtn_open_account_by_bank(spi: *mut Struct_CThostFtdcTraderSpi, pOpenAccount: *const Struct_CThostFtdcOpenAccountField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rtn_cancel_account_by_bank(spi: *mut Struct_CThostFtdcTraderSpi, pCancelAccount: *const Struct_CThostFtdcCancelAccountField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rtn_change_account_by_bank(spi: *mut Struct_CThostFtdcTraderSpi, pChangeAccount: *const Struct_CThostFtdcChangeAccountField) {}

#[repr(C)]
#[derive(Debug)]
struct SpiVTable {
    #[allow(non_snake_case)]
    on_front_connected: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi),
    #[allow(non_snake_case)]
    on_front_disconnected: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, nReason: c_int),
    #[allow(non_snake_case)]
    on_heart_beat_warning: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, nTimeLapse: c_int),
    #[allow(non_snake_case)]
    on_rsp_authenticate: extern "C" fn (spi: *mut Struct_CThostFtdcTraderSpi, pRspAuthenticateField: *const Struct_CThostFtdcRspAuthenticateField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_user_login: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pRspUserLogin: *const Struct_CThostFtdcRspUserLoginField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_user_logout: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pUserLogout: *const Struct_CThostFtdcUserLogoutField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_user_password_update: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pUserPasswordUpdate: *const Struct_CThostFtdcUserPasswordUpdateField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_trading_account_password_update: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pTradingAccountPasswordUpdate: *const Struct_CThostFtdcTradingAccountPasswordUpdateField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_order_insert: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pInputOrder: *const Struct_CThostFtdcInputOrderField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_parked_order_insert: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pParkedOrder: *const Struct_CThostFtdcParkedOrderField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_parked_order_action: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pParkedOrderAction: *const Struct_CThostFtdcParkedOrderActionField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_order_action: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pInputOrderAction: *const Struct_CThostFtdcInputOrderActionField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_query_max_order_volume: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pQueryMaxOrderVolume: *const Struct_CThostFtdcQueryMaxOrderVolumeField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_settlement_info_confirm: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pSettlementInfoConfirm: *const Struct_CThostFtdcSettlementInfoConfirmField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_remove_parked_order: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pRemoveParkedOrder: *const Struct_CThostFtdcRemoveParkedOrderField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_remove_parked_order_action: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pRemoveParkedOrderAction: *const Struct_CThostFtdcRemoveParkedOrderActionField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_exec_order_insert: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pInputExecOrder: *const Struct_CThostFtdcInputExecOrderField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_exec_order_action: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pInputExecOrderAction: *const Struct_CThostFtdcInputExecOrderActionField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_for_quote_insert: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pInputForQuote: *const Struct_CThostFtdcInputForQuoteField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_quote_insert: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pInputQuote: *const Struct_CThostFtdcInputQuoteField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_quote_action: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pInputQuoteAction: *const Struct_CThostFtdcInputQuoteActionField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_comb_action_insert: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pInputCombAction: *const Struct_CThostFtdcInputCombActionField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_order: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pOrder: *const Struct_CThostFtdcOrderField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_trade: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pTrade: *const Struct_CThostFtdcTradeField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_investor_position: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pInvestorPosition: *const Struct_CThostFtdcInvestorPositionField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_trading_account: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pTradingAccount: *const Struct_CThostFtdcTradingAccountField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_investor: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pInvestor: *const Struct_CThostFtdcInvestorField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_trading_code: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pTradingCode: *const Struct_CThostFtdcTradingCodeField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_instrument_margin_rate: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pInstrumentMarginRate: *const Struct_CThostFtdcInstrumentMarginRateField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_instrument_commission_rate: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pInstrumentCommissionRate: *const Struct_CThostFtdcInstrumentCommissionRateField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_exchange: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pExchange: *const Struct_CThostFtdcExchangeField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_product: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pProduct: *const Struct_CThostFtdcProductField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_instrument: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pInstrument: *const Struct_CThostFtdcInstrumentField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_depth_market_data: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pDepthMarketData: *const Struct_CThostFtdcDepthMarketDataField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_settlement_info: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pSettlementInfo: *const Struct_CThostFtdcSettlementInfoField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_transfer_bank: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pTransferBank: *const Struct_CThostFtdcTransferBankField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_investor_position_detail: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pInvestorPositionDetail: *const Struct_CThostFtdcInvestorPositionDetailField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_notice: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pNotice: *const Struct_CThostFtdcNoticeField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_settlement_info_confirm: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pSettlementInfoConfirm: *const Struct_CThostFtdcSettlementInfoConfirmField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_investor_position_combine_detail: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pInvestorPositionCombineDetail: *const Struct_CThostFtdcInvestorPositionCombineDetailField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_cfmmc_trading_account_key: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pCFMMCTradingAccountKey: *const Struct_CThostFtdcCFMMCTradingAccountKeyField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_e_warrant_offset: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pEWarrantOffset: *const Struct_CThostFtdcEWarrantOffsetField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_investor_product_group_margin: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pInvestorProductGroupMargin: *const Struct_CThostFtdcInvestorProductGroupMarginField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_exchange_margin_rate: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pExchangeMarginRate: *const Struct_CThostFtdcExchangeMarginRateField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_exchange_margin_rate_adjust: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pExchangeMarginRateAdjust: *const Struct_CThostFtdcExchangeMarginRateAdjustField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_exchange_rate: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pExchangeRate: *const Struct_CThostFtdcExchangeRateField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_sec_agent_acid_map: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pSecAgentACIDMap: *const Struct_CThostFtdcSecAgentACIDMapField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_product_group: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pProductGroup: *const Struct_CThostFtdcProductGroupField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_instrument_order_comm_rate: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pInstrumentOrderCommRate: *const Struct_CThostFtdcInstrumentOrderCommRateField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_option_instr_trade_cost: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pOptionInstrTradeCost: *const Struct_CThostFtdcOptionInstrTradeCostField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_option_instr_comm_rate: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pOptionInstrCommRate: *const Struct_CThostFtdcOptionInstrCommRateField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_exec_order: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pExecOrder: *const Struct_CThostFtdcExecOrderField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_for_quote: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pForQuote: *const Struct_CThostFtdcForQuoteField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_quote: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pQuote: *const Struct_CThostFtdcQuoteField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_comb_instrument_guard: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pCombInstrumentGuard: *const Struct_CThostFtdcCombInstrumentGuardField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_comb_action: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pCombAction: *const Struct_CThostFtdcCombActionField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_transfer_serial: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pTransferSerial: *const Struct_CThostFtdcTransferSerialField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_accountregister: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pAccountregister: *const Struct_CThostFtdcAccountregisterField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_error: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rtn_order: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pOrder: *const Struct_CThostFtdcOrderField),
    #[allow(non_snake_case)]
    on_rtn_trade: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pTrade: *const Struct_CThostFtdcTradeField),
    #[allow(non_snake_case)]
    on_err_rtn_order_insert: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pInputOrder: *const Struct_CThostFtdcInputOrderField, pRspInfo: *const Struct_CThostFtdcRspInfoField),
    #[allow(non_snake_case)]
    on_err_rtn_order_action: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pOrderAction: *const Struct_CThostFtdcOrderActionField, pRspInfo: *const Struct_CThostFtdcRspInfoField),
    #[allow(non_snake_case)]
    on_rtn_instrument_status: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pInstrumentStatus: *const Struct_CThostFtdcInstrumentStatusField),
    #[allow(non_snake_case)]
    on_rtn_trading_notice: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pTradingNoticeInfo: *const Struct_CThostFtdcTradingNoticeInfoField),
    #[allow(non_snake_case)]
    on_rtn_error_conditional_order: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pErrorConditionalOrder: *const Struct_CThostFtdcErrorConditionalOrderField),
    #[allow(non_snake_case)]
    on_rtn_exec_order: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pExecOrder: *const Struct_CThostFtdcExecOrderField),
    #[allow(non_snake_case)]
    on_err_rtn_exec_order_insert: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pInputExecOrder: *const Struct_CThostFtdcInputExecOrderField, pRspInfo: *const Struct_CThostFtdcRspInfoField),
    #[allow(non_snake_case)]
    on_err_rtn_exec_order_action: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pExecOrderAction: *const Struct_CThostFtdcExecOrderActionField, pRspInfo: *const Struct_CThostFtdcRspInfoField),
    #[allow(non_snake_case)]
    on_err_rtn_for_quote_insert: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pInputForQuote: *const Struct_CThostFtdcInputForQuoteField, pRspInfo: *const Struct_CThostFtdcRspInfoField),
    #[allow(non_snake_case)]
    on_rtn_quote: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pQuote: *const Struct_CThostFtdcQuoteField),
    #[allow(non_snake_case)]
    on_err_rtn_quote_insert: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pInputQuote: *const Struct_CThostFtdcInputQuoteField, pRspInfo: *const Struct_CThostFtdcRspInfoField),
    #[allow(non_snake_case)]
    on_err_rtn_quote_action: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pQuoteAction: *const Struct_CThostFtdcQuoteActionField, pRspInfo: *const Struct_CThostFtdcRspInfoField),
    #[allow(non_snake_case)]
    on_rtn_for_quote_rsp: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pForQuoteRsp: *const Struct_CThostFtdcForQuoteRspField),
    #[allow(non_snake_case)]
    on_rtn_cfmmc_trading_account_token: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pCFMMCTradingAccountToken: *const Struct_CThostFtdcCFMMCTradingAccountTokenField),
    #[allow(non_snake_case)]
    on_rtn_comb_action: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pCombAction: *const Struct_CThostFtdcCombActionField),
    #[allow(non_snake_case)]
    on_err_rtn_comb_action_insert: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pInputCombAction: *const Struct_CThostFtdcInputCombActionField, pRspInfo: *const Struct_CThostFtdcRspInfoField),
    #[allow(non_snake_case)]
    on_rsp_qry_contract_bank: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pContractBank: *const Struct_CThostFtdcContractBankField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_parked_order: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pParkedOrder: *const Struct_CThostFtdcParkedOrderField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_parked_order_action: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pParkedOrderAction: *const Struct_CThostFtdcParkedOrderActionField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_trading_notice: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pTradingNotice: *const Struct_CThostFtdcTradingNoticeField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_broker_trading_params: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pBrokerTradingParams: *const Struct_CThostFtdcBrokerTradingParamsField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_broker_trading_algos: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pBrokerTradingAlgos: *const Struct_CThostFtdcBrokerTradingAlgosField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_query_cfmmc_trading_account_token: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pQueryCFMMCTradingAccountToken: *const Struct_CThostFtdcQueryCFMMCTradingAccountTokenField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rtn_from_bank_to_future_by_bank: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pRspTransfer: *const Struct_CThostFtdcRspTransferField),
    #[allow(non_snake_case)]
    on_rtn_from_future_to_bank_by_bank: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pRspTransfer: *const Struct_CThostFtdcRspTransferField),
    #[allow(non_snake_case)]
    on_rtn_repeal_from_bank_to_future_by_bank: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pRspRepeal: *const Struct_CThostFtdcRspRepealField),
    #[allow(non_snake_case)]
    on_rtn_repeal_from_future_to_bank_by_bank: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pRspRepeal: *const Struct_CThostFtdcRspRepealField),
    #[allow(non_snake_case)]
    on_rtn_from_bank_to_future_by_future: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pRspTransfer: *const Struct_CThostFtdcRspTransferField),
    #[allow(non_snake_case)]
    on_rtn_from_future_to_bank_by_future: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pRspTransfer: *const Struct_CThostFtdcRspTransferField),
    #[allow(non_snake_case)]
    on_rtn_repeal_from_bank_to_future_by_future_manual: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pRspRepeal: *const Struct_CThostFtdcRspRepealField),
    #[allow(non_snake_case)]
    on_rtn_repeal_from_future_to_bank_by_future_manual: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pRspRepeal: *const Struct_CThostFtdcRspRepealField),
    #[allow(non_snake_case)]
    on_rtn_query_bank_balance_by_future: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pNotifyQueryAccount: *const Struct_CThostFtdcNotifyQueryAccountField),
    #[allow(non_snake_case)]
    on_err_rtn_bank_to_future_by_future: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pReqTransfer: *const Struct_CThostFtdcReqTransferField, pRspInfo: *const Struct_CThostFtdcRspInfoField),
    #[allow(non_snake_case)]
    on_err_rtn_future_to_bank_by_future: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pReqTransfer: *const Struct_CThostFtdcReqTransferField, pRspInfo: *const Struct_CThostFtdcRspInfoField),
    #[allow(non_snake_case)]
    on_err_rtn_repeal_bank_to_future_by_future_manual: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pReqRepeal: *const Struct_CThostFtdcReqRepealField, pRspInfo: *const Struct_CThostFtdcRspInfoField),
    #[allow(non_snake_case)]
    on_err_rtn_repeal_future_to_bank_by_future_manual: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pReqRepeal: *const Struct_CThostFtdcReqRepealField, pRspInfo: *const Struct_CThostFtdcRspInfoField),
    #[allow(non_snake_case)]
    on_err_rtn_query_bank_balance_by_future: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pReqQueryAccount: *const Struct_CThostFtdcReqQueryAccountField, pRspInfo: *const Struct_CThostFtdcRspInfoField),
    #[allow(non_snake_case)]
    on_rtn_repeal_from_bank_to_future_by_future: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pRspRepeal: *const Struct_CThostFtdcRspRepealField),
    #[allow(non_snake_case)]
    on_rtn_repeal_from_future_to_bank_by_future: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pRspRepeal: *const Struct_CThostFtdcRspRepealField),
    #[allow(non_snake_case)]
    on_rsp_from_bank_to_future_by_future: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pReqTransfer: *const Struct_CThostFtdcReqTransferField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_from_future_to_bank_by_future: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pReqTransfer: *const Struct_CThostFtdcReqTransferField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_query_bank_account_money_by_future: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pReqQueryAccount: *const Struct_CThostFtdcReqQueryAccountField, pRspInfo: *const Struct_CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rtn_open_account_by_bank: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pOpenAccount: *const Struct_CThostFtdcOpenAccountField),
    #[allow(non_snake_case)]
    on_rtn_cancel_account_by_bank: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pCancelAccount: *const Struct_CThostFtdcCancelAccountField),
    #[allow(non_snake_case)]
    on_rtn_change_account_by_bank: extern "C" fn(spi: *mut Struct_CThostFtdcTraderSpi, pChangeAccount: *const Struct_CThostFtdcChangeAccountField),
}

static SPI_VTABLE: SpiVTable = SpiVTable{
    on_front_connected: spi_on_front_connected,
    on_front_disconnected: spi_on_front_disconnected,
    on_heart_beat_warning: spi_on_heart_beat_warning,
    on_rsp_authenticate: spi_on_rsp_authenticate,
    on_rsp_user_login: spi_on_rsp_user_login,
    on_rsp_user_logout: spi_on_rsp_user_logout,
    on_rsp_user_password_update: spi_on_rsp_user_password_update,
    on_rsp_trading_account_password_update: spi_on_rsp_trading_account_password_update,
    on_rsp_order_insert: spi_on_rsp_order_insert,
    on_rsp_parked_order_insert: spi_on_rsp_parked_order_insert,
    on_rsp_parked_order_action: spi_on_rsp_parked_order_action,
    on_rsp_order_action: spi_on_rsp_order_action,
    on_rsp_query_max_order_volume: spi_on_rsp_query_max_order_volume,
    on_rsp_settlement_info_confirm: spi_on_rsp_settlement_info_confirm,
    on_rsp_remove_parked_order: spi_on_rsp_remove_parked_order,
    on_rsp_remove_parked_order_action: spi_on_rsp_remove_parked_order_action,
    on_rsp_exec_order_insert: spi_on_rsp_exec_order_insert,
    on_rsp_exec_order_action: spi_on_rsp_exec_order_action,
    on_rsp_for_quote_insert: spi_on_rsp_for_quote_insert,
    on_rsp_quote_insert: spi_on_rsp_quote_insert,
    on_rsp_quote_action: spi_on_rsp_quote_action,
    on_rsp_comb_action_insert: spi_on_rsp_comb_action_insert,
    on_rsp_qry_order: spi_on_rsp_qry_order,
    on_rsp_qry_trade: spi_on_rsp_qry_trade,
    on_rsp_qry_investor_position: spi_on_rsp_qry_investor_position,
    on_rsp_qry_trading_account: spi_on_rsp_qry_trading_account,
    on_rsp_qry_investor: spi_on_rsp_qry_investor,
    on_rsp_qry_trading_code: spi_on_rsp_qry_trading_code,
    on_rsp_qry_instrument_margin_rate: spi_on_rsp_qry_instrument_margin_rate,
    on_rsp_qry_instrument_commission_rate: spi_on_rsp_qry_instrument_commission_rate,
    on_rsp_qry_exchange: spi_on_rsp_qry_exchange,
    on_rsp_qry_product: spi_on_rsp_qry_product,
    on_rsp_qry_instrument: spi_on_rsp_qry_instrument,
    on_rsp_qry_depth_market_data: spi_on_rsp_qry_depth_market_data,
    on_rsp_qry_settlement_info: spi_on_rsp_qry_settlement_info,
    on_rsp_qry_transfer_bank: spi_on_rsp_qry_transfer_bank,
    on_rsp_qry_investor_position_detail: spi_on_rsp_qry_investor_position_detail,
    on_rsp_qry_notice: spi_on_rsp_qry_notice,
    on_rsp_qry_settlement_info_confirm: spi_on_rsp_qry_settlement_info_confirm,
    on_rsp_qry_investor_position_combine_detail: spi_on_rsp_qry_investor_position_combine_detail,
    on_rsp_qry_cfmmc_trading_account_key: spi_on_rsp_qry_cfmmc_trading_account_key,
    on_rsp_qry_e_warrant_offset: spi_on_rsp_qry_e_warrant_offset,
    on_rsp_qry_investor_product_group_margin: spi_on_rsp_qry_investor_product_group_margin,
    on_rsp_qry_exchange_margin_rate: spi_on_rsp_qry_exchange_margin_rate,
    on_rsp_qry_exchange_margin_rate_adjust: spi_on_rsp_qry_exchange_margin_rate_adjust,
    on_rsp_qry_exchange_rate: spi_on_rsp_qry_exchange_rate,
    on_rsp_qry_sec_agent_acid_map: spi_on_rsp_qry_sec_agent_acid_map,
    on_rsp_qry_product_group: spi_on_rsp_qry_product_group,
    on_rsp_qry_instrument_order_comm_rate: spi_on_rsp_qry_instrument_order_comm_rate,
    on_rsp_qry_option_instr_trade_cost: spi_on_rsp_qry_option_instr_trade_cost,
    on_rsp_qry_option_instr_comm_rate: spi_on_rsp_qry_option_instr_comm_rate,
    on_rsp_qry_exec_order: spi_on_rsp_qry_exec_order,
    on_rsp_qry_for_quote: spi_on_rsp_qry_for_quote,
    on_rsp_qry_quote: spi_on_rsp_qry_quote,
    on_rsp_qry_comb_instrument_guard: spi_on_rsp_qry_comb_instrument_guard,
    on_rsp_qry_comb_action: spi_on_rsp_qry_comb_action,
    on_rsp_qry_transfer_serial: spi_on_rsp_qry_transfer_serial,
    on_rsp_qry_accountregister: spi_on_rsp_qry_accountregister,
    on_rsp_error: spi_on_rsp_error,
    on_rtn_order: spi_on_rtn_order,
    on_rtn_trade: spi_on_rtn_trade,
    on_err_rtn_order_insert: spi_on_err_rtn_order_insert,
    on_err_rtn_order_action: spi_on_err_rtn_order_action,
    on_rtn_instrument_status: spi_on_rtn_instrument_status,
    on_rtn_trading_notice: spi_on_rtn_trading_notice,
    on_rtn_error_conditional_order: spi_on_rtn_error_conditional_order,
    on_rtn_exec_order: spi_on_rtn_exec_order,
    on_err_rtn_exec_order_insert: spi_on_err_rtn_exec_order_insert,
    on_err_rtn_exec_order_action: spi_on_err_rtn_exec_order_action,
    on_err_rtn_for_quote_insert: spi_on_err_rtn_for_quote_insert,
    on_rtn_quote: spi_on_rtn_quote,
    on_err_rtn_quote_insert: spi_on_err_rtn_quote_insert,
    on_err_rtn_quote_action: spi_on_err_rtn_quote_action,
    on_rtn_for_quote_rsp: spi_on_rtn_for_quote_rsp,
    on_rtn_cfmmc_trading_account_token: spi_on_rtn_cfmmc_trading_account_token,
    on_rtn_comb_action: spi_on_rtn_comb_action,
    on_err_rtn_comb_action_insert: spi_on_err_rtn_comb_action_insert,
    on_rsp_qry_contract_bank: spi_on_rsp_qry_contract_bank,
    on_rsp_qry_parked_order: spi_on_rsp_qry_parked_order,
    on_rsp_qry_parked_order_action: spi_on_rsp_qry_parked_order_action,
    on_rsp_qry_trading_notice: spi_on_rsp_qry_trading_notice,
    on_rsp_qry_broker_trading_params: spi_on_rsp_qry_broker_trading_params,
    on_rsp_qry_broker_trading_algos: spi_on_rsp_qry_broker_trading_algos,
    on_rsp_query_cfmmc_trading_account_token: spi_on_rsp_query_cfmmc_trading_account_token,
    on_rtn_from_bank_to_future_by_bank: spi_on_rtn_from_bank_to_future_by_bank,
    on_rtn_from_future_to_bank_by_bank: spi_on_rtn_from_future_to_bank_by_bank,
    on_rtn_repeal_from_bank_to_future_by_bank: spi_on_rtn_repeal_from_bank_to_future_by_bank,
    on_rtn_repeal_from_future_to_bank_by_bank: spi_on_rtn_repeal_from_future_to_bank_by_bank,
    on_rtn_from_bank_to_future_by_future: spi_on_rtn_from_bank_to_future_by_future,
    on_rtn_from_future_to_bank_by_future: spi_on_rtn_from_future_to_bank_by_future,
    on_rtn_repeal_from_bank_to_future_by_future_manual: spi_on_rtn_repeal_from_bank_to_future_by_future_manual,
    on_rtn_repeal_from_future_to_bank_by_future_manual: spi_on_rtn_repeal_from_future_to_bank_by_future_manual,
    on_rtn_query_bank_balance_by_future: spi_on_rtn_query_bank_balance_by_future,
    on_err_rtn_bank_to_future_by_future: spi_on_err_rtn_bank_to_future_by_future,
    on_err_rtn_future_to_bank_by_future: spi_on_err_rtn_future_to_bank_by_future,
    on_err_rtn_repeal_bank_to_future_by_future_manual: spi_on_err_rtn_repeal_bank_to_future_by_future_manual,
    on_err_rtn_repeal_future_to_bank_by_future_manual: spi_on_err_rtn_repeal_future_to_bank_by_future_manual,
    on_err_rtn_query_bank_balance_by_future: spi_on_err_rtn_query_bank_balance_by_future,
    on_rtn_repeal_from_bank_to_future_by_future: spi_on_rtn_repeal_from_bank_to_future_by_future,
    on_rtn_repeal_from_future_to_bank_by_future: spi_on_rtn_repeal_from_future_to_bank_by_future,
    on_rsp_from_bank_to_future_by_future: spi_on_rsp_from_bank_to_future_by_future,
    on_rsp_from_future_to_bank_by_future: spi_on_rsp_from_future_to_bank_by_future,
    on_rsp_query_bank_account_money_by_future: spi_on_rsp_query_bank_account_money_by_future,
    on_rtn_open_account_by_bank: spi_on_rtn_open_account_by_bank,
    on_rtn_cancel_account_by_bank: spi_on_rtn_cancel_account_by_bank,
    on_rtn_change_account_by_bank: spi_on_rtn_change_account_by_bank,
};

#[repr(C)]
pub struct Struct_CThostFtdcTraderSpi {
    vtable: *const SpiVTable,
    pub trader_spi_ptr: *mut TraderSpi
}

fn new_spi(trader_spi: *mut TraderSpi) -> Struct_CThostFtdcTraderSpi {
    Struct_CThostFtdcTraderSpi{ vtable: &SPI_VTABLE, trader_spi_ptr: trader_spi }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    #[test]
    fn create_release() {
        let flow_path = CString::new("").unwrap();
        let trader_api = TraderApi::new(flow_path);
        drop(trader_api);
        assert!(true);
    }

    #[test]
    fn get_trading_day() {
        let flow_path = CString::new("").unwrap();
        let mut trader_api = TraderApi::new(flow_path);
        let trading_day = trader_api.get_trading_day();
        assert_eq!(b"19700101".len(), trading_day.to_bytes().len());
        let year = ::std::str::from_utf8(&trading_day.to_bytes()[0..4]).unwrap().parse::<i32>().unwrap();
        assert!(year > 1970 && year < 2038);
        let month = ::std::str::from_utf8(&trading_day.to_bytes()[4..6]).unwrap().parse::<i32>().unwrap();
        assert!(month > 0 && month < 13);
        let day = ::std::str::from_utf8(&trading_day.to_bytes()[6..8]).unwrap().parse::<i32>().unwrap();
        assert!(day > 0 && day < 32);
    }
}
