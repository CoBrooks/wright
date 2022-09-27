use proc_macro::TokenStream;
use quote::{ quote, ToTokens };
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    parse_macro_input, Expr, LitBool
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

trait Assert {
    fn parse(input: ParseStream) -> syn::Result<Self> where Self: Sized;
    fn tokens(&self, lhs: Expr, inverse: bool) -> TokenStream;
}

struct Equal {
    rhs: Expr
}

impl Assert for Equal {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<keyword::equal>()?;

        let rhs = input.parse()?;

        Ok(Equal { rhs })
    }

    fn tokens(&self, lhs: Expr, inverse: bool) -> TokenStream {
        let Equal { rhs } = self;

        match inverse {
            false => quote! {
                assert_eq!(#lhs, #rhs);
            },
            true => quote! {
                assert_ne!(#lhs, #rhs);
            },
        }.into()
    }
}

#[non_exhaustive]
enum IdentityType {
    Ok,
    Err,
    Some,
    None,
    True,
    False,
    Empty
}

struct Identity {
    rhs: IdentityType
}

impl Assert for Identity {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<keyword::be>()?;

        let ident = input.parse::<proc_macro2::TokenStream>()?;
        let rhs = match ident.to_string().as_str() {
            "Ok" => IdentityType::Ok,
            "Err" => IdentityType::Err,
            "Some" => IdentityType::Some,
            "None" => IdentityType::None,
            "true" => IdentityType::True,
            "false" => IdentityType::False,
            "empty" => IdentityType::Empty,
            _ => panic!("Unrecognized identity `{ident}`")
        };

        Ok(Identity { rhs })
    }

    fn tokens(&self, lhs: Expr, inverse: bool) -> TokenStream {
        let Identity { rhs } = self;

        let assert_type = match rhs {
            IdentityType::Ok | IdentityType::Err => quote! {
                let _assert_result: Result<_, _> = #lhs;
            },
            IdentityType::Some | IdentityType::None => quote! {
                let _assert_option: Option<_> = #lhs;
            },
            IdentityType::True | IdentityType::False => quote! {
                let _assert_bool: bool = #lhs;
            },
            _ => quote! { }
        };

        let tokens = match inverse {
            false => match rhs {
                IdentityType::Ok => quote! { assert!(#lhs.is_ok()) },
                IdentityType::Err => quote! { assert!(#lhs.is_err()) },
                IdentityType::Some => quote! { assert!(#lhs.is_some()) },
                IdentityType::None => quote! { assert!(#lhs.is_none()) },
                IdentityType::True => quote! { assert!(#lhs) },
                IdentityType::False => quote! { assert!(!#lhs) },
                IdentityType::Empty => quote! { assert!(#lhs.is_empty()) },
            },
            true => match rhs {
                IdentityType::Ok => quote! { assert!(#lhs.is_err()) },
                IdentityType::Err => quote! { assert!(#lhs.is_ok()) },
                IdentityType::Some => quote! { assert!(#lhs.is_none()) },
                IdentityType::None => quote! { assert!(#lhs.is_some()) },
                IdentityType::True => quote! { assert!(!#lhs) },
                IdentityType::False => quote! { assert!(#lhs) },
                IdentityType::Empty => quote! { assert!(!#lhs.is_empty()) },
            },
        };

        quote! { #assert_type #tokens }.into()
    }
}

enum FnResult {
    Succeed,
    Panic
}

struct CallFn {
    rhs: FnResult
}

impl Assert for CallFn {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(keyword::succeed) {
            input.parse::<keyword::succeed>()?;

            Ok(CallFn { rhs: FnResult::Succeed })
        } else if input.peek(keyword::panic) {
            input.parse::<keyword::panic>()?;
            
            Ok(CallFn { rhs: FnResult::Panic })
        } else {
            Err(input.lookahead1().error())
        }
    }

    fn tokens(&self, lhs: Expr, inverse: bool) -> TokenStream {
        let CallFn { rhs } = self;

        match inverse {
            false => match rhs {
                FnResult::Succeed => quote! {
                    assert!(std::thread::spawn(move || #lhs).join().is_ok());
                },
                FnResult::Panic => quote! {
                    assert!(std::thread::spawn(move || #lhs).join().is_err());
                },
            },
            true => match rhs {
                FnResult::Succeed => quote! {
                    assert!(std::thread::spawn(move || #lhs).join().is_err());
                },
                FnResult::Panic => quote! {
                    assert!(std::thread::spawn(move || #lhs).join().is_ok());
                },
            },
        }.into()
    }
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
    fn parse(input: ParseStream) -> syn::Result<Box<dyn Assert>> {
        if input.peek(keyword::equal) {
            Ok(Box::new(Equal::parse(input)?))
        } else if input.peek(keyword::be) {
            Ok(Box::new(Identity::parse(input)?))
        } else if input.peek(keyword::succeed) || input.peek(keyword::panic) {
            Ok(Box::new(CallFn::parse(input)?))
        } else {
            unimplemented!("{input}")
        }
    }
}

struct Expect {
    value: Expr,
    not: bool,
    kind: Box<dyn Assert>,
}

impl Expect {
    fn tokens(self) -> TokenStream {
        let Expect { value, not, kind } = self;

        let tokens: proc_macro2::TokenStream = kind.tokens(value, not).into();

        // Create new scope for assertions to minimize naming conflicts
        quote! { 
            { #tokens }
        }.into()
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

