use crossbeam_channel as channel;

use std::time::SystemTime;

use super::*;

#[derive(Clone, Debug)]
pub struct ChannelTraderSpi {
    sender: channel::Sender<(TraderSpiOutput, SystemTime)>,
}

impl ChannelTraderSpi {
    pub fn new(sender: channel::Sender<(TraderSpiOutput, SystemTime)>) -> Self {
        ChannelTraderSpi {
            sender,
        }
    }
}

impl TraderSpi for ChannelTraderSpi {
    fn on_front_connected(&mut self) {
        self.sender.send((TraderSpiOutput::FrontConnected(TraderSpiOnFrontConnected{ }), SystemTime::now())).expect("cannot send trader spi")
    }

    fn on_front_disconnected(&mut self, reason: DisconnectionReason) {
        self.sender.send((TraderSpiOutput::FrontDisconnected(TraderSpiOnFrontDisconnected{ reason: reason }), SystemTime::now())).expect("cannot send trader spi")
    }

    fn on_rsp_authenticate(&mut self, rsp_authenticate: Option<&CThostFtdcRspAuthenticateField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        self.sender.send((TraderSpiOutput::RspAuthenticate(TraderSpiOnRspAuthenticate{ authenticate: rsp_authenticate.cloned(), result: result, request_id: request_id, is_last: is_last }), SystemTime::now())).expect("cannot send trader spi")
    }

    fn on_rsp_user_login(&mut self, rsp_user_login: Option<&CThostFtdcRspUserLoginField>, result: RspResult, request_id: i32, is_last: bool) {
        self.sender.send((TraderSpiOutput::RspUserLogin(TraderSpiOnRspUserLogin{ user_login: rsp_user_login.cloned(), result: result, request_id: request_id, is_last: is_last }), SystemTime::now())).expect("cannot send trader spi")
    }

    fn on_rsp_user_logout(&mut self, rsp_user_logout: Option<&CThostFtdcUserLogoutField>, result: RspResult, request_id: i32, is_last: bool) {
        self.sender.send((TraderSpiOutput::RspUserLogout(TraderSpiOnRspUserLogout{ user_logout: rsp_user_logout.cloned(), result: result, request_id: request_id, is_last: is_last }), SystemTime::now())).expect("cannot send trader spi")
    }

    fn on_rsp_user_password_update(&mut self, rsp_user_password_update: Option<&CThostFtdcUserPasswordUpdateField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        self.sender.send((TraderSpiOutput::RspUserPasswordUpdate(TraderSpiOnRspUserPasswordUpdate{ user_password_update: rsp_user_password_update.cloned(), result: result, request_id: request_id, is_last: is_last }), SystemTime::now())).expect("cannot send trader spi")
    }

   fn on_rsp_order_insert(&mut self, input_order: Option<&CThostFtdcInputOrderField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        self.sender.send((TraderSpiOutput::RspOrderInsert(TraderSpiOnRspOrderInsert{ input_order: input_order.cloned(), result: result, request_id: request_id, is_last: is_last }), SystemTime::now())).expect("cannot send trader spi")
    }

    fn on_rsp_order_action(&mut self, input_order_action: Option<&CThostFtdcInputOrderActionField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        self.sender.send((TraderSpiOutput::RspOrderAction(TraderSpiOnRspOrderAction{ input_order_action: input_order_action.cloned(), result: result, request_id: request_id, is_last: is_last }), SystemTime::now())).expect("cannot send trader spi")
    }

    fn on_rsp_settlement_info_confirm(&mut self, settlement_info_confirm: Option<&CThostFtdcSettlementInfoConfirmField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        self.sender.send((TraderSpiOutput::RspSettlementInfoConfirm(TraderSpiOnRspSettlementInfoConfirm{ settlement_info_confirm: settlement_info_confirm.cloned(), result: result, request_id: request_id, is_last: is_last }), SystemTime::now())).expect("cannot send trader spi")
    }

    fn on_rsp_qry_order(&mut self, order: Option<&CThostFtdcOrderField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        self.sender.send((TraderSpiOutput::RspQryOrder(TraderSpiOnRspQryOrder{ order: order.cloned(), result: result, request_id: request_id, is_last: is_last }), SystemTime::now())).expect("cannot send trader spi")
    }

