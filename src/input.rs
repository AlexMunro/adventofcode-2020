use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::str::FromStr;

pub fn parse<T>(path: &Path) -> std::io::Result<impl Iterator<Item = T>>
where
    T: FromStr,
{
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    Ok(std::iter::from_fn(move || {
        buf.clear();
        reader
            .read_line(&mut buf)
            .map_err(|_| ())
            .and_then(|_| T::from_str(buf.trim()).map_err(|_| ()))
            .ok()
    })
    .fuse())
}

/// adaptor which plugs into parse, splitting comma-separated items from the line
///
/// This can be flattened or consumed by line, as required
pub struct CommaSep<T>(Vec<T>);

impl<T> FromStr for CommaSep<T>
where
    T: FromStr,
{
    type Err = <T as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split(',')
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()
            .map(CommaSep)
    }
}

impl<T> IntoIterator for CommaSep<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

/// Parse the file at the specified path into a stream of `T`.
///
/// Lines are batched into clusters separated by blank lines. Once a cluster has been
/// collected, it (and internal newlines) are parsed into a `T` instance.
///
/// As whitespace is potentially significant, it is not adjusted in any way before being
/// handed to the parser.
pub fn parse_newline_sep<T>(path: &Path) -> std::io::Result<impl Iterator<Item = T>>
where
    T: FromStr,
{
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buf = String::new();

    fn is_new_field(buf: &str) -> bool {
        let patterns = ["\n\n", "\n\r\n"];
        patterns.iter().any(|pat| {
            buf.as_bytes()
                .iter()
                .rev()
                .zip(pat.as_bytes().iter())
                .all(|(b, p)| b == p)
        })
    }

    Ok(std::iter::from_fn(move || {
        buf.clear();
        while buf.is_empty() || !is_new_field(&buf) {
            if reader.read_line(&mut buf).ok()? == 0 {
                break;
            }
        }
        if buf.is_empty() {
            None
        } else {
            T::from_str(&buf).ok()
        }
    })
    .fuse())
}
