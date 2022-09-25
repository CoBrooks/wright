use proc_macro::TokenStream;
use quote::{ quote, TokenStreamExt };
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    parse_macro_input, Expr, Token, LitBool
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
}

impl Assertion {
    fn tokens(&self, value: Expr, not: bool) -> TokenStream {
        let mut tokens = proc_macro2::TokenStream::new();

        // Ensure value is a Result before checking is_ok / is_err
        if let Assertion::ResultOk { .. } | Assertion::ResultErr { .. } = self { 
            let assert_result = quote! {
                let _assertResult: Result<_, _> = #value;
            }.into_iter();

            tokens.append_all(assert_result);
        }

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

            _ => unimplemented!()
        }.into_iter();

        tokens.append_all(assertion);

        TokenStream::from(tokens)
    }

    fn parse(input: ParseStream) -> syn::Result<Self> {
        let start = input.lookahead1();

        if start.peek(keyword::equal) {
            Ok(Assertion::Equality {
                kw: input.parse()?,
                other: input.parse()?
            })
        } else if start.peek(keyword::be) {
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
            } else {
                Err(next.error())
            }
        } else {
            Err(start.error())
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

