#![feature(ptr_as_ref)]
extern crate encoding;
extern crate simple_error;
extern crate time;

mod binding;

use encoding::{ DecoderTrap, Encoding };
use encoding::all::GB18030;
use simple_error::SimpleError;
use std::ascii::AsciiExt;
use std::borrow::Cow;
use std::os::raw::c_int;
use time::{ Timespec, Tm };

pub use binding::*;

pub fn gb18030_cstr_to_str<'a>(v: &'a [u8]) -> Cow<'a, str> {
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResumeType {
    Restart = Enum_THOST_TE_RESUME_TYPE::THOST_TERT_RESTART as isize,
    Resume = Enum_THOST_TE_RESUME_TYPE::THOST_TERT_RESUME as isize,
    Quick = Enum_THOST_TE_RESUME_TYPE::THOST_TERT_QUICK as isize,
}

impl std::convert::Into<Enum_THOST_TE_RESUME_TYPE> for ResumeType {
    fn into(self) -> Enum_THOST_TE_RESUME_TYPE {
        match self {
            ResumeType::Restart => Enum_THOST_TE_RESUME_TYPE::THOST_TERT_RESTART,
            ResumeType::Resume => Enum_THOST_TE_RESUME_TYPE::THOST_TERT_RESUME,
            ResumeType::Quick => Enum_THOST_TE_RESUME_TYPE::THOST_TERT_QUICK,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ApiError {
    NetworkError = -1,
    QueueFull = -2,
    Throttled = -3,
    Unknown = -4,
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
        _ => Err(ApiError::Unknown),
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RspError {
    pub id: i32,
    pub msg: String,
}


#[must_use]
pub type RspResult = Result<(), RspError>;

pub fn from_rsp_result_to_string(rsp_result: RspResult) -> String {
    match rsp_result {
        Ok(()) => "Ok(())".to_string(),
        Err(err) => format!("Err(RspError{{ id: {}, msg: {} }})", err.id, err.msg),
    }
}

pub fn from_rsp_info_to_rsp_result(rsp_info: *const Struct_CThostFtdcRspInfoField) -> RspResult {
    match unsafe { rsp_info.as_ref() } {
        Some(&info) => match info {
            Struct_CThostFtdcRspInfoField { ErrorID: 0, ErrorMsg: _ } => {
                Ok(())
            },
            Struct_CThostFtdcRspInfoField { ErrorID: id, ErrorMsg: ref msg } => {
                Err(RspError{ id: id, msg: gb18030_cstr_to_str(msg).into_owned() })
            }
        },
        None => {
            Ok(())
        },
    }
}

pub fn get_exchange_timestamp(md: &Struct_CThostFtdcDepthMarketDataField) -> Result<Timespec, SimpleError> {
    let year = match ::std::str::from_utf8(&md.TradingDay[0..4]) {
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
    let month = match ::std::str::from_utf8(&md.TradingDay[4..6]) {
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
    let day = match ::std::str::from_utf8(&md.TradingDay[6..8]) {
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
    let hour = match ::std::str::from_utf8(&md.UpdateTime[0..2]) {
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
    let minute = match ::std::str::from_utf8(&md.UpdateTime[3..5]) {
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
    let second = match ::std::str::from_utf8(&md.UpdateTime[6..8]) {
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
    let nanosec = md.UpdateMillisec as i32 * 1000 * 1000;
    let tm = Tm { tm_sec: second as i32,
                  tm_min: minute as i32,
                  tm_hour: hour as i32,
                  tm_mday: day as i32,
                  tm_mon: month as i32 - 1,
                  tm_year: year as i32 - 1900,
                  tm_wday: 0i32,
                  tm_yday: 0i32,
                  tm_isdst: 0i32,
                  tm_utcoff: 8i32 * 60 * 60, // UTC+8
                  tm_nsec: nanosec };
    Ok(tm.to_timespec())

}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;
    use time::Timespec;
    use super::gb18030_cstr_to_str;
    use super::get_exchange_timestamp;
    use super::Struct_CThostFtdcDepthMarketDataField;

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
    fn exchange_timestamp_conversion() {
        let mut md: Struct_CThostFtdcDepthMarketDataField = Default::default();
        md.TradingDay = *b"19700101\0";
        md.UpdateTime = *b"08:00:00\0";
        md.UpdateMillisec = 0;
        let ts = get_exchange_timestamp(&md);
        assert_eq!(Ok(Timespec{ sec: 0, nsec: 0 }), ts);
    }
}
