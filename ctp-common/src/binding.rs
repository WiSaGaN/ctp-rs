#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/struct.rs.in"));
include!(concat!(env!("OUT_DIR"), "/data_type.rs.in"));

use std::fmt;
use super::gb18030_cstr_to_str;

impl fmt::Debug for Struct_CThostFtdcRspAuthenticateField {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Struct_CThostFtdcRspAuthenticateField")
            .field("BrokerID", &gb18030_cstr_to_str(&self.BrokerID))
            .field("UserID", &gb18030_cstr_to_str(&self.UserID))
            .field("UserProductInfo", &gb18030_cstr_to_str(&self.UserProductInfo))
            .finish()
    }
}

impl fmt::Debug for Struct_CThostFtdcRspUserLoginField {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Struct_CThostFtdcRspUserLoginField")
            .field("TradingDay", &gb18030_cstr_to_str(&self.TradingDay))
            .field("LoginTime", &gb18030_cstr_to_str(&self.LoginTime))
            .field("BrokerID", &gb18030_cstr_to_str(&self.BrokerID))
            .field("UserID", &gb18030_cstr_to_str(&self.UserID))
            .field("SystemName", &gb18030_cstr_to_str(&self.SystemName))
            .field("FrontID", &self.FrontID)
            .field("SessionID", &self.SessionID)
            .field("MaxOrderRef", &gb18030_cstr_to_str(&self.MaxOrderRef))
            .field("SHFETime", &gb18030_cstr_to_str(&self.SHFETime))
            .field("DCETime", &gb18030_cstr_to_str(&self.DCETime))
            .field("CZCETime", &gb18030_cstr_to_str(&self.CZCETime))
            .field("FFEXTime", &gb18030_cstr_to_str(&self.FFEXTime))
            .field("INETime", &gb18030_cstr_to_str(&self.INETime))
            .finish()
    }
}

impl fmt::Debug for Struct_CThostFtdcUserLogoutField {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Struct_CThostFtdcUserLogoutField")
            .field("BrokerID", &gb18030_cstr_to_str(&self.BrokerID))
            .field("UserID", &gb18030_cstr_to_str(&self.UserID))
            .finish()
    }
}

impl fmt::Debug for Struct_CThostFtdcSpecificInstrumentField {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Struct_CThostFtdcSpecificInstrumentField")
            .field("InstrumentID", &gb18030_cstr_to_str(&self.InstrumentID))
            .finish()
    }
}

impl fmt::Debug for Struct_CThostFtdcInstrumentField {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Struct_CThostFtdcInstrumentField")
            .field("InstrumentID", &gb18030_cstr_to_str(&self.InstrumentID))
            .field("ExchangeID", &gb18030_cstr_to_str(&self.ExchangeID))
            .field("InstrumentName", &gb18030_cstr_to_str(&self.InstrumentName))
            .field("ExchangeInstID", &gb18030_cstr_to_str(&self.ExchangeInstID))
            .field("ProductID", &gb18030_cstr_to_str(&self.ProductID))
            .field("ProductClass", &self.ProductClass)
            .field("DeliveryYear", &self.DeliveryYear)
            .field("DeliveryMonth", &self.DeliveryMonth)
            .field("MaxMarketOrderVolume", &self.MaxMarketOrderVolume)
            .field("MinMarketOrderVolume", &self.MinMarketOrderVolume)
            .field("MaxLimitOrderVolume", &self.MaxLimitOrderVolume)
            .field("MinLimitOrderVolume", &self.MinLimitOrderVolume)
            .field("VolumeMultiple", &self.VolumeMultiple)
            .field("PriceTick", &self.PriceTick)
            .field("CreateDate", &gb18030_cstr_to_str(&self.CreateDate))
            .field("OpenDate", &gb18030_cstr_to_str(&self.OpenDate))
            .field("ExpireDate", &gb18030_cstr_to_str(&self.ExpireDate))
            .field("StartDelivDate", &gb18030_cstr_to_str(&self.StartDelivDate))
            .field("EndDelivDate", &gb18030_cstr_to_str(&self.EndDelivDate))
            .field("InstLifePhase", &self.InstLifePhase)
            .field("IsTrading", &self.IsTrading)
            .field("PositionType", &self.PositionType)
            .field("PositionDateType", &self.PositionDateType)
            .field("LongMarginRatio", &self.LongMarginRatio)
            .field("ShortMarginRatio", &self.ShortMarginRatio)
            .field("MaxMarginSideAlgorithm", &self.MaxMarginSideAlgorithm)
            .field("UnderlyingInstrID", &gb18030_cstr_to_str(&self.UnderlyingInstrID))
            .field("StrikePrice", &self.StrikePrice)
            .field("OptionsType", &self.OptionsType)
            .field("UnderlyingMultiple", &self.UnderlyingMultiple)
            .field("CombinationType", &self.CombinationType)
            .finish()
    }
}

