fn main() {}

#[cfg(FALSE)] fn e() { let _ = box #![attr] 0; }
//~^ ERROR an inner attribute is not permitted in this context
#[cfg(FALSE)] fn e() { let _ = [#[attr]]; }
//~^ ERROR expected expression, found `]`
#[cfg(FALSE)] fn e() { let _ = foo#[attr](); }
//~^ ERROR expected one of
#[cfg(FALSE)] fn e() { let _ = foo(#![attr]); }
//~^ ERROR an inner attribute is not permitted in this context
//~| ERROR expected expression, found `)`
#[cfg(FALSE)] fn e() { let _ = x.foo(#![attr]); }
//~^ ERROR an inner attribute is not permitted in this context
//~| ERROR expected expression, found `)`
#[cfg(FALSE)] fn e() { let _ = 0 + #![attr] 0; }
//~^ ERROR an inner attribute is not permitted in this context
#[cfg(FALSE)] fn e() { let _ = !#![attr] 0; }
//~^ ERROR an inner attribute is not permitted in this context
#[cfg(FALSE)] fn e() { let _ = -#![attr] 0; }
//~^ ERROR an inner attribute is not permitted in this context
#[cfg(FALSE)] fn e() { let _ = x #![attr] as Y; }
//~^ ERROR expected one of
#[cfg(FALSE)] fn e() { let _ = || #![attr] foo; }
//~^ ERROR an inner attribute is not permitted in this context
#[cfg(FALSE)] fn e() { let _ = move || #![attr] foo; }
//~^ ERROR an inner attribute is not permitted in this context
#[cfg(FALSE)] fn e() { let _ = || #![attr] {foo}; }
//~^ ERROR an inner attribute is not permitted in this context
#[cfg(FALSE)] fn e() { let _ = move || #![attr] {foo}; }
//~^ ERROR an inner attribute is not permitted in this context
#[cfg(FALSE)] fn e() { let _ = #[attr] ..#[attr] 0; }
//~^ ERROR expected expression, found `..`
#[cfg(FALSE)] fn e() { let _ = #[attr] ..; }
//~^ ERROR expected expression, found `..`
#[cfg(FALSE)] fn e() { let _ = #[attr] &#![attr] 0; }
//~^ ERROR an inner attribute is not permitted in this context
#[cfg(FALSE)] fn e() { let _ = #[attr] &mut #![attr] 0; }
//~^ ERROR an inner attribute is not permitted in this context
#[cfg(FALSE)] fn e() { let _ = #[attr] if 0 {}; }
//~^ ERROR attributes are not yet allowed on `if` expressions
#[cfg(FALSE)] fn e() { let _ = if 0 #[attr] {}; }
//~^ ERROR expected `{`, found `#`
#[cfg(FALSE)] fn e() { let _ = if 0 {#![attr]}; }
//~^ ERROR an inner attribute is not permitted in this context
#[cfg(FALSE)] fn e() { let _ = if 0 {} #[attr] else {}; }
//~^ ERROR expected one of
#[cfg(FALSE)] fn e() { let _ = if 0 {} else #[attr] {}; }
//~^ ERROR expected `{`, found `#`
#[cfg(FALSE)] fn e() { let _ = if 0 {} else {#![attr]}; }
//~^ ERROR an inner attribute is not permitted in this context
#[cfg(FALSE)] fn e() { let _ = if 0 {} else #[attr] if 0 {}; }
//~^ ERROR attributes are not yet allowed on `if` expressions
//~| ERROR expected `{`, found `#`
#[cfg(FALSE)] fn e() { let _ = if 0 {} else if 0 #[attr] {}; }
//~^ ERROR expected `{`, found `#`
#[cfg(FALSE)] fn e() { let _ = if 0 {} else if 0 {#![attr]}; }
//~^ ERROR an inner attribute is not permitted in this context
#[cfg(FALSE)] fn e() { let _ = #[attr] if let _ = 0 {}; }
//~^ ERROR attributes are not yet allowed on `if` expressions
#[cfg(FALSE)] fn e() { let _ = if let _ = 0 #[attr] {}; }
//~^ ERROR expected `{`, found `#`
#[cfg(FALSE)] fn e() { let _ = if let _ = 0 {#![attr]}; }
//~^ ERROR an inner attribute is not permitted in this context
#[cfg(FALSE)] fn e() { let _ = if let _ = 0 {} #[attr] else {}; }
//~^ ERROR expected one of
#[cfg(FALSE)] fn e() { let _ = if let _ = 0 {} else #[attr] {}; }
//~^ ERROR expected `{`, found `#`
#[cfg(FALSE)] fn e() { let _ = if let _ = 0 {} else {#![attr]}; }
//~^ ERROR an inner attribute is not permitted in this context
#[cfg(FALSE)] fn e() { let _ = if let _ = 0 {} else #[attr] if let _ = 0 {}; }
//~^ ERROR attributes are not yet allowed on `if` expressions
//~| ERROR expected `{`, found `#`
#[cfg(FALSE)] fn e() { let _ = if let _ = 0 {} else if let _ = 0 #[attr] {}; }
//~^ ERROR expected `{`, found `#`
#[cfg(FALSE)] fn e() { let _ = if let _ = 0 {} else if let _ = 0 {#![attr]}; }
//~^ ERROR an inner attribute is not permitted in this context

#[cfg(FALSE)] fn s() { #[attr] #![attr] let _ = 0; }
//~^ ERROR an inner attribute is not permitted following an outer attribute
#[cfg(FALSE)] fn s() { #[attr] #![attr] 0; }
//~^ ERROR an inner attribute is not permitted following an outer attribute
#[cfg(FALSE)] fn s() { #[attr] #![attr] foo!(); }
//~^ ERROR an inner attribute is not permitted following an outer attribute
#[cfg(FALSE)] fn s() { #[attr] #![attr] foo![]; }
//~^ ERROR an inner attribute is not permitted following an outer attribute
#[cfg(FALSE)] fn s() { #[attr] #![attr] foo!{}; }
//~^ ERROR an inner attribute is not permitted following an outer attribute

// FIXME: Allow attributes in pattern constexprs?
// note: requires parens in patterns to allow disambiguation

#[cfg(FALSE)] fn e() { match 0 { 0..=#[attr] 10 => () } }
//~^ ERROR `X..=` range patterns are not supported
//~| ERROR expected one of `=>`, `if`, or `|`, found `#`
#[cfg(FALSE)] fn e() { match 0 { 0..=#[attr] -10 => () } }
//~^ ERROR `X..=` range patterns are not supported
//~| ERROR expected one of `=>`, `if`, or `|`, found `#`
#[cfg(FALSE)] fn e() { match 0 { 0..=-#[attr] 10 => () } }
//~^ ERROR unexpected token: `#`
#[cfg(FALSE)] fn e() { match 0 { 0..=#[attr] FOO => () } }
//~^ ERROR `X..=` range patterns are not supported
//~| ERROR expected one of `=>`, `if`, or `|`, found `#`

#[cfg(FALSE)] fn e() { let _ = x.#![attr]foo(); }
//~^ ERROR unexpected token: `#`
//~| ERROR expected one of `.`
#[cfg(FALSE)] fn e() { let _ = x.#[attr]foo(); }
//~^ ERROR unexpected token: `#`
//~| ERROR expected one of `.`

// make sure we don't catch this bug again...
#[cfg(FALSE)] fn e() { { fn foo() { #[attr]; } } }
//~^ ERROR expected statement after outer attribute
#[cfg(FALSE)] fn e() { { fn foo() { #[attr] } } }
//~^ ERROR expected statement after outer attribute
