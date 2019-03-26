#![crate_type = "proc-macro"]
extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use syn::export::Span;
use syn::parse::{Error, Result};
use syn::{parse_macro_input, Attribute, DeriveInput, Ident, Lit, Meta, MetaNameValue};

#[proc_macro_derive(XbusCommandHandler, attributes(handles))]
pub fn add_handle(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let handler_name = ast.ident;
    let command_name: Ident = get_command(&ast.attrs[0]).unwrap();

    let expanded = quote! {
        impl Handler<#command_name> for xbus::CommandBus {
            type Result = Result<(), ()>;

            fn handle(&mut self, command: #command_name, _ctx: &mut Context<Self>) -> Self::Result {
                #handler_name::handle(command)
            }
        }
    };
    return TokenStream::from(expanded);
}

fn get_command(attr: &Attribute) -> Result<Ident> {
    let meta = match attr.interpret_meta() {
        Some(meta) => meta,
        None => return Err(Error::new(Span::call_site(), "An error...")),
    };

    if meta.name() != "handles" {
        let message = "What would you like me to handle?";
        return Err(Error::new(Span::call_site(), message));
    }

    match meta {
        Meta::NameValue(MetaNameValue {
            lit: Lit::Str(lit_str),
            ..
        }) => lit_str.parse(),
        _ => {
            let error_span = attr.bracket_token.span;
            let message = "expected #[handles = \"...\"]";
            Err(Error::new(error_span, message))
        }
    }
}

#[proc_macro_derive(XbusCommand, attributes(handles))]
pub fn add_command_message(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let command_name = ast.ident;

    let expanded = quote! {
        impl actix::Message for #command_name {
            type Result = std::result::Result<(), ()>;
        }
    };
    return TokenStream::from(expanded);
}
