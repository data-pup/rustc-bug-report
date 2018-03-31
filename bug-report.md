# Bug Report

(Note: Below is the bug report template from `CONTRIBUTING.md`)

### Description

Expressions involving an adaptor function like `skip` can cause an internal
compiler error. If a line of code in a submodule uses an undeclared type
or module in an expression using an adaptor, the compiler panics at
`\'Box<Any>\', librustc_errors/lib.rs:543:9`.

Code in a submodule needs to specify a path relative to the crate root, which
means prefixing `::` to a call like `std::env::args()`.

## Example

We will use the following two files to demonstrate the issue:

_main.rs:_

```rust
mod lib;
use lib::foo;

fn main() {
    let _ = foo();
}
```

_lib.rs:_

```rust
pub fn foo() -> Vec<String> {
    std::env::args() // <- [ERROR E0433] This _should_ be `::std::env::args()`.
        .skip(1)     // <- Compiler does not panic if this line is commented.
        .collect()
}
```

Code in a submodule needs to specify a path relative to the crate root, which
means prefixing `::` to a call like `std::env::args()`. If we use
`::std::env::args()` the program compiles and runs successfully.

As written however, the files above cause this internal compiler error:

```
error[E0433]: failed to resolve. Use of undeclared type or module `std`
 --> src/args.rs:6:5
  |
6 |     std::env::args()
  |     ^^^ Use of undeclared type or module `std`

error: internal compiler error: librustc/ich/impls_ty.rs:906: ty::TypeVariants::hash_stable() - Unexpected variant TyInfer(?0).

thread 'rustc' panicked at 'Box<Any>', librustc_errors/lib.rs:543:9
```

After commenting out the `skip(1)` line, the compiler does not panic, and only
this output is seen after trying to compile:

```
error[E0433]: failed to resolve. Use of undeclared type or module `std`
 --> src/lib.rs:5:5
   |
 5 |     std::env::args()
   |     ^^^ Use of undeclared type or module `std`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0433`.
