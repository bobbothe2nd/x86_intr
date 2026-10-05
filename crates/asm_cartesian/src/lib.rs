use proc_macro::{
    Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree, token_stream::IntoIter,
};

macro_rules! error {
    ($span:expr, $($args:tt)*) => {{
        let message = format!($($args)*);

        let mut tokens = TokenStream::new();

        tokens.extend([
            TokenTree::Ident(Ident::new("compile_error", $span)),
            TokenTree::Punct(Punct::new('!', Spacing::Alone)),
        ]);

        let args = TokenStream::from(TokenTree::Literal(Literal::string(&message)));

        tokens.extend([
            TokenTree::Group(Group::new(Delimiter::Parenthesis, args)),
            TokenTree::Punct(Punct::new(';', Spacing::Alone)),
        ]);

        tokens
    }};
}

#[proc_macro_attribute]
pub fn asm_cartesian(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut tokens = attr.into_iter().peekable();

    let instr = match tokens.next() {
        Some(TokenTree::Ident(instr)) => instr.to_string(),
        Some(tok) => return error!(tok.span(), "expected asm instruction"),
        None => return error!(Span::call_site(), "expected asm instruction"),
    };

    match tokens.next() {
        Some(TokenTree::Punct(punct)) if punct == ',' => {}
        Some(tok) => return error!(tok.span(), "expected `,`"),
        None => return error!(Span::call_site(), "expected `,`"),
    }

    let mut sets = Vec::new();

    loop {
        let arg = match tokens.next() {
            Some(TokenTree::Ident(arg)) => arg,
            None => break,
            Some(tok) => return error!(tok.span(), "expected argument name"),
        };

        match tokens.next() {
            Some(TokenTree::Punct(punct)) if punct == '=' => {}
            Some(tok) => return error!(tok.span(), "expected `=`"),
            None => return error!(Span::call_site(), "expected `=`"),
        }

        let kind = match tokens.next() {
            Some(TokenTree::Ident(x)) => x,
            Some(tok) => return error!(tok.span(), "expected register class"),
            None => return error!(Span::call_site(), "expected register class"),
        };

        let group = match tokens.next() {
            Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Brace => g,
            Some(tok) => return error!(tok.span(), "expected `{{ ... }}`"),
            None => return error!(Span::call_site(), "expected `{{ ... }}`"),
        };

        let (start, end) = match parse_range(group.stream().into_iter()) {
            Ok(range) => range,
            Err(stream) => return stream,
        };

        sets.push(RegSet {
            arg,
            kind,
            start,
            end,
        });

        match tokens.next() {
            Some(TokenTree::Punct(p)) if p == ',' => {}
            None => break,
            Some(tok) => return error!(tok.span(), "expected `,`"),
        }
    }

    let mut selected = Vec::new();
    let mut output = Vec::new();

    product(&sets, 0, &mut selected, &mut output);

    let body = make_match(
        &sets,
        &output,
        &instr,
    );

    replace_body(item, body)
}

struct RegSet {
    arg: Ident,
    kind: Ident,
    start: usize,
    end: usize,
}

struct Case {
    regs: Box<[usize]>,
}

fn make_match(
    sets: &[RegSet],
    cases: &[Case],
    instr: &str,
) -> TokenStream {
    let mut stream = TokenStream::new();

    stream.extend([
        TokenTree::Ident(Ident::new("match", Span::call_site())),
    ]);

    stream.extend([make_match_subject(sets)]);

    let mut arms = TokenStream::new();

    for case in cases {
        arms.extend(make_arm(sets, case, instr.to_string()));
    }

    arms.extend(make_wildcard_arm());

    stream.extend([
        TokenTree::Group(Group::new(Delimiter::Brace, arms)),
    ]);

    stream
}

fn make_match_subject(sets: &[RegSet]) -> TokenTree {
    let mut stream = TokenStream::new();

    for (i, set) in sets.iter().enumerate() {
        if i != 0 {
            stream.extend([
                TokenTree::Punct(Punct::new(',', Spacing::Alone)),
            ]);
        }

        stream.extend([
            TokenTree::Ident(set.arg.clone()),
        ]);
    }

    TokenTree::Group(Group::new(Delimiter::Parenthesis, stream))
}

fn replace_body(
    item: TokenStream,
    body: TokenStream,
) -> TokenStream {
    let mut output = TokenStream::new();

    for token in item {
        match token {
            TokenTree::Group(group)
                if group.delimiter() == Delimiter::Brace =>
            {
                output.extend([
                    TokenTree::Group(Group::new(
                        Delimiter::Brace,
                        body.clone(),
                    )),
                ]);

                return output;
            }

            token => output.extend([token]),
        }
    }

    error!(Span::call_site(), "expected function body")
}

