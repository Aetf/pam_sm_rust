#![allow(dead_code)]
#![allow(non_camel_case_types)]

use libc;
use pam::PamError;
use std::option::Option;
use std::os::raw::{c_char, c_int, c_uint, c_void};
use std::ptr::write_volatile;

pub type PamHandle = *const c_uint;

#[repr(C)]
pub enum PamMsgStyle {
    PROMPT_ECHO_OFF = 1, /* Ask for password without echo */
    PROMPT_ECHO_ON = 2,  /* Ask for password with echo */
    ERROR_MSG = 3,       /* Display an error message */
    TEXT_INFO = 4,       /* Display arbitrary text */
    // Linux extensions
    PAM_MAX_NUM_MSG = 32,
    PAM_RADIO_TYPE = 5, /* yes/no/maybe conditionals */
    PAM_BINARY_PROMPT = 7,
}

#[repr(C)]
pub struct PamMessage {
    pub msg_style: PamMsgStyle,
    pub msg: *const c_char,
}

#[repr(C)]
pub struct PamResponse {
    pub resp: *mut c_char,
    pub resp_retcode: PamError,
}

impl PamResponse {
    pub fn get_buff(&self) -> *const c_char {
        self.resp as *const c_char
    }

    pub fn cleanup(&mut self) {
        unsafe {
            if !self.resp.is_null() {
                for _ in 0..libc::strlen(self.resp) {
                    write_volatile(self.resp, 0i8);
                }
                libc::free(self.resp as *mut libc::c_void);
            }
            let asptr: *mut PamResponse = self;
            libc::free(asptr as *mut libc::c_void);
        }
    }
}

pub(crate) type PamConvCallback = extern "C" fn(
    arg1: c_int,
    arg2: *mut *const PamMessage,
    arg3: *mut *mut PamResponse,
    arg4: *mut c_void,
) -> c_int;

#[repr(C)]
pub(crate) struct PamConv {
    pub(crate) cb: Option<PamConvCallback>,
    pub(crate) appdata_ptr: *mut c_void,
}

#[repr(C)]
pub enum LogLvl {
    EMERG = 0,   /* system is unusable */
    ALERT = 1,   /* action must be taken immediately */
    CRIT = 2,    /* critical conditions */
    ERR = 3,     /* error conditions */
    WARNING = 4, /* warning conditions */
    NOTICE = 5,  /* normal but significant condition */
    INFO = 6,    /* informational */
    DEBUG = 7,   /* debug-level messages */
}

#[repr(C)]
pub enum PamItemType {
    SERVICE = 1,       /* The service name */
    USER = 2,          /* The user name */
    TTY = 3,           /* The tty name */
    RHOST = 4,         /* The remote host name */
    CONV = 5,          /* The pam_conv structure */
    AUTHTOK = 6,       /* The authentication token (password) */
    OLDAUTHTOK = 7,    /* The old authentication token */
    RUSER = 8,         /* The remote user name */
    USER_PROMPT = 9,   /* the prompt for getting a username */
    FAIL_DELAY = 10,   /* app supplied function to override failure delays */
    XDISPLAY = 11,     /* X display name */
    XAUTHDATA = 12,    /* X server authentication data */
    AUTHTOK_TYPE = 13, /* The type for pam_get_authtok */
}