```


## Meta

`rustc --version --verbose`:

```
rustc 1.26.0-nightly (5508b2714 2018-03-18)
binary: rustc
commit-hash: 5508b27145cfb82896ae838e6aca0cd48750796f
commit-date: 2018-03-18
host: x86_64-unknown-linux-gnu
release: 1.26.0-nightly
LLVM version: 6.0
```

Backtrace:

```
   Compiling bug-report v0.1.0 (file:///home/user/Projects/rustc-bug)
error[E0433]: failed to resolve. Use of undeclared type or module `std`
 --> src/args.rs:6:5
  |
6 |     std::env::args()
  |     ^^^ Use of undeclared type or module `std`

error: internal compiler error: librustc/ich/impls_ty.rs:906: ty::TypeVariants::hash_stable() - Unexpected variant TyInfer(?0).

thread 'rustc' panicked at 'Box<Any>', librustc_errors/lib.rs:543:9
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::print
             at libstd/sys_common/backtrace.rs:71
             at libstd/sys_common/backtrace.rs:59
   2: std::panicking::default_hook::{{closure}}
             at libstd/panicking.rs:207
   3: std::panicking::default_hook
             at libstd/panicking.rs:223
   4: core::ops::function::Fn::call
   5: std::panicking::rust_panic_with_hook
             at libstd/panicking.rs:403
   6: std::panicking::begin_panic
   7: rustc_errors::Handler::bug
   8: rustc::session::opt_span_bug_fmt::{{closure}}
   9: rustc::ty::context::tls::with_opt::{{closure}}
  10: <std::thread::local::LocalKey<T>>::try_with
  11: <std::thread::local::LocalKey<T>>::with
  12: rustc::ty::context::tls::with
  13: rustc::ty::context::tls::with_opt
  14: rustc::session::opt_span_bug_fmt
  15: rustc::session::bug_fmt
  16: rustc::ich::impls_ty::<impl rustc_data_structures::stable_hasher::HashStable<rustc::ich::hcx::StableHashingContext<'a>> for rustc::ty::sty::TypeVariants<'gcx>>::hash_stable
  17: <T as rustc::dep_graph::dep_node::DepNodeParams<'a, 'gcx, 'tcx>>::to_fingerprint
  18: rustc::dep_graph::dep_node::DepNode::new
  19: rustc::ty::maps::<impl rustc::ty::maps::queries::dropck_outlives<'tcx>>::try_get
  20: rustc::ty::maps::TyCtxtAt::dropck_outlives
  21: rustc::traits::query::dropck_outlives::<impl rustc::infer::at::At<'cx, 'gcx, 'tcx>>::dropck_outlives
  22: rustc_typeck::check::dropck::check_safety_of_destructor_if_necessary
  23: rustc_typeck::check::regionck::RegionCtxt::check_safety_of_rvalue_destructor_if_necessary
  24: <rustc_typeck::check::regionck::RegionCtxt<'a, 'gcx, 'tcx> as rustc::hir::intravisit::Visitor<'gcx>>::visit_expr
  25: rustc::hir::intravisit::walk_expr
  26: <rustc_typeck::check::regionck::RegionCtxt<'a, 'gcx, 'tcx> as rustc::hir::intravisit::Visitor<'gcx>>::visit_expr
  27: rustc::hir::intravisit::walk_expr
  28: <rustc_typeck::check::regionck::RegionCtxt<'a, 'gcx, 'tcx> as rustc::hir::intravisit::Visitor<'gcx>>::visit_expr
  29: <rustc_typeck::check::regionck::RegionCtxt<'a, 'gcx, 'tcx> as rustc::hir::intravisit::Visitor<'gcx>>::visit_expr
  30: rustc_typeck::check::regionck::RegionCtxt::visit_fn_body
  31: rustc_typeck::check::regionck::<impl rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx>>::regionck_fn
  32: rustc::ty::context::tls::enter
  33: rustc::infer::InferCtxtBuilder::enter
  34: rustc_typeck::check::typeck_tables_of
  35: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables_of<'tcx>>::compute_result
  36: rustc::dep_graph::graph::DepGraph::with_task_impl
  37: rustc_errors::Handler::track_diagnostics
  38: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::cycle_check
  39: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables_of<'tcx>>::force
  40: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables_of<'tcx>>::try_get
  41: rustc::ty::maps::TyCtxtAt::typeck_tables_of
  42: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables_of<'tcx>>::ensure
  43: rustc::session::Session::track_errors
  44: rustc_typeck::check::typeck_item_bodies
  45: rustc::dep_graph::graph::DepGraph::with_task_impl
  46: rustc_errors::Handler::track_diagnostics
  47: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::cycle_check
  48: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_item_bodies<'tcx>>::force
  49: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_item_bodies<'tcx>>::try_get
  50: rustc::ty::maps::TyCtxtAt::typeck_item_bodies
  51: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::typeck_item_bodies
  52: rustc_typeck::check_crate
  53: <std::thread::local::LocalKey<T>>::with
  54: <std::thread::local::LocalKey<T>>::with
  55: rustc::ty::context::TyCtxt::create_and_enter
  56: rustc_driver::driver::compile_input
  57: rustc_driver::run_compiler_impl
  58: syntax::with_globals

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.26.0-nightly (5508b2714 2018-03-18) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `bug-report`.

To learn more, run the command again with --verbose.
```

### Potential UX Fixes

I ran into this error because the `::` error is not required for code in the
crate root. If `foo` is moved from `lib.rc` to `main.rs`, then the internal
compiler error no longer occurs.

If code outside of the crate root causes an error due to the use of an
undeclared type or module, it would be nice if the compiler warning would
mention the possibility of a missing `::` prefix, if the name that could not
be resolved was a type or module in the standard library.

When I reviewed the information given by running `rustc --explain E0433`, it
mostly mentions the possibility of typos being the cause of this error.
Mentioning some other common situations that lead to symbols failing to
resolve might be helpful.

