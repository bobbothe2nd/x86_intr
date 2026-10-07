use core::range::Range;

use alloc::string::ToString;

use proc_macro::{Delimiter, Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree, token_stream::IntoIter};

pub(crate) fn gen_core_asm(
    args: TokenStream,
) -> TokenStream {
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
    let start = match stream.next() {
        Some(TokenTree::Literal(lit)) => lit
            .to_string()
            .parse()
            .map_err(|_| error!(lit.span(), "expected integer literal"))?,
        Some(tok) => return Err(error!(tok.span(), "expected range start")),
        None => return Err(error!(Span::call_site(), "expected range start")),
    };

    match stream.next() {
        Some(TokenTree::Punct(p)) if p == '.' && p.spacing() == Spacing::Joint => {}
        Some(tok) => return Err(error!(tok.span(), "expected `..`")),
        None => return Err(error!(Span::call_site(), "expected `..`")),
    }

    match stream.next() {
        Some(TokenTree::Punct(p)) if p == '.' && p.spacing() == Spacing::Alone => {}
        Some(tok) => return Err(error!(tok.span(), "expected `..`")),
        None => return Err(error!(Span::call_site(), "expected `..`")),
    }

    let end = match stream.next() {
        Some(TokenTree::Literal(lit)) => lit
            .to_string()
            .parse()
            .map_err(|_| error!(lit.span(), "expected integer literal"))?,
        Some(tok) => return Err(error!(tok.span(), "expected range end")),
        None => return Err(error!(Span::call_site(), "expected range end")),
    };

    if start >= end {
        error!(Span::call_site(), "unexpected reverse or empty range");
    }

    Ok(Range { start, end })
}
