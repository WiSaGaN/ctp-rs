#![feature(ptr_as_ref)]
extern crate encoding;

mod binding {
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/struct.rs.in"));
include!(concat!(env!("OUT_DIR"), "/data_type.rs.in"));
}

use encoding::{ DecoderTrap, Encoding };
use encoding::all::GB18030;
use std::ascii::AsciiExt;
use std::borrow::Cow;
use std::fmt;
use std::os::raw::c_int;

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

#[cfg(test)]
mod tests {
    use std::borrow::Cow;
    use super::gb18030_cstr_to_str;

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
}
