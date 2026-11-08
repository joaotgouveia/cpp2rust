// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

struct Args {
    max_arity: usize,
    each: bool,
}

// Splits `N [, each] ; item` into its arguments and the item.
fn parse_args(input: TokenStream) -> Result<(Args, TokenStream), TokenStream> {
    let error = |msg: &str| quote::quote! { compile_error!(#msg); };
    let mut tokens = input.into_iter();
    let max_arity = match tokens.next() {
        Some(TokenTree::Literal(lit)) => lit.to_string().parse::<usize>().map_err(|_| {
            error("expand_pack_to_concrete_arity_impl: the first argument is the maximum arity")
        })?,
        _ => {
            return Err(error(
                "expand_pack_to_concrete_arity_impl: expected `expand_pack_to_concrete_arity_impl!(N; item)` or `expand_pack_to_concrete_arity_impl!(N, each; item)`",
            ));
        }
    };
    let mut each = false;
    match tokens.next() {
        Some(TokenTree::Punct(p)) if p.as_char() == ';' => {}
        Some(TokenTree::Punct(p)) if p.as_char() == ',' => {
            match tokens.next() {
                Some(TokenTree::Ident(ident)) if ident == "each" => each = true,
                _ => {
                    return Err(error(
                        "expand_pack_to_concrete_arity_impl: expected `each` after the comma",
                    ));
                }
            }
            match tokens.next() {
                Some(TokenTree::Punct(p)) if p.as_char() == ';' => {}
                _ => {
                    return Err(error(
                        "expand_pack_to_concrete_arity_impl: expected `;` before the item",
                    ));
                }
            }
        }
        _ => {
            return Err(error(
                "expand_pack_to_concrete_arity_impl: expected `expand_pack_to_concrete_arity_impl!(N; item)` or `expand_pack_to_concrete_arity_impl!(N, each; item)`",
            ));
        }
    }
    Ok((Args { max_arity, each }, tokens.collect()))
}

struct Expansion {
    arity: usize,
    // the element the block is expanded for in `each` mode
    index: Option<usize>,
}

fn element_ident(name: &str, i: usize, span: Span) -> TokenTree {
    TokenTree::Ident(Ident::new(&format!("{name}{i}"), span))
}

// Substitutes the placeholders of one element inside a `#( ... )*` body.
fn substitute_element(body: TokenStream, i: usize) -> TokenStream {
    let mut out = TokenStream::new();
    let mut tokens = body.into_iter().peekable();
    while let Some(tt) = tokens.next() {
        match tt {
            TokenTree::Ident(ident) if ident == "T" || ident == "S" => {
                out.extend([element_ident(&ident.to_string(), i, ident.span())]);
            }
            TokenTree::Punct(p) if p.as_char() == '#' => match tokens.peek() {
                Some(TokenTree::Ident(ident)) if ident == "i" => {
                    tokens.next();
                    out.extend([TokenTree::Literal(Literal::usize_unsuffixed(i))]);
                }
                _ => out.extend([TokenTree::Punct(p)]),
            },
            TokenTree::Group(g) => {
                let inner = substitute_element(g.stream(), i);
                let mut group = Group::new(g.delimiter(), inner);
                group.set_span(g.span());
                out.extend([TokenTree::Group(group)]);
            }
            other => out.extend([other]),
        }
    }
    out
}

// Expands `#( body )sep*` repetitions, `#i` and `#T` for one arity.
fn expand_block(block: TokenStream, exp: &Expansion) -> TokenStream {
    let mut out = TokenStream::new();
    let tokens: Vec<TokenTree> = block.into_iter().collect();
    let mut pos = 0;
    while pos < tokens.len() {
        let is_hash = matches!(&tokens[pos], TokenTree::Punct(p) if p.as_char() == '#');
        if !is_hash {
            if let TokenTree::Group(g) = &tokens[pos] {
                let inner = expand_block(g.stream(), exp);
                let mut group = Group::new(g.delimiter(), inner);
                group.set_span(g.span());
                out.extend([TokenTree::Group(group)]);
            } else {
                out.extend([tokens[pos].clone()]);
            }
            pos += 1;
            continue;
        }
        match tokens.get(pos + 1) {
            Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Parenthesis => {
                // the separator is everything up to the closing `*`
                let mut end = pos + 2;
                let mut separator = TokenStream::new();
                loop {
                    match tokens.get(end) {
                        Some(TokenTree::Punct(p)) if p.as_char() == '*' => break,
                        Some(tt) => {
                            separator.extend([tt.clone()]);
                            end += 1;
                        }
                        None => {
                            return quote::quote! { compile_error!("expand_pack_to_concrete_arity_impl: `#( ... )` is not closed by `*`"); };
                        }
                    }
                }
                let body = g.stream();
                for i in 0..exp.arity {
                    if i > 0 {
                        out.extend(separator.clone());
                    }
                    out.extend(substitute_element(body.clone(), i));
                }
                pos = end + 1;
            }
            Some(TokenTree::Ident(ident)) if ident == "i" || ident == "T" => {
                let Some(index) = exp.index else {
                    return quote::quote! { compile_error!("expand_pack_to_concrete_arity_impl: `#i` and `#T` outside a repetition need `each`"); };
                };
                if ident == "i" {
                    out.extend([TokenTree::Literal(Literal::usize_unsuffixed(index))]);
                } else {
                    out.extend([element_ident("T", index, ident.span())]);
                }
                pos += 2;
            }
            _ => {
                out.extend([TokenTree::Punct(Punct::new('#', Spacing::Alone))]);
                pos += 1;
            }
        }
    }
    out
}

pub fn expand(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let (args, item) = match parse_args(input.into()) {
        Ok(parsed) => parsed,
        Err(error) => return error.into(),
    };
    let mut out = TokenStream::new();
    for arity in 0..=args.max_arity {
        if args.each {
            for index in 0..arity {
                out.extend(expand_block(
                    item.clone(),
                    &Expansion {
                        arity,
                        index: Some(index),
                    },
                ));
            }
        } else {
            out.extend(expand_block(
                item.clone(),
                &Expansion { arity, index: None },
            ));
        }
    }
    out.into()
}