impl fmt::Debug for Struct_CThostFtdcInstrumentStatusField {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Struct_CThostFtdcInstrumentStatusField ")
            .field("ExchangeID", &gb18030_cstr_to_str(&self.ExchangeID))
            .field("ExchangeInstID", &gb18030_cstr_to_str(&self.ExchangeInstID))
            .field("SettlementGroupID", &gb18030_cstr_to_str(&self.SettlementGroupID))
            .field("InstrumentID", &gb18030_cstr_to_str(&self.InstrumentID))
            .field("InstrumentStatus", &self.InstrumentStatus)
            .field("TradingSegmentSN", &self.TradingSegmentSN)
            .field("EnterTime", &gb18030_cstr_to_str(&self.EnterTime))
            .field("EnterReason", &self.EnterReason)
            .finish()
    }
}

impl fmt::Debug for Struct_CThostFtdcOrderField {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Struct_CThostFtdcOrderField")
            .field("BrokerID", &gb18030_cstr_to_str(&self.BrokerID))
            .field("InvestorID", &gb18030_cstr_to_str(&self.InvestorID))
            .field("InstrumentID", &gb18030_cstr_to_str(&self.InstrumentID))
            .field("OrderRef", &gb18030_cstr_to_str(&self.OrderRef))
            .field("UserID", &gb18030_cstr_to_str(&self.UserID))
            .field("OrderPriceType", &self.OrderPriceType)
            .field("Direction", &self.Direction)
            .field("CombOffsetFlag", &self.CombOffsetFlag)
            .field("CombHedgeFlag", &self.CombHedgeFlag)
            .field("LimitPrice", &self.LimitPrice)
            .field("VolumeTotalOriginal", &self.VolumeTotalOriginal)
            .field("TimeCondition", &self.TimeCondition)
            .field("GTDDate", &gb18030_cstr_to_str(&self.GTDDate))
            .field("VolumeCondition", &self.VolumeCondition)
            .field("MinVolume", &self.MinVolume)
            .field("ContingentCondition", &self.ContingentCondition)
            .field("StopPrice", &self.StopPrice)
            .field("ForceCloseReason", &self.ForceCloseReason)
            .field("IsAutoSuspend", &self.IsAutoSuspend)
            .field("BusinessUnit", &gb18030_cstr_to_str(&self.BusinessUnit))
            .field("RequestID", &self.RequestID)
            .field("OrderLocalID", &gb18030_cstr_to_str(&self.OrderLocalID))
            .field("ExchangeID", &gb18030_cstr_to_str(&self.ExchangeID))
            .field("ParticipantID", &gb18030_cstr_to_str(&self.ParticipantID))
            .field("ClientID", &gb18030_cstr_to_str(&self.ClientID))
            .field("ExchangeInstID", &gb18030_cstr_to_str(&self.ExchangeInstID))
            .field("TraderID", &gb18030_cstr_to_str(&self.TraderID))
            .field("InstallID", &self.InstallID)
            .field("OrderSubmitStatus", &self.OrderSubmitStatus)
            .field("NotifySequence", &self.NotifySequence)
            .field("TradingDay", &gb18030_cstr_to_str(&self.TradingDay))
            .field("SettlementID", &self.SettlementID)
            .field("OrderSysID", &gb18030_cstr_to_str(&self.OrderSysID))
            .field("OrderSource", &self.OrderSource)
            .field("OrderStatus", &self.OrderStatus)
            .field("OrderType", &self.OrderType)
            .field("VolumeTraded", &self.VolumeTraded)
            .field("VolumeTotal", &self.VolumeTotal)
            .field("InsertDate", &gb18030_cstr_to_str(&self.InsertDate))
            .field("InsertTime", &gb18030_cstr_to_str(&self.InsertTime))
            .field("ActiveTime", &gb18030_cstr_to_str(&self.ActiveTime))
            .field("SuspendTime", &gb18030_cstr_to_str(&self.SuspendTime))
            .field("UpdateTime", &gb18030_cstr_to_str(&self.UpdateTime))
            .field("CancelTime", &gb18030_cstr_to_str(&self.CancelTime))
            .field("ActiveTraderID", &self.ActiveTraderID)
            .field("ClearingPartID", &self.ClearingPartID)
            .field("SequenceNo", &self.SequenceNo)
            .field("FrontID", &self.FrontID)
            .field("SessionID", &self.SessionID)
            .field("UserProductInfo", &self.UserProductInfo)
            .field("StatusMsg", &gb18030_cstr_to_str(&self.StatusMsg))
            .field("UserForceClose", &self.UserForceClose)
            .field("ActiveUserID", &self.ActiveUserID)
            .field("BrokerOrderSeq", &self.BrokerOrderSeq)
            .field("RelativeOrderSysID", &self.RelativeOrderSysID)
            .field("ZCETotalTradedVolume", &self.ZCETotalTradedVolume)
            .field("IsSwapOrder", &self.IsSwapOrder)
            .finish()
    }
}