    fn on_rsp_qry_trade(&mut self, trade: Option<&CThostFtdcTradeField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        self.sender.send((TraderSpiOutput::RspQryTrade(TraderSpiOnRspQryTrade{ trade: trade.cloned(), result: result, request_id: request_id, is_last: is_last }), SystemTime::now())).expect("cannot send trader spi")
    }

    fn on_rsp_qry_investor_position(&mut self, investor_position: Option<&CThostFtdcInvestorPositionField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        self.sender.send((TraderSpiOutput::RspQryInvestorPosition(TraderSpiOnRspQryInvestorPosition{ investor_position: investor_position.cloned(), result: result, request_id: request_id, is_last: is_last }), SystemTime::now())).expect("cannot send trader spi")
    }

    fn on_rsp_qry_trading_account(&mut self, trading_account: Option<&CThostFtdcTradingAccountField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        self.sender.send((TraderSpiOutput::RspQryTradingAccount(TraderSpiOnRspQryTradingAccount{ trading_account: trading_account.cloned(), result: result, request_id: request_id, is_last: is_last }), SystemTime::now())).expect("cannot send trader spi")
    }

    fn on_rsp_qry_investor(&mut self, investor: Option<&CThostFtdcInvestorField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        self.sender.send((TraderSpiOutput::RspQryInvestor(TraderSpiOnRspQryInvestor{ investor: investor.cloned(), result: result, request_id: request_id, is_last: is_last }), SystemTime::now())).expect("cannot send trader spi")
    }

    fn on_rsp_qry_trading_code(&mut self, trading_code: Option<&CThostFtdcTradingCodeField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        self.sender.send((TraderSpiOutput::RspQryTradingCode(TraderSpiOnRspQryTradingCode{ trading_code: trading_code.cloned(), result: result, request_id: request_id, is_last: is_last }), SystemTime::now())).expect("cannot send trader spi")
    }

    fn on_rsp_qry_instrument_margin_rate(&mut self, instrument_margin_rate: Option<&CThostFtdcInstrumentMarginRateField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        self.sender.send((TraderSpiOutput::RspQryInstrumentMarginRate(TraderSpiOnRspQryInstrumentMarginRate{ instrument_margin_rate: instrument_margin_rate.cloned(), result: result, request_id: request_id, is_last: is_last }), SystemTime::now())).expect("cannot send trader spi")
    }

    fn on_rsp_qry_instrument_commission_rate(&mut self, instrument_commission_rate: Option<&CThostFtdcInstrumentCommissionRateField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        self.sender.send((TraderSpiOutput::RspQryInstrumentCommissionRate(TraderSpiOnRspQryInstrumentCommissionRate{ instrument_commission_rate: instrument_commission_rate.cloned(), result: result, request_id: request_id, is_last: is_last }), SystemTime::now())).expect("cannot send trader spi")
    }

    fn on_rsp_qry_exchange(&mut self, exchange: Option<&CThostFtdcExchangeField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        self.sender.send((TraderSpiOutput::RspQryExchange(TraderSpiOnRspQryExchange{ exchange: exchange.cloned(), result: result, request_id: request_id, is_last: is_last }), SystemTime::now())).expect("cannot send trader spi")
    }

    fn on_rsp_qry_product(&mut self, product: Option<&CThostFtdcProductField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        self.sender.send((TraderSpiOutput::RspQryProduct(TraderSpiOnRspQryProduct{ product: product.cloned(), result: result, request_id: request_id, is_last: is_last }), SystemTime::now())).expect("cannot send trader spi")
    }

    fn on_rsp_qry_instrument(&mut self, instrument: Option<&CThostFtdcInstrumentField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        self.sender.send((TraderSpiOutput::RspQryInstrument(TraderSpiOnRspQryInstrument{ instrument: instrument.cloned(), result: result, request_id: request_id, is_last: is_last }), SystemTime::now())).expect("cannot send trader spi")
    }

    fn on_rsp_qry_settlement_info(&mut self, settlement_info: Option<&CThostFtdcSettlementInfoField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        self.sender.send((TraderSpiOutput::RspQrySettlementInfo(TraderSpiOnRspQrySettlementInfo{ settlement_info: settlement_info.cloned(), result: result, request_id: request_id, is_last: is_last }), SystemTime::now())).expect("cannot send trader spi")
    }

