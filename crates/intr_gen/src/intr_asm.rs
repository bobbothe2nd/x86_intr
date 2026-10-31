use core::range::Range;

use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

use crate::internal::{gen_core_asm, gen_unreachable, parse_range};

pub(crate) fn intr_asm_internal(input: TokenStream) -> TokenStream {
    let tokens: Vec<_> = input.into_iter().collect();
    let mut pos = 0;

    let mut current_class = 0;
    let mut classes = std::collections::HashMap::new();

    let mut clobbers = Vec::new();

    loop {
        let start = pos;

        let reg_class = match tokens.get(pos) {
            Some(TokenTree::Ident(ident)) => {
                pos += 1;
                ident.clone()
            }
            Some(tok) => return error!(tok.span(), "expected register class, found `{tok}`"),
            None => return error!(Span::call_site(), "expected register class"),
        };

        let group = match tokens.get(pos) {
            Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Brace => {
                pos += 1;
                group.clone()
            }
            Some(tok) => {
                return error!(
                    tok.span(),
                    "expected `{{}}` register identifier, found `{tok}`"
                );
            }
            None => return error!(reg_class.span(), "expected `{{...}}` register identifier"),
        };

        let asm_const = match group.stream().into_iter().next() {
            Some(TokenTree::Ident(ident)) => ident,
            Some(tok) => return error!(tok.span(), "expected identifier, found `{tok}`"),
            None => return error!(group.span(), "expected identifier"),
        };

        match tokens.get(pos) {
            Some(TokenTree::Punct(p)) if p.as_char() == '=' => pos += 1,
            Some(tok) => return error!(tok.span(), "expected `=`"),
            None => return error!(asm_const.span(), "expected `=`"),
        }

        let subject_const = match tokens.get(pos) {
            Some(TokenTree::Ident(ident)) => {
                pos += 1;
                ident.clone()
            }
            Some(tok) => return error!(tok.span(), "expected identifier, found `{tok}`"),
            None => return error!(asm_const.span(), "expected identifier"),
        };

        let range = match tokens.get(pos) {
            Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Bracket => {
                pos += 1;
                try_stream!(parse_range(&mut group.stream().into_iter()))
            }
            Some(tok) => return error!(tok.span(), "expected bracketed group, found `{tok}`"),
            None => return error!(subject_const.span(), "expected bracketed group"),
        };

        let reg_class_id = classes
            .get(&reg_class.to_string())
            .copied()
            .unwrap_or_else(|| {
                let id = current_class;
                classes.insert(reg_class.to_string(), id);
                current_class += 1;
                id
            });

        clobbers.push(ClobberSpec {
            reg_class,
            reg_class_id,
            asm_const,
            subject_const,
            range,
        });

        match tokens.get(pos) {
            Some(TokenTree::Punct(p)) if p.as_char() == ',' => pos += 1,
            Some(tok) => return error!(tok.span(), "expected `,`"),
            None => return error!(Span::call_site(), "expected asm"),
        };

        if unlikely_clobber(&tokens[pos..]) {
            let asm: TokenStream = tokens[pos..].iter().cloned().collect();

            return gen_intr_asm(&clobbers, asm);
        }

        debug_assert!(pos > start);
    }
}

fn unlikely_clobber(tokens: &[TokenTree]) -> bool {
    !matches!(
        tokens,
        [
            TokenTree::Ident(_),
            TokenTree::Group(group),
            TokenTree::Punct(eq),
            TokenTree::Ident(_),
            TokenTree::Group(range),
            ..
        ]
        if group.delimiter() == Delimiter::Brace
            && eq.as_char() == '='
            && range.delimiter() == Delimiter::Bracket
    )
}

struct ClobberSpec {
    reg_class: Ident,
    reg_class_id: usize,
    asm_const: Ident,
    subject_const: Ident,
    range: Range<usize>,
}

struct Case {
    regs: Box<[usize]>,
}

fn product(
    clobbers: &[ClobberSpec],
    index: usize,
    selected: &mut Vec<usize>,
    output: &mut Vec<Case>,
) {
    if index == clobbers.len() {
        output.push(Case {
            regs: selected.clone().into_boxed_slice(),
        });
        return;
    }

    let clobber = &clobbers[index];

    'register: for register in clobber.range {
        for previous in 0..index {
            if clobbers[previous].reg_class_id == clobber.reg_class_id
                && selected[previous] == register
            {
                continue 'register;
            }
        }

        selected.push(register);

        product(clobbers, index + 1, selected, output);

        selected.pop();
    }
}