impl fmt::Debug for Struct_CThostFtdcTradeField {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Struct_CThostFtdcTradeField")
            .field("BrokerID", &gb18030_cstr_to_str(&self.BrokerID))
            .field("InvestorID", &gb18030_cstr_to_str(&self.InvestorID))
            .field("InstrumentID", &gb18030_cstr_to_str(&self.InstrumentID))
            .field("OrderRef", &gb18030_cstr_to_str(&self.OrderRef))
            .field("UserID", &gb18030_cstr_to_str(&self.UserID))
            .field("ExchangeID", &gb18030_cstr_to_str(&self.ExchangeID))
            .field("TradeID", &gb18030_cstr_to_str(&self.TradeID))
            .field("Direction", &self.Direction)
            .field("OrderSysID", &gb18030_cstr_to_str(&self.OrderSysID))
            .field("ParticipantID", &gb18030_cstr_to_str(&self.ParticipantID))
            .field("ClientID", &gb18030_cstr_to_str(&self.ClientID))
            .field("TradingRole", &self.TradingRole)
            .field("ExchangeInstID", &gb18030_cstr_to_str(&self.ExchangeInstID))
            .field("OffsetFlag", &self.OffsetFlag)
            .field("HedgeFlag", &self.HedgeFlag)
            .field("Price", &self.Price)
            .field("Volume", &self.Volume)
            .field("TradeDate", &gb18030_cstr_to_str(&self.TradeDate))
            .field("TradeTime", &gb18030_cstr_to_str(&self.TradeTime))
            .field("TradeType", &self.TradeType)
            .field("PriceSource", &self.PriceSource)
            .field("TraderID", &gb18030_cstr_to_str(&self.TraderID))
            .field("OrderLocalID", &gb18030_cstr_to_str(&self.OrderLocalID))
            .field("ClearingPartID", &self.ClearingPartID)
            .field("BusinessUnit", &gb18030_cstr_to_str(&self.BusinessUnit))
            .field("SequenceNo", &self.SequenceNo)
            .field("TradingDay", &self.TradingDay)
            .field("SettlementID", &self.SettlementID)
            .field("BrokerOrderSeq", &self.BrokerOrderSeq)
            .field("TradeSource", &self.TradeSource)
            .finish()
    }
}

