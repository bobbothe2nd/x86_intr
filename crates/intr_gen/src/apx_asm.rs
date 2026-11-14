// TODO: handle att syntax

use proc_macro::{
    Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree,
};

use crate::internal::{RegArg, forward_expr_to_comma, mov, parse_string_literal};

pub(crate) fn asm_internal(input: TokenStream) -> TokenStream {
    let mut iter = input.into_iter();
    let mut args = Vec::new();

    while let Some(token) = iter.next() {
        let TokenTree::Ident(ident) = &token else {
            args.push(AsmArg::Raw(TokenStream::from(token)));
            continue;
        };

        let arg = match ident.to_string().as_str() {
            "in" => parse_reg_expr(&mut iter, |reg, expr| {
                AsmArg::In { reg, expr }
            }),

            "out" => parse_reg_expr(&mut iter, |reg, expr| {
                AsmArg::Out { reg, expr }
            }),

            "lateout" => parse_reg_expr(&mut iter, |reg, expr| {
                AsmArg::LateOut { reg, expr }
            }),

            "inout" => parse_reg_expr(&mut iter, |reg, expr| {
                AsmArg::InOut { reg, expr }
            }),

            "inlateout" => parse_reg_expr(&mut iter, |reg, expr| {
                AsmArg::InLateOut { reg, expr }
            }),

            "preserve" => parse_preserve(&mut iter),

            "clobber" => parse_clobber(&mut iter, false),

            "lateclobber" => parse_clobber(&mut iter, true),

            _ => Ok(AsmArg::Raw(token.into())),
        };

        args.push(try_stream!(arg));
    }

    lower(args, todo!())
}

enum AsmArg {
    Raw(TokenStream),

    In {
        reg: Reg,
        expr: TokenStream,
    },
    Out {
        reg: Reg,
        expr: TokenStream,
    },
    LateOut {
        reg: Reg,
        expr: TokenStream,
    },
    InOut {
        reg: Reg,
        expr: TokenStream,
    },
    InLateOut {
        reg: Reg,
        expr: TokenStream,
    },

    Preserve {
        reg: Reg,
        expr: Option<TokenStream>,
    },

    Clobber(Reg),
    LateClobber(Reg),
}

enum Reg {
    Class(Ident),
    Exact {
        name: Box<str>,
        span: Span,
    },
}

impl Reg {
    fn span(&self) -> Span {
        match self {
            Self::Class(ident) => ident.span(),
            Self::Exact { span, .. } => *span,
        }
    }

    fn name(&self) -> String {
        match self {
            Self::Class(ident) => ident.to_string(),
            Self::Exact { name, .. } => name.to_string(),
        }
    }

    fn into_ident(self) -> Ident {
        match self {
            Self::Class(ident) => ident,
            Self::Exact { name, span } => Ident::new(&name, span),
        }
    }
}

fn parse_reg_expr(
    iter: &mut impl Iterator<Item = TokenTree>,
    make: impl FnOnce(Reg, TokenStream) -> AsmArg,
) -> Result<AsmArg, TokenStream> {
    let reg = parse_reg(iter)?;
    let expr = forward_expr_to_comma(iter)?;

    Ok(make(reg, expr))
}

fn parse_preserve(
    iter: &mut impl Iterator<Item = TokenTree>,
) -> Result<AsmArg, TokenStream> {
    let reg = parse_reg(iter)?;

    let expr = match iter.next() {
        Some(TokenTree::Ident(ident)) if ident.to_string() == "_" => None,
        Some(token) => {
            let mut iter = std::iter::once(token).chain(iter.by_ref());
            Some(forward_expr_to_comma(&mut iter)?)
        }
        None => return Err(error!(Span::call_site(), "expected preserve operand expression")),
    };

    Ok(AsmArg::Preserve { reg, expr })
}

fn parse_clobber(
    iter: &mut impl Iterator<Item = TokenTree>,
    late: bool,
) -> Result<AsmArg, TokenStream> {
    let reg = parse_reg(iter)?;

    Ok(if late {
        AsmArg::LateClobber(reg)
    } else {
        AsmArg::Clobber(reg)
    })
}

