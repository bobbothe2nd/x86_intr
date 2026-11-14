use core::range::Range;

use proc_macro::{
    Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree,
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

pub(crate) enum RegArg {
    Fmt(Box<str>),
    Reg(Box<str>),
    Imm(Literal),
}

impl RegArg {
    fn display(&self, att: bool) -> String {
        match self {
            Self::Fmt(reg) => reg.to_string(),
            Self::Reg(reg) => if att {
                format!("%{reg}")
            } else {
                reg.to_string()
            },
            Self::Imm(_lit) => todo!(),
        }
    }
}

pub(crate) fn mov(output: &mut TokenStream, src: RegArg, dst: RegArg, att: bool) {
    let instr = if att {
        format!("mov {}, {}", src.display(att), dst.display(att))
    } else {
        format!("mov {}, {}", dst.display(att), src.display(att))
    };

    output.extend([Literal::string(&instr)]);
}

pub(crate) fn parse_range<I: Iterator<Item = TokenTree>>(stream: &mut I) -> Result<Range<usize>, TokenStream> {
    fn parse_end<I: Iterator<Item = TokenTree>>(stream: &mut I, depth: usize) -> Result<(Span, usize), TokenStream> {
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
            Some(tok) => Err(error!(tok.span(), "expected range end, found `{tok}`")),
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
        Some(tok) => return Err(error!(tok.span(), "expected range start, found `{tok}`")),
        None => return Err(error!(Span::call_site(), "expected range start")),
    };

    match stream.next() {
        Some(TokenTree::Punct(punct)) if punct == '.' && punct.spacing() == Spacing::Joint => {}
        Some(tok) => return Err(error!(tok.span(), "expected `..`, found `{tok}`")),
        None => return Err(error!(Span::call_site(), "expected `..`")),
    }

    match stream.next() {
        Some(TokenTree::Punct(punct)) if punct == '.' => {}
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

pub(crate) fn forward_expr_to_comma<I: Iterator<Item = TokenTree>>(stream: &mut I) -> Result<TokenStream, TokenStream> {
    let mut output = TokenStream::new();

    let mut depth = 0;
    let mut turbofish = 0;

    loop {
        let tok = stream.next();

        match &tok {
            Some(TokenTree::Punct(punct)) if *punct == ':' => turbofish += 1,
            Some(TokenTree::Punct(punct)) if *punct == '<' && turbofish == 2 => depth += 1,
            Some(TokenTree::Punct(punct)) if *punct == '>' && turbofish == 2 => {
                depth -= 1;

                if depth == 0 {
                    turbofish = 0;
                }
            },
            Some(TokenTree::Punct(punct)) if *punct == ',' && depth == 0 => return Ok(output),
            None => return Ok(output),
            _ => if depth == 0 { turbofish = 0 },
        }

        output.extend([tok.unwrap()]);
    }
}

pub(crate) fn parse_string_literal(lit: &Literal) -> Result<Box<str>, TokenStream> {
    let lit_str = lit.to_string();
    let no_prefix = lit_str.strip_prefix("\"").ok_or(error!(lit.span(), "expected string literal"))?;
    let no_suffix = no_prefix.strip_suffix("\"").ok_or(error!(lit.span(), "expected string literal"))?;
    Ok(no_suffix.into())
}