impl fmt::Debug for Struct_CThostFtdcSettlementInfoField {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Struct_CThostFtdcSettlementInfoField")
            .field("TradingDay", &gb18030_cstr_to_str(&self.TradingDay))
            .field("SettlementID", &self.SettlementID)
            .field("BrokerID", &gb18030_cstr_to_str(&self.BrokerID))
            .field("InvestorID", &gb18030_cstr_to_str(&self.InvestorID))
            .field("SequenceNo", &self.SequenceNo)
            .field("Content", &gb18030_cstr_to_str(&self.Content))
            .finish()
    }
}

impl fmt::Debug for Struct_CThostFtdcSettlementInfoConfirmField {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Struct_CThostFtdcSettlementInfoConfirmField")
            .field("BrokerID", &gb18030_cstr_to_str(&self.BrokerID))
            .field("InvestorID", &gb18030_cstr_to_str(&self.InvestorID))
            .field("ConfirmDate", &gb18030_cstr_to_str(&self.ConfirmDate))
            .field("ConfirmTime", &gb18030_cstr_to_str(&self.ConfirmTime))
            .finish()
    }
}

impl fmt::Debug for Struct_CThostFtdcInputOrderField {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Struct_CThostFtdcInputOrderField")
            .field("BrokerID", &gb18030_cstr_to_str(&self.BrokerID))
            .field("InvestorID", &gb18030_cstr_to_str(&self.InvestorID))
            .field("InstrumentID", &gb18030_cstr_to_str(&self.InstrumentID))
            .field("OrderRef", &gb18030_cstr_to_str(&self.OrderRef))
            .field("UserID", &gb18030_cstr_to_str(&self.UserID))
            .field("OrderPriceType", &self.OrderPriceType)
            .field("Direction", &self.Direction)
            .field("CombOffsetFlag", &self.CombOffsetFlag)
            .field("CombHedgeFlag", &self.CombHedgeFlag)
            .field("LimitPrice", &self.LimitPrice)
            .field("VolumeTotalOriginal", &self.VolumeTotalOriginal)
            .field("TimeCondition", &self.TimeCondition)
            .field("GTDDate", &gb18030_cstr_to_str(&self.GTDDate))
            .field("VolumeCondition", &self.VolumeCondition)
            .field("MinVolume", &self.MinVolume)
            .field("ContingentCondition", &self.ContingentCondition)
            .field("StopPrice", &self.StopPrice)
            .field("ForceCloseReason", &self.ForceCloseReason)
            .field("IsAutoSuspend", &self.IsAutoSuspend)
            .field("BusinessUnit", &gb18030_cstr_to_str(&self.BusinessUnit))
            .field("RequestID", &self.RequestID)
            .field("UserForceClose", &self.UserForceClose)
            .field("IsSwapOrder", &self.IsSwapOrder)
            .finish()
    }
}

impl fmt::Debug for Struct_CThostFtdcOrderActionField {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Struct_CThostFtdcOrderActionField")
            .field("BrokerID", &gb18030_cstr_to_str(&self.BrokerID))
            .field("InvestorID", &gb18030_cstr_to_str(&self.InvestorID))
            .field("OrderActionRef", &self.OrderActionRef)
            .field("OrderRef", &gb18030_cstr_to_str(&self.OrderRef))
            .field("RequestID", &self.RequestID)
            .field("FrontID", &self.FrontID)
            .field("SessionID", &self.SessionID)
            .field("ExchangeID", &gb18030_cstr_to_str(&self.ExchangeID))
            .field("OrderSysID", &gb18030_cstr_to_str(&self.OrderSysID))
            .field("ActionFlag", &self.ActionFlag)
            .field("LimitPrice", &self.LimitPrice)
            .field("VolumeChange", &self.VolumeChange)
            .field("ActionDate", &gb18030_cstr_to_str(&self.ActionDate))
            .field("ActionTime", &gb18030_cstr_to_str(&self.ActionTime))
            .field("TraderID", &gb18030_cstr_to_str(&self.TraderID))
            .field("InstallID", &self.InstallID)
            .field("OrderLocalID", &gb18030_cstr_to_str(&self.OrderLocalID))
            .field("ActionLocalID", &gb18030_cstr_to_str(&self.ActionLocalID))
            .field("ParticipantID", &gb18030_cstr_to_str(&self.ParticipantID))
            .field("ClientID", &gb18030_cstr_to_str(&self.ClientID))
            .field("BusinessUnit", &gb18030_cstr_to_str(&self.BusinessUnit))
            .field("OrderActionStatus", &self.OrderActionStatus)
            .field("UserID", &gb18030_cstr_to_str(&self.UserID))
            .field("StatusMsg", &gb18030_cstr_to_str(&self.StatusMsg))
            .field("InstrumentID", &gb18030_cstr_to_str(&self.InstrumentID))
            .finish()
    }
}

