use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    parse_macro_input, Expr,
};

mod keyword {
    use syn::custom_keyword;

    // Expect
    custom_keyword!(to);
    custom_keyword!(not);

    // Assertion::Equality
    custom_keyword!(equal);

    // Assertion::Result
    custom_keyword!(be);
    custom_keyword!(ok);
    custom_keyword!(err);
}

#[allow(dead_code)]
#[non_exhaustive]
enum Assertion {
    Equality { kw: keyword::equal, other: Box<Expr> },
    ResultOk { kw: keyword::be, ok: keyword::ok },
    ResultErr { kw: keyword::be, err: keyword::err },
}

impl Assertion {
    fn tokens(&self, value: Expr, not: bool) -> TokenStream {
        let expanded = match self {
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

            _ => unimplemented!()
        };

        TokenStream::from(expanded)
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
            let ok_or_err = input.lookahead1();

            if ok_or_err.peek(keyword::ok) {
                Ok(Assertion::ResultOk {
                    kw,
                    ok: input.parse()?
                })
            } else if ok_or_err.peek(keyword::err) {
                Ok(Assertion::ResultErr {
                    kw,
                    err: input.parse()?
                })
            } else {
                Err(ok_or_err.error())
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
        let value;
        parenthesized!(value in input);
        let value = value.parse()?;

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
