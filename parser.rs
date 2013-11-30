// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Higher-level Rust constructs for http_parser

use std::vec::raw::from_buf_raw;
use std::libc::{c_int, c_void, c_char, size_t};
use std::ptr::{null, to_unsafe_ptr};
use std::str;
use http_parser;
use http_parser::{http_parser_settings, HTTP_REQUEST};
use http_parser::{http_parser_init, http_parser_execute};
use http_parser::{enum_http_errno, http_errno_name, http_errno_description};

// pub type HttpCallback = || -> bool;
// pub type HttpDataCallback = |data: ~[u8]| -> bool;

pub struct ParserCallbacks<'self> {
    on_message_begin: &'self fn () -> bool,
    on_url: &'self fn (data: ~[u8]) -> bool,
    on_status_complete: &'self fn () -> bool,
    on_header_field: &'self fn (data: ~[u8]) -> bool,
    on_header_value: &'self fn (data: ~[u8]) -> bool,
    on_headers_complete: &'self fn () -> bool,
    on_body: &'self fn (data: ~[u8]) -> bool,
    on_message_complete: &'self fn () -> bool
}

pub struct Parser {
    http_parser: http_parser::http_parser,
    settings: http_parser_settings
}

pub fn Parser() -> Parser {
    #[fixed_stack_segment];
    let http_parser = http_parser::struct_http_parser {
        _type_flags: 0,
        state: 0,
        header_state: 0,
        index: 0,
        nread: 0,
        content_length: 0,
        http_major: 0,
        http_minor: 0,
        status_code: 0,
        method: 0,
        http_errno_upgrade: 0,
        data: null()
    };

    unsafe {
        http_parser_init(&http_parser, HTTP_REQUEST);
    }

    let settings = http_parser::struct_http_parser_settings {
        on_message_begin: on_message_begin as *u8,
        on_url: on_url as *u8,
        on_status_complete: on_status_complete as *u8,
        on_header_field: on_header_field as *u8,
        on_header_value: on_header_value as *u8,
        on_headers_complete: on_headers_complete as *u8,
        on_body: on_body as *u8,
        on_message_complete: on_message_complete as *u8
    };

    Parser {
        http_parser: http_parser,
        settings: settings
    }
}

impl Parser {
    pub fn execute(&mut self, data: &[u8], callbacks: &ParserCallbacks) -> uint {
        #[fixed_stack_segment];
        unsafe {
            self.http_parser.data = to_unsafe_ptr(callbacks) as *c_void;
            do data.as_imm_buf |buf, _| {
                http_parser_execute(&self.http_parser,
                                    &self.settings,
                                    buf as *c_char,
                                    data.len() as size_t) as uint
            }
        }
    }

    pub fn status_code(&self) -> uint {
        self.http_parser.status_code as uint
    }

    pub fn method(&self) -> uint {
        self.http_parser.method as uint
    }

    pub fn error(&self) -> (~str, ~str) {
        #[fixed_stack_segment];
        let err = (self.http_parser.http_errno_upgrade & 0x7f) as enum_http_errno;
        unsafe {
            (str::raw::from_c_str(http_errno_name(err)),
             str::raw::from_c_str(http_errno_description(err)))
        }
    }
}

fn callbacks(http_parser: *http_parser::http_parser) -> *ParserCallbacks {
    unsafe {
        assert!((*http_parser).data.is_not_null());
        return (*http_parser).data as *ParserCallbacks;
    }
}

extern fn on_message_begin(http_parser: *http_parser::http_parser) -> c_int {
    unsafe {
        (!((*callbacks(http_parser)).on_message_begin)()) as c_int
    }
}

extern fn on_url(http_parser: *http_parser::http_parser, at: *u8, length: size_t) -> c_int {
    unsafe {
        (!(((*callbacks(http_parser)).on_url)(from_buf_raw(at, length as uint)))) as c_int
    }
}

extern fn on_status_complete(http_parser: *http_parser::http_parser) -> c_int {
    unsafe {
        (!((*callbacks(http_parser)).on_status_complete)()) as c_int
    }
}

extern fn on_header_field(http_parser: *http_parser::http_parser, at: *u8, length: size_t) ->
        c_int {
    unsafe {
        (!((*callbacks(http_parser)).on_header_field)(from_buf_raw(at, length as uint))) as c_int
    }
}

extern fn on_header_value(http_parser: *http_parser::http_parser, at: *u8, length: size_t) ->
        c_int {
    unsafe {
        (!((*callbacks(http_parser)).on_header_value)(from_buf_raw(at, length as uint))) as c_int
    }
}

extern fn on_headers_complete(http_parser: *http_parser::http_parser) -> c_int {
    unsafe {
        (!((*callbacks(http_parser)).on_headers_complete)()) as c_int
    }
}

extern fn on_body(http_parser: *http_parser::http_parser, at: *u8, length: size_t) -> c_int {
    unsafe {
        (!((*callbacks(http_parser)).on_body)(from_buf_raw(at, length as uint))) as c_int
    }
}

extern fn on_message_complete(http_parser: *http_parser::http_parser) -> c_int {
    unsafe {
        (!((*callbacks(http_parser)).on_message_complete)()) as c_int
    }
}
