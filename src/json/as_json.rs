use super::JsonEvent::*;
use super::ErrorCode::*;
use super::ParserError::*;
use super::DecoderError::*;
// use super::ParserState::*;
// use super::InternalStackElement::*;

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

pub struct AsJson<'a, T: 'a>
{
    inner: &'a T
}

impl<'a, T: Encodable> fmt::Display for AsJson<'a, T> {
    /// Encodes a json value into a string
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut shim = FormatShim { inner: f };
        let mut encoder = Encoder::new(&mut shim);
        match self.inner.encode(&mut encoder) {
            Ok(_) => Ok(()),
            Err(_) => Err(fmt::Error)
        }
    }
}
