macro_rules! error {
    ($span:expr, $($args:tt)*) => {{
        let message = ::std::format!($($args)*);

        let mut tokens = ::proc_macro::TokenStream::new();

        tokens.extend([
            ::proc_macro::TokenTree::Ident(::proc_macro::Ident::new("compile_error", $span)),
            ::proc_macro::TokenTree::Punct(::proc_macro::Punct::new('!', ::proc_macro::Spacing::Alone)),
        ]);

        let args = ::proc_macro::TokenStream::from(::proc_macro::TokenTree::Literal(::proc_macro::Literal::string(&message)));

        tokens.extend([
            ::proc_macro::TokenTree::Group(::proc_macro::Group::new(::proc_macro::Delimiter::Parenthesis, args)),
            ::proc_macro::TokenTree::Punct(::proc_macro::Punct::new(';', ::proc_macro::Spacing::Alone)),
        ]);

        tokens
    }};
}

macro_rules! expect_punct {
    ($expected:literal, $(= $Err:ident =)? $found:expr) => {
        match $found {
            Some(::proc_macro::TokenTree::Punct(punct)) if punct == $expected && punct.spacing() == ::proc_macro::Spacing::Alone => {}
            Some(tok) => return $($Err)? (error!(tok.span(), "expected {:?}, found {tok}", $expected)),
            None => return $($Err)? (error!(::proc_macro::Span::call_site(), "expected {:?}, found end of macro", $expected)),
        }
    };
}

macro_rules! try_stream {
    ($val:expr) => {
        match $val {
            Ok(val) => val,
            Err(stream) => return stream,
        }
    };
}

mod internal;
mod intr_asm;

use proc_macro::TokenStream;

/// Inline assembly with better support for arbitrary registers
///
/// For x86 registers which are clobber-only, this macro will allow you to easily clobber
/// only the correct output register by constant expression rather than a string literal.
///
/// ```rust
/// const DST: u8 = 3;
///
/// let a = 123;
///
/// unsafe {
///     intr_gen::intr_asm!(
///         // destination register
///         mm{DST} = DST[0..8],
///
///         // remaining tokens are regular inline assembly syntax
///         "movd mm{DST}, {src:e}",
///         src = in(reg) a,
///         options(nostack, nomem, preserves_flags)
///     );
/// }
/// ```
///
/// Other range syntax is accepted too:
///
/// ```rust
/// const DST: u8 = 3;
///
/// let a = 123;
///
/// unsafe {
///     intr_gen::intr_asm!(
///         // destination register
///         mm{DST} = DST[0..=7],
///
///         // remaining tokens are regular inline assembly syntax
///         "movd mm{DST}, {src:e}",
///         src = in(reg) a,
///         options(nostack, nomem, preserves_flags)
///     );
/// }
/// ```
#[proc_macro]
pub fn intr_asm(input: TokenStream) -> TokenStream {
    intr_asm::intr_asm_internal(input)
}
