// #![ cfg_attr( feature = "no_std", no_std ) ]
#![doc(html_logo_url = "https: //raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png")]
#![doc(
  html_favicon_url = "https: //raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico"
)]
#![doc(html_root_url = "https: //docs.rs/unilang_meta/latest/unilang_meta/")]
#![ cfg_attr( doc, doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "readme.md" ) ) ) ]
#![ cfg_attr( not( doc ), doc = "Universal language macro support" ) ]

extern crate proc_macro;

#[ cfg( feature = "enabled" ) ]
mod impl_core
{
  use macro_tools::prelude::*;
  use macro_tools::attr_prop::{ AttributePropertyComponent, AttributePropertySyn };

  // ============================================================
  // Attribute markers
  // ============================================================

  #[ derive( Debug, Default, Clone, Copy ) ]
  pub struct NameMarker;
  impl AttributePropertyComponent for NameMarker
  {
    const KEYWORD : &'static str = "name";
  }

  #[ derive( Debug, Default, Clone, Copy ) ]
  pub struct NamespaceMarker;
  impl AttributePropertyComponent for NamespaceMarker
  {
    const KEYWORD : &'static str = "namespace";
  }

  #[ derive( Debug, Default, Clone, Copy ) ]
  pub struct HintMarker;
  impl AttributePropertyComponent for HintMarker
  {
    const KEYWORD : &'static str = "hint";
  }

  #[ derive( Debug, Default, Clone, Copy ) ]
  pub struct DescriptionMarker;
  impl AttributePropertyComponent for DescriptionMarker
  {
    const KEYWORD : &'static str = "description";
  }

  // ============================================================
  // CommandAttributes — parsed from the macro attribute tokens
  // ============================================================

  #[ derive( Debug, Default ) ]
  pub struct CommandAttributes
  {
    pub name        : Option< AttributePropertySyn< syn::LitStr, NameMarker > >,
    pub namespace   : Option< AttributePropertySyn< syn::LitStr, NamespaceMarker > >,
    pub hint        : Option< AttributePropertySyn< syn::LitStr, HintMarker > >,
    pub description : Option< AttributePropertySyn< syn::LitStr, DescriptionMarker > >,
  }

  impl syn::parse::Parse for CommandAttributes
  {
    fn parse( input : syn::parse::ParseStream< '_ > ) -> syn::Result< Self >
    {
      let mut attrs = CommandAttributes::default();
      while !input.is_empty()
      {
        let ident : syn::Ident = input.parse()?;
        match ident.to_string().as_str()
        {
          NameMarker::KEYWORD        => attrs.name        = Some( input.parse()? ),
          NamespaceMarker::KEYWORD   => attrs.namespace   = Some( input.parse()? ),
          HintMarker::KEYWORD        => attrs.hint        = Some( input.parse()? ),
          DescriptionMarker::KEYWORD => attrs.description = Some( input.parse()? ),
          _ => return Err( syn::Error::new( ident.span(), format!( "unknown command attribute: {}", ident ) ) ),
        }
        if input.peek( syn::Token![ , ] )
        {
          let _ : syn::Token![ , ] = input.parse()?;
        }
      }
      Ok( attrs )
    }
  }

  // ============================================================
  // Parameter information for code generation
  // ============================================================

  pub struct ParamInfo
  {
    /// Parameter name as a string (for ArgumentDefinition name and error messages).
    pub name            : String,
    /// Parameter name as a syn::Ident (for generated let bindings and fn call).
    pub name_ident      : syn::Ident,
    /// Token stream for the Kind variant: `::unilang::data::Kind::String`, etc.
    pub kind_tokens     : proc_macro2::TokenStream,
    /// True when the Rust type is `Option<T>`.
    pub is_optional     : bool,
    /// Base type name for value extraction: "String", "i64", "bool", "PathBuf", etc.
    pub inner_type_name : String,
  }

