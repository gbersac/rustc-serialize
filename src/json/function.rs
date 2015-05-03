use json::json::{Json};
use json::encoder::{Encoder, EncodeResult};
use json::decoder::{Decoder, DecodeResult};
use json::as_json::{AsJson};
use json::pretty_json::{AsPrettyJson};
use json::error::{ErrorCode, DecoderError};
use Encodable;

use std::string;


/// Shortcut function to decode a JSON `&str` into an object
pub fn decode<T: ::Decodable>(s: &str) -> DecodeResult<T> {
    let json = match Json::from_str(s) {
        Ok(x) => x,
        Err(e) => return Err(DecoderError::ParseError(e))
    };

    let mut decoder = Decoder::new(json);
    ::Decodable::decode(&mut decoder)
}

/// Shortcut function to encode a `T` into a JSON `String`
pub fn encode<T: ::Encodable>(object: &T) -> EncodeResult<string::String> {
    let mut s = String::new();
    {
        let mut encoder = Encoder::new(&mut s);
        try!(object.encode(&mut encoder));
    }
    Ok(s)
}

/// Create an `AsJson` wrapper which can be used to print a value as JSON
/// on-the-fly via `write!`
pub fn as_json<T: Encodable>(t: &T) -> AsJson<T> {
    AsJson { inner: t }
}

/// Create an `AsPrettyJson` wrapper which can be used to print a value as JSON
/// on-the-fly via `write!`
pub fn as_pretty_json<T: Encodable>(t: &T) -> AsPrettyJson<T> {
    AsPrettyJson { inner: t, indent: None }
}

/// Returns a readable error string for a given error code.
pub fn error_str(error: ErrorCode) -> &'static str {
    match error {
        ErrorCode::InvalidSyntax => "invalid syntax",
        ErrorCode::InvalidNumber => "invalid number",
        ErrorCode::EOFWhileParsingObject => "EOF While parsing object",
        ErrorCode::EOFWhileParsingArray => "EOF While parsing array",
        ErrorCode::EOFWhileParsingValue => "EOF While parsing value",
        ErrorCode::EOFWhileParsingString => "EOF While parsing string",
        ErrorCode::KeyMustBeAString => "key must be a string",
        ErrorCode::ExpectedColon => "expected `:`",
        ErrorCode::TrailingCharacters => "trailing characters",
        ErrorCode::TrailingComma => "trailing comma",
        ErrorCode::InvalidEscape => "invalid escape",
        ErrorCode::UnrecognizedHex => "invalid \\u{ esc}ape (unrecognized hex)",
        ErrorCode::NotFourDigit => "invalid \\u{ esc}ape (not four digits)",
        ErrorCode::ControlCharacterInString => "unescaped control character in string",
        ErrorCode::NotUtf8 => "contents not utf-8",
        ErrorCode::InvalidUnicodeCodePoint => "invalid Unicode code point",
        ErrorCode::LoneLeadingSurrogateInHexEscape => "lone leading surrogate in hex escape",
        ErrorCode::UnexpectedEndOfHexEscape => "unexpected end of hex escape",
    }
}