impl fmt::Debug for Struct_CThostFtdcInputOrderActionField {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Struct_CThostFtdcInputOrderActionField")
            .field("BrokerID", &gb18030_cstr_to_str(&self.BrokerID))
            .field("InvestorID", &gb18030_cstr_to_str(&self.InvestorID))
            .field("OrderActionRef", &self.OrderActionRef)
            .field("OrderRef", &gb18030_cstr_to_str(&self.OrderRef))
            .field("RequestID", &self.RequestID)
            .field("FrontID", &self.FrontID)
            .field("SessionID", &self.SessionID)
            .field("ExchangeID", &gb18030_cstr_to_str(&self.ExchangeID))
            .field("OrderSysID", &gb18030_cstr_to_str(&self.OrderSysID))
            .field("ActionFlag", &self.ActionFlag)
            .field("LimitPrice", &self.LimitPrice)
            .field("VolumeChange", &self.VolumeChange)
            .field("UserID", &gb18030_cstr_to_str(&self.UserID))
            .field("InstrumentID", &gb18030_cstr_to_str(&self.InstrumentID))
            .finish()
    }
}

impl fmt::Debug for Struct_CThostFtdcTradingNoticeInfoField {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Struct_CThostFtdcTradingNoticeInfoField")
            .field("BrokerID", &gb18030_cstr_to_str(&self.BrokerID))
            .field("InvestorID", &gb18030_cstr_to_str(&self.InvestorID))
            .field("SendTime", &gb18030_cstr_to_str(&self.SendTime))
            .field("FieldContent", &gb18030_cstr_to_str(&self.FieldContent))
            .field("SequenceSeries", &self.SequenceSeries)
            .field("SequenceNo", &self.SequenceNo)
            .finish()
    }
}

impl fmt::Debug for Struct_CThostFtdcInvestorField {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Struct_CThostFtdcInvestorField")
            .field("InvestorID", &gb18030_cstr_to_str(&self.InvestorID))
            .field("BrokerID", &gb18030_cstr_to_str(&self.BrokerID))
            .field("InvestorGroupID", &gb18030_cstr_to_str(&self.InvestorGroupID))
            .field("InvestorName", &gb18030_cstr_to_str(&self.InvestorName))
            .field("IdentifiedCardType", &self.IdentifiedCardType)
            .field("IdentifiedCardNo", &gb18030_cstr_to_str(&self.IdentifiedCardNo))
            .field("IsActive", &self.IsActive)
            .field("Telephone", &gb18030_cstr_to_str(&self.Telephone))
            .field("Address", &gb18030_cstr_to_str(&self.Address))
            .field("OpenDate", &gb18030_cstr_to_str(&self.OpenDate))
            .field("Mobile", &gb18030_cstr_to_str(&self.Mobile))
            .field("CommModelID", &gb18030_cstr_to_str(&self.CommModelID))
            .field("MarginModelID", &gb18030_cstr_to_str(&self.MarginModelID))
            .finish()
    }
}

