use proc_macro2::Span;
use quote::quote;
use syn::{GenericArgument, PathArguments, Type};

pub struct Info<'a> {
	pub(crate) access: Access<'a>,
	optional: bool,
	validation: Params,
}
impl<'a> Info<'a> {
	pub fn from_syn(field: &'a syn::Field, idx: usize) -> Option<Self> {
		let optional = is_optional_string(&field.ty)?;
		let params = Params::parse(&field.attrs).unwrap(); // この関数はfilter_mapに渡すのでこの場でパニックさせた方が話が早い
		if params.min.is_none() && params.max.is_none() {
			return None;
		}
		let access = match field.ident.as_ref() {
			Some(ident) => Access::Ident(ident),
			None => Access::Index(syn::Index::from(idx)),
		};
		Some(Self { access, optional, validation: params })
	}

	pub fn gen_quote(self, access: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
		let name = self.validation.name.unwrap_or_else(|| match self.access {
			Access::Ident(ident) => ident.to_string(),
			Access::Index(index) => format!("フィールド{}", index.index),
		});
		let check = match (self.validation.min, self.validation.max) {
			(Some(min), Some(max)) => quote! {
				if len < #min || len > #max {
					return Err(format!("{} は {}文字以上 {}文字以下 で設定してください", #name, #min, #max))
				}
			},
			(Some(min), None) => quote! {
				if len < #min {
					return Err(format!("{} は {}文字以上 で設定してください", #name, #min))
				}
			},
			(None, Some(max)) => quote! {
				if len > #max {
					return Err(format!("{} は {}文字以下 で設定してください", #name, #max))
				}
			},
			(None, None) => unreachable!(),
		};
		if self.optional {
			quote! {
				if let Some(s) = #access {
					let len = s.chars().count();
					#check
				}
			}
		} else {
			quote! {
				let len = #access.chars().count();
				#check
			}
		}
	}
	pub fn gen_quote_with_ident(self) -> (proc_macro2::TokenStream, syn::Ident) {
		let ident = match self.access {
			Access::Ident(ident) => ident.clone(),
			Access::Index(ref index) => syn::Ident::new(&format!("v{}", index.index), Span::call_site()),
		};
		(self.gen_quote(quote! {#ident}), ident)
	}
}

pub enum Access<'a> {
	Ident(&'a syn::Ident),
	Index(syn::Index),
}

#[derive(Default)]
struct Params {
	name: Option<String>,
	min: Option<usize>,
	max: Option<usize>,
}
impl Params {
	fn parse(attrs: &Vec<syn::Attribute>) -> Result<Self, syn::Error> {
		let mut params = Self::default();
		for attr in attrs {
			if attr.path().is_ident("validation") {
				attr.parse_nested_meta(|meta| {
					if meta.path.is_ident("name") {
						params.name = Some(meta.value()?.parse::<syn::LitStr>()?.value());
					}
					if meta.path.is_ident("min") {
						params.min = Some(meta.value()?.parse::<syn::LitInt>()?.base10_parse()?);
					}
					if meta.path.is_ident("max") {
						params.max = Some(meta.value()?.parse::<syn::LitInt>()?.base10_parse()?);
					}
					Ok(())
				})
				.unwrap();
			}
		}
		Ok(params)
	}
}

fn is_optional_string(ty: &Type) -> Option<bool> {
	match ty {
		Type::Path(p) if p.path.is_ident("String") => Some(false),
		Type::Path(p) if p.path.is_ident("Option") => {
			let args = &p.path.segments.last()?.arguments;
			if let PathArguments::AngleBracketed(a) = args {
				if let GenericArgument::Type(Type::Path(inner)) = &a.args[0] {
					if inner.path.is_ident("String") {
						return Some(true);
					}
				}
			}
			None
		}
		_ => None,
	}
}
