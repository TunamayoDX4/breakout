//! テキストのオブジェクト

use std::borrow::Cow;

/// テキストのオブジェクト
pub struct TextBody(Cow<'static, str>);
impl TextBody {
	pub fn new<S: Into<Cow<'static, str>>>(str: S) -> Self { 
		Self(str.into())
	}
	pub fn write<S: Into<Cow<'static, str>>>(&mut self, str: S) {
		self.0 = str.into()
	}
	pub fn get(&self) -> &Cow<'static, str> {
		&self.0
	}
}
impl<T: Into<Cow<'static, str>>> From<T> for TextBody {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}
impl<'a> From<&'a TextBody> for &'a str {
    fn from(value: &'a TextBody) -> Self { &value.0 }
}