impl fmt::Debug for Struct_CThostFtdcTradingCodeField {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Struct_CThostFtdcTradingCodeField")
            .field("InvestorID", &gb18030_cstr_to_str(&self.InvestorID))
            .field("BrokerID", &gb18030_cstr_to_str(&self.BrokerID))
            .field("ExchangeID", &gb18030_cstr_to_str(&self.ExchangeID))
            .field("ClientID", &gb18030_cstr_to_str(&self.ClientID))
            .field("IsActive", &self.IsActive)
            .field("ClientIDType", &self.ClientIDType)
            .finish()
    }
}


impl fmt::Debug for Struct_CThostFtdcTradingAccountField {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Struct_CThostFtdcTradingAccountField")
            .field("BrokerID", &gb18030_cstr_to_str(&self.BrokerID))
            .field("AccountID", &gb18030_cstr_to_str(&self.AccountID))
            .field("PreMortgage", &self.PreMortgage)
            .field("PreCredit", &self.PreCredit)
            .field("PreDeposit", &self.PreDeposit)
            .field("PreBalance", &self.PreBalance)
            .field("PreMargin", &self.PreMargin)
            .field("InterestBase", &self.InterestBase)
            .field("Interest", &self.Interest)
            .field("Deposit", &self.Deposit)
            .field("Withdraw", &self.Withdraw)
            .field("FrozenMargin", &self.FrozenMargin)
            .field("FrozenCash", &self.FrozenCash)
            .field("FrozenCommission", &self.FrozenCommission)
            .field("CurrMargin", &self.CurrMargin)
            .field("CashIn", &self.CashIn)
            .field("Commission", &self.Commission)
            .field("CloseProfit", &self.CloseProfit)
            .field("PositionProfit", &self.PositionProfit)
            .field("Balance", &self.Balance)
            .field("Available", &self.Available)
            .field("WithdrawQuota", &self.WithdrawQuota)
            .field("Reserve", &self.Reserve)
            .field("TradingDay", &gb18030_cstr_to_str(&self.TradingDay))
            .field("SettlementID", &self.SettlementID)
            .field("Credit", &self.Credit)
            .field("Mortgage", &self.Mortgage)
            .field("ExchangeMargin", &self.ExchangeMargin)
            .field("DeliveryMargin", &self.DeliveryMargin)
            .field("ExchangeDeliveryMargin", &self.ExchangeDeliveryMargin)
            .field("ReserveBalance", &self.ReserveBalance)
            .field("CurrencyID", &self.CurrencyID)
            .field("PreFundMortgageIn", &self.PreFundMortgageIn)
            .field("PreFundMortgageOut", &self.PreFundMortgageOut)
            .field("FundMortgageIn", &self.FundMortgageIn)
            .field("FundMortgageOut", &self.FundMortgageOut)
            .field("FundMortgageAvailable", &self.FundMortgageAvailable)
            .field("MortgageableFund", &self.MortgageableFund)
            .field("SpecProductMargin", &self.SpecProductMargin)
            .field("SpecProductFrozenMargin", &self.SpecProductFrozenMargin)
            .field("SpecProductCommission", &self.SpecProductCommission)
            .field("SpecProductFrozenCommission", &self.SpecProductFrozenCommission)
            .field("SpecProductPositionProfit", &self.SpecProductPositionProfit)
            .field("SpecProductCloseProfit", &self.SpecProductCloseProfit)
            .field("SpecProductPositionProfitByAlg", &self.SpecProductPositionProfitByAlg)
            .field("SpecProductExchangeMargin", &self.SpecProductExchangeMargin)
            .finish()
    }
}


