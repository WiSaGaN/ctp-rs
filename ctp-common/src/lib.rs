mod binding;
pub use binding::*;

use encoding::{ DecoderTrap, Encoding };
use encoding::all::GB18030;
use simple_error::SimpleError;
use std::borrow::Cow;
use std::fmt;
use std::os::raw::c_int;
use time::{ Timespec, Tm };

/// 交易接口中的查询操作的限制为:
///   每秒钟最多只能进行一次查询操作。
///   在途的查询操作最多只能有一个。
/// 在途:查询操作从发送请求,到接收到响应为一个完整的过程。如果请求已经发送,但是未收到响应,则称
/// 该查询操作在途。
/// 上述限制只针对交易接口中的数据查询操作(ReqQryXXX),对报单,撤单,报价,询价等操作没有影响。
pub const DEFAULT_MAX_NUM_QUERY_REQUEST_PER_SECOND: usize = 1;

/// 报单流量限制是由期货公司通过在系统中配置相关参数实现限制的。
/// 不进行配置的情况下,默认流量限制为:
/// 在一个连接会话(Session)中,每个客户端每秒钟最多只能发送 6 笔交易相关的指令(报单,撤单等)。
pub const DEFAULT_MAX_NUM_ORDER_REQUEST_PER_SECOND: usize = 6;
/// 同一个账户同时最多只能建立 6 个会话(Session)。
pub const DEFAULT_MAX_NUM_CONCURRENT_SESSION: usize = 6;

pub fn ascii_cstr_to_str(s: &[u8]) -> Result<&str, SimpleError> {
    match s.last() {
        Some(&0u8) => {
            let len = memchr::memchr(0, s).unwrap();
            let ascii_s = &s[0..len];
            if ascii_s.is_ascii() {
                unsafe {
                    Ok(std::str::from_utf8_unchecked(ascii_s))
                }
            } else {
                Err(SimpleError::new("cstr is not ascii"))
            }
        },
        Some(&c) => {
            Err(SimpleError::new(format!("cstr should terminate with null instead of {:#x}", c)))
        },
        None => {
            Err(SimpleError::new("cstr cannot have 0 length"))
        }
    }
}

pub fn gb18030_cstr_to_str(v: &[u8]) -> Cow<str> {
    let slice = v.split(|&c| c == 0u8).next().unwrap();
    if slice.is_ascii() {
        unsafe {
            return Cow::Borrowed::<str>(std::str::from_utf8_unchecked(slice));
        }
    }
    match GB18030.decode(slice, DecoderTrap::Replace) {
        Ok(s) => Cow::Owned(s),
        Err(e) => e,
    }
}

pub fn reduce_comb_flags(flags: &[u8]) -> String {
    flags.iter().filter(|&&c| c != 0).map(|&c| char::from(c)).collect()
}