  /// Maps a syn::Type to a (Kind tokens, is_optional, inner_type_name) triple.
  ///
  /// Supports: String, i64/i32/u64/u32/usize/isize, bool, PathBuf, Option<T>.
  /// Returns a syn::Error for any unsupported type.
  pub fn type_to_kind_and_inner(
    ty : &syn::Type,
  ) -> macro_tools::Result< ( proc_macro2::TokenStream, bool, String ) >
  {
    let syn::Type::Path( type_path ) = ty
    else
    {
      return Err( syn_err!( _, "unsupported parameter type: expected a path type" ) );
    };

    let last = type_path.path.segments.last()
      .ok_or_else( || syn_err!( _, "unsupported parameter type: empty path" ) )?;
    let ident_str = last.ident.to_string();

    match ident_str.as_str()
    {
      "String"  => Ok( ( qt! { ::unilang::data::Kind::String  }, false, "String".into()  ) ),
      "bool"    => Ok( ( qt! { ::unilang::data::Kind::Boolean }, false, "bool".into()    ) ),
      "PathBuf" => Ok( ( qt! { ::unilang::data::Kind::Path    }, false, "PathBuf".into() ) ),
      "i64"     => Ok( ( qt! { ::unilang::data::Kind::Integer }, false, "i64".into()     ) ),
      "i32"     => Ok( ( qt! { ::unilang::data::Kind::Integer }, false, "i32".into()     ) ),
      "u64"     => Ok( ( qt! { ::unilang::data::Kind::Integer }, false, "u64".into()     ) ),
      "u32"     => Ok( ( qt! { ::unilang::data::Kind::Integer }, false, "u32".into()     ) ),
      "usize"   => Ok( ( qt! { ::unilang::data::Kind::Integer }, false, "usize".into()   ) ),
      "isize"   => Ok( ( qt! { ::unilang::data::Kind::Integer }, false, "isize".into()   ) ),
      "Option"  =>
      {
        let syn::PathArguments::AngleBracketed( ref angle ) = last.arguments
        else
        {
          return Err( syn_err!( last.ident.span(), "Option requires a type argument: Option<T>" ) );
        };
        let inner = angle.args.first()
          .ok_or_else( || syn_err!( last.ident.span(), "Option requires a type argument" ) )?;
        let syn::GenericArgument::Type( inner_ty ) = inner
        else
        {
          return Err( syn_err!( last.ident.span(), "Option argument must be a type" ) );
        };
        let ( kind_tokens, _, inner_name ) = type_to_kind_and_inner( inner_ty )?;
        Ok( ( kind_tokens, true, inner_name ) )
      }
      _ => Err( syn_err!( last.ident.span(), "unsupported parameter type: {}", ident_str ) ),
    }
  }

  // ============================================================
  // Code generation helpers
  // ============================================================

  /// Generates the value-match arm body for extracting a Value variant.
  pub fn gen_value_match( inner_type_name : &str ) -> proc_macro2::TokenStream
  {
    match inner_type_name
    {
      "String"  => qt! { if let ::unilang::types::Value::String(  s ) = v { Some( s.clone() ) } else { None } },
      "bool"    => qt! { if let ::unilang::types::Value::Boolean( b ) = v { Some( *b ) }         else { None } },
      "PathBuf" => qt! { if let ::unilang::types::Value::Path(    p ) = v { Some( p.clone() ) }  else { None } },
      "i64"     => qt! { if let ::unilang::types::Value::Integer( i ) = v { Some( *i ) }         else { None } },
      "i32"     => qt! { if let ::unilang::types::Value::Integer( i ) = v { Some( *i as i32 ) }  else { None } },
      "u64"     => qt! { if let ::unilang::types::Value::Integer( i ) = v { Some( *i as u64 ) }  else { None } },
      "u32"     => qt! { if let ::unilang::types::Value::Integer( i ) = v { Some( *i as u32 ) }  else { None } },
      "usize"   => qt! { if let ::unilang::types::Value::Integer( i ) = v { Some( *i as usize ) } else { None } },
      "isize"   => qt! { if let ::unilang::types::Value::Integer( i ) = v { Some( *i as isize ) } else { None } },
      _         => qt! { None },
    }
  }

