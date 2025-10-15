use proc_macro2::{Ident, TokenStream};

use super::parsers::ParamType;

pub mod collectors;
pub mod generate;
pub mod parse;

#[derive(Debug)]
pub struct PropDesc {
    pub ident: Ident,

    pub name_literal: TokenStream,
    pub name_ru_literal: TokenStream,

    pub name_const: TokenStream,
    pub name_ru_const: TokenStream,
    pub name_slice_const: TokenStream,
    pub name_ru_slice_const: TokenStream,

    pub readable: bool,
    pub writable: bool,
    pub ty: ParamType,
}
