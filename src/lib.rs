#![doc = include_str!("../README.md")]

// Instrumenting the async fn is not as straight forward as expected because `async_trait`
// rewrites `async fn` into a normal fn which returns `Box<impl Future>`, and this stops
// the macro from distinguishing `async fn` from `fn`.
//
// The following code reused the `async_trait` probes from tokio-tracing [1].
//
// [1] https://github.com/tokio-rs/tracing/blob/6a61897a/tracing-attributes/src/expand.rs

use proc_macro2::Span;
use proc_macro_error2::abort_call_site;
use proc_macro_error2::proc_macro_error;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::spanned::Spanned;
use syn::Block;
use syn::Expr;
use syn::ExprAsync;
use syn::ExprCall;
use syn::FnArg;
use syn::Generics;
use syn::Ident;
use syn::Item;
use syn::ItemFn;
use syn::LitStr;
use syn::Pat;
use syn::PatType;
use syn::Path;
use syn::Signature;
use syn::Stmt;
use syn::Token;

#[derive(Debug)]
enum Args {
    Simple {
        level: String,
        input_format: Option<String>,
        output_format: Option<String>,
    },
    Result {
        ok_level: Option<String>,
        err_level: Option<String>,
        input_format: Option<String>,
        output_format: Option<String>,
    },
    Option {
        some_level: Option<String>,
        none_level: Option<String>,
        input_format: Option<String>,
        output_format: Option<String>,
    },
}

impl Parse for Args {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        #[derive(Default)]
        struct ArgContext {
            simple_level: Option<String>,
            ok_level: Option<String>,
            err_level: Option<String>,
            some_level: Option<String>,
            none_level: Option<String>,
            input_format: Option<String>,
            output_format: Option<String>,
        }

        impl Parse for ArgContext {
            fn parse(input: ParseStream) -> syn::Result<Self> {
                let mut ctx = ArgContext::default();
                loop {
                    if input.is_empty() {
                        return Ok(ctx);
                    }

                    if input.peek(LitStr) {
                        let level = input.parse::<LitStr>()?;
                        if ctx.simple_level.is_some() {
                            return Err(syn::Error::new(
                                level.span(),
                                "simple_level specified multiple times",
                            ));
                        }
                        ctx.simple_level = Some(level.value());
                    } else if input.peek(Ident) {
                        let ident = input.parse::<Ident>()?;
                        input.parse::<Token![=]>()?;
                        let level = input.parse::<LitStr>()?;
                        match ident.to_string().as_str() {
                            "some" => {
                                if ctx.some_level.is_some() {
                                    return Err(syn::Error::new(
                                        level.span(),
                                        "some_level specified multiple times",
                                    ));
                                }
                                ctx.some_level = Some(level.value());
                            }
                            "none" => {
                                if ctx.none_level.is_some() {
                                    return Err(syn::Error::new(
                                        level.span(),
                                        "none_level specified multiple times",
                                    ));
                                }
                                ctx.none_level = Some(level.value());
                            }
                            "ok" => {
                                if ctx.ok_level.is_some() {
                                    return Err(syn::Error::new(
                                        level.span(),
                                        "ok_level specified multiple times",
                                    ));
                                }
                                ctx.ok_level = Some(level.value());
                            }
                            "err" => {
                                if ctx.err_level.is_some() {
                                    return Err(syn::Error::new(
                                        level.span(),
                                        "err_level specified multiple times",
                                    ));
                                }
                                ctx.err_level = Some(level.value());
                            }
                            "input" => {
                                if ctx.input_format.is_some() {
                                    return Err(syn::Error::new(
                                        level.span(),
                                        "input specified multiple times",
                                    ));
                                }
                                ctx.input_format = Some(level.value());
                            }
                            "output" => {
                                if ctx.output_format.is_some() {
                                    return Err(syn::Error::new(
                                        level.span(),
                                        "output specified multiple times",
                                    ));
                                }
                                ctx.output_format = Some(level.value());
                            }
                            _ => {
                                return Err(syn::Error::new(
                                    ident.span(),
                                    "unknown attribute argument",
                                ))
                            }
                        }
                    } else {
                        return Err(input.error("unexpected token"));
                    }

                    if input.is_empty() {
                        return Ok(ctx);
                    }
                    input.parse::<Token![,]>()?;
                }
            }
        }

