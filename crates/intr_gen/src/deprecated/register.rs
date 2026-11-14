use core::range::Range;

use proc_macro::{Ident, Span, TokenStream, TokenTree};

pub(crate) fn register_internal(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = match parse_register(attr) {
        Ok(args) => args,
        Err(err) => return err,
    };

    todo!()
}

#[derive(Debug)]
struct RegisterArgs {
    size: usize,
    class: RegisterClass,
    mask: bool,
    range: Range<usize>,
}

#[derive(Debug)]
enum RegisterClass {
    Mm,
    Xmm,
    Ymm,
    Zmm,
    K,
    Tmm,
}

fn parse_register(input: TokenStream) -> Result<RegisterArgs, TokenStream> {
    let mut tokens = input.into_iter();

    let mut size = None;
    let mut class = None;
    let mut mask = None;
    let mut range = None;

    while let Some(token) = tokens.next() {
        let key = match token {
            TokenTree::Ident(ident) => ident,
            other => {
                return Err(error!(
                    other.span(),
                    "expected register attribute name, found {other}"
                ));
            }
        };

        expect_punct!('=', = Err = tokens.next());

        match key.to_string().as_str() {
            "size" => {
                if size.is_some() {
                    return Err(error!(
                        key.span(),
                        "duplicate `size` attribute"
                    ));
                }

                let token = match tokens.next() {
                    Some(token) => token,
                    None => {
                        return Err(error!(
                            key.span(),
                            "expected integer literal after `size =`"
                        ));
                    }
                };

                let literal = match token {
                    TokenTree::Literal(literal) => literal,
                    other => {
                        return Err(error!(
                            other.span(),
                            "expected integer literal for `size`, found {other}"
                        ));
                    }
                };

                let value = match literal.to_string().parse::<usize>() {
                    Ok(value) => value,
                    Err(_) => {
                        return Err(error!(
                            literal.span(),
                            "invalid integer literal for `size`"
                        ));
                    }
                };

                size = Some(value);
            }

            "class" => {
                if class.is_some() {
                    return Err(error!(
                        key.span(),
                        "duplicate `class` attribute"
                    ));
                }

                let token = match tokens.next() {
                    Some(token) => token,
                    None => {
                        return Err(error!(
                            key.span(),
                            "expected register class after `class =`"
                        ));
                    }
                };

                let ident = match token {
                    TokenTree::Ident(ident) => ident,
                    other => {
                        return Err(error!(
                            other.span(),
                            "expected register class, found {other}"
                        ));
                    }
                };

                let value = match ident.to_string().as_str() {
                    "mm" => RegisterClass::Mm,
                    "xmm" => RegisterClass::Xmm,
                    "ymm" => RegisterClass::Ymm,
                    "zmm" => RegisterClass::Zmm,
                    "k" => RegisterClass::K,
                    "tmm" => RegisterClass::Tmm,
                    _ => {
                        return Err(error!(
                            ident.span(),
                            "unknown register class `{ident}`"
                        ));
                    }
                };

                class = Some(value);
            }

            "mask" => {
                if mask.is_some() {
                    return Err(error!(
                        key.span(),
                        "duplicate `mask` attribute"
                    ));
                }

                let token = match tokens.next() {
                    Some(token) => token,
                    None => {
                        return Err(error!(
                            key.span(),
                            "expected `true` or `false` after `mask =`"
                        ));
                    }
                };

                let ident = match token {
                    TokenTree::Ident(ident) => ident,
                    other => {
                        return Err(error!(
                            other.span(),
                            "expected `true` or `false` for `mask`, found {other}"
                        ));
                    }
                };

                mask = match ident.to_string().as_str() {
                    "true" => Some(true),
                    "false" => Some(false),
                    _ => {
                        return Err(error!(
                            ident.span(),
                            "expected `true` or `false` for `mask`, found `{ident}`"
                        ));
                    }
                };
            }

            "range" => {
                if range.is_some() {
                    return Err(error!(
                        key.span(),
                        "duplicate `range` attribute"
                    ));
                }

                range = Some(crate::internal::parse_range(&mut tokens)?);
            }

            _ => {
                return Err(error!(
                    key.span(),
                    "unknown `register` attribute `{key}`"
                ));
            }
        }

        match tokens.next() {
            None => break,
            Some(TokenTree::Punct(punct)) if punct.as_char() == ',' => {}
            Some(token) => {
                return Err(error!(
                    token.span(),
                    "expected `,` or end of attribute"
                ));
            }
        }
    }

    let size = match size {
        Some(size) => size,
        None => {
            return Err(error!(
                Span::call_site(),
                "missing required `size` attribute"
            ));
        }
    };

    let class = match class {
        Some(class) => class,
        None => {
            return Err(error!(
                Span::call_site(),
                "missing required `class` attribute"
            ));
        }
    };

    let range = match range {
        Some(range) => range,
        None => {
            return Err(error!(
                Span::call_site(),
                "missing required `range` attribute"
            ));
        }
    };

    Ok(RegisterArgs {
        size,
        class,
        mask: mask.unwrap_or_default(),
        range,
    })
}