impl fmt::Debug for Struct_CThostFtdcInvestorPositionField {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Struct_CThostFtdcInvestorPositionField")
            .field("InstrumentID", &gb18030_cstr_to_str(&self.InstrumentID))
            .field("BrokerID", &gb18030_cstr_to_str(&self.BrokerID))
            .field("InvestorID", &gb18030_cstr_to_str(&self.InvestorID))
            .field("PosiDirection", &self.PosiDirection)
            .field("HedgeFlag", &self.HedgeFlag)
            .field("PositionDate", &self.PositionDate)
            .field("YdPosition", &self.YdPosition)
            .field("Position", &self.Position)
            .field("LongFrozen", &self.LongFrozen)
            .field("ShortFrozen", &self.ShortFrozen)
            .field("LongFrozenAmount", &self.LongFrozenAmount)
            .field("ShortFrozenAmount", &self.ShortFrozenAmount)
            .field("OpenVolume", &self.OpenVolume)
            .field("CloseVolume", &self.CloseVolume)
            .field("OpenAmount", &self.OpenAmount)
            .field("CloseAmount", &self.CloseAmount)
            .field("PositionCost", &self.PositionCost)
            .field("PreMargin", &self.PreMargin)
            .field("UseMargin", &self.UseMargin)
            .field("FrozenMargin", &self.FrozenMargin)
            .field("FrozenCash", &self.FrozenCash)
            .field("FrozenCommission", &self.FrozenCommission)
            .field("CashIn", &self.CashIn)
            .field("Commission", &self.Commission)
            .field("CloseProfit", &self.CloseProfit)
            .field("PositionProfit", &self.PositionProfit)
            .field("PreSettlementPrice", &self.PreSettlementPrice)
            .field("SettlementPrice", &self.SettlementPrice)
            .field("TradingDay", &gb18030_cstr_to_str(&self.TradingDay))
            .field("SettlementID", &self.SettlementID)
            .field("OpenCost", &self.OpenCost)
            .field("ExchangeMargin", &self.ExchangeMargin)
            .field("CombPosition", &self.CombPosition)
            .field("CombLongFrozen", &self.CombLongFrozen)
            .field("CombShortFrozen", &self.CombShortFrozen)
            .field("CloseProfitByDate", &self.CloseProfitByDate)
            .field("CloseProfitByTrade", &self.CloseProfitByTrade)
            .field("TodayPosition", &self.TodayPosition)
            .field("MarginRateByMoney", &self.MarginRateByMoney)
            .field("MarginRateByVolume", &self.MarginRateByVolume)
            .field("StrikeFrozen", &self.StrikeFrozen)
            .field("StrikeFrozenAmount", &self.StrikeFrozenAmount)
            .field("AbandonFrozen", &self.AbandonFrozen)
            .finish()
    }
}

impl fmt::Debug for Struct_CThostFtdcDepthMarketDataField {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let mut debug = fmt.debug_struct("Struct_CThostFtdcDepthMarketDataField");
        debug.field("TradingDay", &gb18030_cstr_to_str(&self.TradingDay))
            .field("InstrumentID", &gb18030_cstr_to_str(&self.InstrumentID))
            .field("ExchangeID", &gb18030_cstr_to_str(&self.ExchangeID))
            .field("ExchangeInstID", &gb18030_cstr_to_str(&self.ExchangeInstID))
            .field("LastPrice", &self.LastPrice)
            .field("PreSettlementPrice", &self.PreSettlementPrice)
            .field("PreClosePrice", &self.PreClosePrice)
            .field("PreOpenInterest", &self.PreOpenInterest)
            .field("OpenPrice", &self.OpenPrice)
            .field("HighestPrice", &self.HighestPrice)
            .field("LowestPrice", &self.LowestPrice)
            .field("Volume", &self.Volume)
            .field("Turnover", &self.Turnover)
            .field("OpenInterest", &self.OpenInterest)
            .field("ClosePrice", &self.ClosePrice)
            .field("SettlementPrice", &self.SettlementPrice)
            .field("UpperLimitPrice", &self.UpperLimitPrice)
            .field("LowerLimitPrice", &self.LowerLimitPrice)
            .field("PreDelta", &self.PreDelta)
            .field("CurrDelta", &self.CurrDelta)
            .field("UpdateTime", &gb18030_cstr_to_str(&self.UpdateTime))
            .field("UpdateMillisec", &self.UpdateMillisec)
            .field("BidPrice1", &self.BidPrice1)
            .field("BidVolume1", &self.BidVolume1)
            .field("AskPrice1", &self.AskPrice1)
            .field("AskVolume1", &self.AskVolume1);
        if !(self.BidVolume2 == 0 &&
             self.AskVolume2 == 0 &&
             self.BidVolume3 == 0 &&
             self.AskVolume3 == 0 &&
             self.BidVolume4 == 0 &&
             self.AskVolume4 == 0 &&
             self.BidVolume5 == 0 &&
             self.AskVolume5 == 0) {
                 debug.field("BidPrice2", &self.BidPrice2)
                     .field("BidVolume2", &self.BidVolume2)
                     .field("AskPrice2", &self.AskPrice2)
                     .field("AskVolume2", &self.AskVolume2)
                     .field("BidPrice3", &self.BidPrice3)
                     .field("BidVolume3", &self.BidVolume3)
                     .field("AskPrice3", &self.AskPrice3)
                     .field("AskVolume3", &self.AskVolume3)
                     .field("BidPrice4", &self.BidPrice4)
                     .field("BidVolume4", &self.BidVolume4)
                     .field("AskPrice4", &self.AskPrice4)
                     .field("AskVolume4", &self.AskVolume4)
                     .field("BidPrice5", &self.BidPrice5)
                     .field("BidVolume5", &self.BidVolume5)
                     .field("AskPrice5", &self.AskPrice5)
                     .field("AskVolume5", &self.AskVolume5);
             }
        debug.field("AveragePrice", &self.AveragePrice)
            .field("ActionDay", &gb18030_cstr_to_str(&self.ActionDay))
            .finish()
    }
}