fn make_core_asm(
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

fn asm_string(
    sets: &[RegSet],
    case: &Case,
    mut text: String,
) -> Literal {
    let registers = case_registers(sets, case);

    text.push(' ');

    for (i, register) in registers.iter().enumerate() {
        if i != 0 {
            text.push_str(", ");
        }

        match register {
            TokenTree::Ident(ident) => {
                text.push_str(&ident.to_string());
            }
            _ => unreachable!(),
        }
    }

    Literal::string(&text)
}

fn make_arm(
    sets: &[RegSet],
    case: &Case,
    instr: String,
) -> TokenStream {
    let mut arm = TokenStream::new();

    arm.extend([
        make_pattern(case),
        TokenTree::Punct(Punct::new('=', Spacing::Joint)),
        TokenTree::Punct(Punct::new('>', Spacing::Alone)),
    ]);

    let args = TokenStream::from(TokenTree::Literal(asm_string(sets, case, instr)));

    let asm = make_core_asm(args);

    let mut body = TokenStream::new();
    body.extend([asm]);

    arm.extend([
        TokenTree::Group(Group::new(Delimiter::Brace, body)),
    ]);

    arm
}

fn make_wildcard_arm() -> TokenStream {
    let mut arm = TokenStream::new();

    arm.extend([
        TokenTree::Ident(Ident::new("_", Span::call_site())),
    ]);

    arm.extend([
        TokenTree::Punct(Punct::new('=', Spacing::Joint)),
        TokenTree::Punct(Punct::new('>', Spacing::Alone)),
    ]);

    let mut body = TokenStream::new();

    body.extend([
        TokenTree::Ident(Ident::new("unsafe", Span::call_site())),
    ]);

    let mut unsafe_body = TokenStream::new();

    let mut call = TokenStream::new();

    call.extend([
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new("core", Span::call_site())),

        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new("hint", Span::call_site())),

        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new(
            "unreachable_unchecked",
            Span::call_site(),
        )),

        TokenTree::Group(Group::new(
            Delimiter::Parenthesis,
            TokenStream::new(),
        )),
    ]);

    unsafe_body.extend(call);

    body.extend([
        TokenTree::Group(Group::new(
            Delimiter::Brace,
            unsafe_body,
        )),
    ]);

    arm.extend([
        TokenTree::Group(Group::new(
            Delimiter::Brace,
            body,
        )),
    ]);

    arm
}

fn product(
    sets: &[RegSet],
    index: usize,
    selected: &mut Vec<usize>,
    output: &mut Vec<Case>,
) {
    if index == sets.len() {
        output.push(Case {
            regs: selected.clone().into_boxed_slice(),
        });
        return;
    }

    let set = &sets[index];

    for r in set.start..set.end {
        selected.push(r);

        product(
            sets,
            index + 1,
            selected,
            output,
        );

        selected.pop();
    }
}

fn parse_range(mut stream: IntoIter) -> Result<(usize, usize), TokenStream> {
    let start = match stream.next() {
        Some(TokenTree::Literal(lit)) => lit
            .to_string()
            .parse()
            .map_err(|_| error!(lit.span(), "expected integer literal"))?,
        Some(tok) => return Err(error!(tok.span(), "expected range start")),
        None => return Err(error!(Span::call_site(), "expected range start")),
    };

    match stream.next() {
        Some(TokenTree::Punct(p)) if p == '.' => {}
        Some(tok) => return Err(error!(tok.span(), "expected `..`")),
        None => return Err(error!(Span::call_site(), "expected `..`")),
    }

    match stream.next() {
        Some(TokenTree::Punct(p)) if p == '.' => {}
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

    if stream.next().is_some() {
        error!(Span::call_site(), "unexpected tokens after range");
    }

    if start >= end {
        error!(Span::call_site(), "unexpected reverse or empty range");
    }

    Ok((start, end))
}

fn make_pattern(case: &Case) -> TokenTree {
    let mut stream = TokenStream::new();

    for (i, &reg) in case.regs.iter().enumerate() {
        if i != 0 {
            stream.extend([TokenTree::Punct(Punct::new(',', Spacing::Alone))]);
        }

        stream.extend([
            TokenTree::Literal(Literal::usize_unsuffixed(reg)),
        ]);
    }

    TokenTree::Group(Group::new(Delimiter::Parenthesis, stream))
}

fn case_registers(
    sets: &[RegSet],
    case: &Case,
) -> Vec<TokenTree> {
    fn register_ident(kind: &Ident, reg: &usize) -> TokenTree {
        TokenTree::Ident(Ident::new(&format!("{kind}{reg}"), kind.span()))
    }

    sets.iter()
        .zip(case.regs.iter())
        .map(|(set, reg)| register_ident(&set.kind, reg))
        .collect()
}
