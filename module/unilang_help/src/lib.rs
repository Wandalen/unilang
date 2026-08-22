//! Reusable help-page domain model, verbosity levels, and renderers for
//! unilang-style CLIs.
//!
//! This crate carries no dependency on any command framework: the producing
//! side converts its own command definitions into the pre-rendered-string
//! model in [`model`], and every renderer consumes that model unchanged.
//!
//! ## Components
//!
//! - [`HelpCommandData`] / [`HelpParamData`] — renderer-independent help data
//!   for one command and its parameters, built from pre-rendered strings only.
//! - [`HelpVerbosity`] — five detail levels (Minimal..Comprehensive), settable
//!   from the `UNILANG_HELP_VERBOSITY` environment variable.
//! - [`HelpDisplayOptions`] — global visibility toggles for version, status,
//!   aliases, and tags; honors `UNILANG_HELP_HIDE_VERSION`.
//! - [`PlainRenderer`] — plain-text command pages at all five verbosity
//!   levels (line-faithful port of the original `unilang` formats) plus a
//!   parameter detail page.
//! - [`CliFmtRenderer`] — column-aligned, colour-aware command and parameter
//!   pages via `cli_fmt`'s detail-page template; mandatory whenever this
//!   crate is enabled, not a separately-optional feature.
//!
//! ## Example
//!
//! ```
//! use unilang_help::{ HelpCommandData, HelpParamData, HelpVerbosity, PlainRenderer };
//!
//! let mut param = HelpParamData::default();
//! param.name = "scope".into();
//! param.kind_compact = "enum".into();
//!
//! let mut cmd = HelpCommandData::default();
//! cmd.name = ".rollup".into();
//! cmd.description = "Aggregate readme files.".into();
//! cmd.params.push( param );
//!
//! let renderer = PlainRenderer::default().with_verbosity( HelpVerbosity::Basic );
//! let page = renderer.render( &cmd );
//! assert!( page.contains( ".rollup - Aggregate readme files." ) );
//! assert!( page.contains( "  scope::enum" ) );
//! ```
#![ cfg_attr( docsrs, feature( doc_auto_cfg ) ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_hr.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_hr.png" ) ]
#![ warn( missing_docs ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( rust_2018_idioms ) ]

/// Renderer-independent help-page data model.
#[ cfg( feature = "enabled" ) ]
pub mod model;
/// Verbosity levels and global display options.
#[ cfg( feature = "enabled" ) ]
pub mod verbosity;
/// Plain-text renderer with five verbosity levels.
#[ cfg( feature = "enabled" ) ]
pub mod plain;
/// `cli_fmt`-backed column-aligned renderer.
#[ cfg( feature = "enabled" ) ]
pub mod cli_fmt_renderer;

/// Prelude for commonly used items.
#[ cfg( feature = "enabled" ) ]
pub mod prelude
{
  pub use super::model::{ HelpCommandData, HelpParamData };
  pub use super::verbosity::{ HelpVerbosity, HelpDisplayOptions };
  pub use super::plain::PlainRenderer;
  pub use super::cli_fmt_renderer::CliFmtRenderer;
}

#[ cfg( feature = "enabled" ) ]
pub use prelude::*;