        let ArgContext {
            simple_level,
            ok_level,
            err_level,
            some_level,
            none_level,
            input_format,
            output_format,
        } = input.parse::<ArgContext>()?;

        if ok_level.is_some() || err_level.is_some() {
            if simple_level.is_some() {
                abort_call_site!("plain level cannot be specified with `ok` or `err` levels");
            }
            if some_level.is_some() || none_level.is_some() {
                abort_call_site!(
                    "`some` and `none` levels cannot be specified with `ok` or `err` levels"
                );
            }
            Ok(Args::Result {
                ok_level,
                err_level,
                input_format,
                output_format,
            })
        } else if some_level.is_some() || none_level.is_some() {
            if simple_level.is_some() {
                abort_call_site!("plain level cannot be specified with `some` or `none` levels");
            }
            Ok(Args::Option {
                some_level,
                none_level,
                input_format,
                output_format,
            })
        } else {
            Ok(Args::Simple {
                level: simple_level.unwrap_or_else(|| "debug".to_string()),
                input_format,
                output_format,
            })
        }
    }
}

/// `logcall` attribute macro that logs the function inputs and return values.
#[proc_macro_attribute]
#[proc_macro_error]
pub fn logcall(
    args: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(item as ItemFn);

    let args = syn::parse_macro_input!(args as Args);

    // check for async_trait-like patterns in the block, and instrument
    // the future instead of the wrapper
    let func_body = if let Some(internal_fun) =
        get_async_trait_info(&input.block, input.sig.asyncness.is_some())
    {
        // let's rewrite some statements!
        match internal_fun.kind {
            // async-trait <= 0.1.43
            AsyncTraitKind::Function => {
                unimplemented!(
                    "Please upgrade the crate `async-trait` to a version higher than 0.1.44"
                )
            }
            // async-trait >= 0.1.44
            AsyncTraitKind::Async(async_expr) => {
                // fallback if we couldn't find the '__async_trait' binding, might be
                // useful for crates exhibiting the same behaviors as async-trait
                let instrumented_block =
                    gen_block(&async_expr.block, true, false, &input.sig, args);
                let async_attrs = &async_expr.attrs;
                quote::quote_spanned! {async_expr.span()=>
                    Box::pin(#(#async_attrs) * #instrumented_block )
                }
            }
        }
    } else {
        gen_block(
            &input.block,
            input.sig.asyncness.is_some(),
            input.sig.asyncness.is_some(),
            &input.sig,
            args,
        )
    };

    let ItemFn {
        attrs, vis, sig, ..
    } = input.clone();

    let Signature {
        output: return_type,
        inputs: params,
        unsafety,
        constness,
        abi,
        ident,
        asyncness,
        generics:
            Generics {
                params: gen_params,
                where_clause,
                ..
            },
        ..
    } = sig;

    quote::quote_spanned!(input.span()=>
        #(#attrs) *
        #vis #constness #unsafety #asyncness #abi fn #ident<#gen_params>(#params) #return_type
        #where_clause
        {
            #func_body
        }
    )
    .into()
}

/// Instrument a block
fn gen_block(
    block: &Block,
    async_context: bool,
    async_keyword: bool,
    sig: &Signature,
    args: Args,
) -> proc_macro2::TokenStream {
    match args {
        Args::Simple {
            level,
            input_format,
            output_format,
        } => gen_plain_label_block(
            block,
            async_context,
            async_keyword,
            sig,
            &level,
            input_format,
            output_format,
        ),
        Args::Result {
            ok_level,
            err_level,
            input_format,
            output_format,
        } => gen_result_label_block(
            block,
            async_context,
            async_keyword,
            sig,
            ok_level,
            err_level,
            input_format,
            output_format,
        ),
        Args::Option {
            some_level,
            none_level,
            input_format,
            output_format,
        } => gen_option_label_block(
            block,
            async_context,
            async_keyword,
            sig,
            some_level,
            none_level,
            input_format,
            output_format,
        ),
    }
}

fn gen_plain_label_block(
    block: &Block,
    async_context: bool,
    async_keyword: bool,
    sig: &Signature,
    level: &str,
    input_format: Option<String>,
    output_format: Option<String>,
) -> proc_macro2::TokenStream {
    // Generate the instrumented function body.
    // If the function is an `async fn`, this will wrap it in an async block.
    if async_context {
        let input_format = input_format.unwrap_or_else(|| gen_input_format(sig));
        let output_format = output_format.unwrap_or_else(gen_output_format);
        let log = gen_log(level, "__input_string", &output_format, "__ret_value");
        let block = quote::quote_spanned!(block.span()=>
            #[allow(unknown_lints)]
            #[allow(clippy::useless_format)]
            let __input_string = format!(#input_format);
            let __ret_value = async { #block }.await;
            #log;
            __ret_value
        );

        if async_keyword {
            block
        } else {
            quote::quote_spanned!(block.span()=>
                async move {
                    #block
                }
            )
        }
    } else {
        let input_format = input_format.unwrap_or_else(|| gen_input_format(sig));
        let output_format = output_format.unwrap_or_else(gen_output_format);
        let log = gen_log(level, "__input_string", &output_format, "__ret_value");
        quote::quote_spanned!(block.span()=>
            #[allow(unknown_lints)]
            #[allow(clippy::useless_format)]
            let __input_string = format!(#input_format);
            #[allow(unknown_lints)]
            #[allow(clippy::redundant_closure_call)]
            #[allow(clippy::let_unit_value)]
            let __ret_value = (move || #block)();
            #log;
            __ret_value
        )
    }
}

#[allow(clippy::too_many_arguments)]
fn gen_result_label_block(
    block: &Block,
    async_context: bool,
    async_keyword: bool,
    sig: &Signature,
    ok_level: Option<String>,
    err_level: Option<String>,
    input_format: Option<String>,
    output_format: Option<String>,
) -> proc_macro2::TokenStream {
    let output_format = output_format.unwrap_or_else(gen_output_format);
    let ok_arm = if let Some(ok_level) = ok_level {
        let log_ok = gen_log(&ok_level, "__input_string", &output_format, "__ret_value");
        quote::quote_spanned!(block.span()=>
            __ret_value@Ok(_) => {
                #log_ok;
                __ret_value
            }
        )
    } else {
        quote::quote_spanned!(block.span()=>
            Ok(__ret_value) => Ok(__ret_value),
        )
    };
    let err_arm = if let Some(err_level) = err_level {
        let log_err = gen_log(&err_level, "__input_string", &output_format, "__ret_value");
        quote::quote_spanned!(block.span()=>
            __ret_value@Err(_) => {
                #log_err;
                __ret_value
            }
        )
    } else {
        quote::quote_spanned!(block.span()=>
            Err(__ret_value) => Err(__ret_value),
        )
    };

    // Generate the instrumented function body.
    // If the function is an `async fn`, this will wrap it in an async block.
    if async_context {
        let input_format = input_format.unwrap_or_else(|| gen_input_format(sig));
        let block = quote::quote_spanned!(block.span()=>
            #[allow(unknown_lints)]
            #[allow(clippy::useless_format)]
            let __input_string = format!(#input_format);
            let __ret_value = async { #block }.await;
            #[allow(unknown_lints)]
            #[allow(clippy::ignored_unit_patterns)]
            match __ret_value {
                #ok_arm
                #err_arm
            }
        );

        if async_keyword {
            block
        } else {
            quote::quote_spanned!(block.span()=>
                async move {
                    #block
                }
            )
        }
    } else {
        let input_format = input_format.unwrap_or_else(|| gen_input_format(sig));
        quote::quote_spanned!(block.span()=>
            #[allow(unknown_lints)]
            #[allow(clippy::useless_format)]
            let __input_string = format!(#input_format);
            #[allow(unknown_lints)]
            #[allow(clippy::redundant_closure_call)]
            #[allow(clippy::let_unit_value)]
            let __ret_value = (move || #block)();
            #[allow(unknown_lints)]
            #[allow(clippy::ignored_unit_patterns)]
            match __ret_value {
                #ok_arm
                #err_arm
            }
        )
    }
}

#[allow(clippy::too_many_arguments)]
fn gen_option_label_block(
    block: &Block,
    async_context: bool,
    async_keyword: bool,
    sig: &Signature,
    some_level: Option<String>,
    none_level: Option<String>,
    input_format: Option<String>,
    output_format: Option<String>,
) -> proc_macro2::TokenStream {
    let output_format = output_format.unwrap_or_else(gen_output_format);
    let some_arm = if let Some(some_level) = some_level {
        let log_some = gen_log(&some_level, "__input_string", &output_format, "__ret_value");
        quote::quote_spanned!(block.span()=>
            __ret_value@Some(_) => {
                #log_some;
                __ret_value
            }
        )
    } else {
        quote::quote_spanned!(block.span()=>
            Some(__ret_value) => Some(__ret_value),
        )
    };
    let none_arm = if let Some(none_level) = none_level {
        let log_none = gen_log(&none_level, "__input_string", &output_format, "__ret_value");
        quote::quote_spanned!(block.span()=>
            None => {
                #log_none;
                None
            }
        )
    } else {
        quote::quote_spanned!(block.span()=>
            None => None,
        )
    };

    // Generate the instrumented function body.
    // If the function is an `async fn`, this will wrap it in an async block.
    if async_context {
        let input_format = input_format.unwrap_or_else(|| gen_input_format(sig));
        let block = quote::quote_spanned!(block.span()=>
            #[allow(unknown_lints)]
            #[allow(clippy::useless_format)]
            let __input_string = format!(#input_format);
            let __ret_value = async { #block }.await;
            #[allow(unknown_lints)]
            #[allow(clippy::ignored_unit_patterns)]
            match __ret_value {
                #some_arm
                #none_arm
            }
        );

        if async_keyword {
            block
        } else {
            quote::quote_spanned!(block.span()=>
                async move {
                    #block
                }
            )
        }
    } else {
        let input_format = input_format.unwrap_or_else(|| gen_input_format(sig));
        quote::quote_spanned!(block.span()=>
            #[allow(unknown_lints)]
            #[allow(clippy::useless_format)]
            let __input_string = format!(#input_format);
            #[allow(unknown_lints)]
            #[allow(clippy::redundant_closure_call)]
            #[allow(clippy::let_unit_value)]
            let __ret_value = (move || #block)();
            #[allow(unknown_lints)]
            #[allow(clippy::ignored_unit_patterns)]
            match __ret_value {
                #some_arm
                #none_arm
            }
        )
    }
}

fn gen_log(
    level: &str,
    input_string: &str,
    output_format: &str,
    return_value: &str,
) -> proc_macro2::TokenStream {
    let level = level.to_lowercase();
    if !["error", "warn", "info", "debug", "trace"].contains(&level.as_str()) {
        abort_call_site!("unknown log level");
    }
    let level: Ident = Ident::new(&level, Span::call_site());
    let fn_name = quote::quote! {
        {
            fn f() {}
            fn type_name_of<T>(_: T) -> &'static str {
                std::any::type_name::<T>()
            }
            let name = type_name_of(f);
            let name = &name[..name.len() - 3];
            name.trim_end_matches("::{{closure}}")
        }
    };
    let input_string: Ident = Ident::new(input_string, Span::call_site());
    let format_string = format!("{{}}({{}}){output_format}");

    if output_format.replace("{{", "").contains("{") {
        let return_value: Ident = Ident::new(return_value, Span::call_site());
        quote::quote!(
            log::#level! (#format_string, #fn_name, #input_string, &#return_value)
        )
    } else {
        quote::quote!(
            log::#level! (#format_string, #fn_name, #input_string)
        )
    }
}

// fn(a: usize, b: usize) => "a = {a:?}, b = {b:?}"
fn gen_input_format(sig: &Signature) -> String {
    let mut input_format = String::new();
    for (i, input) in sig.inputs.iter().enumerate() {
        if i > 0 {
            input_format.push_str(", ");
        }
        match input {
            FnArg::Typed(PatType { pat, .. }) => {
                if let Pat::Ident(pat_ident) = &**pat {
                    let ident = &pat_ident.ident.to_string();
                    // Skip async-trait generated anonymous arguments.
                    if ident.starts_with("__arg") {
                        continue;
                    }
                    input_format.push_str(&format!("{ident} = {{{ident}:?}}"));
                }
            }
            FnArg::Receiver(_) => {
                input_format.push_str("self");
            }
        }
    }
    input_format
}

fn gen_output_format() -> String {
    " => {:?}".to_string()
}

enum AsyncTraitKind<'a> {
    // old construction. Contains the function
    Function,
    // new construction. Contains a reference to the async block
    Async(&'a ExprAsync),
}

struct AsyncTraitInfo<'a> {
    // statement that must be patched
    _source_stmt: &'a Stmt,
    kind: AsyncTraitKind<'a>,
}

// Get the AST of the inner function we need to hook, if it was generated
// by async-trait.
// When we are given a function annotated by async-trait, that function
// is only a placeholder that returns a pinned future containing the
// user logic, and it is that pinned future that needs to be instrumented.
// Were we to instrument its parent, we would only collect information
// regarding the allocation of that future, and not its own span of execution.
// Depending on the version of async-trait, we inspect the block of the function
// to find if it matches the pattern
// `async fn foo<...>(...) {...}; Box::pin(foo<...>(...))` (<=0.1.43), or if
// it matches `Box::pin(async move { ... }) (>=0.1.44). We the return the
// statement that must be instrumented, along with some other information.
// 'gen_body' will then be able to use that information to instrument the
// proper function/future.
// (this follows the approach suggested in
// https://github.com/dtolnay/async-trait/issues/45#issuecomment-571245673)
fn get_async_trait_info(block: &Block, block_is_async: bool) -> Option<AsyncTraitInfo<'_>> {
    // are we in an async context? If yes, this isn't an async_trait-like pattern
    if block_is_async {
        return None;
    }

    // list of async functions declared inside the block
    let inside_fns = block.stmts.iter().filter_map(|stmt| {
        if let Stmt::Item(Item::Fn(fun)) = &stmt {
            // If the function is async, this is a candidate
            if fun.sig.asyncness.is_some() {
                return Some((stmt, fun));
            }
        }
        None
    });

    // last expression of the block (it determines the return value
    // of the block, so that if we are working on a function whose
    // `trait` or `impl` declaration is annotated by async_trait,
    // this is quite likely the point where the future is pinned)
    let (last_expr_stmt, last_expr) = block.stmts.iter().rev().find_map(|stmt| {
        if let Stmt::Expr(expr, ..) = stmt {
            Some((stmt, expr))
        } else {
            None
        }
    })?;

    // is the last expression a function call?
    let (outside_func, outside_args) = match last_expr {
        Expr::Call(ExprCall { func, args, .. }) => (func, args),
        _ => return None,
    };

    // is it a call to `Box::pin()`?
    let path = match outside_func.as_ref() {
        Expr::Path(path) => &path.path,
        _ => return None,
    };
    if !path_to_string(path).ends_with("Box::pin") {
        return None;
    }

    // Does the call take an argument? If it doesn't,
    // it's not going to compile anyway, but that's no reason
    // to (try to) perform an out-of-bounds access
    if outside_args.is_empty() {
        return None;
    }

    // Is the argument to Box::pin an async block that
    // captures its arguments?
    if let Expr::Async(async_expr) = &outside_args[0] {
        // check that the move 'keyword' is present
        async_expr.capture?;

        return Some(AsyncTraitInfo {
            _source_stmt: last_expr_stmt,
            kind: AsyncTraitKind::Async(async_expr),
        });
    }

    // Is the argument to Box::pin a function call itself?
    let func = match &outside_args[0] {
        Expr::Call(ExprCall { func, .. }) => func,
        _ => return None,
    };

    // "stringify" the path of the function called
    let func_name = match **func {
        Expr::Path(ref func_path) => path_to_string(&func_path.path),
        _ => return None,
    };

    // Was that function defined inside the current block?
    // If so, retrieve the statement where it was declared and the function itself
    let (stmt_func_declaration, _) = inside_fns
        .into_iter()
        .find(|(_, fun)| fun.sig.ident == func_name)?;

    Some(AsyncTraitInfo {
        _source_stmt: stmt_func_declaration,
        kind: AsyncTraitKind::Function,
    })
}

// Return a path as a String
fn path_to_string(path: &Path) -> String {
    use std::fmt::Write;
    // some heuristic to prevent too many allocations
    let mut res = String::with_capacity(path.segments.len() * 5);
    for i in 0..path.segments.len() {
        write!(res, "{}", path.segments[i].ident).expect("writing to a String should never fail");
        if i < path.segments.len() - 1 {
            res.push_str("::");
        }
    }
    res
}
