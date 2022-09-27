use proc_macro::TokenStream;
use quote::{ quote, ToTokens };
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    parse_macro_input, Expr, LitBool,
};

mod keyword {
    use syn::custom_keyword;

    // Expect
    custom_keyword!(to);
    custom_keyword!(not);

    // Assertion::Equality
    custom_keyword!(equal);

    custom_keyword!(be);

    // Assertion::Result
    custom_keyword!(Ok);
    custom_keyword!(Err);
    
    // Assertion::Result
    custom_keyword!(Some);
    custom_keyword!(None);

    // Assertion::Iter
    custom_keyword!(empty);

    // Assertion::fn
    custom_keyword!(succeed);
    custom_keyword!(panic);
}

#[allow(dead_code)]
#[non_exhaustive]
enum Assertion {
    Equality { kw: keyword::equal, other: Box<Expr> },

    ResultOk { kw: keyword::be, ok: keyword::Ok },
    ResultErr { kw: keyword::be, err: keyword::Err },

    OptionSome { kw: keyword::be, some: keyword::Some },
    OptionNone { kw: keyword::be, none: keyword::None },

    Bool { kw: keyword::be, literal: LitBool },

    Empty { kw: keyword::be, empty: keyword::empty },

    FnSucceed { kw: keyword::succeed },
    FnPanic { kw: keyword::panic },
}

impl Assertion {
    fn assert_type(&self, value: &Expr) -> impl ToTokens {
        match self {
            Assertion::ResultOk { .. } | Assertion::ResultErr { .. } => quote! {
                let _assert_result: Result<_, _> = #value;
            },

            Assertion::OptionSome { .. } | Assertion::OptionNone { .. } => quote! {
                let _assert_option: Option<_> = #value;
            },

            Assertion::Bool { .. } => quote! {
                let _assert_bool: bool = #value;
            },

            _ => quote! { }
        }
    }

    fn tokens(&self, value: Expr, not: bool) -> TokenStream {
        let assert_type = self.assert_type(&value);

        let escaped_value = value.to_token_stream()
            .to_string()
            .replace(|c: char| !c.is_ascii_alphanumeric(), "_");

        let escaped_value = syn::parse_str::<syn::Ident>(&escaped_value).unwrap_or_else(|_| syn::parse_str::<syn::Ident>("_unused").unwrap());

        let assertion = match self {
            Assertion::Equality { other, .. } if !not => quote! {
                assert_eq!(#value, #other);
            },
            Assertion::Equality { other, .. } if not => quote! {
                assert_ne!(#value, #other);
            },

            Assertion::ResultOk { .. } if !not => quote! {
                assert!(#value.is_ok());
            },
            Assertion::ResultErr { .. } if not => quote! {
                assert!(#value.is_ok());
            },
            
            Assertion::ResultErr { .. } if !not => quote! {
                assert!(#value.is_err());
            },
            Assertion::ResultOk { .. } if not => quote! {
                assert!(#value.is_err());
            },

            Assertion::OptionSome { .. } if !not => quote! {
                assert!(#value.is_some());
            },
            Assertion::OptionNone { .. } if not => quote! {
                assert!(#value.is_some());
            },
            
            Assertion::OptionNone { .. } if !not => quote! {
                assert!(#value.is_none());
            },
            Assertion::OptionSome { .. } if not => quote! {
                assert!(#value.is_none());
            },

            Assertion::Bool { literal, .. } if !not => quote! {
                assert_eq!(#value, #literal);
            },
            Assertion::Bool { literal, .. } if not => quote! {
                assert_ne!(#value, #literal);
            },

            Assertion::Empty { .. } if !not => quote! {
                assert!(#value.is_empty());
            },
            Assertion::Empty { .. } if not => quote! {
                assert!(!#value.is_empty());
            },

            Assertion::FnSucceed { .. } if !not => quote! {
                fn #escaped_value() { #value; }

                assert!(std::thread::spawn(#escaped_value).join().is_ok());
            },
            Assertion::FnPanic { .. } if not => quote! {
                fn #escaped_value() { #value; }

                assert!(std::thread::spawn(#escaped_value).join().is_ok());
            },

            Assertion::FnPanic { .. } if !not => quote! {
                fn #escaped_value() { #value; }

                assert!(std::thread::spawn(#escaped_value).join().is_err());
            },
            Assertion::FnSucceed { .. } if not => quote! {
                fn #escaped_value() { #value; }

                assert!(std::thread::spawn(#escaped_value).join().is_err());
            },
            _ => unimplemented!()
        };

        let expanded: TokenStream = quote! {
            {
                #assert_type
                #assertion
            }
        }.into();
        
        // eprintln!("{}\n", expanded);

        expanded
    }

    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(keyword::equal) {
            Ok(Assertion::Equality {
                kw: input.parse()?,
                other: input.parse()?
            })
        } else if input.peek(keyword::be) {
            let kw = input.parse()?;
            let next = input.lookahead1();

            if next.peek(keyword::Ok) {
                Ok(Assertion::ResultOk {
                    kw,
                    ok: input.parse()?
                })
            } else if next.peek(keyword::Err) {
                Ok(Assertion::ResultErr {
                    kw,
                    err: input.parse()?
                })
            } else if next.peek(keyword::Some) {
                Ok(Assertion::OptionSome {
                    kw,
                    some: input.parse()?
                })
            } else if next.peek(keyword::None) {
                Ok(Assertion::OptionNone {
                    kw,
                    none: input.parse()?
                })
            } else if next.peek(LitBool) {
                Ok(Assertion::Bool {
                    kw,
                    literal: input.parse()?
                })
            } else if next.peek(keyword::empty) {
                Ok(Assertion::Empty {
                    kw,
                    empty: input.parse()?
                })
            } else {
                Err(next.error())
            }
        } else if input.peek(keyword::succeed) {
            Ok(Assertion::FnSucceed { kw: input.parse()? })
        } else if input.peek(keyword::panic) {
            Ok(Assertion::FnPanic { kw: input.parse()? })
        } else {
            Err(input.lookahead1().error())
        }
    }
}

struct Expect {
    value: Expr,
    not: bool,
    kind: Assertion,
}

impl Expect {
    fn tokens(self) -> TokenStream {
        self.kind.tokens(self.value, self.not)
    }
}

impl Parse for Expect {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let unwrap = if input.peek(keyword::Ok) {
            input.parse::<keyword::Ok>()?;
            true
        } else if input.peek(keyword::Some) {
            input.parse::<keyword::Some>()?;
            true
        } else {
            false
        };
        
        let unwrap_err = if input.peek(keyword::Err) {
            input.parse::<keyword::Err>()?;
            true
        } else {
            false
        };

        let value;
        parenthesized!(value in input);
        let mut value = value.parse()?;

        if unwrap {
            value = Expr::Verbatim(quote! { #value.unwrap() });
        } else if unwrap_err {
            value = Expr::Verbatim(quote! { #value.unwrap_err() });
        }

        input.parse::<keyword::to>()?;

        let not_or_kind = input.lookahead1();
        let not = not_or_kind.peek(keyword::not);

        if not { input.parse::<keyword::not>()?; }

        let kind = Assertion::parse(input)?;

        Ok(Expect { value, not, kind })
    }
}

#[proc_macro]
pub fn expect(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as Expect).tokens()
}

