use crossbeam_channel as channel;

use std::time::SystemTime;

use super::*;

#[derive(Clone, Debug)]
pub struct ChannelMdSpi {
    sender: channel::Sender<(MdSpiOutput, SystemTime)>,
}

impl ChannelMdSpi {
    pub fn new(sender: channel::Sender<(MdSpiOutput, SystemTime)>) -> Self {
        ChannelMdSpi {
            sender,
        }
    }
}

impl MdSpi for ChannelMdSpi {
    fn on_front_connected(&mut self) {
        self.sender.send((MdSpiOutput::FrontConnected(MdSpiOnFrontConnected{ }), SystemTime::now())).expect("cannot send md spi")
    }

    fn on_front_disconnected(&mut self, reason: DisconnectionReason) {
        self.sender.send((MdSpiOutput::FrontDisconnected(MdSpiOnFrontDisconnected{ reason: reason }), SystemTime::now())).expect("cannot send md spi")
    }

    fn on_rsp_user_login(&mut self, rsp_user_login: Option<&CThostFtdcRspUserLoginField>, result: RspResult, request_id: i32, is_last: bool) {
        self.sender.send((MdSpiOutput::RspUserLogin(MdSpiOnRspUserLogin{ user_login: rsp_user_login.cloned(), result: result, request_id: request_id, is_last: is_last }), SystemTime::now())).expect("cannot send md spi")
    }

    fn on_rsp_user_logout(&mut self, rsp_user_logout: Option<&CThostFtdcUserLogoutField>, result: RspResult, request_id: i32, is_last: bool) {
        self.sender.send((MdSpiOutput::RspUserLogout(MdSpiOnRspUserLogout{ user_logout: rsp_user_logout.cloned(), result: result, request_id: request_id, is_last: is_last }), SystemTime::now())).expect("cannot send md spi")
    }

    fn on_rsp_error(&mut self, result: RspResult, request_id: i32, is_last: bool) {
        self.sender.send((MdSpiOutput::RspError(MdSpiOnRspError{ result: result, request_id: request_id, is_last: is_last }), SystemTime::now())).expect("cannot send md spi")
    }

    fn on_rsp_sub_market_data(&mut self, specific_instrument: Option<&CThostFtdcSpecificInstrumentField>, result: RspResult, request_id: i32, is_last: bool) {
        self.sender.send((MdSpiOutput::SubMarketData(MdSpiOnRspSubMarketData{ specific_instrument: specific_instrument.cloned(), result: result, request_id: request_id, is_last: is_last }), SystemTime::now())).expect("cannot send md spi")
    }

    fn on_rsp_un_sub_market_data(&mut self, specific_instrument: Option<&CThostFtdcSpecificInstrumentField>, result: RspResult, request_id: i32, is_last: bool) {
        self.sender.send((MdSpiOutput::UnSubMarketData(MdSpiOnRspUnSubMarketData{ specific_instrument: specific_instrument.cloned(), result: result, request_id: request_id, is_last: is_last }), SystemTime::now())).expect("cannot send md spi")
    }

    fn on_rsp_sub_for_quote_rsp(&mut self, specific_instrument: Option<&CThostFtdcSpecificInstrumentField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        self.sender.send((MdSpiOutput::SubForQuoteRsp(MdSpiOnRspSubForQuoteRsp{ specific_instrument: specific_instrument.cloned(), result: result, request_id: request_id, is_last: is_last }), SystemTime::now())).expect("cannot send md spi")
    }

    fn on_rsp_un_sub_for_quote_rsp(&mut self, specific_instrument: Option<&CThostFtdcSpecificInstrumentField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        self.sender.send((MdSpiOutput::UnSubForQuoteRsp(MdSpiOnRspUnSubForQuoteRsp{ specific_instrument: specific_instrument.cloned(), result: result, request_id: request_id, is_last: is_last }), SystemTime::now())).expect("cannot send md spi")
    }

    fn on_rtn_depth_market_data(&mut self, depth_market_data: Option<&CThostFtdcDepthMarketDataField>) {
        self.sender.send((MdSpiOutput::DepthMarketData(MdSpiOnRtnDepthMarketData{ depth_market_data: depth_market_data.expect("depth_market_data is none").clone() }), SystemTime::now())).expect("cannot send md spi")
    }

    fn on_rtn_for_quote_rsp(&mut self, for_quote_rsp: Option<&CThostFtdcForQuoteRspField>) {
        self.sender.send((MdSpiOutput::ForQuoteRsp(MdSpiOnRtnForQuoteRsp{ for_quote_rsp: for_quote_rsp.expect("for_quote_rsp is none").clone() }), SystemTime::now())).expect("cannot send md spi")
    }
}