impl fmt::Debug for Struct_CThostFtdcExchangeField {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Struct_CThostFtdcExchangeField")
            .field("ExchangeID", &gb18030_cstr_to_str(&self.ExchangeID))
            .field("ExchangeName", &gb18030_cstr_to_str(&self.ExchangeName))
            .field("ExchangeProperty", &self.ExchangeProperty)
            .finish()
    }
}

impl fmt::Debug for Struct_CThostFtdcProductField {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Struct_CThostFtdcProductField")
            .field("ProductID", &gb18030_cstr_to_str(&self.ProductID))
            .field("ProductName", &gb18030_cstr_to_str(&self.ProductName))
            .field("ExchangeID", &gb18030_cstr_to_str(&self.ExchangeID))
            .field("ProductClass", &self.ProductClass)
            .field("VolumeMultiple", &self.VolumeMultiple)
            .field("PriceTick", &self.PriceTick)
            .field("MaxMarketOrderVolume", &self.MaxMarketOrderVolume)
            .field("MinMarketOrderVolume", &self.MinMarketOrderVolume)
            .field("MaxLimitOrderVolume", &self.MaxLimitOrderVolume)
            .field("MinLimitOrderVolume", &self.MinLimitOrderVolume)
            .field("PositionType", &self.PositionType)
            .field("PositionDateType", &self.PositionDateType)
            .field("CloseDealType", &self.CloseDealType)
            .field("TradeCurrencyID", &self.TradeCurrencyID)
            .field("MortgageFundUseRange", &self.MortgageFundUseRange)
            .field("ExchangeProductID", &gb18030_cstr_to_str(&self.ExchangeProductID))
            .field("UnderlyingMultiple", &self.UnderlyingMultiple)
            .finish()
    }
}

impl fmt::Debug for Struct_CThostFtdcForQuoteRspField {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Struct_CThostFtdcForQuoteRspField")
            .field("TradingDay", &gb18030_cstr_to_str(&self.TradingDay))
            .field("InstrumentID", &gb18030_cstr_to_str(&self.InstrumentID))
            .field("ForQuoteSysID", &gb18030_cstr_to_str(&self.ForQuoteSysID))
            .field("ForQuoteTime", &gb18030_cstr_to_str(&self.ForQuoteTime))
            .field("ActionDay", &gb18030_cstr_to_str(&self.ActionDay))
            .field("ExchangeID", &gb18030_cstr_to_str(&self.ExchangeID))
            .finish()
    }
}
