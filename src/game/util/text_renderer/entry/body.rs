//! テキストのオブジェクト

use std::borrow::Borrow;

/// テキストのオブジェクト
pub struct TextBody(TextBodyInner);
impl TextBody {
	pub fn from_direct(str: std::borrow::Cow<'static, str>) -> Self {
		str.into()
	}
	pub fn from_polyobj(obj: Box<dyn ToString>) -> Self {
		obj.into()
	}
	pub fn direct(&self) -> Option<&std::borrow::Cow<'static, str>> {
		self.0.direct()
	}
	pub fn direct_mut(&mut self) -> Option<&mut std::borrow::Cow<'static, str>> {
		self.0.direct_mut()
	}
	pub fn poly_obj(&self) -> Option<&Box<dyn ToString>> {
		self.0.poly_obj()
	}
	pub fn poly_obj_mut(&mut self) -> Option<&mut Box<dyn ToString>> {
		self.0.poly_obj_mut()
	}
}
impl From<std::borrow::Cow<'static, str>> for TextBody {
    fn from(value: std::borrow::Cow<'static, str>) -> Self {
        Self(TextBodyInner::from_direct(value))
    }
}
impl From<Box<dyn ToString>> for TextBody {
    fn from(value: Box<dyn ToString>) -> Self {
        Self(TextBodyInner::from_polyobj(value))
    }
}
impl<'a> From<&'a mut TextBody> for &'a str {
    fn from(value: &'a mut TextBody) -> Self { value.into() }
}

/// テキストのオブジェクトの内部型
enum TextBodyInner {
	Direct(std::borrow::Cow<'static, str>), 
	PolyObj{
		obj: Box<dyn ToString>, 
		gen: Option<String>, 
	}, 
}
impl TextBodyInner {
	pub fn from_direct(str: std::borrow::Cow<'static, str>) -> Self {
		Self::Direct(str)
	}
	pub fn from_polyobj(obj: Box<dyn ToString>) -> Self {
		Self::PolyObj{
			obj, 
			gen: Default::default(), 
		}
	}
	pub fn direct(&self) -> Option<&std::borrow::Cow<'static, str>> {
		if let Self::Direct(o) = self { Some(o) }
		else { None }
	}
	pub fn direct_mut(&mut self) -> Option<&mut std::borrow::Cow<'static, str>> {
		if let Self::Direct(o) = self { Some(o) }
		else { None }
	}
	pub fn poly_obj(&self) -> Option<&Box<dyn ToString>> {
		if let Self::PolyObj { obj, .. } = self {
			Some(obj)
		} else {
			None
		}
	}
	pub fn poly_obj_mut(&mut self) -> Option<&mut Box<dyn ToString>> {
		if let Self::PolyObj { obj, .. } = self {
			Some(obj)
		} else {
			None
		}
	}
}
impl<'a> From<&'a mut TextBodyInner> for &'a str {
    fn from(value: &'a mut TextBodyInner) -> Self {
        match value {
            TextBodyInner::Direct(str) => str.borrow(),
            TextBodyInner::PolyObj{
                obj,
                gen,
            } => {
				*gen = Some(obj.to_string());
				gen.unwrap().as_str()
			},
        }
    }
}