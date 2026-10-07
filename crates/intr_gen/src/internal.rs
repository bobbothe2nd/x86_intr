use core::range::Range;

use proc_macro::{
    token_stream::IntoIter, Delimiter, Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree,
};

pub(crate) fn gen_core_asm(args: TokenStream) -> TokenStream {
    let mut asm = TokenStream::new();

    asm.extend([
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new("core", Span::call_site())),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new("arch", Span::call_site())),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new("asm", Span::call_site())),
        TokenTree::Punct(Punct::new('!', Spacing::Alone)),
        TokenTree::Group(Group::new(Delimiter::Parenthesis, args)),
    ]);

    asm
}

pub(crate) fn gen_unreachable() -> TokenStream {
    let mut asm = TokenStream::new();

    asm.extend([
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new("core", Span::call_site())),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new("hint", Span::call_site())),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new("unreachable_unchecked", Span::call_site())),
        TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::new())),
    ]);

    asm
}

pub(crate) fn parse_range(stream: &mut IntoIter) -> Result<Range<usize>, TokenStream> {
    fn parse_end(stream: &mut IntoIter, depth: usize) -> Result<(Span, usize), TokenStream> {
        if depth > 1 {
            return Err(error!(Span::call_site(), "expected range, found `==`"));
        }

        match stream.next() {
            Some(TokenTree::Literal(lit)) => Ok((
                lit.span(),
                lit.to_string()
                    .parse::<usize>()
                    .map_err(|_| error!(lit.span(), "expected integer literal"))?
                    + depth,
            )),
            Some(TokenTree::Punct(punct)) if punct == '=' => parse_end(stream, depth + 1),
            Some(tok) => Err(error!(tok.span(), "expected range end")),
            None => Err(error!(Span::call_site(), "expected range end")),
        }
    }

    let (start_span, start) = match stream.next() {
        Some(TokenTree::Literal(lit)) => (
            lit.span(),
            lit.to_string()
                .parse()
                .map_err(|_| error!(lit.span(), "expected integer literal"))?,
        ),
        Some(tok) => return Err(error!(tok.span(), "expected range start")),
        None => return Err(error!(Span::call_site(), "expected range start")),
    };

    match stream.next() {
        Some(TokenTree::Punct(p)) if p == '.' && p.spacing() == Spacing::Joint => {}
        Some(tok) => return Err(error!(tok.span(), "expected `..`, found `{tok}`")),
        None => return Err(error!(Span::call_site(), "expected `..`")),
    }

    match stream.next() {
        Some(TokenTree::Punct(p)) if p == '.' => {}
        Some(tok) => return Err(error!(tok.span(), "expected `..`, found `{tok}`")),
        None => return Err(error!(Span::call_site(), "expected `..`")),
    }

    let (end_span, end) = parse_end(stream, 0)?;

    if start >= end {
        return Err(error!(
            end_span.located_at(start_span),
            "unexpected reverse or empty range"
        ));
    }

    Ok(Range { start, end })
}