fn make_arm(clobbers: &[ClobberSpec], case: &Case, asm: &TokenStream) -> TokenStream {
    let subject_const = &clobbers[0].subject_const;

    let mut arm = TokenStream::new();

    arm.extend([
        make_pattern(case),
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

    let needs_comma = match asm_tokens.clone().into_iter().last() {
        Some(TokenTree::Punct(p)) => p.as_char() != ',',
        Some(_) => true,
        None => {
            return error!(Span::call_site(), "expected non-empty asm");
        }
    };

    if needs_comma {
        asm_tokens.extend([Punct::new(',', Spacing::Alone)]);
    }

    for (clobber, &register) in clobbers.iter().zip(case.regs.iter()) {
        append_clobber(&mut asm_tokens, clobber, register);
    }

    let body = Group::new(Delimiter::Brace, gen_core_asm(asm_tokens));

    arm.extend([TokenTree::Group(body)]);

    arm.extend([Punct::new(',', Spacing::Alone)]);

    arm
}

fn append_clobber(asm: &mut TokenStream, clobber: &ClobberSpec, register: usize) {
    let literal = {
        let mut literal = Literal::usize_unsuffixed(register);
        literal.set_span(clobber.asm_const.span());
        literal
    };

    asm.extend([
        TokenTree::Ident(clobber.asm_const.clone()),
        TokenTree::Punct(Punct::new('=', Spacing::Joint)),
        TokenTree::Ident(Ident::new("const", clobber.asm_const.span())),
        TokenTree::Literal(literal),
        TokenTree::Punct(Punct::new(',', Spacing::Alone)),
    ]);

    asm.extend([
        TokenTree::Ident(Ident::new("out", clobber.asm_const.span())),
        register_group(clobber, register),
        TokenTree::Ident(Ident::new("_", clobber.asm_const.span())),
        TokenTree::Punct(Punct::new(',', Spacing::Alone)),
    ]);
}

fn register_group(clobber: &ClobberSpec, register: usize) -> TokenTree {
    let mut string = Literal::string(&format!("{}{}", clobber.reg_class, register,));
    string.set_span(clobber.reg_class.span());

    let mut stream = TokenStream::new();
    stream.extend([TokenTree::Literal(string)]);

    let mut group = Group::new(Delimiter::Parenthesis, stream);
    group.set_span(clobber.reg_class.span());

    TokenTree::Group(group)
}

fn make_pattern(case: &Case) -> TokenTree {
    let mut stream = TokenStream::new();

    for (i, &register) in case.regs.iter().enumerate() {
        if i != 0 {
            stream.extend([TokenTree::Punct(Punct::new(',', Spacing::Alone))]);
        }

        stream.extend([TokenTree::Literal(Literal::usize_unsuffixed(register))]);
    }

    TokenTree::Group(Group::new(Delimiter::Parenthesis, stream))
}

fn make_match_subject(clobbers: &[ClobberSpec]) -> TokenTree {
    let mut stream = TokenStream::new();

    for (i, clobber) in clobbers.iter().enumerate() {
        if i != 0 {
            stream.extend([TokenTree::Punct(Punct::new(',', Spacing::Alone))]);
        }

        stream.extend([TokenTree::Ident(clobber.subject_const.clone())]);
    }

    TokenTree::Group(Group::new(Delimiter::Parenthesis, stream))
}

fn gen_intr_asm(clobbers: &[ClobberSpec], asm: TokenStream) -> TokenStream {
    if clobbers.is_empty() {
        return error!(Span::call_site(), "expected at least one clobber");
    }

    let mut cases = Vec::new();
    let mut selected = Vec::with_capacity(clobbers.len());

    product(clobbers, 0, &mut selected, &mut cases);

    let mut arms = TokenStream::new();

    for case in &cases {
        arms.extend(make_arm(clobbers, case, &asm));
    }

    arms.extend([
        TokenTree::Ident(Ident::new("_", Span::call_site())),
        TokenTree::Punct(Punct::new('=', Spacing::Joint)),
        TokenTree::Punct(Punct::new('>', Spacing::Alone)),
    ]);

    arms.extend(gen_unreachable());

    let mut output = TokenStream::new();

    output.extend([
        TokenTree::Ident(Ident::new("match", Span::call_site())),
        make_match_subject(clobbers),
    ]);

    let body = Group::new(Delimiter::Brace, arms);

    output.extend([TokenTree::Group(body)]);

    output
}
