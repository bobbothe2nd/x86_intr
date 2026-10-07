use core::range::Range;

use proc_macro::{
    Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree,
};

use crate::internal::{gen_core_asm, gen_unreachable, parse_range};

pub(crate) fn intr_asm_internal(input: TokenStream) -> TokenStream {
    let mut iter = input.into_iter();

    let reg_class = match iter.next() {
        Some(TokenTree::Ident(ident)) => ident,
        Some(tok) => return error!(tok.span(), "expected register class, found `{tok}`"),
        None => return error!(Span::call_site(), "expected register class"),
    };

    let group = match iter.next() {
        Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Brace => group,
        Some(tok) => return error!(tok.span(), "expected `{{}}` register identifier, found `{tok}`"),
        None => return error!(reg_class.span(), "expected `{{...}}` register identifier"),
    };

    let asm_const = match group.stream().into_iter().next() {
        Some(TokenTree::Ident(ident)) => ident,
        Some(tok) => return error!(tok.span(), "expected identifier, found `{tok}`"),
        None => return error!(group.span(), "expected identifier"),
    };

    expect_punct!('=', iter.next());

    let subject_const = match iter.next() {
        Some(TokenTree::Ident(ident)) => ident,
        Some(tok) => return error!(tok.span(), "expected identifier, found `{tok}`"),
        None => return error!(asm_const.span(), "expected identifier"),
    };

    let range = match iter.next() {
        Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Bracket => try_stream!(parse_range(&mut group.stream().into_iter())),
        Some(tok) => return error!(tok.span(), "expected bracketed group, found `{tok}`"),
        None => return error!(Span::call_site(), "expected bracketed group"),
    };

    expect_punct!(',', iter.next());

    gen_intr_asm(
        &reg_class,
        &asm_const,
        &subject_const,
        range,
        iter.collect(),
    )
}

fn gen_intr_asm(
    reg_class: &Ident,
    asm_const: &Ident,
    subject_const: &Ident,
    range: Range<usize>,
    asm: TokenStream,
) -> TokenStream {
    let start_comma = match asm.clone().into_iter().last() {
        Some(TokenTree::Punct(punct)) => punct != ',',
        Some(_) => true,
        None => return error!(Span::call_site(), "expected non-empty asm"),
    };

    let mut arms = TokenStream::new();

    for register in range.iter() {
        let mut literal = Literal::usize_unsuffixed(register);
        literal.set_span(subject_const.span());

        arms.extend([
            TokenTree::Literal(literal.clone()),
            TokenTree::Punct({
                let mut p = Punct::new('=', Spacing::Joint);
                p.set_span(subject_const.span());
                p
            }),
            TokenTree::Punct({
                let mut p = Punct::new('>', Spacing::Alone);
                p.set_span(subject_const.span());
                p
            }),
        ]);

        let mut asm_tokens = asm.clone();

        if start_comma {
            asm_tokens.extend([
                TokenTree::Punct({
                    let mut p = Punct::new(',', Spacing::Alone);
                    p.set_span(asm_const.span());
                    p
                }),
            ]);
        }

        asm_tokens.extend([
            TokenTree::Ident(asm_const.clone()),
            TokenTree::Punct({
                let mut p = Punct::new('=', Spacing::Alone);
                p.set_span(asm_const.span());
                p
            }),
            TokenTree::Ident(Ident::new("const", asm_const.span())),
            TokenTree::Literal(literal),
            TokenTree::Punct({
                let mut p = Punct::new(',', Spacing::Alone);
                p.set_span(asm_const.span());
                p
            }),
            TokenTree::Ident(Ident::new("out", asm_const.span())),
            TokenTree::Group({
                let mut group = Group::new(
                    Delimiter::Parenthesis,
                    {
                        let mut stream = TokenStream::new();

                        let mut string = Literal::string(
                            &alloc::format!("{}{}", reg_class, register)
                        );
                        string.set_span(reg_class.span());

                        stream.extend([TokenTree::Literal(string)]);
                        stream
                    },
                );
                group.set_span(reg_class.span());
                group
            }),
            TokenTree::Ident(Ident::new("_", asm_const.span())),
            TokenTree::Punct({
                let mut p = Punct::new(',', Spacing::Alone);
                p.set_span(asm_const.span());
                p
            }),
        ]);

        arms.extend([
            TokenTree::Group({
                let mut group = Group::new(Delimiter::Brace, gen_core_asm(asm_tokens));
                group.set_span(subject_const.span());
                group
            }),
            TokenTree::Punct({
                let mut p = Punct::new(',', Spacing::Alone);
                p.set_span(asm_const.span());
                p
            }),
        ]);
    }

    arms.extend([
        TokenTree::Ident(Ident::new("_", subject_const.span())),
        TokenTree::Punct({
            let mut p = Punct::new('=', Spacing::Joint);
            p.set_span(subject_const.span());
            p
        }),
        TokenTree::Punct({
            let mut p = Punct::new('>', Spacing::Alone);
            p.set_span(subject_const.span());
            p
        }),
    ]);

    arms.extend(gen_unreachable());

    let mut output = TokenStream::new();

    output.extend([
        TokenTree::Ident(Ident::new("match", subject_const.span())),
        TokenTree::Ident(subject_const.clone()),
    ]);

    let mut body = Group::new(Delimiter::Brace, arms);
    body.set_span(subject_const.span());

    output.extend([TokenTree::Group(body)]);

    output
}