  /// Generates the `let param = ...` extraction statement for one parameter.
  pub fn gen_extraction( info : &ParamInfo ) -> proc_macro2::TokenStream
  {
    let name_lit    = syn::LitStr::new( &info.name, info.name_ident.span() );
    let ident       = &info.name_ident;
    let miss_msg    = format!( "Missing required argument: {}", info.name );
    let value_match = gen_value_match( &info.inner_type_name );

    if info.is_optional
    {
      qt!
      {
        let #ident = command.arguments.get( #name_lit )
          .and_then( | v | #value_match );
      }
    }
    else
    {
      qt!
      {
        let #ident = command.arguments.get( #name_lit )
          .and_then( | v | #value_match )
          .ok_or_else( || ::unilang::data::ErrorData::new(
            ::unilang::data::ErrorCode::ArgumentMissing,
            format!( #miss_msg ),
          ) )?;
      }
    }
  }

  /// Generates an `ArgumentDefinition::new(...)` expression for one parameter.
  pub fn gen_arg_def( info : &ParamInfo ) -> proc_macro2::TokenStream
  {
    let name_lit = syn::LitStr::new( &info.name, proc_macro2::Span::call_site() );
    let kind     = &info.kind_tokens;

    if info.is_optional
    {
      qt!
      {
        ::unilang::data::ArgumentDefinition::new( #name_lit, #kind )
          .with_optional( None ::< &str > )
      }
    }
    else
    {
      qt!
      {
        ::unilang::data::ArgumentDefinition::new( #name_lit, #kind )
      }
    }
  }

  // ============================================================
  // Core implementation
  // ============================================================

  pub fn command_impl(
    attr : proc_macro2::TokenStream,
    item : proc_macro2::TokenStream,
  ) -> macro_tools::Result< proc_macro2::TokenStream >
  {
    // Parse attribute tokens into CommandAttributes
    let cmd_attrs : CommandAttributes = syn::parse2( attr )?;

    // `name` is required
    let name_prop = cmd_attrs.name
      .ok_or_else( || syn_err!( _, "name attribute is required" ) )?;
    let name_lit : syn::LitStr = name_prop.ref_internal().clone();
    let name_str = name_lit.value();

    // The annotated item must be a function
    let func : syn::ItemFn = syn::parse2( item )
      .map_err( | e | syn::Error::new( e.span(), "unilang::command can only be applied to functions" ) )?;
    let fn_ident = func.sig.ident.clone();

    // Collect parameters (excluding `self`)
    let mut params : Vec< ParamInfo > = Vec::new();
    for input in &func.sig.inputs
    {
      let syn::FnArg::Typed( pat_type ) = input else { continue };
      let syn::Pat::Ident( pat_ident )  = &*pat_type.pat else { continue };
      let ( kind_tokens, is_optional, inner_type_name ) =
        type_to_kind_and_inner( &pat_type.ty )?;
      params.push( ParamInfo
      {
        name            : pat_ident.ident.to_string(),
        name_ident      : pat_ident.ident.clone(),
        kind_tokens,
        is_optional,
        inner_type_name,
      });
    }

    // Generate derived identifiers
    let fn_name_upper = fn_ident.to_string().to_uppercase();
    let static_ident   = format_ident!( "__UNILANG_DEF_{}", fn_name_upper );
    let wrapper_ident  = format_ident!( "__unilang_wrapper_{}", fn_ident );
    let register_ident = format_ident!( "__unilang_register_{}", fn_ident );

    // Collect generated code fragments
    let extractions : Vec< proc_macro2::TokenStream > =
      params.iter().map( gen_extraction ).collect();
    let arg_defs : Vec< proc_macro2::TokenStream > =
      params.iter().map( gen_arg_def ).collect();
    let param_idents : Vec< &syn::Ident > =
      params.iter().map( | p | &p.name_ident ).collect();

    // description: use provided value or fall back to name
    let description_str = cmd_attrs.description
      .as_ref()
      .map( | d | d.ref_internal().value() )
      .unwrap_or_else( || name_str.clone() );
    let description_lit =
      syn::LitStr::new( &description_str, proc_macro2::Span::call_site() );

    // namespace: use provided value or empty (root-level command)
    let namespace_str = cmd_attrs.namespace
      .as_ref()
      .map( | ns | ns.ref_internal().value() )
      .unwrap_or_default();
    let namespace_lit =
      syn::LitStr::new( &namespace_str, proc_macro2::Span::call_site() );

    // hint: use provided value or empty
    let hint_str = cmd_attrs.hint
      .as_ref()
      .map( | h | h.ref_internal().value() )
      .unwrap_or_default();
    let hint_lit = syn::LitStr::new( &hint_str, proc_macro2::Span::call_site() );

    let output = qt!
    {
      // --- Original user function kept unchanged ---
      #func

      // --- Static storage (lazy-initialised) ---
      static #static_ident : ::std::sync::OnceLock< ::unilang::data::CommandDefinition >
        = ::std::sync::OnceLock::new();

      // --- Interpreter-compatible wrapper ---
      #[ allow( non_snake_case ) ]
      fn #wrapper_ident(
        command  : ::unilang::semantic::VerifiedCommand,
        _context : ::unilang::interpreter::ExecutionContext,
      ) -> ::std::result::Result< ::unilang::data::OutputData, ::unilang::data::ErrorData >
      {
        #( #extractions )*
        let result = #fn_ident( #( #param_idents ),* );
        ::std::result::Result::Ok( ::unilang::data::OutputData::new( result, "text" ) )
      }

      // --- Public registration accessor ---
      pub fn #register_ident() -> &'static ::unilang::data::CommandDefinition
      {
        #static_ident.get_or_init( ||
        {
          let mut def = ::unilang::data::CommandDefinition::former()
            .name( #name_lit )
            .description( #description_lit )
            .hint( #hint_lit )
            .arguments( vec![ #( #arg_defs ),* ] )
            .end();
          def.namespace = #namespace_lit.to_string();
          def
        })
      }
    };

    Ok( output )
  }
}

///
/// Attribute macro for defining unilang commands as plain Rust functions.
///
/// The macro inspects the annotated function, infers argument metadata from
/// parameter names and types, and generates:
///
/// - A `static OnceLock<CommandDefinition>` populated from the macro attributes
///   and inferred parameter list.
/// - A wrapper function bridging the interpreter signature
///   `fn(VerifiedCommand, ExecutionContext) -> Result<OutputData, ErrorData>`
///   to the user's simpler parameter-by-name signature.
/// - A public registration accessor `fn __unilang_register_<name>() -> &'static CommandDefinition`.
///
/// # Required attributes
///
/// - `name = ".<name>"` — command name with dot prefix (e.g. `".greet"`).
///
/// # Optional attributes
///
/// - `namespace = ".<ns>"` — command namespace (default: empty = root).
/// - `description = "..."` — long description (default: same as name).
/// - `hint = "..."` — short hint for help listings (default: empty).
///
/// # Supported parameter types
///
/// `String`, `bool`, `PathBuf`, `i64`, `i32`, `u64`, `u32`, `usize`, `isize`,
/// and `Option<T>` for each of those (produces an optional argument).
///
/// # Example
///
/// ```ignore
/// #[ unilang_meta::command( name = ".greet", description = "Greet the user" ) ]
/// fn greet( name : String ) -> String
/// {
///   format!( "Hello, {}!", name )
/// }
///
/// // Generated:
/// // pub fn __unilang_register_greet() -> &'static unilang::data::CommandDefinition
/// // fn __unilang_wrapper_greet(command, context) -> Result<OutputData, ErrorData>
/// ```
#[ cfg( feature = "enabled" ) ]
#[ proc_macro_attribute ]
pub fn command(
  attr : proc_macro::TokenStream,
  item : proc_macro::TokenStream,
) -> proc_macro::TokenStream
{
  match impl_core::command_impl( attr.into(), item.into() )
  {
    Ok( tokens ) => tokens.into(),
    Err( err )   => err.to_compile_error().into(),
  }
}
