extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr, ItemStruct};

#[proc_macro_attribute]
pub fn packet_id(args: TokenStream, input: TokenStream) -> TokenStream {
    let strukt: ItemStruct = parse_macro_input!(input as ItemStruct);
    let id: Expr = parse_macro_input!(args as Expr);
    let ident = strukt.ident.clone();
    let out = quote! {
        #strukt

        impl crate::packets::PacketId for #ident {
            const ID: u8 = #id;
        }
    };
    out.into()
}
