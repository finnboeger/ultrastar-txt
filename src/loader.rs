extern crate chardet;
extern crate encoding;

use crate::parser::{parse_txt_header_str, parse_txt_lines_str};
use crate::structs::{TXTSong, Source};
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use regex::Regex;

error_chain! {
    errors {
        #[doc="input output error while handling the file"]
        IOError {
            description("io error")
        }
        #[doc="error in encoding detection"]
        EncodingDetectionError {
            description("encoding detection error")
        }
        #[doc="error while decoding"]
        DecodingError(msg: String) {
            description("decoding error")
            display("decoding error: {}", msg)
        }
        #[doc="error in path canonicalization"]
        CanonicalizationError(msg: String) {
            description("canonicalization error")
            display("canonicalization error: {}", msg)
        }
        #[doc="error in parsing the song header"]
        HeaderParsingError(msg: String) {
            description("header parsing error")
            display("header parsing error: {}", msg)
        }
        #[doc="error in parsing the songs lines"]
        LinesParsingError(msg: String) {
            description("lines parsing error")
            display("lines parsing error: {}", msg)
        }
    }
}

fn read_file_to_string<P: AsRef<Path>>(p: P) -> Result<String> {
    let p = p.as_ref();
    let mut f = File::open(p).chain_err(|| ErrorKind::IOError)?;
    let mut reader: Vec<u8> = Vec::new();
    f.read_to_end(&mut reader)
        .chain_err(|| ErrorKind::IOError)?;

    // decode as ascii and search for ENCODING Header
    let test_coder = encoding::label::encoding_from_whatwg_label("ascii").unwrap();
    let test_content = match test_coder.decode(&reader, encoding::DecoderTrap::Ignore) {
        Ok(x) => x,
        Err(e) => bail!(ErrorKind::DecodingError(e.into_owned())),
    };
    let mut whtwg_label = String::new();
    match Regex::new(r"#ENCODING:([A-Za-z0-9\-_:.]+)\s*\n").unwrap().captures(&test_content) {
        Some(cap) => {
            // get encoding from header
            whtwg_label.push_str(cap.get(1).unwrap().as_str());
        },
        None => {
            // detect encoding
            let chardet_result = chardet::detect(&reader);
            whtwg_label.push_str(chardet::charset2encoding(&chardet_result.0));
        },
    };
    // decode to String
    let coder = encoding::label::encoding_from_whatwg_label(&whtwg_label);
    let file_content = match coder {
        Some(c) => match c.decode(&reader, encoding::DecoderTrap::Ignore) {
            Ok(x) => {
                if x.len() > 0 && vec!["utf-8", "utf8"].contains(&&*whtwg_label.to_lowercase()) {
                    let mut chars = x.chars();
                    let first = chars.next().unwrap();
                    if first == '\u{feff}' {
                        chars.as_str().to_string()
                    } else {
                        x
                    }
                } else {
                    x
                }
            },
            Err(e) => bail!(ErrorKind::DecodingError(e.into_owned())),
        },
        None => bail!(ErrorKind::EncodingDetectionError),
    };

    // handle UTF8 BOM manually


    Ok(file_content)
}

fn canonicalize_path<B: AsRef<Path>>(
    path: &Option<Source>,
    base_path: B,
) -> Result<Option<Source>> {
    Ok( if let Some(source) = path {
        Some(match source {
            #[cfg(feature = "url-support")]
            Source::Remote(x) => Source::Remote(x.to_owned()),
            Source::Local(x) => {
                let mut tmp_path = PathBuf::from(base_path.as_ref());
                tmp_path.push(x);
                let result = tmp_path
                    .canonicalize()
                    .chain_err(|| ErrorKind::CanonicalizationError(format!("{:?}", tmp_path)))?;
                Source::Local(result)
            }
        })
    } else {
        None
    })
}

/// Takes path to a song file and returns TXTSong struct with canonicalized local sources
///
/// # Arguments
/// * path - the path to the song file to parse
///
pub fn parse_txt_song<P: AsRef<Path>>(path: P) -> Result<TXTSong> {
    let path = path.as_ref();
    let txt = read_file_to_string(path)?;

    let mut txt_song = TXTSong {
        header: parse_txt_header_str(txt.as_ref())
            .map_err(|e| {
                let s = e.to_string();
                Error::with_chain(
                    e,
                    ErrorKind::HeaderParsingError(s),
                )
            })?,
        lines: parse_txt_lines_str(txt.as_ref())
            .map_err(|e| {
                let s = e.to_string();
                Error::with_chain(
                    e,
                    ErrorKind::LinesParsingError(s)
                )
            })?,
    };

    // canonicalize paths
    if let Some(base_path) = path.parent() {
        // canonicalize audio path
        txt_song.header.audio_path =
            canonicalize_path(&Some(txt_song.header.audio_path), base_path)?.unwrap();

        // canonicalize other path
        txt_song.header.video_path = canonicalize_path(&txt_song.header.video_path, base_path)?;
        txt_song.header.cover_path = canonicalize_path(&txt_song.header.cover_path, base_path)?;
        txt_song.header.background_path =
            canonicalize_path(&txt_song.header.background_path, base_path)?;
    }

    Ok(txt_song)
}
