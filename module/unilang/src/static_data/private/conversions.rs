//! Conversion impls from static command structs to dynamic data types.

use super::*;

  /// Converts static command definition to dynamic command definition
  ///
  /// # Status String Conversion
  ///
  /// Static status strings are converted to `CommandStatus` enum:
  /// - `"experimental"` → `CommandStatus::Experimental`
  /// - `"internal"` → `CommandStatus::Internal`
  /// - `"stable"`, `"active"`, or any other value → `CommandStatus::Active`
  ///
  /// **Important:** The string `"stable"` converts to `CommandStatus::Active`, which
  /// displays as `"active"`. Tests comparing status strings must expect `"active"`,
  /// not `"stable"`.
  ///
  /// If `deprecation_message` is non-empty, the status becomes `CommandStatus::Deprecated`
  /// regardless of the status string.
  impl From< &'static StaticCommandDefinition > for crate::data::CommandDefinition
  {
    fn from( static_cmd : &'static StaticCommandDefinition ) -> Self
    {
      use crate::data::{ CommandName, CommandStatus, VersionType };

      // Destructure all fields so the compiler errors if StaticCommandDefinition gains a new field
      // without this conversion being updated — prevents the silent field-loss bug pattern.
      let &StaticCommandDefinition
      {
        name,
        namespace,
        description,
        hint,
        arguments,
        routine_link,
        status,
        version,
        tags,
        aliases,
        permissions,
        idempotent,
        deprecation_message,
        http_method_hint,
        examples,
        auto_help_enabled,
        category,
        show_version_in_help,
      } = static_cmd;

      let cmd_status = if deprecation_message.is_empty()
      {
        // "stable" and unrecognized values map to Active
        CommandStatus::from_str_lossy( status )
      }
      else
      {
        CommandStatus::Deprecated
        {
          reason : deprecation_message.to_string(),
          since : None,
          replacement : None,
        }
      };

      crate::data::CommandDefinition::new(
        CommandName::new( name ).expect( "valid static command name" ),
        description.to_string(),
      )
      .with_namespace( namespace.to_string() )
      .with_hint( hint )
      .with_arguments( arguments.iter().map( core::convert::Into::into ).collect() )
      .with_routine_link( routine_link.map( str::to_string ) )
      .with_status( cmd_status )
      .with_version( VersionType::new( version ).expect( "valid static version" ) )
      .with_tags( tags.iter().map( | &s | s.to_string() ).collect() )
      .with_aliases( aliases.iter().map( | &s | s.to_string() ).collect() )
      .with_permissions( permissions.iter().map( | &s | s.to_string() ).collect() )
      .with_idempotent( idempotent )
      .with_deprecation_message( deprecation_message )
      .with_http_method_hint( http_method_hint )
      .with_examples( examples.iter().map( | &s | s.to_string() ).collect() )
      .with_auto_help( auto_help_enabled )
      .with_category( category )
      .with_short_desc( "" )
      .with_hidden_from_list( false )
      .with_priority( 0 )
      .with_group( "" )
      .with_show_version_in_help( show_version_in_help )
    }
  }

  impl From< &StaticArgumentDefinition > for crate::data::ArgumentDefinition
  {
    fn from( static_arg : &StaticArgumentDefinition ) -> Self
    {
      // Destructure by reference so the compiler errors if StaticArgumentDefinition gains a new field.
      let StaticArgumentDefinition { name, kind, attributes, hint, description, validation_rules, aliases, tags } = static_arg;
      crate::data::ArgumentDefinition
      {
        name : name.to_string(),
        kind : kind.into(),
        attributes : attributes.into(),
        hint : hint.to_string(),
        description : description.to_string(),
        validation_rules : validation_rules.iter().map( core::convert::Into::into ).collect(),
        aliases : aliases.iter().map( | &s | s.to_string() ).collect(),
        tags : tags.iter().map( | &s | s.to_string() ).collect(),
      }
    }
  }

  impl From< &StaticArgumentAttributes > for crate::data::ArgumentAttributes
  {
    fn from( static_attrs : &StaticArgumentAttributes ) -> Self
    {
      // Destructure-copy all fields so the compiler errors if StaticArgumentAttributes gains a new field.
      let &StaticArgumentAttributes { optional, multiple, default, sensitive, interactive } = static_attrs;
      crate::data::ArgumentAttributes
      {
        optional,
        multiple,
        default : default.map( str::to_string ),
        sensitive,
        interactive,
      }
    }
  }

  impl From< &StaticKind > for crate::data::Kind
  {
    fn from( static_kind : &StaticKind ) -> Self
    {
      match static_kind
      {
        StaticKind::String => crate::data::Kind::String,
        StaticKind::Integer => crate::data::Kind::Integer,
        StaticKind::Float => crate::data::Kind::Float,
        StaticKind::Boolean => crate::data::Kind::Boolean,
        StaticKind::Path => crate::data::Kind::Path,
        StaticKind::File => crate::data::Kind::File,
        StaticKind::Directory => crate::data::Kind::Directory,
        StaticKind::Enum( choices ) => crate::data::Kind::Enum( choices.iter().map( | &s | s.to_string() ).collect() ),
        StaticKind::Url => crate::data::Kind::Url,
        StaticKind::DateTime => crate::data::Kind::DateTime,
        StaticKind::Pattern => crate::data::Kind::Pattern,
        StaticKind::List( item_kind, delimiter ) => crate::data::Kind::List( Box::new( ( *item_kind ).into() ), *delimiter ),
        StaticKind::Map( key_kind, value_kind, entry_delimiter, kv_delimiter ) => 
          crate::data::Kind::Map( Box::new( ( *key_kind ).into() ), Box::new( ( *value_kind ).into() ), *entry_delimiter, *kv_delimiter ),
        StaticKind::JsonString => crate::data::Kind::JsonString,
        StaticKind::Object => crate::data::Kind::Object,
      }
    }
  }

  impl From< &StaticValidationRule > for crate::data::ValidationRule
  {
    fn from( static_rule : &StaticValidationRule ) -> Self
    {
      match static_rule
      {
        StaticValidationRule::Min( value ) => crate::data::ValidationRule::Min( *value ),
        StaticValidationRule::Max( value ) => crate::data::ValidationRule::Max( *value ),
        StaticValidationRule::MinLength( value ) => crate::data::ValidationRule::MinLength( *value ),
        StaticValidationRule::MaxLength( value ) => crate::data::ValidationRule::MaxLength( *value ),
        StaticValidationRule::Pattern( pattern ) => crate::data::ValidationRule::Pattern( (*pattern).to_string() ),
        StaticValidationRule::MinItems( value ) => crate::data::ValidationRule::MinItems( *value ),
      }
    }
  }