fn parse_reg(
    iter: &mut impl Iterator<Item = TokenTree>,
) -> Result<Reg, TokenStream> {
    let group = match iter.next() {
        Some(TokenTree::Group(group))
            if group.delimiter() == Delimiter::Parenthesis =>
        {
            group
        }
        Some(tok) => return Err(error!(tok.span(), "expected register specification")),
        _ => return Err(error!(Span::call_site(), "expected register specification")),
    };

    let mut tokens = group.stream().into_iter();

    let lit = match tokens.next() {
        Some(TokenTree::Literal(lit)) => lit,
        Some(TokenTree::Ident(ident)) => return Ok(Reg::Class(ident)),
        Some(tok) => return Err(error!(tok.span(), "expected register string literal or class")),
        None => return Err(error!(Span::call_site(), "expected register string literal or class")),
    };

    if let Some(tok) = tokens.next() {
        return Err(error!(tok.span(), "unexpected register tokens"));
    }

    Ok(Reg::Exact {
        name: parse_string_literal(&lit)?,
        span: lit.span(),
    })
}

struct Lowering {
    prefix: TokenStream,
    suffix: TokenStream,
    operands: TokenStream,
    next_id: usize,
    att: bool,
}

impl Lowering {
    fn next_name(&mut self) -> String {
        let id = self.next_id;
        self.next_id += 1;

        format!("__asm_internal_{id}")
    }
}

enum RegKind {
    Normal,
    Apx,
}

impl Reg {
    fn kind(&self) -> RegKind {
        let name = self.name();
        let Some(n) = name.strip_prefix('r') else {
            return RegKind::Normal;
        };

        match n.parse::<u8>() {
            Ok(_n @ 16..=31) => RegKind::Apx,
            _ => RegKind::Normal,
        }
    }
}

fn lower(args: Vec<AsmArg>, att: bool) -> TokenStream {
    let mut start = TokenStream::new();
    let mut output = TokenStream::new();

    for arg in args {
        match arg {
            AsmArg::Raw(tokens) => {
                output.extend(tokens);
                continue;
            }

            AsmArg::In { reg, expr } => {
                lower_reg_arg(&mut start, &mut output, "in", reg, expr, att);
            }

            AsmArg::Out { reg, expr } => {
                lower_reg_arg(&mut start, &mut output, "out", reg, expr, att);
            }

            AsmArg::LateOut { reg, expr } => {
                lower_reg_arg(&mut start, &mut output, "lateout", reg, expr, att);
            }

            AsmArg::InOut { reg, expr } => {
                lower_reg_arg(&mut start, &mut output, "inout", reg, expr, att);
            }

            AsmArg::InLateOut { reg, expr } => {
                lower_reg_arg(&mut start, &mut output, "inlateout", reg, expr, att);
            }

            AsmArg::Clobber(reg) => {
                lower_clobber(&mut output, reg, "out");
            }

            AsmArg::LateClobber(reg) => {
                lower_clobber(&mut output, reg, "lateout");
            }

            AsmArg::Preserve { reg, expr } => {
                lower_preserve(&mut start, &mut output, reg, expr, att);
            }
        }

        output.extend([TokenTree::Punct(Punct::new(
            ',',
            Spacing::Alone,
        ))]);
    }

    start.extend(output);
    start
}

fn lower_reg_arg(
    start: &mut TokenStream,
    output: &mut TokenStream,
    kind: &str,
    reg: Reg,
    expr: TokenStream,
    att: bool,
) {
    if kind.contains("in") {
        let src: (RegArg, Option<usize>) = fmt_reg(output, expr.clone());
        mov(start, src.0, RegArg::Reg(reg.name().into_boxed_str()), att);
    }

    output.extend([
        TokenTree::Ident(Ident::new(kind, reg.span())),
        TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenTree::Ident(Ident::new("reg", reg.span())).into()))
    ]);
    output.extend(expr);
    output.extend([TokenTree::Punct(Punct::new(',', Spacing::Alone))]);
}

fn fmt_reg(output: &TokenStream, input: TokenStream) -> (RegArg, Option<usize>) {
    // __tmp_reg{output.len()}
    todo!()
}

fn lower_clobber(
    output: &mut TokenStream,
    reg: Reg,
    late: &str,
) {
    let reg_span = reg.span();
    output.extend([
        TokenTree::Ident(Ident::new(&late, reg_span)),
        TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenTree::Ident(reg.into_ident()).into())),
        TokenTree::Ident(Ident::new("_", reg_span)),
    ]);
}

fn lower_preserve(
    start: &mut TokenStream,
    output: &mut TokenStream,
    reg: Reg,
    expr: Option<TokenStream>,
    att: bool,
) {
    // push save to start and restore to output
    // save/restore = (>8 other regs already) push/pop; else tmp reg+(2x)mov
    todo!()
}