    fn on_rsp_qry_settlement_info_confirm(&mut self, settlement_info_confirm: Option<&CThostFtdcSettlementInfoConfirmField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        self.sender.send((TraderSpiOutput::RspQrySettlementInfoConfirm(TraderSpiOnRspQrySettlementInfoConfirm{ settlement_info_confirm: settlement_info_confirm.cloned(), result: result, request_id: request_id, is_last: is_last }), SystemTime::now())).expect("cannot send trader spi")
    }

    fn on_rsp_qry_exchange_margin_rate(&mut self, exchange_margin_rate: Option<&CThostFtdcExchangeMarginRateField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        self.sender.send((TraderSpiOutput::RspQryExchangeMarginRate(TraderSpiOnRspQryExchangeMarginRate{ exchange_margin_rate: exchange_margin_rate.cloned(), result: result, request_id: request_id, is_last: is_last }), SystemTime::now())).expect("cannot send trader spi")
    }

    fn on_rsp_qry_exchange_margin_rate_adjust(&mut self, exchange_margin_rate_adjust: Option<&CThostFtdcExchangeMarginRateAdjustField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        self.sender.send((TraderSpiOutput::RspQryExchangeMarginRateAdjust(TraderSpiOnRspQryExchangeMarginRateAdjust{ exchange_margin_rate_adjust: exchange_margin_rate_adjust.cloned(), result: result, request_id: request_id, is_last: is_last }), SystemTime::now())).expect("cannot send trader spi")
    }

    fn on_rsp_qry_exchange_rate(&mut self, exchange_rate: Option<&CThostFtdcExchangeRateField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        self.sender.send((TraderSpiOutput::RspQryExchangeRate(TraderSpiOnRspQryExchangeRate{ exchange_rate: exchange_rate.cloned(), result: result, request_id: request_id, is_last: is_last }), SystemTime::now())).expect("cannot send trader spi")
    }

    fn on_rsp_error(&mut self, result: RspResult, request_id: i32, is_last: bool) {
        self.sender.send((TraderSpiOutput::RspError(TraderSpiOnRspError{ result: result, request_id: request_id, is_last: is_last }), SystemTime::now())).expect("cannot send trader spi")
    }

    fn on_rtn_order(&mut self, order: Option<&CThostFtdcOrderField>) {
        self.sender.send((TraderSpiOutput::RtnOrder(TraderSpiOnRtnOrder{ order: order.cloned() }), SystemTime::now())).expect("cannot send trader spi")
    }

    fn on_rtn_trade(&mut self, trade: Option<&CThostFtdcTradeField>) {
        self.sender.send((TraderSpiOutput::RtnTrade(TraderSpiOnRtnTrade{ trade: trade.cloned() }), SystemTime::now())).expect("cannot send trader spi")
    }

    fn on_err_rtn_order_insert(&mut self, input_order: Option<&CThostFtdcInputOrderField>, result: RspResult) {
        self.sender.send((TraderSpiOutput::ErrRtnOrderInsert(TraderSpiOnErrRtnOrderInsert{ input_order: input_order.cloned(), result: result }), SystemTime::now())).expect("cannot send trader spi")
    }

    fn on_err_rtn_order_action(&mut self, order_action: Option<&CThostFtdcOrderActionField>, result: RspResult) {
        self.sender.send((TraderSpiOutput::ErrRtnOrderAction(TraderSpiOnErrRtnOrderAction{ order_action: order_action.cloned(), result: result }), SystemTime::now())).expect("cannot send trader spi")
    }

    fn on_rtn_instrument_status(&mut self, instrument_status: Option<&CThostFtdcInstrumentStatusField>) {
        self.sender.send((TraderSpiOutput::RtnInstrumentStatus(TraderSpiOnRtnInstrumentStatus{ instrument_status: instrument_status.cloned() }), SystemTime::now())).expect("cannot send trader spi")
    }

    fn on_rtn_trading_notice(&mut self, trading_notice_info: Option<&CThostFtdcTradingNoticeInfoField>) {
        self.sender.send((TraderSpiOutput::RtnTradingNotice(TraderSpiOnRtnTradingNotice{ trading_notice_info: trading_notice_info.cloned() }), SystemTime::now())).expect("cannot send trader spi")
    }
}
