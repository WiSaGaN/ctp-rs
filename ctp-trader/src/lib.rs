extern crate ctp_common;

use std::ffi::{ CStr, CString };
use std::mem::transmute;
use std::os::raw::{ c_void, c_char, c_int };
use std::sync::mpsc;

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
    fn _ZN18CFtdcTraderApiImpl20RegisterFensUserInfoEP27CThostFtdcFensUserInfoField(api: *mut c_void, pFensUserInfo: *const CThostFtdcFensUserInfoField);
    fn _ZN18CFtdcTraderApiImpl11RegisterSpiEP19CThostFtdcTraderSpi(api: *mut c_void, pSpi: *mut c_void);
    fn _ZN18CFtdcTraderApiImpl21SubscribePrivateTopicE20THOST_TE_RESUME_TYPE(api: *mut c_void, nResumeType: THOST_TE_RESUME_TYPE);
    fn _ZN18CFtdcTraderApiImpl20SubscribePublicTopicE20THOST_TE_RESUME_TYPE(api: *mut c_void, nResumeType: THOST_TE_RESUME_TYPE);
    fn _ZN18CFtdcTraderApiImpl15ReqAuthenticateEP30CThostFtdcReqAuthenticateFieldi(api: *mut c_void, pReqAuthenticateField: *const CThostFtdcReqAuthenticateField, nRequestID: c_int) -> c_int;
    fn _ZN18CFtdcTraderApiImpl12ReqUserLoginEP27CThostFtdcReqUserLoginFieldi(api: *mut c_void, pReqUserLoginField: *const CThostFtdcReqUserLoginField, nRequestID: c_int) -> c_int;
    fn _ZN18CFtdcTraderApiImpl13ReqUserLogoutEP25CThostFtdcUserLogoutFieldi(api: *mut c_void, pUserLogoutField: *const CThostFtdcUserLogoutField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl21ReqUserPasswordUpdateEP33CThostFtdcUserPasswordUpdateFieldi(api: *mut c_void, pUserPasswordUpdate: *const CThostFtdcUserPasswordUpdateField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl31ReqTradingAccountPasswordUpdateEP43CThostFtdcTradingAccountPasswordUpdateFieldi(api: *mut c_void, pTradingAccountPasswordUpdate: *const CThostFtdcTradingAccountPasswordUpdateField, nRequestID: c_int) -> c_int;
    fn _ZN18CFtdcTraderApiImpl14ReqOrderInsertEP25CThostFtdcInputOrderFieldi(api: *mut c_void, pInputOrder: *const CThostFtdcInputOrderField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl20ReqParkedOrderInsertEP26CThostFtdcParkedOrderFieldi(api: *mut c_void, pParkedOrder: *const CThostFtdcParkedOrderField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl20ReqParkedOrderActionEP32CThostFtdcParkedOrderActionFieldi(api: *mut c_void, pParkedOrderAction: *const CThostFtdcParkedOrderActionField, nRequestID: c_int) -> c_int;
    fn _ZN18CFtdcTraderApiImpl14ReqOrderActionEP31CThostFtdcInputOrderActionFieldi(api: *mut c_void, pInputOrderAction: *const CThostFtdcInputOrderActionField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl22ReqQueryMaxOrderVolumeEP34CThostFtdcQueryMaxOrderVolumeFieldi(api: *mut c_void, pQueryMaxOrderVolume: *const CThostFtdcQueryMaxOrderVolumeField, nRequestID: c_int) -> c_int;
    fn _ZN18CFtdcTraderApiImpl24ReqSettlementInfoConfirmEP36CThostFtdcSettlementInfoConfirmFieldi(api: *mut c_void, pSettlementInfoConfirm: *const CThostFtdcSettlementInfoConfirmField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl20ReqRemoveParkedOrderEP32CThostFtdcRemoveParkedOrderFieldi(api: *mut c_void, pRemoveParkedOrder: *const CThostFtdcRemoveParkedOrderField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl26ReqRemoveParkedOrderActionEP38CThostFtdcRemoveParkedOrderActionFieldi(api: *mut c_void, pRemoveParkedOrderAction: *const CThostFtdcRemoveParkedOrderActionField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl18ReqExecOrderInsertEP29CThostFtdcInputExecOrderFieldi(api: *mut c_void, pInputExecOrder: *const CThostFtdcInputExecOrderField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl18ReqExecOrderActionEP35CThostFtdcInputExecOrderActionFieldi(api: *mut c_void, pInputExecOrderAction: *const CThostFtdcInputExecOrderActionField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl17ReqForQuoteInsertEP28CThostFtdcInputForQuoteFieldi(api: *mut c_void, pInputForQuote: *const CThostFtdcInputForQuoteField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl14ReqQuoteInsertEP25CThostFtdcInputQuoteFieldi(api: *mut c_void, pInputQuote: *const CThostFtdcInputQuoteField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl14ReqQuoteActionEP31CThostFtdcInputQuoteActionFieldi(api: *mut c_void, pInputQuoteAction: *const CThostFtdcInputQuoteActionField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl19ReqCombActionInsertEP30CThostFtdcInputCombActionFieldi(api: *mut c_void, pInputCombAction: *const CThostFtdcInputCombActionField, nRequestID: c_int) -> c_int;
    fn _ZN18CFtdcTraderApiImpl11ReqQryOrderEP23CThostFtdcQryOrderFieldi(api: *mut c_void, pQryOrder: *const CThostFtdcQryOrderField, nRequestID: c_int) -> c_int;
    fn _ZN18CFtdcTraderApiImpl11ReqQryTradeEP23CThostFtdcQryTradeFieldi(api: *mut c_void, pQryTrade: *const CThostFtdcQryTradeField, nRequestID: c_int) -> c_int;
    fn _ZN18CFtdcTraderApiImpl22ReqQryInvestorPositionEP34CThostFtdcQryInvestorPositionFieldi(api: *mut c_void, pQryInvestorPosition: *const CThostFtdcQryInvestorPositionField, nRequestID: c_int) -> c_int;
    fn _ZN18CFtdcTraderApiImpl20ReqQryTradingAccountEP32CThostFtdcQryTradingAccountFieldi(api: *mut c_void, pQryTradingAccount: *const CThostFtdcQryTradingAccountField, nRequestID: c_int) -> c_int;
    fn _ZN18CFtdcTraderApiImpl14ReqQryInvestorEP26CThostFtdcQryInvestorFieldi(api: *mut c_void, pQryInvestor: *const CThostFtdcQryInvestorField, nRequestID: c_int) -> c_int;
    fn _ZN18CFtdcTraderApiImpl17ReqQryTradingCodeEP29CThostFtdcQryTradingCodeFieldi(api: *mut c_void, pQryTradingCode: *const CThostFtdcQryTradingCodeField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl26ReqQryInstrumentMarginRateEP38CThostFtdcQryInstrumentMarginRateFieldi(api: *mut c_void, pQryInstrumentMarginRate: *const CThostFtdcQryInstrumentMarginRateField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl30ReqQryInstrumentCommissionRateEP42CThostFtdcQryInstrumentCommissionRateFieldi(api: *mut c_void, pQryInstrumentCommissionRate: *const CThostFtdcQryInstrumentCommissionRateField, nRequestID: c_int) -> c_int;
    fn _ZN18CFtdcTraderApiImpl14ReqQryExchangeEP26CThostFtdcQryExchangeFieldi(api: *mut c_void, pQryExchange: *const CThostFtdcQryExchangeField, nRequestID: c_int) -> c_int;
    fn _ZN18CFtdcTraderApiImpl13ReqQryProductEP25CThostFtdcQryProductFieldi(api: *mut c_void, pQryProduct: *const CThostFtdcQryProductField, nRequestID: c_int) -> c_int;
    fn _ZN18CFtdcTraderApiImpl16ReqQryInstrumentEP28CThostFtdcQryInstrumentFieldi(api: *mut c_void, pQryInstrument: *const CThostFtdcQryInstrumentField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl21ReqQryDepthMarketDataEP33CThostFtdcQryDepthMarketDataFieldi(api: *mut c_void, pQryDepthMarketData: *const CThostFtdcQryDepthMarketDataField, nRequestID: c_int) -> c_int;
    fn _ZN18CFtdcTraderApiImpl20ReqQrySettlementInfoEP32CThostFtdcQrySettlementInfoFieldi(api: *mut c_void, pQrySettlementInfo: *const CThostFtdcQrySettlementInfoField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl18ReqQryTransferBankEP30CThostFtdcQryTransferBankFieldi(api: *mut c_void, pQryTransferBank: *const CThostFtdcQryTransferBankField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl28ReqQryInvestorPositionDetailEP40CThostFtdcQryInvestorPositionDetailFieldi(api: *mut c_void, pQryInvestorPositionDetail: *const CThostFtdcQryInvestorPositionDetailField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl12ReqQryNoticeEP24CThostFtdcQryNoticeFieldi(api: *mut c_void, pQryNotice: *const CThostFtdcQryNoticeField, nRequestID: c_int) -> c_int;
    fn _ZN18CFtdcTraderApiImpl27ReqQrySettlementInfoConfirmEP39CThostFtdcQrySettlementInfoConfirmFieldi(api: *mut c_void, pQrySettlementInfoConfirm: *const CThostFtdcQrySettlementInfoConfirmField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl35ReqQryInvestorPositionCombineDetailEP47CThostFtdcQryInvestorPositionCombineDetailFieldi(api: *mut c_void, pQryInvestorPositionCombineDetail: *const CThostFtdcQryInvestorPositionCombineDetailField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl28ReqQryCFMMCTradingAccountKeyEP40CThostFtdcQryCFMMCTradingAccountKeyFieldi(api: *mut c_void, pQryCFMMCTradingAccountKey: *const CThostFtdcQryCFMMCTradingAccountKeyField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl20ReqQryEWarrantOffsetEP32CThostFtdcQryEWarrantOffsetFieldi(api: *mut c_void, pQryEWarrantOffset: *const CThostFtdcQryEWarrantOffsetField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl32ReqQryInvestorProductGroupMarginEP44CThostFtdcQryInvestorProductGroupMarginFieldi(api: *mut c_void, pQryInvestorProductGroupMargin: *const CThostFtdcQryInvestorProductGroupMarginField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl24ReqQryExchangeMarginRateEP36CThostFtdcQryExchangeMarginRateFieldi(api: *mut c_void, pQryExchangeMarginRate: *const CThostFtdcQryExchangeMarginRateField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl30ReqQryExchangeMarginRateAdjustEP42CThostFtdcQryExchangeMarginRateAdjustFieldi(api: *mut c_void, pQryExchangeMarginRateAdjust: *const CThostFtdcQryExchangeMarginRateAdjustField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl18ReqQryExchangeRateEP30CThostFtdcQryExchangeRateFieldi(api: *mut c_void, pQryExchangeRate: *const CThostFtdcQryExchangeRateField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl21ReqQrySecAgentACIDMapEP33CThostFtdcQrySecAgentACIDMapFieldi(api: *mut c_void, pQrySecAgentACIDMap: *const CThostFtdcQrySecAgentACIDMapField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl18ReqQryProductGroupEP30CThostFtdcQryProductGroupFieldi(api: *mut c_void, pQryProductGroup: *const CThostFtdcQryProductGroupField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl29ReqQryInstrumentOrderCommRateEP41CThostFtdcQryInstrumentOrderCommRateFieldi(api: *mut c_void, pQryInstrumentOrderCommRate: *const CThostFtdcQryInstrumentOrderCommRateField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl26ReqQryOptionInstrTradeCostEP38CThostFtdcQryOptionInstrTradeCostFieldi(api: *mut c_void, pQryOptionInstrTradeCost: *const CThostFtdcQryOptionInstrTradeCostField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl25ReqQryOptionInstrCommRateEP37CThostFtdcQryOptionInstrCommRateFieldi(api: *mut c_void, pQryOptionInstrCommRate: *const CThostFtdcQryOptionInstrCommRateField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl15ReqQryExecOrderEP27CThostFtdcQryExecOrderFieldi(api: *mut c_void, pQryExecOrder: *const CThostFtdcQryExecOrderField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl14ReqQryForQuoteEP26CThostFtdcQryForQuoteFieldi(api: *mut c_void, pQryForQuote: *const CThostFtdcQryForQuoteField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl11ReqQryQuoteEP23CThostFtdcQryQuoteFieldi(api: *mut c_void, pQryQuote: *const CThostFtdcQryQuoteField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl25ReqQryCombInstrumentGuardEP37CThostFtdcQryCombInstrumentGuardFieldi(api: *mut c_void, pQryCombInstrumentGuard: *const CThostFtdcQryCombInstrumentGuardField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl16ReqQryCombActionEP28CThostFtdcQryCombActionFieldi(api: *mut c_void, pQryCombAction: *const CThostFtdcQryCombActionField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl20ReqQryTransferSerialEP32CThostFtdcQryTransferSerialFieldi(api: *mut c_void, pQryTransferSerial: *const CThostFtdcQryTransferSerialField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl21ReqQryAccountregisterEP33CThostFtdcQryAccountregisterFieldi(api: *mut c_void, pQryAccountregister: *const CThostFtdcQryAccountregisterField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl18ReqQryContractBankEP30CThostFtdcQryContractBankFieldi(api: *mut c_void, pQryContractBank: *const CThostFtdcQryContractBankField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl17ReqQryParkedOrderEP29CThostFtdcQryParkedOrderFieldi(api: *mut c_void, pQryParkedOrder: *const CThostFtdcQryParkedOrderField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl23ReqQryParkedOrderActionEP35CThostFtdcQryParkedOrderActionFieldi(api: *mut c_void, pQryParkedOrderAction: *const CThostFtdcQryParkedOrderActionField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl19ReqQryTradingNoticeEP31CThostFtdcQryTradingNoticeFieldi(api: *mut c_void, pQryTradingNotice: *const CThostFtdcQryTradingNoticeField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl25ReqQryBrokerTradingParamsEP37CThostFtdcQryBrokerTradingParamsFieldi(api: *mut c_void, pQryBrokerTradingParams: *const CThostFtdcQryBrokerTradingParamsField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl24ReqQryBrokerTradingAlgosEP36CThostFtdcQryBrokerTradingAlgosFieldi(api: *mut c_void, pQryBrokerTradingAlgos: *const CThostFtdcQryBrokerTradingAlgosField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl32ReqQueryCFMMCTradingAccountTokenEP44CThostFtdcQueryCFMMCTradingAccountTokenFieldi(api: *mut c_void, pQueryCFMMCTradingAccountToken: *const CThostFtdcQueryCFMMCTradingAccountTokenField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl27ReqFromBankToFutureByFutureEP26CThostFtdcReqTransferFieldi(api: *mut c_void, pReqTransfer: *const CThostFtdcReqTransferField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl27ReqFromFutureToBankByFutureEP26CThostFtdcReqTransferFieldi(api: *mut c_void, pReqTransfer: *const CThostFtdcReqTransferField, nRequestID: c_int) -> c_int;
	fn _ZN18CFtdcTraderApiImpl32ReqQueryBankAccountMoneyByFutureEP30CThostFtdcReqQueryAccountFieldi(api: *mut c_void, pReqQueryAccount: *const CThostFtdcReqQueryAccountField, nRequestID: c_int) -> c_int;
}

pub trait GenericTraderApi {
    fn new(flow_path: CString) -> Self;
    fn init(&mut self);
    fn join(&mut self) -> ApiResult;
    fn get_trading_day<'a>(&mut self) -> &'a CStr;
    fn register_front(&mut self, front_socket_address: CString);
    fn register_name_server(&mut self, name_server: CString);
    fn register_fens_user_info(&mut self, fens_user_info: &CThostFtdcFensUserInfoField);
    fn register_spi(&mut self, trader_spi: Box<TraderSpi>);
    fn subscribe_private_topic(&mut self, resume_type: ResumeType);
    fn subscribe_public_topic(&mut self, resume_type: ResumeType);
    fn req_authenticate(&mut self, req_authenticate: &CThostFtdcReqAuthenticateField, request_id: TThostFtdcRequestIDType) -> ApiResult;
    fn req_user_login(&mut self, req_user_login: &CThostFtdcReqUserLoginField, request_id: TThostFtdcRequestIDType) -> ApiResult;
    fn req_user_logout(&mut self, req_user_logout: &CThostFtdcUserLogoutField, request_id: TThostFtdcRequestIDType) -> ApiResult;
    fn req_order_insert(&mut self, input_order: &CThostFtdcInputOrderField, request_id: TThostFtdcRequestIDType) -> ApiResult;
    fn req_order_action(&mut self, input_order_action: &CThostFtdcInputOrderActionField, request_id: TThostFtdcRequestIDType) -> ApiResult;
    fn req_settlement_info_confirm(&mut self, settlement_info_confirm: &CThostFtdcSettlementInfoConfirmField, request_id: TThostFtdcRequestIDType) -> ApiResult;
    fn req_qry_order(&mut self, qry_order: &CThostFtdcQryOrderField, request_id: TThostFtdcRequestIDType) -> ApiResult;
    fn req_qry_trade(&mut self, qry_trade: &CThostFtdcQryTradeField, request_id: TThostFtdcRequestIDType) -> ApiResult;
    fn req_qry_investor_position(&mut self, qry_investor_position: &CThostFtdcQryInvestorPositionField, request_id: TThostFtdcRequestIDType) -> ApiResult;
    fn req_qry_trading_account(&mut self, qry_trading_account: &CThostFtdcQryTradingAccountField, request_id: TThostFtdcRequestIDType) -> ApiResult;
    fn req_qry_investor(&mut self, qry_investor: &CThostFtdcQryInvestorField, request_id: TThostFtdcRequestIDType) -> ApiResult;
    fn req_qry_trading_code(&mut self, qry_trading_code: &CThostFtdcQryTradingCodeField, request_id: TThostFtdcRequestIDType) -> ApiResult;
    fn req_qry_instrument_margin_rate(&mut self, qry_instrument_margin_rate: &CThostFtdcQryInstrumentMarginRateField, request_id: TThostFtdcRequestIDType) -> ApiResult;
    fn req_qry_instrument_commission_rate(&mut self, qry_instrument_commission_rate: &CThostFtdcQryInstrumentCommissionRateField, request_id: TThostFtdcRequestIDType) -> ApiResult;
    fn req_qry_exchange(&mut self, qry_exchange: &CThostFtdcQryExchangeField, request_id: TThostFtdcRequestIDType) -> ApiResult;
    fn req_qry_product(&mut self, qry_product: &CThostFtdcQryProductField, request_id: TThostFtdcRequestIDType) -> ApiResult;
    fn req_qry_instrument(&mut self, qry_instrument: &CThostFtdcQryInstrumentField, request_id: TThostFtdcRequestIDType) -> ApiResult;
    fn req_qry_settlement_info(&mut self, qry_settlement_info: &CThostFtdcQrySettlementInfoField, request_id: TThostFtdcRequestIDType) -> ApiResult;
    fn req_qry_settlement_info_confirm(&mut self, qry_settlement_info_confirm: &CThostFtdcQrySettlementInfoConfirmField, request_id: TThostFtdcRequestIDType) -> ApiResult;
    fn req_qry_exchange_margin_rate(&mut self, qry_exchange_margin_rate: &CThostFtdcQryExchangeMarginRateField, request_id: TThostFtdcRequestIDType) -> ApiResult;
    fn req_qry_exchange_margin_rate_adjust(&mut self, qry_exchange_margin_rate_adjust: &CThostFtdcQryExchangeMarginRateAdjustField, request_id: TThostFtdcRequestIDType) -> ApiResult;
    fn req_qry_exchange_rate(&mut self, qry_exchange_rate: &CThostFtdcQryExchangeRateField, request_id: TThostFtdcRequestIDType) -> ApiResult;
}

pub struct TraderApi {
    trader_api_ptr: *mut c_void,
    registered_spi: Option<*mut CThostFtdcTraderSpi>,
}

unsafe impl Send for TraderApi {}

impl GenericTraderApi for TraderApi {
    fn new(flow_path: CString) -> Self {
        let flow_path_ptr = flow_path.into_raw();
        let api = unsafe { _ZN19CThostFtdcTraderApi19CreateFtdcTraderApiEPKc(flow_path_ptr) };
        let flow_path = unsafe { CString::from_raw(flow_path_ptr) };
        drop(flow_path);
        TraderApi{ trader_api_ptr: api, registered_spi: None }
    }

    fn init(&mut self) {
        unsafe { _ZN18CFtdcTraderApiImpl4InitEv(self.trader_api_ptr) };
    }

    fn join(&mut self) -> ApiResult {
        from_api_return_to_api_result(unsafe { _ZN18CFtdcTraderApiImpl4JoinEv(self.trader_api_ptr) })
    }

    fn get_trading_day<'a>(&mut self) -> &'a CStr {
        let trading_day_cstr = unsafe { _ZN18CFtdcTraderApiImpl13GetTradingDayEv(self.trader_api_ptr) };
        unsafe { CStr::from_ptr(trading_day_cstr) }
    }

    fn register_front(&mut self, front_socket_address: CString) {
        let front_socket_address_ptr = front_socket_address.into_raw();
        unsafe { _ZN18CFtdcTraderApiImpl13RegisterFrontEPc(self.trader_api_ptr, front_socket_address_ptr) };
        let front_socket_address = unsafe { CString::from_raw(front_socket_address_ptr) };
        drop(front_socket_address);
    }

    fn register_name_server(&mut self, name_server: CString) {
        let name_server_ptr = name_server.into_raw();
        unsafe { _ZN18CFtdcTraderApiImpl18RegisterNameServerEPc(self.trader_api_ptr, name_server_ptr) };
        let name_server = unsafe { CString::from_raw(name_server_ptr) };
        drop(name_server);
    }

    fn register_fens_user_info(&mut self, fens_user_info: &CThostFtdcFensUserInfoField) {
        unsafe { _ZN18CFtdcTraderApiImpl20RegisterFensUserInfoEP27CThostFtdcFensUserInfoField(self.trader_api_ptr, fens_user_info) };
    }

    fn register_spi(&mut self, trader_spi: Box<TraderSpi>) {
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

    fn subscribe_private_topic(&mut self, resume_type: ResumeType) {
        unsafe { _ZN18CFtdcTraderApiImpl21SubscribePrivateTopicE20THOST_TE_RESUME_TYPE(self.trader_api_ptr, resume_type.into()) };
    }

    fn subscribe_public_topic(&mut self, resume_type: ResumeType) {
        unsafe { _ZN18CFtdcTraderApiImpl20SubscribePublicTopicE20THOST_TE_RESUME_TYPE(self.trader_api_ptr, resume_type.into()) };
    }

    fn req_authenticate(&mut self, req_authenticate: &CThostFtdcReqAuthenticateField, request_id: TThostFtdcRequestIDType) -> ApiResult {
        from_api_return_to_api_result(unsafe { _ZN18CFtdcTraderApiImpl15ReqAuthenticateEP30CThostFtdcReqAuthenticateFieldi(self.trader_api_ptr, req_authenticate, request_id) })
    }

    fn req_user_login(&mut self, req_user_login: &CThostFtdcReqUserLoginField, request_id: TThostFtdcRequestIDType) -> ApiResult {
        from_api_return_to_api_result(unsafe { _ZN18CFtdcTraderApiImpl12ReqUserLoginEP27CThostFtdcReqUserLoginFieldi(self.trader_api_ptr, req_user_login, request_id) })
    }

    fn req_user_logout(&mut self, req_user_logout: &CThostFtdcUserLogoutField, request_id: TThostFtdcRequestIDType) -> ApiResult {
        from_api_return_to_api_result(unsafe { _ZN18CFtdcTraderApiImpl13ReqUserLogoutEP25CThostFtdcUserLogoutFieldi(self.trader_api_ptr, req_user_logout, request_id) })
    }

    fn req_order_insert(&mut self, input_order: &CThostFtdcInputOrderField, request_id: TThostFtdcRequestIDType) -> ApiResult {
        from_api_return_to_api_result(unsafe { _ZN18CFtdcTraderApiImpl14ReqOrderInsertEP25CThostFtdcInputOrderFieldi(self.trader_api_ptr, input_order, request_id) })
    }

    fn req_order_action(&mut self, input_order_action: &CThostFtdcInputOrderActionField, request_id: TThostFtdcRequestIDType) -> ApiResult {
        from_api_return_to_api_result(unsafe { _ZN18CFtdcTraderApiImpl14ReqOrderActionEP31CThostFtdcInputOrderActionFieldi(self.trader_api_ptr, input_order_action, request_id) })
    }

    fn req_settlement_info_confirm(&mut self, settlement_info_confirm: &CThostFtdcSettlementInfoConfirmField, request_id: TThostFtdcRequestIDType) -> ApiResult {
        from_api_return_to_api_result(unsafe { _ZN18CFtdcTraderApiImpl24ReqSettlementInfoConfirmEP36CThostFtdcSettlementInfoConfirmFieldi(self.trader_api_ptr, settlement_info_confirm, request_id) })
    }

    fn req_qry_order(&mut self, qry_order: &CThostFtdcQryOrderField, request_id: TThostFtdcRequestIDType) -> ApiResult {
        from_api_return_to_api_result(unsafe { _ZN18CFtdcTraderApiImpl11ReqQryOrderEP23CThostFtdcQryOrderFieldi(self.trader_api_ptr, qry_order, request_id) })
    }

    fn req_qry_trade(&mut self, qry_trade: &CThostFtdcQryTradeField, request_id: TThostFtdcRequestIDType) -> ApiResult {
        from_api_return_to_api_result(unsafe { _ZN18CFtdcTraderApiImpl11ReqQryTradeEP23CThostFtdcQryTradeFieldi(self.trader_api_ptr, qry_trade, request_id) })
    }

    fn req_qry_investor_position(&mut self, qry_investor_position: &CThostFtdcQryInvestorPositionField, request_id: TThostFtdcRequestIDType) -> ApiResult {
        from_api_return_to_api_result(unsafe { _ZN18CFtdcTraderApiImpl22ReqQryInvestorPositionEP34CThostFtdcQryInvestorPositionFieldi(self.trader_api_ptr, qry_investor_position, request_id) })
    }

    fn req_qry_trading_account(&mut self, qry_trading_account: &CThostFtdcQryTradingAccountField, request_id: TThostFtdcRequestIDType) -> ApiResult {
        from_api_return_to_api_result(unsafe { _ZN18CFtdcTraderApiImpl20ReqQryTradingAccountEP32CThostFtdcQryTradingAccountFieldi(self.trader_api_ptr, qry_trading_account, request_id) })
    }

    fn req_qry_investor(&mut self, qry_investor: &CThostFtdcQryInvestorField, request_id: TThostFtdcRequestIDType) -> ApiResult {
        from_api_return_to_api_result(unsafe { _ZN18CFtdcTraderApiImpl14ReqQryInvestorEP26CThostFtdcQryInvestorFieldi(self.trader_api_ptr, qry_investor, request_id) })
    }

    fn req_qry_trading_code(&mut self, qry_trading_code: &CThostFtdcQryTradingCodeField, request_id: TThostFtdcRequestIDType) -> ApiResult {
        from_api_return_to_api_result(unsafe { _ZN18CFtdcTraderApiImpl17ReqQryTradingCodeEP29CThostFtdcQryTradingCodeFieldi(self.trader_api_ptr, qry_trading_code, request_id) })
    }

    fn req_qry_instrument_margin_rate(&mut self, qry_instrument_margin_rate: &CThostFtdcQryInstrumentMarginRateField, request_id: TThostFtdcRequestIDType) -> ApiResult {
        from_api_return_to_api_result(unsafe { _ZN18CFtdcTraderApiImpl26ReqQryInstrumentMarginRateEP38CThostFtdcQryInstrumentMarginRateFieldi(self.trader_api_ptr, qry_instrument_margin_rate, request_id) })
    }

    fn req_qry_instrument_commission_rate(&mut self, qry_instrument_commission_rate: &CThostFtdcQryInstrumentCommissionRateField, request_id: TThostFtdcRequestIDType) -> ApiResult {
        from_api_return_to_api_result(unsafe { _ZN18CFtdcTraderApiImpl30ReqQryInstrumentCommissionRateEP42CThostFtdcQryInstrumentCommissionRateFieldi(self.trader_api_ptr, qry_instrument_commission_rate, request_id) })
    }

    fn req_qry_exchange(&mut self, qry_exchange: &CThostFtdcQryExchangeField, request_id: TThostFtdcRequestIDType) -> ApiResult {
        from_api_return_to_api_result(unsafe { _ZN18CFtdcTraderApiImpl14ReqQryExchangeEP26CThostFtdcQryExchangeFieldi(self.trader_api_ptr, qry_exchange, request_id) })
    }

    fn req_qry_product(&mut self, qry_product: &CThostFtdcQryProductField, request_id: TThostFtdcRequestIDType) -> ApiResult {
        from_api_return_to_api_result(unsafe { _ZN18CFtdcTraderApiImpl13ReqQryProductEP25CThostFtdcQryProductFieldi(self.trader_api_ptr, qry_product, request_id) })
    }

    fn req_qry_instrument(&mut self, qry_instrument: &CThostFtdcQryInstrumentField, request_id: TThostFtdcRequestIDType) -> ApiResult {
        from_api_return_to_api_result(unsafe { _ZN18CFtdcTraderApiImpl16ReqQryInstrumentEP28CThostFtdcQryInstrumentFieldi(self.trader_api_ptr, qry_instrument, request_id) })
    }

    fn req_qry_settlement_info(&mut self, qry_settlement_info: &CThostFtdcQrySettlementInfoField, request_id: TThostFtdcRequestIDType) -> ApiResult {
        from_api_return_to_api_result(unsafe { _ZN18CFtdcTraderApiImpl20ReqQrySettlementInfoEP32CThostFtdcQrySettlementInfoFieldi(self.trader_api_ptr, qry_settlement_info, request_id) })
    }

    fn req_qry_settlement_info_confirm(&mut self, qry_settlement_info_confirm: &CThostFtdcQrySettlementInfoConfirmField, request_id: TThostFtdcRequestIDType) -> ApiResult {
        from_api_return_to_api_result(unsafe { _ZN18CFtdcTraderApiImpl27ReqQrySettlementInfoConfirmEP39CThostFtdcQrySettlementInfoConfirmFieldi(self.trader_api_ptr, qry_settlement_info_confirm, request_id) })
    }

    fn req_qry_exchange_margin_rate(&mut self, qry_exchange_margin_rate: &CThostFtdcQryExchangeMarginRateField, request_id: TThostFtdcRequestIDType) -> ApiResult {
        from_api_return_to_api_result(unsafe { _ZN18CFtdcTraderApiImpl24ReqQryExchangeMarginRateEP36CThostFtdcQryExchangeMarginRateFieldi(self.trader_api_ptr, qry_exchange_margin_rate, request_id) })
    }

    fn req_qry_exchange_margin_rate_adjust(&mut self, qry_exchange_margin_rate_adjust: &CThostFtdcQryExchangeMarginRateAdjustField, request_id: TThostFtdcRequestIDType) -> ApiResult {
        from_api_return_to_api_result(unsafe { _ZN18CFtdcTraderApiImpl30ReqQryExchangeMarginRateAdjustEP42CThostFtdcQryExchangeMarginRateAdjustFieldi(self.trader_api_ptr, qry_exchange_margin_rate_adjust, request_id) })
    }

    fn req_qry_exchange_rate(&mut self, qry_exchange_rate: &CThostFtdcQryExchangeRateField, request_id: TThostFtdcRequestIDType) -> ApiResult {
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

pub trait TraderSpi : Send {
    fn on_front_connected(&mut self) {
        println!("on_front_connected");
    }

    fn on_front_disconnected(&mut self, reason: DisconnectionReason) {
        println!("on_front_disconnected: {:?}", reason);
    }

    fn on_rsp_authenticate(&mut self, rsp_authenticate: Option<&CThostFtdcRspAuthenticateField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        println!("on_rsp_authenticate: {:?}, {}, {:?}, {:?}", rsp_authenticate, from_rsp_result_to_string(&result), request_id, is_last);
    }

    fn on_rsp_user_login(&mut self, rsp_user_login: Option<&CThostFtdcRspUserLoginField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        println!("on_rsp_user_login: {:?}, {}, {:?}, {:?}", rsp_user_login, from_rsp_result_to_string(&result), request_id, is_last);
    }

    fn on_rsp_user_logout(&mut self, rsp_user_logout: Option<&CThostFtdcUserLogoutField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        println!("on_rsp_user_logout: {:?}, {}, {:?}, {:?}", rsp_user_logout, from_rsp_result_to_string(&result), request_id, is_last);
    }

    fn on_rsp_order_insert(&mut self, input_order: Option<&CThostFtdcInputOrderField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        println!("on_rsp_order_insert: {:?}, {}, {:?}, {:?}", input_order, from_rsp_result_to_string(&result), request_id, is_last);
    }

    fn on_rsp_order_action(&mut self, input_order_action: Option<&CThostFtdcInputOrderActionField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        println!("on_rsp_order_action: {:?}, {}, {:?}, {:?}", input_order_action, from_rsp_result_to_string(&result), request_id, is_last);
    }

    fn on_rsp_settlement_info_confirm(&mut self, settlement_info_confirm: Option<&CThostFtdcSettlementInfoConfirmField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        println!("on_rsp_settlement_info_confirm: {:?}, {}, {:?}, {:?}", settlement_info_confirm, from_rsp_result_to_string(&result), request_id, is_last);
    }

    fn on_rsp_qry_order(&mut self, order: Option<&CThostFtdcOrderField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        println!("on_rsp_qry_order: {:?}, {}, {:?}, {:?}", order, from_rsp_result_to_string(&result), request_id, is_last);
    }

    fn on_rsp_qry_trade(&mut self, trade: Option<&CThostFtdcTradeField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        println!("on_rsp_qry_trade: {:?}, {}, {:?}, {:?}", trade, from_rsp_result_to_string(&result), request_id, is_last);
    }

    fn on_rsp_qry_investor_position(&mut self, investor_position: Option<&CThostFtdcInvestorPositionField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        println!("on_rsp_qry_investor_position: {:?}, {}, {:?}, {:?}", investor_position, from_rsp_result_to_string(&result), request_id, is_last);
    }

    fn on_rsp_qry_trading_account(&mut self, trading_account: Option<&CThostFtdcTradingAccountField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        println!("on_rsp_qry_trading_account: {:?}, {}, {:?}, {:?}", trading_account, from_rsp_result_to_string(&result), request_id, is_last);
    }

    fn on_rsp_qry_investor(&mut self, investor: Option<&CThostFtdcInvestorField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        println!("on_rsp_qry_investor: {:?}, {}, {:?}, {:?}", investor, from_rsp_result_to_string(&result), request_id, is_last);
    }

    fn on_rsp_qry_trading_code(&mut self, trading_code: Option<&CThostFtdcTradingCodeField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        println!("on_rsp_qry_trading_code: {:?}, {}, {:?}, {:?}", trading_code, from_rsp_result_to_string(&result), request_id, is_last);
    }

	fn on_rsp_qry_instrument_margin_rate(&mut self, instrument_margin_rate: Option<&CThostFtdcInstrumentMarginRateField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        println!("on_rsp_qry_instrument_margin_rate: {:?}, {}, {:?}, {:?}", instrument_margin_rate, from_rsp_result_to_string(&result), request_id, is_last);
    }

	fn on_rsp_qry_instrument_commission_rate(&mut self, instrument_commission_rate: Option<&CThostFtdcInstrumentCommissionRateField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        println!("on_rsp_qry_instrument_commission_rate: {:?}, {}, {:?}, {:?}", instrument_commission_rate, from_rsp_result_to_string(&result), request_id, is_last);
    }

    fn on_rsp_qry_exchange(&mut self, exchange: Option<&CThostFtdcExchangeField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        println!("on_rsp_qry_exchange: {:?}, {}, {:?}, {:?}", exchange, from_rsp_result_to_string(&result), request_id, is_last);
    }

    fn on_rsp_qry_product(&mut self, product: Option<&CThostFtdcProductField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        println!("on_rsp_qry_product: {:?}, {}, {:?}, {:?}", product, from_rsp_result_to_string(&result), request_id, is_last);
    }

    fn on_rsp_qry_instrument(&mut self, instrument: Option<&CThostFtdcInstrumentField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        println!("on_rsp_qry_instrument: {:?}, {}, {:?}, {:?}", instrument, from_rsp_result_to_string(&result), request_id, is_last);
    }

    fn on_rsp_qry_settlement_info(&mut self, settlement_info: Option<&CThostFtdcSettlementInfoField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        println!("on_rsp_qry_settlement_info: {:?}, {}, {:?}, {:?}", settlement_info, from_rsp_result_to_string(&result), request_id, is_last);
    }

    fn on_rsp_qry_settlement_info_confirm(&mut self, settlement_info_confirm: Option<&CThostFtdcSettlementInfoConfirmField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        println!("on_rsp_qry_settlement_info_confirm: {:?}, {}, {:?}, {:?}", settlement_info_confirm, from_rsp_result_to_string(&result), request_id, is_last);
    }

	fn on_rsp_qry_exchange_margin_rate(&mut self, exchange_margin_rate: Option<&CThostFtdcExchangeMarginRateField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        println!("on_rsp_qry_exchange_margin_rate: {:?}, {}, {:?}, {:?}", exchange_margin_rate, from_rsp_result_to_string(&result), request_id, is_last);
    }

	fn on_rsp_qry_exchange_margin_rate_adjust(&mut self, exchange_margin_rate_adjust: Option<&CThostFtdcExchangeMarginRateAdjustField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        println!("on_rsp_qry_exchange_margin_rate_adjust: {:?}, {}, {:?}, {:?}", exchange_margin_rate_adjust, from_rsp_result_to_string(&result), request_id, is_last);
    }

	fn on_rsp_qry_exchange_rate(&mut self, exchange_rate: Option<&CThostFtdcExchangeRateField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        println!("on_rsp_qry_exchange_rate: {:?}, {}, {:?}, {:?}", exchange_rate, from_rsp_result_to_string(&result), request_id, is_last);
    }

    fn on_rsp_error(&mut self, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        println!("on_rsp_error: {}, {:?}, {:?}", from_rsp_result_to_string(&result), request_id, is_last);
    }

    fn on_rtn_order(&mut self, order: Option<&CThostFtdcOrderField>) {
        println!("on_rtn_order: {:?}", order);
    }

    fn on_rtn_trade(&mut self, trade: Option<&CThostFtdcTradeField>) {
        println!("on_rtn_trade: {:?}", trade);
    }

    fn on_err_rtn_order_insert(&mut self, input_order: Option<&CThostFtdcInputOrderField>, result: RspResult) {
        println!("on_err_rtn_order_insert: {:?}, {}", input_order, from_rsp_result_to_string(&result));
    }

    fn on_err_rtn_order_action(&mut self, order_action: Option<&CThostFtdcOrderActionField>, result: RspResult) {
        println!("on_err_rtn_order_action: {:?}, {}", order_action, from_rsp_result_to_string(&result));
    }

    fn on_rtn_instrument_status(&mut self, instrument_status: Option<&CThostFtdcInstrumentStatusField>) {
        println!("on_rtn_instrument_status: {:?}", instrument_status);
    }

    fn on_rtn_trading_notice(&mut self, trading_notice_info: Option<&CThostFtdcTradingNoticeInfoField>) {
        println!("on_rtn_trading_notice: {:?}", trading_notice_info);
    }
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnFrontConnected {
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnFrontDisconnected {
    pub reason: DisconnectionReason,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspAuthenticate {
    pub authenticate: Option<CThostFtdcRspAuthenticateField>,
    pub result: RspResult,
    pub request_id: TThostFtdcRequestIDType,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspUserLogin {
    pub user_login: Option<CThostFtdcRspUserLoginField>,
    pub result: RspResult,
    pub request_id: TThostFtdcRequestIDType,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspUserLogout {
    pub user_logout: Option<CThostFtdcUserLogoutField>,
    pub result: RspResult,
    pub request_id: TThostFtdcRequestIDType,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspError {
    pub result: RspResult,
    pub request_id: TThostFtdcRequestIDType,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryOrder {
    pub order: Option<CThostFtdcOrderField>,
    pub result: RspResult,
    pub request_id: TThostFtdcRequestIDType,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryTrade {
    pub trade: Option<CThostFtdcTradeField>,
    pub result: RspResult,
    pub request_id: TThostFtdcRequestIDType,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryInvestorPosition {
    pub investor_position: Option<CThostFtdcInvestorPositionField>,
    pub result: RspResult,
    pub request_id: TThostFtdcRequestIDType,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryTradingAccount {
    pub trading_account: Option<CThostFtdcTradingAccountField>,
    pub result: RspResult,
    pub request_id: TThostFtdcRequestIDType,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryInvestor {
    pub investor: Option<CThostFtdcInvestorField>,
    pub result: RspResult,
    pub request_id: TThostFtdcRequestIDType,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryTradingCode {
    pub trading_code: Option<CThostFtdcTradingCodeField>,
    pub result: RspResult,
    pub request_id: TThostFtdcRequestIDType,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryInstrumentMarginRate {
    pub instrument_margin_rate: Option<CThostFtdcInstrumentMarginRateField>,
    pub result: RspResult,
    pub request_id: TThostFtdcRequestIDType,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryInstrumentCommissionRate {
    pub instrument_commission_rate: Option<CThostFtdcInstrumentCommissionRateField>,
    pub result: RspResult,
    pub request_id: TThostFtdcRequestIDType,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryExchange {
    pub exchange: Option<CThostFtdcExchangeField>,
    pub result: RspResult,
    pub request_id: TThostFtdcRequestIDType,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryProduct {
    pub product: Option<CThostFtdcProductField>,
    pub result: RspResult,
    pub request_id: TThostFtdcRequestIDType,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryInstrument {
    pub instrument: Option<CThostFtdcInstrumentField>,
    pub result: RspResult,
    pub request_id: TThostFtdcRequestIDType,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryExchangeMarginRate {
    pub exchange_margin_rate: Option<CThostFtdcExchangeMarginRateField>,
    pub result: RspResult,
    pub request_id: TThostFtdcRequestIDType,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryExchangeMarginRateAdjust {
    pub exchange_margin_rate_adjust: Option<CThostFtdcExchangeMarginRateAdjustField>,
    pub result: RspResult,
    pub request_id: TThostFtdcRequestIDType,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryExchangeRate {
    pub exchange_rate: Option<CThostFtdcExchangeRateField>,
    pub result: RspResult,
    pub request_id: TThostFtdcRequestIDType,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspOrderInsert {
    pub input_order: Option<CThostFtdcInputOrderField>,
    pub result: RspResult,
    pub request_id: TThostFtdcRequestIDType,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspOrderAction {
    pub input_order_action: Option<CThostFtdcInputOrderActionField>,
    pub result: RspResult,
    pub request_id: TThostFtdcRequestIDType,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnErrRtnOrderInsert {
    pub input_order: Option<CThostFtdcInputOrderField>,
    pub result: RspResult,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnErrRtnOrderAction {
    pub order_action: Option<CThostFtdcOrderActionField>,
    pub result: RspResult,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRtnOrder {
    pub order: Option<CThostFtdcOrderField>,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRtnTrade {
    pub trade: Option<CThostFtdcTradeField>,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRtnInstrumentStatus {
    pub instrument_status: Option<CThostFtdcInstrumentStatusField>,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRtnTradingNotice {
    pub trading_notice_info: Option<CThostFtdcTradingNoticeInfoField>,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQrySettlementInfo {
    pub settlement_info: Option<CThostFtdcSettlementInfoField>,
    pub result: RspResult,
    pub request_id: TThostFtdcRequestIDType,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQrySettlementInfoConfirm {
    pub settlement_info_confirm: Option<CThostFtdcSettlementInfoConfirmField>,
    pub result: RspResult,
    pub request_id: TThostFtdcRequestIDType,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspSettlementInfoConfirm {
    pub settlement_info_confirm: Option<CThostFtdcSettlementInfoConfirmField>,
    pub result: RspResult,
    pub request_id: TThostFtdcRequestIDType,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub enum TraderSpiOutput {
    FrontConnected(TraderSpiOnFrontConnected),
    FrontDisconnected(TraderSpiOnFrontDisconnected),
    RspAuthenticate(TraderSpiOnRspAuthenticate),
    RspUserLogin(TraderSpiOnRspUserLogin),
    RspUserLogout(TraderSpiOnRspUserLogout),
    RspError(TraderSpiOnRspError),
    RspQryOrder(TraderSpiOnRspQryOrder),
    RspQryTrade(TraderSpiOnRspQryTrade),
    RspQryInvestorPosition(TraderSpiOnRspQryInvestorPosition),
    RspQryTradingAccount(TraderSpiOnRspQryTradingAccount),
    RspQryInvestor(TraderSpiOnRspQryInvestor),
    RspQryTradingCode(TraderSpiOnRspQryTradingCode),
    RspQryInstrumentMarginRate(TraderSpiOnRspQryInstrumentMarginRate),
    RspQryInstrumentCommissionRate(TraderSpiOnRspQryInstrumentCommissionRate),
    RspQryExchange(TraderSpiOnRspQryExchange),
    RspQryProduct(TraderSpiOnRspQryProduct),
    RspQryInstrument(TraderSpiOnRspQryInstrument),
    RspQryExchangeMarginRate(TraderSpiOnRspQryExchangeMarginRate),
    RspQryExchangeMarginRateAdjust(TraderSpiOnRspQryExchangeMarginRateAdjust),
    RspQryExchangeRate(TraderSpiOnRspQryExchangeRate),
    RspQrySettlementInfo(TraderSpiOnRspQrySettlementInfo),
    RspQrySettlementInfoConfirm(TraderSpiOnRspQrySettlementInfoConfirm),
    RspSettlementInfoConfirm(TraderSpiOnRspSettlementInfoConfirm),
    RspOrderInsert(TraderSpiOnRspOrderInsert),
    RspOrderAction(TraderSpiOnRspOrderAction),
    ErrRtnOrderInsert(TraderSpiOnErrRtnOrderInsert),
    ErrRtnOrderAction(TraderSpiOnErrRtnOrderAction),
    RtnOrder(TraderSpiOnRtnOrder),
    RtnTrade(TraderSpiOnRtnTrade),
    RtnInstrumentStatus(TraderSpiOnRtnInstrumentStatus),
    RtnTradingNotice(TraderSpiOnRtnTradingNotice),
}

#[derive(Clone, Debug)]
pub struct SyncSenderTraderSpi<T: From<TraderSpiOutput> + Send + 'static> {
    sender: mpsc::SyncSender<T>,
}

impl<T> SyncSenderTraderSpi<T> where T: From<TraderSpiOutput> + Send + 'static {
    pub fn new(sender: mpsc::SyncSender<T>) -> Self {
        SyncSenderTraderSpi {
            sender: sender,
        }
    }
}

impl<T> TraderSpi for SyncSenderTraderSpi<T> where T: From<TraderSpiOutput> + Send + 'static {
    fn on_front_connected(&mut self) {
        self.sender.send(T::from(TraderSpiOutput::FrontConnected(TraderSpiOnFrontConnected{ }))).expect("spi callback send front_connected failed");
    }

    fn on_front_disconnected(&mut self, reason: DisconnectionReason) {
        self.sender.send(T::from(TraderSpiOutput::FrontDisconnected(TraderSpiOnFrontDisconnected{ reason: reason }))).expect("spi callback send front_disconnected failed");
    }

    fn on_rsp_authenticate(&mut self, rsp_authenticate: Option<&CThostFtdcRspAuthenticateField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        self.sender.send(T::from(TraderSpiOutput::RspAuthenticate(TraderSpiOnRspAuthenticate{ authenticate: rsp_authenticate.cloned(), result: result, request_id: request_id, is_last: is_last }))).expect("spi callback send rsp_user_login failed");
    }

    fn on_rsp_user_login(&mut self, rsp_user_login: Option<&CThostFtdcRspUserLoginField>, result: RspResult, request_id: i32, is_last: bool) {
        self.sender.send(T::from(TraderSpiOutput::RspUserLogin(TraderSpiOnRspUserLogin{ user_login: rsp_user_login.cloned(), result: result, request_id: request_id, is_last: is_last }))).expect("spi callback send rsp_user_login failed");
    }

    fn on_rsp_user_logout(&mut self, rsp_user_logout: Option<&CThostFtdcUserLogoutField>, result: RspResult, request_id: i32, is_last: bool) {
        self.sender.send(T::from(TraderSpiOutput::RspUserLogout(TraderSpiOnRspUserLogout{ user_logout: rsp_user_logout.cloned(), result: result, request_id: request_id, is_last: is_last }))).expect("spi callback send rsp_user_logout failed");
    }

   fn on_rsp_order_insert(&mut self, input_order: Option<&CThostFtdcInputOrderField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        self.sender.send(T::from(TraderSpiOutput::RspOrderInsert(TraderSpiOnRspOrderInsert{ input_order: input_order.cloned(), result: result, request_id: request_id, is_last: is_last }))).expect("spi callback send rsp_order_insert failed");
    }

    fn on_rsp_order_action(&mut self, input_order_action: Option<&CThostFtdcInputOrderActionField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        self.sender.send(T::from(TraderSpiOutput::RspOrderAction(TraderSpiOnRspOrderAction{ input_order_action: input_order_action.cloned(), result: result, request_id: request_id, is_last: is_last }))).expect("spi callback send rsp_order_action failed");
    }

    fn on_rsp_settlement_info_confirm(&mut self, settlement_info_confirm: Option<&CThostFtdcSettlementInfoConfirmField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        self.sender.send(T::from(TraderSpiOutput::RspSettlementInfoConfirm(TraderSpiOnRspSettlementInfoConfirm{ settlement_info_confirm: settlement_info_confirm.cloned(), result: result, request_id: request_id, is_last: is_last }))).expect("spi callback send rsp_settlement_info_confirm failed");
    }

    fn on_rsp_qry_order(&mut self, order: Option<&CThostFtdcOrderField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        self.sender.send(T::from(TraderSpiOutput::RspQryOrder(TraderSpiOnRspQryOrder{ order: order.cloned(), result: result, request_id: request_id, is_last: is_last }))).expect("spi callback send rsp_qry_order failed");
    }

    fn on_rsp_qry_trade(&mut self, trade: Option<&CThostFtdcTradeField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        self.sender.send(T::from(TraderSpiOutput::RspQryTrade(TraderSpiOnRspQryTrade{ trade: trade.cloned(), result: result, request_id: request_id, is_last: is_last }))).expect("spi callback send rsp_qry_trade failed");
    }

    fn on_rsp_qry_investor_position(&mut self, investor_position: Option<&CThostFtdcInvestorPositionField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        self.sender.send(T::from(TraderSpiOutput::RspQryInvestorPosition(TraderSpiOnRspQryInvestorPosition{ investor_position: investor_position.cloned(), result: result, request_id: request_id, is_last: is_last }))).expect("spi callback send rsp_qry_investor_position failed");
    }

    fn on_rsp_qry_trading_account(&mut self, trading_account: Option<&CThostFtdcTradingAccountField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        self.sender.send(T::from(TraderSpiOutput::RspQryTradingAccount(TraderSpiOnRspQryTradingAccount{ trading_account: trading_account.cloned(), result: result, request_id: request_id, is_last: is_last }))).expect("spi callback send rsp_qry_trading_account failed");
    }

    fn on_rsp_qry_investor(&mut self, investor: Option<&CThostFtdcInvestorField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        self.sender.send(T::from(TraderSpiOutput::RspQryInvestor(TraderSpiOnRspQryInvestor{ investor: investor.cloned(), result: result, request_id: request_id, is_last: is_last }))).expect("spi callback send rsp_qry_investor failed");
    }

    fn on_rsp_qry_trading_code(&mut self, trading_code: Option<&CThostFtdcTradingCodeField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        self.sender.send(T::from(TraderSpiOutput::RspQryTradingCode(TraderSpiOnRspQryTradingCode{ trading_code: trading_code.cloned(), result: result, request_id: request_id, is_last: is_last }))).expect("spi callback send rsp_qry_trading_code failed");
    }

	fn on_rsp_qry_instrument_margin_rate(&mut self, instrument_margin_rate: Option<&CThostFtdcInstrumentMarginRateField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        self.sender.send(T::from(TraderSpiOutput::RspQryInstrumentMarginRate(TraderSpiOnRspQryInstrumentMarginRate{ instrument_margin_rate: instrument_margin_rate.cloned(), result: result, request_id: request_id, is_last: is_last }))).expect("spi callback send rsp_qry_instrument_margin_rate failed");
    }

	fn on_rsp_qry_instrument_commission_rate(&mut self, instrument_commission_rate: Option<&CThostFtdcInstrumentCommissionRateField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        self.sender.send(T::from(TraderSpiOutput::RspQryInstrumentCommissionRate(TraderSpiOnRspQryInstrumentCommissionRate{ instrument_commission_rate: instrument_commission_rate.cloned(), result: result, request_id: request_id, is_last: is_last }))).expect("spi callback send rsp_qry_instrument_commission_rate failed");
    }

    fn on_rsp_qry_exchange(&mut self, exchange: Option<&CThostFtdcExchangeField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        self.sender.send(T::from(TraderSpiOutput::RspQryExchange(TraderSpiOnRspQryExchange{ exchange: exchange.cloned(), result: result, request_id: request_id, is_last: is_last }))).expect("spi callback send rsp_qry_exchange failed");
    }

    fn on_rsp_qry_product(&mut self, product: Option<&CThostFtdcProductField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        self.sender.send(T::from(TraderSpiOutput::RspQryProduct(TraderSpiOnRspQryProduct{ product: product.cloned(), result: result, request_id: request_id, is_last: is_last }))).expect("spi callback send rsp_qry_product failed");
    }

    fn on_rsp_qry_instrument(&mut self, instrument: Option<&CThostFtdcInstrumentField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        self.sender.send(T::from(TraderSpiOutput::RspQryInstrument(TraderSpiOnRspQryInstrument{ instrument: instrument.cloned(), result: result, request_id: request_id, is_last: is_last }))).expect("spi callback send rsp_qry_instrument failed");
    }

    fn on_rsp_qry_settlement_info(&mut self, settlement_info: Option<&CThostFtdcSettlementInfoField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        self.sender.send(T::from(TraderSpiOutput::RspQrySettlementInfo(TraderSpiOnRspQrySettlementInfo{ settlement_info: settlement_info.cloned(), result: result, request_id: request_id, is_last: is_last }))).expect("spi callback send rsp_qry_settlement_info failed");
    }

    fn on_rsp_qry_settlement_info_confirm(&mut self, settlement_info_confirm: Option<&CThostFtdcSettlementInfoConfirmField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        self.sender.send(T::from(TraderSpiOutput::RspQrySettlementInfoConfirm(TraderSpiOnRspQrySettlementInfoConfirm{ settlement_info_confirm: settlement_info_confirm.cloned(), result: result, request_id: request_id, is_last: is_last }))).expect("spi callback send rsp_qry_settlement_info_confirm failed");
    }

	fn on_rsp_qry_exchange_margin_rate(&mut self, exchange_margin_rate: Option<&CThostFtdcExchangeMarginRateField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        self.sender.send(T::from(TraderSpiOutput::RspQryExchangeMarginRate(TraderSpiOnRspQryExchangeMarginRate{ exchange_margin_rate: exchange_margin_rate.cloned(), result: result, request_id: request_id, is_last: is_last }))).expect("spi callback send rsp_qry_exchange_margin_rate failed");
    }

	fn on_rsp_qry_exchange_margin_rate_adjust(&mut self, exchange_margin_rate_adjust: Option<&CThostFtdcExchangeMarginRateAdjustField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        self.sender.send(T::from(TraderSpiOutput::RspQryExchangeMarginRateAdjust(TraderSpiOnRspQryExchangeMarginRateAdjust{ exchange_margin_rate_adjust: exchange_margin_rate_adjust.cloned(), result: result, request_id: request_id, is_last: is_last }))).expect("spi callback send rsp_qry_exchange_margin_rate_adjust failed");
    }

	fn on_rsp_qry_exchange_rate(&mut self, exchange_rate: Option<&CThostFtdcExchangeRateField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        self.sender.send(T::from(TraderSpiOutput::RspQryExchangeRate(TraderSpiOnRspQryExchangeRate{ exchange_rate: exchange_rate.cloned(), result: result, request_id: request_id, is_last: is_last }))).expect("spi callback send rsp_qry_exchange_rate failed");
    }

    fn on_rsp_error(&mut self, result: RspResult, request_id: i32, is_last: bool) {
        self.sender.send(T::from(TraderSpiOutput::RspError(TraderSpiOnRspError{ result: result, request_id: request_id, is_last: is_last }))).expect("spi callback send rsp_error failed");
    }

    fn on_rtn_order(&mut self, order: Option<&CThostFtdcOrderField>) {
        self.sender.send(T::from(TraderSpiOutput::RtnOrder(TraderSpiOnRtnOrder{ order: order.cloned() }))).expect("spi callback send rtn_order failed");
    }

    fn on_rtn_trade(&mut self, trade: Option<&CThostFtdcTradeField>) {
        self.sender.send(T::from(TraderSpiOutput::RtnTrade(TraderSpiOnRtnTrade{ trade: trade.cloned() }))).expect("spi callback send rtn_trade failed");
    }

    fn on_err_rtn_order_insert(&mut self, input_order: Option<&CThostFtdcInputOrderField>, result: RspResult) {
        self.sender.send(T::from(TraderSpiOutput::ErrRtnOrderInsert(TraderSpiOnErrRtnOrderInsert{ input_order: input_order.cloned(), result: result }))).expect("spi callback send err_rtn_order_insert failed");
    }

    fn on_err_rtn_order_action(&mut self, order_action: Option<&CThostFtdcOrderActionField>, result: RspResult) {
        self.sender.send(T::from(TraderSpiOutput::ErrRtnOrderAction(TraderSpiOnErrRtnOrderAction{ order_action: order_action.cloned(), result: result }))).expect("spi callback send err_rtn_order_action failed");
    }

    fn on_rtn_instrument_status(&mut self, instrument_status: Option<&CThostFtdcInstrumentStatusField>) {
        self.sender.send(T::from(TraderSpiOutput::RtnInstrumentStatus(TraderSpiOnRtnInstrumentStatus{ instrument_status: instrument_status.cloned() }))).expect("spi callback send rtn_instrument_status failed");
    }

    fn on_rtn_trading_notice(&mut self, trading_notice_info: Option<&CThostFtdcTradingNoticeInfoField>) {
        self.sender.send(T::from(TraderSpiOutput::RtnTradingNotice(TraderSpiOnRtnTradingNotice{ trading_notice_info: trading_notice_info.cloned() }))).expect("spi callback send rtn_trading_notice_info failed");
    }

}

#[allow(non_snake_case)]
extern "C" fn spi_on_front_connected(spi: *mut CThostFtdcTraderSpi) {
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut CThostFtdcTraderSpi, &mut CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_front_connected() };
}

#[allow(non_snake_case)]
extern "C" fn spi_on_front_disconnected(spi: *mut CThostFtdcTraderSpi, nReason: c_int) {
    let reason = std::convert::From::from(nReason);
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut CThostFtdcTraderSpi, &mut CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_front_disconnected(reason) };
}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_heart_beat_warning(spi: *mut CThostFtdcTraderSpi, nTimeLapse: c_int) {
    // CTP API specification shows this will never be called
    unreachable!();
}

#[allow(non_snake_case)]
extern "C" fn spi_on_rsp_authenticate(spi: *mut CThostFtdcTraderSpi, pRspAuthenticateField: *const CThostFtdcRspAuthenticateField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut CThostFtdcTraderSpi, &mut CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_rsp_authenticate(pRspAuthenticateField.as_ref(), rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case)]
extern "C" fn spi_on_rsp_user_login(spi: *mut CThostFtdcTraderSpi, pRspUserLogin: *const CThostFtdcRspUserLoginField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut CThostFtdcTraderSpi, &mut CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_rsp_user_login(pRspUserLogin.as_ref(), rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case)]
extern "C" fn spi_on_rsp_user_logout(spi: *mut CThostFtdcTraderSpi, pUserLogout: *const CThostFtdcUserLogoutField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut CThostFtdcTraderSpi, &mut CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_rsp_user_logout(pUserLogout.as_ref(), rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_user_password_update(spi: *mut CThostFtdcTraderSpi, pUserPasswordUpdate: *const CThostFtdcUserPasswordUpdateField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_trading_account_password_update(spi: *mut CThostFtdcTraderSpi, pTradingAccountPasswordUpdate: *const CThostFtdcTradingAccountPasswordUpdateField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case)]
extern "C" fn spi_on_rsp_order_insert(spi: *mut CThostFtdcTraderSpi, pInputOrder: *const CThostFtdcInputOrderField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut CThostFtdcTraderSpi, &mut CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_rsp_order_insert(pInputOrder.as_ref(), rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_parked_order_insert(spi: *mut CThostFtdcTraderSpi, pParkedOrder: *const CThostFtdcParkedOrderField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_parked_order_action(spi: *mut CThostFtdcTraderSpi, pParkedOrderAction: *const CThostFtdcParkedOrderActionField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case)]
extern "C" fn spi_on_rsp_order_action(spi: *mut CThostFtdcTraderSpi, pInputOrderAction: *const CThostFtdcInputOrderActionField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut CThostFtdcTraderSpi, &mut CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_rsp_order_action(pInputOrderAction.as_ref(), rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_query_max_order_volume(spi: *mut CThostFtdcTraderSpi, pQueryMaxOrderVolume: *const CThostFtdcQueryMaxOrderVolumeField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case)]
extern "C" fn spi_on_rsp_settlement_info_confirm(spi: *mut CThostFtdcTraderSpi, pSettlementInfoConfirm: *const CThostFtdcSettlementInfoConfirmField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut CThostFtdcTraderSpi, &mut CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_rsp_settlement_info_confirm(pSettlementInfoConfirm.as_ref(), rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_remove_parked_order(spi: *mut CThostFtdcTraderSpi, pRemoveParkedOrder: *const CThostFtdcRemoveParkedOrderField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_remove_parked_order_action(spi: *mut CThostFtdcTraderSpi, pRemoveParkedOrderAction: *const CThostFtdcRemoveParkedOrderActionField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_exec_order_insert(spi: *mut CThostFtdcTraderSpi, pInputExecOrder: *const CThostFtdcInputExecOrderField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_exec_order_action(spi: *mut CThostFtdcTraderSpi, pInputExecOrderAction: *const CThostFtdcInputExecOrderActionField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_for_quote_insert(spi: *mut CThostFtdcTraderSpi, pInputForQuote: *const CThostFtdcInputForQuoteField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_quote_insert(spi: *mut CThostFtdcTraderSpi, pInputQuote: *const CThostFtdcInputQuoteField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_quote_action(spi: *mut CThostFtdcTraderSpi, pInputQuoteAction: *const CThostFtdcInputQuoteActionField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_comb_action_insert(spi: *mut CThostFtdcTraderSpi, pInputCombAction: *const CThostFtdcInputCombActionField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case)]
extern "C" fn spi_on_rsp_qry_order(spi: *mut CThostFtdcTraderSpi, pOrder: *const CThostFtdcOrderField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut CThostFtdcTraderSpi, &mut CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_rsp_qry_order(pOrder.as_ref(), rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case)]
extern "C" fn spi_on_rsp_qry_trade(spi: *mut CThostFtdcTraderSpi, pTrade: *const CThostFtdcTradeField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut CThostFtdcTraderSpi, &mut CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_rsp_qry_trade(pTrade.as_ref(), rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case)]
extern "C" fn spi_on_rsp_qry_investor_position(spi: *mut CThostFtdcTraderSpi, pInvestorPosition: *const CThostFtdcInvestorPositionField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut CThostFtdcTraderSpi, &mut CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_rsp_qry_investor_position(pInvestorPosition.as_ref(), rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case)]
extern "C" fn spi_on_rsp_qry_trading_account(spi: *mut CThostFtdcTraderSpi, pTradingAccount: *const CThostFtdcTradingAccountField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut CThostFtdcTraderSpi, &mut CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_rsp_qry_trading_account(pTradingAccount.as_ref(), rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case)]
extern "C" fn spi_on_rsp_qry_investor(spi: *mut CThostFtdcTraderSpi, pInvestor: *const CThostFtdcInvestorField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut CThostFtdcTraderSpi, &mut CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_rsp_qry_investor(pInvestor.as_ref(), rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case)]
extern "C" fn spi_on_rsp_qry_trading_code(spi: *mut CThostFtdcTraderSpi, pTradingCode: *const CThostFtdcTradingCodeField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut CThostFtdcTraderSpi, &mut CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_rsp_qry_trading_code(pTradingCode.as_ref(), rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_instrument_margin_rate(spi: *mut CThostFtdcTraderSpi, pInstrumentMarginRate: *const CThostFtdcInstrumentMarginRateField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut CThostFtdcTraderSpi, &mut CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_rsp_qry_instrument_margin_rate(pInstrumentMarginRate.as_ref(), rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_instrument_commission_rate(spi: *mut CThostFtdcTraderSpi, pInstrumentCommissionRate: *const CThostFtdcInstrumentCommissionRateField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut CThostFtdcTraderSpi, &mut CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_rsp_qry_instrument_commission_rate(pInstrumentCommissionRate.as_ref(), rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_exchange(spi: *mut CThostFtdcTraderSpi, pExchange: *const CThostFtdcExchangeField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut CThostFtdcTraderSpi, &mut CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_rsp_qry_exchange(pExchange.as_ref(), rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_product(spi: *mut CThostFtdcTraderSpi, pProduct: *const CThostFtdcProductField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut CThostFtdcTraderSpi, &mut CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_rsp_qry_product(pProduct.as_ref(), rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_instrument(spi: *mut CThostFtdcTraderSpi, pInstrument: *const CThostFtdcInstrumentField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut CThostFtdcTraderSpi, &mut CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_rsp_qry_instrument(pInstrument.as_ref(), rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_depth_market_data(spi: *mut CThostFtdcTraderSpi, pDepthMarketData: *const CThostFtdcDepthMarketDataField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_settlement_info(spi: *mut CThostFtdcTraderSpi, pSettlementInfo: *const CThostFtdcSettlementInfoField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut CThostFtdcTraderSpi, &mut CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_rsp_qry_settlement_info(pSettlementInfo.as_ref(), rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_transfer_bank(spi: *mut CThostFtdcTraderSpi, pTransferBank: *const CThostFtdcTransferBankField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_investor_position_detail(spi: *mut CThostFtdcTraderSpi, pInvestorPositionDetail: *const CThostFtdcInvestorPositionDetailField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_notice(spi: *mut CThostFtdcTraderSpi, pNotice: *const CThostFtdcNoticeField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_settlement_info_confirm(spi: *mut CThostFtdcTraderSpi, pSettlementInfoConfirm: *const CThostFtdcSettlementInfoConfirmField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut CThostFtdcTraderSpi, &mut CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_rsp_qry_settlement_info_confirm(pSettlementInfoConfirm.as_ref(), rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_investor_position_combine_detail(spi: *mut CThostFtdcTraderSpi, pInvestorPositionCombineDetail: *const CThostFtdcInvestorPositionCombineDetailField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_cfmmc_trading_account_key(spi: *mut CThostFtdcTraderSpi, pCFMMCTradingAccountKey: *const CThostFtdcCFMMCTradingAccountKeyField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_e_warrant_offset(spi: *mut CThostFtdcTraderSpi, pEWarrantOffset: *const CThostFtdcEWarrantOffsetField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_investor_product_group_margin(spi: *mut CThostFtdcTraderSpi, pInvestorProductGroupMargin: *const CThostFtdcInvestorProductGroupMarginField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_exchange_margin_rate(spi: *mut CThostFtdcTraderSpi, pExchangeMarginRate: *const CThostFtdcExchangeMarginRateField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut CThostFtdcTraderSpi, &mut CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_rsp_qry_exchange_margin_rate(pExchangeMarginRate.as_ref(), rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_exchange_margin_rate_adjust(spi: *mut CThostFtdcTraderSpi, pExchangeMarginRateAdjust: *const CThostFtdcExchangeMarginRateAdjustField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut CThostFtdcTraderSpi, &mut CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_rsp_qry_exchange_margin_rate_adjust(pExchangeMarginRateAdjust.as_ref(), rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_exchange_rate(spi: *mut CThostFtdcTraderSpi, pExchangeRate: *const CThostFtdcExchangeRateField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut CThostFtdcTraderSpi, &mut CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_rsp_qry_exchange_rate(pExchangeRate.as_ref(), rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_sec_agent_acid_map(spi: *mut CThostFtdcTraderSpi, pSecAgentACIDMap: *const CThostFtdcSecAgentACIDMapField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_product_group(spi: *mut CThostFtdcTraderSpi, pProductGroup: *const CThostFtdcProductGroupField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_instrument_order_comm_rate(spi: *mut CThostFtdcTraderSpi, pInstrumentOrderCommRate: *const CThostFtdcInstrumentOrderCommRateField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_option_instr_trade_cost(spi: *mut CThostFtdcTraderSpi, pOptionInstrTradeCost: *const CThostFtdcOptionInstrTradeCostField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_option_instr_comm_rate(spi: *mut CThostFtdcTraderSpi, pOptionInstrCommRate: *const CThostFtdcOptionInstrCommRateField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_exec_order(spi: *mut CThostFtdcTraderSpi, pExecOrder: *const CThostFtdcExecOrderField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_for_quote(spi: *mut CThostFtdcTraderSpi, pForQuote: *const CThostFtdcForQuoteField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_quote(spi: *mut CThostFtdcTraderSpi, pQuote: *const CThostFtdcQuoteField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_comb_instrument_guard(spi: *mut CThostFtdcTraderSpi, pCombInstrumentGuard: *const CThostFtdcCombInstrumentGuardField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_comb_action(spi: *mut CThostFtdcTraderSpi, pCombAction: *const CThostFtdcCombActionField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_transfer_serial(spi: *mut CThostFtdcTraderSpi, pTransferSerial: *const CThostFtdcTransferSerialField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_accountregister(spi: *mut CThostFtdcTraderSpi, pAccountregister: *const CThostFtdcAccountregisterField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case)]
extern "C" fn spi_on_rsp_error(spi: *mut CThostFtdcTraderSpi, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut CThostFtdcTraderSpi, &mut CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_rsp_error(rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case)]
extern "C" fn spi_on_rtn_order(spi: *mut CThostFtdcTraderSpi, pOrder: *const CThostFtdcOrderField) {
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut CThostFtdcTraderSpi, &mut CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_rtn_order(pOrder.as_ref()) };
}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rtn_trade(spi: *mut CThostFtdcTraderSpi, pTrade: *const CThostFtdcTradeField) {
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut CThostFtdcTraderSpi, &mut CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_rtn_trade(pTrade.as_ref()) };
}

#[allow(non_snake_case)]
extern "C" fn spi_on_err_rtn_order_insert(spi: *mut CThostFtdcTraderSpi, pInputOrder: *const CThostFtdcInputOrderField, pRspInfo: *const CThostFtdcRspInfoField) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut CThostFtdcTraderSpi, &mut CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_err_rtn_order_insert(pInputOrder.as_ref(), rsp_info) };
}

#[allow(non_snake_case)]
extern "C" fn spi_on_err_rtn_order_action(spi: *mut CThostFtdcTraderSpi, pOrderAction: *const CThostFtdcOrderActionField, pRspInfo: *const CThostFtdcRspInfoField) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut CThostFtdcTraderSpi, &mut CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_err_rtn_order_action(pOrderAction.as_ref(), rsp_info) };
}

#[allow(non_snake_case)]
extern "C" fn spi_on_rtn_instrument_status(spi: *mut CThostFtdcTraderSpi, pInstrumentStatus: *const CThostFtdcInstrumentStatusField) {
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut CThostFtdcTraderSpi, &mut CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_rtn_instrument_status(pInstrumentStatus.as_ref()) };
}

#[allow(non_snake_case)]
extern "C" fn spi_on_rtn_trading_notice(spi: *mut CThostFtdcTraderSpi, pTradingNoticeInfo: *const CThostFtdcTradingNoticeInfoField) {
    unsafe { transmute::<*mut TraderSpi, &mut TraderSpi>(transmute::<*mut CThostFtdcTraderSpi, &mut CThostFtdcTraderSpi>(spi).trader_spi_ptr).on_rtn_trading_notice(pTradingNoticeInfo.as_ref()) };
}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rtn_error_conditional_order(spi: *mut CThostFtdcTraderSpi, pErrorConditionalOrder: *const CThostFtdcErrorConditionalOrderField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rtn_exec_order(spi: *mut CThostFtdcTraderSpi, pExecOrder: *const CThostFtdcExecOrderField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_err_rtn_exec_order_insert(spi: *mut CThostFtdcTraderSpi, pInputExecOrder: *const CThostFtdcInputExecOrderField, pRspInfo: *const CThostFtdcRspInfoField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_err_rtn_exec_order_action(spi: *mut CThostFtdcTraderSpi, pExecOrderAction: *const CThostFtdcExecOrderActionField, pRspInfo: *const CThostFtdcRspInfoField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_err_rtn_for_quote_insert(spi: *mut CThostFtdcTraderSpi, pInputForQuote: *const CThostFtdcInputForQuoteField, pRspInfo: *const CThostFtdcRspInfoField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rtn_quote(spi: *mut CThostFtdcTraderSpi, pQuote: *const CThostFtdcQuoteField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_err_rtn_quote_insert(spi: *mut CThostFtdcTraderSpi, pInputQuote: *const CThostFtdcInputQuoteField, pRspInfo: *const CThostFtdcRspInfoField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_err_rtn_quote_action(spi: *mut CThostFtdcTraderSpi, pQuoteAction: *const CThostFtdcQuoteActionField, pRspInfo: *const CThostFtdcRspInfoField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rtn_for_quote_rsp(spi: *mut CThostFtdcTraderSpi, pForQuoteRsp: *const CThostFtdcForQuoteRspField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rtn_cfmmc_trading_account_token(spi: *mut CThostFtdcTraderSpi, pCFMMCTradingAccountToken: *const CThostFtdcCFMMCTradingAccountTokenField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rtn_comb_action(spi: *mut CThostFtdcTraderSpi, pCombAction: *const CThostFtdcCombActionField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_err_rtn_comb_action_insert(spi: *mut CThostFtdcTraderSpi, pInputCombAction: *const CThostFtdcInputCombActionField, pRspInfo: *const CThostFtdcRspInfoField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_contract_bank(spi: *mut CThostFtdcTraderSpi, pContractBank: *const CThostFtdcContractBankField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_parked_order(spi: *mut CThostFtdcTraderSpi, pParkedOrder: *const CThostFtdcParkedOrderField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_parked_order_action(spi: *mut CThostFtdcTraderSpi, pParkedOrderAction: *const CThostFtdcParkedOrderActionField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_trading_notice(spi: *mut CThostFtdcTraderSpi, pTradingNotice: *const CThostFtdcTradingNoticeField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_broker_trading_params(spi: *mut CThostFtdcTraderSpi, pBrokerTradingParams: *const CThostFtdcBrokerTradingParamsField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_qry_broker_trading_algos(spi: *mut CThostFtdcTraderSpi, pBrokerTradingAlgos: *const CThostFtdcBrokerTradingAlgosField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_query_cfmmc_trading_account_token(spi: *mut CThostFtdcTraderSpi, pQueryCFMMCTradingAccountToken: *const CThostFtdcQueryCFMMCTradingAccountTokenField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rtn_from_bank_to_future_by_bank(spi: *mut CThostFtdcTraderSpi, pRspTransfer: *const CThostFtdcRspTransferField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rtn_from_future_to_bank_by_bank(spi: *mut CThostFtdcTraderSpi, pRspTransfer: *const CThostFtdcRspTransferField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rtn_repeal_from_bank_to_future_by_bank(spi: *mut CThostFtdcTraderSpi, pRspRepeal: *const CThostFtdcRspRepealField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rtn_repeal_from_future_to_bank_by_bank(spi: *mut CThostFtdcTraderSpi, pRspRepeal: *const CThostFtdcRspRepealField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rtn_from_bank_to_future_by_future(spi: *mut CThostFtdcTraderSpi, pRspTransfer: *const CThostFtdcRspTransferField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rtn_from_future_to_bank_by_future(spi: *mut CThostFtdcTraderSpi, pRspTransfer: *const CThostFtdcRspTransferField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rtn_repeal_from_bank_to_future_by_future_manual(spi: *mut CThostFtdcTraderSpi, pRspRepeal: *const CThostFtdcRspRepealField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rtn_repeal_from_future_to_bank_by_future_manual(spi: *mut CThostFtdcTraderSpi, pRspRepeal: *const CThostFtdcRspRepealField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rtn_query_bank_balance_by_future(spi: *mut CThostFtdcTraderSpi, pNotifyQueryAccount: *const CThostFtdcNotifyQueryAccountField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_err_rtn_bank_to_future_by_future(spi: *mut CThostFtdcTraderSpi, pReqTransfer: *const CThostFtdcReqTransferField, pRspInfo: *const CThostFtdcRspInfoField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_err_rtn_future_to_bank_by_future(spi: *mut CThostFtdcTraderSpi, pReqTransfer: *const CThostFtdcReqTransferField, pRspInfo: *const CThostFtdcRspInfoField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_err_rtn_repeal_bank_to_future_by_future_manual(spi: *mut CThostFtdcTraderSpi, pReqRepeal: *const CThostFtdcReqRepealField, pRspInfo: *const CThostFtdcRspInfoField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_err_rtn_repeal_future_to_bank_by_future_manual(spi: *mut CThostFtdcTraderSpi, pReqRepeal: *const CThostFtdcReqRepealField, pRspInfo: *const CThostFtdcRspInfoField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_err_rtn_query_bank_balance_by_future(spi: *mut CThostFtdcTraderSpi, pReqQueryAccount: *const CThostFtdcReqQueryAccountField, pRspInfo: *const CThostFtdcRspInfoField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rtn_repeal_from_bank_to_future_by_future(spi: *mut CThostFtdcTraderSpi, pRspRepeal: *const CThostFtdcRspRepealField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rtn_repeal_from_future_to_bank_by_future(spi: *mut CThostFtdcTraderSpi, pRspRepeal: *const CThostFtdcRspRepealField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_from_bank_to_future_by_future(spi: *mut CThostFtdcTraderSpi, pReqTransfer: *const CThostFtdcReqTransferField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_from_future_to_bank_by_future(spi: *mut CThostFtdcTraderSpi, pReqTransfer: *const CThostFtdcReqTransferField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rsp_query_bank_account_money_by_future(spi: *mut CThostFtdcTraderSpi, pReqQueryAccount: *const CThostFtdcReqQueryAccountField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rtn_open_account_by_bank(spi: *mut CThostFtdcTraderSpi, pOpenAccount: *const CThostFtdcOpenAccountField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rtn_cancel_account_by_bank(spi: *mut CThostFtdcTraderSpi, pCancelAccount: *const CThostFtdcCancelAccountField) {}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_rtn_change_account_by_bank(spi: *mut CThostFtdcTraderSpi, pChangeAccount: *const CThostFtdcChangeAccountField) {}

#[repr(C)]
#[derive(Debug)]
struct SpiVTable {
    #[allow(non_snake_case)]
    on_front_connected: extern "C" fn(spi: *mut CThostFtdcTraderSpi),
    #[allow(non_snake_case)]
    on_front_disconnected: extern "C" fn(spi: *mut CThostFtdcTraderSpi, nReason: c_int),
    #[allow(non_snake_case)]
    on_heart_beat_warning: extern "C" fn(spi: *mut CThostFtdcTraderSpi, nTimeLapse: c_int),
    #[allow(non_snake_case)]
    on_rsp_authenticate: extern "C" fn (spi: *mut CThostFtdcTraderSpi, pRspAuthenticateField: *const CThostFtdcRspAuthenticateField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_user_login: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pRspUserLogin: *const CThostFtdcRspUserLoginField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_user_logout: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pUserLogout: *const CThostFtdcUserLogoutField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_user_password_update: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pUserPasswordUpdate: *const CThostFtdcUserPasswordUpdateField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_trading_account_password_update: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pTradingAccountPasswordUpdate: *const CThostFtdcTradingAccountPasswordUpdateField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_order_insert: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pInputOrder: *const CThostFtdcInputOrderField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_parked_order_insert: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pParkedOrder: *const CThostFtdcParkedOrderField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_parked_order_action: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pParkedOrderAction: *const CThostFtdcParkedOrderActionField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_order_action: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pInputOrderAction: *const CThostFtdcInputOrderActionField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_query_max_order_volume: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pQueryMaxOrderVolume: *const CThostFtdcQueryMaxOrderVolumeField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_settlement_info_confirm: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pSettlementInfoConfirm: *const CThostFtdcSettlementInfoConfirmField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_remove_parked_order: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pRemoveParkedOrder: *const CThostFtdcRemoveParkedOrderField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_remove_parked_order_action: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pRemoveParkedOrderAction: *const CThostFtdcRemoveParkedOrderActionField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_exec_order_insert: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pInputExecOrder: *const CThostFtdcInputExecOrderField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_exec_order_action: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pInputExecOrderAction: *const CThostFtdcInputExecOrderActionField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_for_quote_insert: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pInputForQuote: *const CThostFtdcInputForQuoteField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_quote_insert: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pInputQuote: *const CThostFtdcInputQuoteField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_quote_action: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pInputQuoteAction: *const CThostFtdcInputQuoteActionField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_comb_action_insert: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pInputCombAction: *const CThostFtdcInputCombActionField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_order: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pOrder: *const CThostFtdcOrderField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_trade: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pTrade: *const CThostFtdcTradeField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_investor_position: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pInvestorPosition: *const CThostFtdcInvestorPositionField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_trading_account: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pTradingAccount: *const CThostFtdcTradingAccountField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_investor: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pInvestor: *const CThostFtdcInvestorField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_trading_code: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pTradingCode: *const CThostFtdcTradingCodeField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_instrument_margin_rate: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pInstrumentMarginRate: *const CThostFtdcInstrumentMarginRateField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_instrument_commission_rate: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pInstrumentCommissionRate: *const CThostFtdcInstrumentCommissionRateField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_exchange: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pExchange: *const CThostFtdcExchangeField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_product: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pProduct: *const CThostFtdcProductField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_instrument: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pInstrument: *const CThostFtdcInstrumentField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_depth_market_data: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pDepthMarketData: *const CThostFtdcDepthMarketDataField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_settlement_info: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pSettlementInfo: *const CThostFtdcSettlementInfoField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_transfer_bank: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pTransferBank: *const CThostFtdcTransferBankField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_investor_position_detail: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pInvestorPositionDetail: *const CThostFtdcInvestorPositionDetailField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_notice: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pNotice: *const CThostFtdcNoticeField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_settlement_info_confirm: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pSettlementInfoConfirm: *const CThostFtdcSettlementInfoConfirmField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_investor_position_combine_detail: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pInvestorPositionCombineDetail: *const CThostFtdcInvestorPositionCombineDetailField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_cfmmc_trading_account_key: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pCFMMCTradingAccountKey: *const CThostFtdcCFMMCTradingAccountKeyField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_e_warrant_offset: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pEWarrantOffset: *const CThostFtdcEWarrantOffsetField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_investor_product_group_margin: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pInvestorProductGroupMargin: *const CThostFtdcInvestorProductGroupMarginField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_exchange_margin_rate: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pExchangeMarginRate: *const CThostFtdcExchangeMarginRateField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_exchange_margin_rate_adjust: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pExchangeMarginRateAdjust: *const CThostFtdcExchangeMarginRateAdjustField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_exchange_rate: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pExchangeRate: *const CThostFtdcExchangeRateField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_sec_agent_acid_map: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pSecAgentACIDMap: *const CThostFtdcSecAgentACIDMapField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_product_group: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pProductGroup: *const CThostFtdcProductGroupField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_instrument_order_comm_rate: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pInstrumentOrderCommRate: *const CThostFtdcInstrumentOrderCommRateField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_option_instr_trade_cost: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pOptionInstrTradeCost: *const CThostFtdcOptionInstrTradeCostField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_option_instr_comm_rate: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pOptionInstrCommRate: *const CThostFtdcOptionInstrCommRateField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_exec_order: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pExecOrder: *const CThostFtdcExecOrderField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_for_quote: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pForQuote: *const CThostFtdcForQuoteField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_quote: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pQuote: *const CThostFtdcQuoteField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_comb_instrument_guard: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pCombInstrumentGuard: *const CThostFtdcCombInstrumentGuardField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_comb_action: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pCombAction: *const CThostFtdcCombActionField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_transfer_serial: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pTransferSerial: *const CThostFtdcTransferSerialField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_accountregister: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pAccountregister: *const CThostFtdcAccountregisterField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_error: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rtn_order: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pOrder: *const CThostFtdcOrderField),
    #[allow(non_snake_case)]
    on_rtn_trade: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pTrade: *const CThostFtdcTradeField),
    #[allow(non_snake_case)]
    on_err_rtn_order_insert: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pInputOrder: *const CThostFtdcInputOrderField, pRspInfo: *const CThostFtdcRspInfoField),
    #[allow(non_snake_case)]
    on_err_rtn_order_action: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pOrderAction: *const CThostFtdcOrderActionField, pRspInfo: *const CThostFtdcRspInfoField),
    #[allow(non_snake_case)]
    on_rtn_instrument_status: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pInstrumentStatus: *const CThostFtdcInstrumentStatusField),
    #[allow(non_snake_case)]
    on_rtn_trading_notice: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pTradingNoticeInfo: *const CThostFtdcTradingNoticeInfoField),
    #[allow(non_snake_case)]
    on_rtn_error_conditional_order: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pErrorConditionalOrder: *const CThostFtdcErrorConditionalOrderField),
    #[allow(non_snake_case)]
    on_rtn_exec_order: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pExecOrder: *const CThostFtdcExecOrderField),
    #[allow(non_snake_case)]
    on_err_rtn_exec_order_insert: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pInputExecOrder: *const CThostFtdcInputExecOrderField, pRspInfo: *const CThostFtdcRspInfoField),
    #[allow(non_snake_case)]
    on_err_rtn_exec_order_action: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pExecOrderAction: *const CThostFtdcExecOrderActionField, pRspInfo: *const CThostFtdcRspInfoField),
    #[allow(non_snake_case)]
    on_err_rtn_for_quote_insert: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pInputForQuote: *const CThostFtdcInputForQuoteField, pRspInfo: *const CThostFtdcRspInfoField),
    #[allow(non_snake_case)]
    on_rtn_quote: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pQuote: *const CThostFtdcQuoteField),
    #[allow(non_snake_case)]
    on_err_rtn_quote_insert: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pInputQuote: *const CThostFtdcInputQuoteField, pRspInfo: *const CThostFtdcRspInfoField),
    #[allow(non_snake_case)]
    on_err_rtn_quote_action: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pQuoteAction: *const CThostFtdcQuoteActionField, pRspInfo: *const CThostFtdcRspInfoField),
    #[allow(non_snake_case)]
    on_rtn_for_quote_rsp: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pForQuoteRsp: *const CThostFtdcForQuoteRspField),
    #[allow(non_snake_case)]
    on_rtn_cfmmc_trading_account_token: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pCFMMCTradingAccountToken: *const CThostFtdcCFMMCTradingAccountTokenField),
    #[allow(non_snake_case)]
    on_rtn_comb_action: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pCombAction: *const CThostFtdcCombActionField),
    #[allow(non_snake_case)]
    on_err_rtn_comb_action_insert: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pInputCombAction: *const CThostFtdcInputCombActionField, pRspInfo: *const CThostFtdcRspInfoField),
    #[allow(non_snake_case)]
    on_rsp_qry_contract_bank: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pContractBank: *const CThostFtdcContractBankField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_parked_order: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pParkedOrder: *const CThostFtdcParkedOrderField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_parked_order_action: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pParkedOrderAction: *const CThostFtdcParkedOrderActionField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_trading_notice: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pTradingNotice: *const CThostFtdcTradingNoticeField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_broker_trading_params: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pBrokerTradingParams: *const CThostFtdcBrokerTradingParamsField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_qry_broker_trading_algos: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pBrokerTradingAlgos: *const CThostFtdcBrokerTradingAlgosField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_query_cfmmc_trading_account_token: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pQueryCFMMCTradingAccountToken: *const CThostFtdcQueryCFMMCTradingAccountTokenField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rtn_from_bank_to_future_by_bank: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pRspTransfer: *const CThostFtdcRspTransferField),
    #[allow(non_snake_case)]
    on_rtn_from_future_to_bank_by_bank: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pRspTransfer: *const CThostFtdcRspTransferField),
    #[allow(non_snake_case)]
    on_rtn_repeal_from_bank_to_future_by_bank: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pRspRepeal: *const CThostFtdcRspRepealField),
    #[allow(non_snake_case)]
    on_rtn_repeal_from_future_to_bank_by_bank: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pRspRepeal: *const CThostFtdcRspRepealField),
    #[allow(non_snake_case)]
    on_rtn_from_bank_to_future_by_future: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pRspTransfer: *const CThostFtdcRspTransferField),
    #[allow(non_snake_case)]
    on_rtn_from_future_to_bank_by_future: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pRspTransfer: *const CThostFtdcRspTransferField),
    #[allow(non_snake_case)]
    on_rtn_repeal_from_bank_to_future_by_future_manual: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pRspRepeal: *const CThostFtdcRspRepealField),
    #[allow(non_snake_case)]
    on_rtn_repeal_from_future_to_bank_by_future_manual: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pRspRepeal: *const CThostFtdcRspRepealField),
    #[allow(non_snake_case)]
    on_rtn_query_bank_balance_by_future: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pNotifyQueryAccount: *const CThostFtdcNotifyQueryAccountField),
    #[allow(non_snake_case)]
    on_err_rtn_bank_to_future_by_future: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pReqTransfer: *const CThostFtdcReqTransferField, pRspInfo: *const CThostFtdcRspInfoField),
    #[allow(non_snake_case)]
    on_err_rtn_future_to_bank_by_future: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pReqTransfer: *const CThostFtdcReqTransferField, pRspInfo: *const CThostFtdcRspInfoField),
    #[allow(non_snake_case)]
    on_err_rtn_repeal_bank_to_future_by_future_manual: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pReqRepeal: *const CThostFtdcReqRepealField, pRspInfo: *const CThostFtdcRspInfoField),
    #[allow(non_snake_case)]
    on_err_rtn_repeal_future_to_bank_by_future_manual: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pReqRepeal: *const CThostFtdcReqRepealField, pRspInfo: *const CThostFtdcRspInfoField),
    #[allow(non_snake_case)]
    on_err_rtn_query_bank_balance_by_future: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pReqQueryAccount: *const CThostFtdcReqQueryAccountField, pRspInfo: *const CThostFtdcRspInfoField),
    #[allow(non_snake_case)]
    on_rtn_repeal_from_bank_to_future_by_future: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pRspRepeal: *const CThostFtdcRspRepealField),
    #[allow(non_snake_case)]
    on_rtn_repeal_from_future_to_bank_by_future: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pRspRepeal: *const CThostFtdcRspRepealField),
    #[allow(non_snake_case)]
    on_rsp_from_bank_to_future_by_future: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pReqTransfer: *const CThostFtdcReqTransferField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_from_future_to_bank_by_future: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pReqTransfer: *const CThostFtdcReqTransferField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_query_bank_account_money_by_future: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pReqQueryAccount: *const CThostFtdcReqQueryAccountField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rtn_open_account_by_bank: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pOpenAccount: *const CThostFtdcOpenAccountField),
    #[allow(non_snake_case)]
    on_rtn_cancel_account_by_bank: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pCancelAccount: *const CThostFtdcCancelAccountField),
    #[allow(non_snake_case)]
    on_rtn_change_account_by_bank: extern "C" fn(spi: *mut CThostFtdcTraderSpi, pChangeAccount: *const CThostFtdcChangeAccountField),
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
pub struct CThostFtdcTraderSpi {
    vtable: *const SpiVTable,
    pub trader_spi_ptr: *mut TraderSpi
}

fn new_spi(trader_spi: *mut TraderSpi) -> CThostFtdcTraderSpi {
    CThostFtdcTraderSpi{ vtable: &SPI_VTABLE, trader_spi_ptr: trader_spi }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    use std::mem::size_of;

    #[test]
    fn spi_output_size() {
        let expected_size = 632;
        let actual_size = size_of::<TraderSpiOutput>();
        assert_eq!(expected_size, actual_size, "TraderSpiOutput expected size {}, actual size {}", expected_size, actual_size);
    }

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