pub fn maybe_char(c: u8) -> Option<char> {
    if c != 0u8 {
        Some(char::from(c))
    } else {
        None
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct OrderIdLocalTrio {
    pub front_id: TThostFtdcFrontIDType,
    pub session_id: TThostFtdcSessionIDType,
    pub order_ref: TThostFtdcOrderRefType,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct OrderIdExchangeDuo {
    pub exchange_id: TThostFtdcExchangeIDType,
    pub order_sys_id: TThostFtdcOrderSysIDType,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResumeType {
    Restart = THOST_TE_RESUME_TYPE::THOST_TERT_RESTART as isize,
    Resume = THOST_TE_RESUME_TYPE::THOST_TERT_RESUME as isize,
    Quick = THOST_TE_RESUME_TYPE::THOST_TERT_QUICK as isize,
}

impl std::convert::Into<THOST_TE_RESUME_TYPE> for ResumeType {
    fn into(self) -> THOST_TE_RESUME_TYPE {
        match self {
            ResumeType::Restart => THOST_TE_RESUME_TYPE::THOST_TERT_RESTART,
            ResumeType::Resume => THOST_TE_RESUME_TYPE::THOST_TERT_RESUME,
            ResumeType::Quick => THOST_TE_RESUME_TYPE::THOST_TERT_QUICK,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DisconnectionReason {
    ReadError = 0x1001,
    WriteError = 0x1002,
    HeartbeatTimeout = 0x2001,
    HeartbeatSendError = 0x2002,
    ErrorMessageReceived = 0x2003,
    Unknown = 0x0000,
}

impl std::convert::From<c_int> for DisconnectionReason {
    fn from(reason: c_int) -> DisconnectionReason {
        match reason {
            0x1001 => DisconnectionReason::ReadError,
            0x1002 => DisconnectionReason::WriteError,
            0x2001 => DisconnectionReason::HeartbeatTimeout,
            0x2002 => DisconnectionReason::HeartbeatSendError,
            0x2003 => DisconnectionReason::ErrorMessageReceived,
            _ => DisconnectionReason::Unknown,
        }
    }
}

impl fmt::Display for DisconnectionReason {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use DisconnectionReason::*;
        match *self {
            ReadError => f.write_str("read error"),
            WriteError => f.write_str("write error"),
            HeartbeatTimeout => f.write_str("heartbeat timeout"),
            HeartbeatSendError => f.write_str("heatbeat send error"),
            ErrorMessageReceived => f.write_str("error message received"),
            Unknown => f.write_str("unknown"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ApiError {
    NetworkError = -1,
    QueueFull = -2,
    Throttled = -3,
}

impl std::error::Error for ApiError {
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ApiError::NetworkError => f.write_str("network error"),
            ApiError::QueueFull => f.write_str("queue full"),
            ApiError::Throttled => f.write_str("throttled"),
        }
    }
}

#[must_use]
pub type ApiResult = Result<(), ApiError>;

// TODO: use std::convert::From after trait specialization?
pub fn from_api_return_to_api_result(api_return: c_int) -> ApiResult {
    match api_return {
        0 => Ok(()),
        -1 => Err(ApiError::NetworkError),
        -2 => Err(ApiError::QueueFull),
        -3 => Err(ApiError::Throttled),
        // no other values are possible in specification
        i => panic!("api return unspecified {}", i),
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RspError {
    pub id: TThostFtdcErrorIDType,
    pub msg: String,
}

impl std::error::Error for RspError {
}

impl fmt::Display for RspError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.id, self.msg)
    }
}

#[must_use]
pub type RspResult = Result<(), RspError>;

pub fn from_rsp_result_to_string(rsp_result: &RspResult) -> String {
    match rsp_result {
        Ok(()) => "Ok(())".to_string(),
        Err(err) => format!("Err(RspError{{ id: {}, msg: {} }})", err.id, err.msg),
    }
}

/// # Safety
///
/// `rsp_info` needs to be either a null pointer or a valid pointer of type `CThostFtdcRspInfoField`.
pub unsafe fn from_rsp_info_to_rsp_result(rsp_info: *const CThostFtdcRspInfoField) -> RspResult {
    #[allow(unused_unsafe)] // for future "unsafe blocks in unsafe fn" feature
    match unsafe { rsp_info.as_ref() } {
        Some(info) => match info {
            CThostFtdcRspInfoField { ErrorID: 0, .. } => {
                Ok(())
            },
            CThostFtdcRspInfoField { ErrorID: id, ErrorMsg: msg } => {
                Err(RspError{ id: *id, msg: gb18030_cstr_to_str(msg).into_owned() })
            }
        },
        None => {
            Ok(())
        },
    }
}

pub fn is_terminal_order_status(order_status: TThostFtdcOrderStatusType) -> bool {
    order_status == THOST_FTDC_OST_AllTraded ||
        order_status == THOST_FTDC_OST_PartTradedNotQueueing ||
        order_status == THOST_FTDC_OST_NoTradeNotQueueing ||
        order_status == THOST_FTDC_OST_Canceled
}

pub fn is_valid_order_sys_id(order_sys_id: &TThostFtdcOrderSysIDType) -> bool {
    order_sys_id[0] != b'\0'
}

#[allow(clippy::trivially_copy_pass_by_ref)] // Will be removed
#[deprecated(since = "0.9.0", note = "This will be removed in 0.10.0")]
pub fn to_exchange_timestamp(trading_day: &TThostFtdcDateType,
                              update_time: &TThostFtdcTimeType,
                              update_millisec: &TThostFtdcMillisecType) -> Result<Timespec, SimpleError> {
    let year = match ::std::str::from_utf8(&trading_day[0..4]) {
        Ok(year_str) => {
            match year_str.parse::<u16>() {
                Ok(year) => year,
                Err(err) => {
                    return Err(SimpleError::new(format!("invalid year string, {}", err)));
                },
            }
        },
        Err(err) => {
            return Err(SimpleError::new(format!("year not utf8, {}", err)));
        },
    };
    let month = match ::std::str::from_utf8(&trading_day[4..6]) {
        Ok(month_str) => {
            match month_str.parse::<u8>() {
                Ok(month) => month,
                Err(err) => {
                    return Err(SimpleError::new(format!("invalid month string, {}", err)));
                },
            }
        },
        Err(err) => {
            return Err(SimpleError::new(format!("month not utf8, {}", err)));
        },
    };
    let day = match ::std::str::from_utf8(&trading_day[6..8]) {
        Ok(day_str) => {
            match day_str.parse::<u8>() {
                Ok(day) => day,
                Err(err) => {
                    return Err(SimpleError::new(format!("invalid day string, {}", err)));
                },
            }
        },
        Err(err) => {
            return Err(SimpleError::new(format!("day not utf8, {}", err)));
        },
    };
    let hour = match ::std::str::from_utf8(&update_time[0..2]) {
        Ok(hour_str) => {
            match hour_str.parse::<u8>() {
                Ok(hour) => hour,
                Err(err) => {
                    return Err(SimpleError::new(format!("invalid hour string, {}", err)));
                },
            }
        },
        Err(err) => {
            return Err(SimpleError::new(format!("hour not utf8, {}", err)));
        },
    };
    let minute = match ::std::str::from_utf8(&update_time[3..5]) {
        Ok(minute_str) => {
            match minute_str.parse::<u8>() {
                Ok(minute) => minute,
                Err(err) => {
                    return Err(SimpleError::new(format!("invalid minute string, {}", err)));
                },
            }
        },
        Err(err) => {
            return Err(SimpleError::new(format!("minute not utf8, {}", err)));
        },
    };
    let second = match ::std::str::from_utf8(&update_time[6..8]) {
        Ok(second_str) => {
            match second_str.parse::<u8>() {
                Ok(second) => second,
                Err(err) => {
                    return Err(SimpleError::new(format!("invalid second string, {}", err)));
                },
            }
        },
        Err(err) => {
            return Err(SimpleError::new(format!("second not utf8, {}", err)));
        },
    };
    let nanosec = *update_millisec as i32 * 1000 * 1000;
    let tm = Tm { tm_sec: second as i32,
                  tm_min: minute as i32,
                  tm_hour: hour as i32 - 8, // UTC+8
                  tm_mday: day as i32,
                  tm_mon: month as i32 - 1,
                  tm_year: year as i32 - 1900,
                  tm_wday: 0i32,
                  tm_yday: 0i32,
                  tm_isdst: 0i32,
                  tm_utcoff: 0i32,
                  tm_nsec: nanosec };
    Ok(tm.to_timespec())
}

pub fn set_cstr_from_str(buffer: &mut [u8], text: &str) -> Result<(), SimpleError> {
    if let Some(i) = memchr::memchr(0, text.as_bytes()) {
        return Err(SimpleError::new(format!("null found in str at offset {} when filling cstr", i)));
    }
    if text.len() + 1 > buffer.len() {
        return Err(SimpleError::new(format!("str len {} too long when filling cstr with buffer len {}", text.len(), buffer.len())));
    }
    unsafe {
        std::ptr::copy_nonoverlapping(text.as_ptr(), buffer.as_mut_ptr(), text.len());
        *buffer.get_unchecked_mut(text.len()) = 0u8;
    }
    Ok(())
}

pub fn set_cstr_from_str_truncate(buffer: &mut [u8], text: &str) {
    for (place, data) in buffer.split_last_mut().expect("buffer len 0 in set_cstr_from_str_truncate").1.iter_mut().zip(text.as_bytes().iter()) {
        *place = *data;
    }
    unsafe {
        *buffer.get_unchecked_mut(text.len()) = 0u8;
    }
}

pub fn normalize_double(d: f64) -> Option<f64> {
    if d == std::f64::MAX {
        None
    } else {
        Some(d)
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;
    use time::Timespec;
    use super::ascii_cstr_to_str;
    use super::gb18030_cstr_to_str;
    use super::to_exchange_timestamp;
    use super::{ set_cstr_from_str, set_cstr_from_str_truncate };
    use super::CThostFtdcDepthMarketDataField;

    #[test]
    fn len_0_ascii_cstr_to_str() {
        assert!(ascii_cstr_to_str(b"").is_err());
    }

    #[test]
    fn ascii_cstr_to_str_trivial() {
        assert_eq!(ascii_cstr_to_str(b"hello\0"), Ok("hello"));
    }

    #[test]
    fn non_null_terminated_ascii_cstr_to_str() {
        assert!(ascii_cstr_to_str(b"hello").is_err());
    }

    #[test]
    fn non_ascii_cstr_to_str() {
        assert!(ascii_cstr_to_str(b"\xd5\xfd\xc8\xb7\0").is_err());
    }

    #[test]
    fn cstr_conversion_empty_str() {
        match gb18030_cstr_to_str(b"") {
            Cow::Borrowed::<str>(s) => assert_eq!(s, ""),
            Cow::Owned::<str>(_) => panic!("ascii str should not allocate"),
        };
        match gb18030_cstr_to_str(b"\0") {
            Cow::Borrowed::<str>(s) => assert_eq!(s, ""),
            Cow::Owned::<str>(_) => panic!("ascii str should not allocate"),
        };
    }

    #[test]
    fn cstr_conversion_ascii() {
        match gb18030_cstr_to_str(b"ascii") {
            Cow::Borrowed::<str>(s) => assert_eq!(s, "ascii"),
            Cow::Owned::<str>(_) => panic!("ascii str should not allocate"),
        };
    }

    #[test]
    fn cstr_conversion_ascii_cstr() {
        match gb18030_cstr_to_str(b"ascii\0") {
            Cow::Borrowed::<str>(s) => assert_eq!(s, "ascii"),
            Cow::Owned::<str>(_) => panic!("ascii str should not allocate"),
        };
    }

    #[test]
    fn cstr_conversion_gb2312() {
        assert_eq!(gb18030_cstr_to_str(b"\xd5\xfd\xc8\xb7"), "正确");
    }

    #[test]
    fn cstr_conversion_gb2312_cstr() {
        assert_eq!(gb18030_cstr_to_str(b"\xd5\xfd\xc8\xb7\0"), "正确");
    }

    #[test]
    fn fill_cstr_with_str() {
        let mut buffer: [u8; 8] = Default::default();
        set_cstr_from_str(buffer.as_mut(), "hello").unwrap();
        assert_eq!(buffer.as_ref(), b"hello\0\0\0");
    }

    #[test]
    fn fill_cstr_with_long_str() {
        let mut buffer: [u8; 1] = Default::default();
        assert!(set_cstr_from_str(buffer.as_mut(), "hello").is_err());
    }

    #[test]
    fn fill_cstr_with_str_containing_null() {
        let mut buffer: [u8; 8] = Default::default();
        assert!(set_cstr_from_str(buffer.as_mut(), "he\0llo").is_err());
    }

    #[test]
    fn fill_cstr_with_str_truncate() {
        let mut buffer: [u8; 8] = Default::default();
        set_cstr_from_str_truncate(buffer.as_mut(), "hello");
        assert_eq!(buffer.as_ref(), b"hello\0\0\0");
    }

    #[test]
    #[should_panic]
    fn fill_0_len_cstr_with_str_truncate_panic() {
        let mut buffer: [u8; 0] = Default::default();
        set_cstr_from_str_truncate(buffer.as_mut(), "hello");
    }

    #[test]
    fn fill_cstr_with_long_str_truncate() {
        let mut buffer: [u8; 6] = Default::default();
        set_cstr_from_str_truncate(buffer.as_mut(), "hello world");
        assert_eq!(buffer.as_ref(), b"hello\0");
    }

    #[test]
    fn exchange_timestamp_conversion() {
        let mut md: CThostFtdcDepthMarketDataField = Default::default();
        md.TradingDay = *b"19700101\0";
        md.UpdateTime = *b"08:00:00\0";
        md.UpdateMillisec = 0;
        let ts1 = to_exchange_timestamp(&md.TradingDay, &md.UpdateTime, &md.UpdateMillisec);
        assert_eq!(Ok(Timespec{ sec: 0, nsec: 0 }), ts1);
        md.TradingDay = *b"19700102\0";
        md.UpdateTime = *b"00:00:00\0";
        let ts2 = to_exchange_timestamp(&md.TradingDay, &md.UpdateTime, &md.UpdateMillisec);
        assert_eq!(Ok(Timespec{ sec: 57600, nsec: 0 }), ts2);
    }
}
