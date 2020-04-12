#![feature(with_options)]

use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader},
    path::Path,
};

/**
 * Parse txt files under the directory `public/UCD/` of Unicode Consortium.
 *
 * # Example
 *
 * ```rust
 * # use ucd_parse::UCD;
 * # fn main() {
 *
 * let ucd = UCD::new(r"
 * ## Comment
 *
 * 0000..007F ; Basic Latin # Basic Latin
 * ## Comment
 * 3040..309F ; Hiragana
 *
 * 30A0..30FF ; Katakana
 *
 * ## Comment
 * ");
 *
 * let mut lines = ucd.ucd_lines();
 * let line = lines.next().unwrap().into_iter().collect::<Vec<_>>();
 * assert_eq!(line, vec!["0000..007F", "Basic Latin"]);
 * let line = lines.next().unwrap().into_iter().collect::<Vec<_>>();
 * assert_eq!(line, vec!["3040..309F", "Hiragana"]);
 * let line = lines.next().unwrap().into_iter().collect::<Vec<_>>();
 * assert_eq!(line, vec!["30A0..30FF", "Katakana"]);
 * assert!(lines.next().is_none());
 * # }
 * ```
 */
pub struct UCD<T>(T);

impl<T> UCD<T> {
    pub fn new(src: T) -> Self {
        Self(src)
    }
}

impl UCD<BufReader<File>> {
    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        Ok(Self::new(BufReader::new(File::open(path)?)))
    }
    pub fn with_options<P: AsRef<Path>>() -> OpenOptions {
        OpenOptions(File::with_options())
    }
}

impl<T: io::Read> UCD<BufReader<T>> {
    pub fn ucd_lines(self) -> UCDLines<io::Lines<BufReader<T>>> {
        UCDLines(self.0.lines())
    }
}

impl<'a> UCD<&'a str> {
    pub fn ucd_lines(self) -> UCDLines<std::str::Lines<'a>> {
        UCDLines(self.0.lines())
    }
}

pub struct OpenOptions(fs::OpenOptions);

impl OpenOptions {
    pub fn new() -> Self {
        Self(fs::OpenOptions::new())
    }
    pub fn read(&mut self, read: bool) -> &mut Self {
        self.0.read(read);
        self
    }
    pub fn write(&mut self, write: bool) -> &mut Self {
        self.0.write(write);
        self
    }
    pub fn append(&mut self, append: bool) -> &mut Self {
        self.0.append(append);
        self
    }
    pub fn truncate(&mut self, truncate: bool) -> &mut Self {
        self.0.truncate(truncate);
        self
    }
    pub fn create(&mut self, create: bool) -> &mut Self {
        self.0.create(create);
        self
    }
    pub fn create_new(&mut self, create_new: bool) -> &mut Self {
        self.0.create_new(create_new);
        self
    }
    pub fn open<P: AsRef<Path>>(&self, path: P) -> io::Result<UCD<BufReader<File>>> {
        Ok(UCD::new(BufReader::new(self.0.open(path)?)))
    }
}

/// iterator of the row containing the column in the UCD text (ignore blank lines)
pub struct UCDLines<T>(T);

impl<T: BufRead> Iterator for UCDLines<io::Lines<T>> {
    type Item = io::Result<UCDLine<String>>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.0.next()? {
                Ok(line) => {
                    if let Some('#') | None = line.trim_start().chars().next() {
                        continue;
                    } else {
                        return Some(Ok(UCDLine(line)));
                    }
                }
                Err(e) => return Some(Err(e)),
            }
        }
    }
}

impl<'a> Iterator for UCDLines<std::str::Lines<'a>> {
    type Item = UCDLine<&'a str>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let line = self.0.next()?;
            if let Some('#') | None = line.trim_start().chars().next() {
                continue;
            } else {
                return Some(UCDLine(line));
            }
        }
    }
}

/// A non-empty line
pub struct UCDLine<T>(T);

impl<'a> IntoIterator for &'a UCDLine<String> {
    type Item = &'a str;
    type IntoIter = UCDLineIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        UCDLineIter(self.0.split('#').next().unwrap().split(';'))
    }
}

impl<'a> IntoIterator for UCDLine<&'a str> {
    type Item = &'a str;
    type IntoIter = UCDLineIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        UCDLineIter(self.0.split('#').next().unwrap().split(';'))
    }
}

pub struct UCDLineIter<'a>(std::str::Split<'a, char>);

impl<'a> Iterator for UCDLineIter<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        let s = self.0.next()?.trim();
        if s == "" {
            None
        } else {
            Some(s)
        }
    }
}
