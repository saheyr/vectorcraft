//! Temporary Android stand-in for `arboard`: reports "no clipboard" for now.
use std::borrow::Cow;
use std::fmt;

#[derive(Debug)]
pub struct Error;
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "system clipboard is not available on Android yet")
    }
}
impl std::error::Error for Error {}

pub struct ImageData<'a> {
    pub width: usize,
    pub height: usize,
    pub bytes: Cow<'a, [u8]>,
}

pub struct Clipboard;
impl Clipboard {
    pub fn new() -> Result<Self, Error> { Err(Error) }
    pub fn get_text(&mut self) -> Result<String, Error> { Err(Error) }
    pub fn set_text<'a, T: Into<Cow<'a, str>>>(&mut self, _t: T) -> Result<(), Error> { Err(Error) }
    pub fn get_image(&mut self) -> Result<ImageData<'static>, Error> { Err(Error) }
    pub fn set_image(&mut self, _i: ImageData<'_>) -> Result<(), Error> { Err(Error) }
    pub fn clear(&mut self) -> Result<(), Error> { Err(Error) }
}

use std::marker::PhantomData;
use std::path::PathBuf;

pub struct Get<'a>(PhantomData<&'a mut Clipboard>);
impl<'a> Get<'a> {
    pub fn text(self) -> Result<String, Error> { Err(Error) }
    pub fn html(self) -> Result<String, Error> { Err(Error) }
    pub fn image(self) -> Result<ImageData<'static>, Error> { Err(Error) }
    pub fn file_list(self) -> Result<Vec<PathBuf>, Error> { Err(Error) }
}

pub struct Set<'a>(PhantomData<&'a mut Clipboard>);
impl<'a> Set<'a> {
    pub fn text<T: Into<Cow<'a, str>>>(self, _t: T) -> Result<(), Error> { Err(Error) }
    pub fn html<T: Into<Cow<'a, str>>>(self, _h: T, _alt: Option<T>) -> Result<(), Error> { Err(Error) }
    pub fn image(self, _i: ImageData<'a>) -> Result<(), Error> { Err(Error) }
}

impl Clipboard {
    pub fn get(&mut self) -> Get<'_> { Get(PhantomData) }
    pub fn set(&mut self) -> Set<'_> { Set(PhantomData) }
}
