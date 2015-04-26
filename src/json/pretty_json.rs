use json::JsonEvent::*;
use json::ErrorCode::*;
use json::ParserError::*;
use json::DecoderError::*;
// use json::ParserState::*;
// use json::InternalStackElement::*;

use std::collections::{HashMap, BTreeMap};
use std::error::Error as StdError;
use std::i64;
use std::io::prelude::*;
use std::mem::swap;
use std::ops::Index;
use std::str::FromStr;
use std::string;
use std::{char, f64, fmt, io, str};

use Encodable;

pub struct PrettyJson<'a>
{
    inner: &'a Json
}

impl<'a> fmt::Display for PrettyJson<'a> {
    /// Encodes a json value into a string
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut shim = FormatShim { inner: f };
        let mut encoder = Encoder::new_pretty(&mut shim);
        match self.inner.encode(&mut encoder) {
            Ok(_) => Ok(()),
            Err(_) => Err(fmt::Error)
        }
    }
}
