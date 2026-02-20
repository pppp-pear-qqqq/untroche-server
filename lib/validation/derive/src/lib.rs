mod types;

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

use crate::types::{Access, Info};

#[proc_macro_derive(Validation, attributes(validation))]
pub fn derive(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	let name = &input.ident;
	match input.data {
		syn::Data::Struct(data) => {
			let infos = data.fields.iter().enumerate().filter_map(|(idx, f)| Info::from_syn(f, idx));
			let checks: Vec<_> = infos
				.map(|info| {
					let access = match info.access {
						Access::Ident(ident) => quote! {self.#ident},
						Access::Index(ref index) => quote! {self.#index},
					};
					Info::gen_quote(info, access)
				})
				.collect();
			TokenStream::from(quote! {
				impl Validation for #name {
					fn validate(&self) -> Result<(), String> {
						#(#checks)*
						Ok(())
					}
				}
			})
		}
		syn::Data::Enum(data) => {
			let mut matches = Vec::new();
			for v in data.variants {
				let infos = v.fields.iter().enumerate().filter_map(|(idx, f)| Info::from_syn(f, idx));
				let (checks, idents): (Vec<_>, Vec<_>) = infos.map(Info::gen_quote_with_ident).unzip();
				if !checks.is_empty() {
					let ident = &v.ident;
					match v.fields {
						syn::Fields::Named(_) => {
							matches.push(quote! {
								Self::#ident{#(#idents),*} => {
									#(#checks)*
								}
							});
						}
						syn::Fields::Unnamed(_) => {
							matches.push(quote! {
								Self::#ident(#(#idents),*) => {
									#(#checks)*
								}
							});
						}
						syn::Fields::Unit => (),
					}
				}
			}
			TokenStream::from(quote! {
				impl Validation for #name {
					fn validate(&self) -> Result<(), String> {
						match self {
							#(#matches),*
							_ => (),
						}
						Ok(())
					}
				}
			})
		}
		syn::Data::Union(_) => panic!("union not supported"),
	}
}
