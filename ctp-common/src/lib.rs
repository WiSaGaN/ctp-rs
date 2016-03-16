#![feature(ptr_as_ref)]
extern crate encoding;

mod binding {
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/ctp-api.rs.in"));
}

use encoding::{ decode, DecoderTrap };
use encoding::all::GB18030;
use std::ffi::CStr;
use std::os::raw::{ c_char, c_int };

pub use binding::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResumeType {
    Restart = 0,
    Resume = 1,
    Quick = 2,
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

pub fn gb18030_cstr_to_string(cstr: &CStr) -> String {
    decode(cstr.to_bytes(), DecoderTrap::Replace, GB18030).0.unwrap_or_else(|e| e.into_owned())
}

pub fn from_rsp_info_to_rsp_result(rsp_info: *const Struct_CThostFtdcRspInfoField) -> RspResult {
    match unsafe { rsp_info.as_ref() } {
        Some(&info) => match info {
            Struct_CThostFtdcRspInfoField { ErrorID: 0, ErrorMsg: _ } => {
                Ok(())
            },
            Struct_CThostFtdcRspInfoField { ErrorID: id, ErrorMsg: ref msg } => {
                Err(RspError{ id: id, msg: gb18030_cstr_to_string(unsafe { CStr::from_ptr(msg as *const u8 as *const c_char) }) })
            }
        },
        None => {
            Ok(())
        },
    }

}
