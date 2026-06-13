//! Serde Serialize/Deserialize implementations for CommandDefinition.

use super::super::command_name::CommandName;
use super::super::namespace_type::NamespaceType;
use super::super::version_type::VersionType;
use super::super::command_status::CommandStatus;
use super::super::argument_types::ArgumentDefinition;
use super::core::CommandDefinition;

impl serde::Serialize for CommandDefinition
{
  fn serialize< S >( &self, serializer : S ) -> Result< S::Ok, S::Error >
  where
    S : serde::Serializer,
  {
    use serde::ser::SerializeStruct;

    let mut state = serializer.serialize_struct( "CommandDefinition", 22 )?;

    state.serialize_field( "name", &self.name )?;
    state.serialize_field( "description", &self.description )?;
    state.serialize_field( "arguments", &self.arguments )?;
    state.serialize_field( "routine_link", &self.routine_link )?;
    state.serialize_field( "namespace", &self.namespace )?;
    state.serialize_field( "hint", &self.hint )?;
    state.serialize_field( "status", &self.status )?;
    state.serialize_field( "version", &self.version )?;
    state.serialize_field( "tags", &self.tags )?;
    state.serialize_field( "aliases", &self.aliases )?;
    state.serialize_field( "permissions", &self.permissions )?;
    state.serialize_field( "idempotent", &self.idempotent )?;
    state.serialize_field( "deprecation_message", &self.deprecation_message )?;
    state.serialize_field( "http_method_hint", &self.http_method_hint )?;
    state.serialize_field( "examples", &self.examples )?;
    state.serialize_field( "auto_help_enabled", &self.auto_help_enabled )?;
    state.serialize_field( "category", &self.category )?;
    state.serialize_field( "short_desc", &self.short_desc )?;
    state.serialize_field( "hidden_from_list", &self.hidden_from_list )?;
    state.serialize_field( "priority", &self.priority )?;
    state.serialize_field( "group", &self.group )?;
    state.serialize_field( "show_version_in_help", &self.show_version_in_help )?;

    state.end()
  }
}

impl< 'de > serde::Deserialize< 'de > for CommandDefinition
{
  #[ allow( clippy::too_many_lines ) ]
  fn deserialize< D >( deserializer : D ) -> Result< Self, D::Error >
  where
    D : serde::Deserializer< 'de >,
  {
    use serde::de::{ self, Visitor, MapAccess };

    #[ derive( serde::Deserialize ) ]
    #[ serde( field_identifier, rename_all = "snake_case" ) ]
    enum Field
    {
      Name,
      Description,
      Arguments,
      RoutineLink,
      Namespace,
      Hint,
      Status,
      Version,
      Tags,
      Aliases,
      Permissions,
      Idempotent,
      DeprecationMessage,
      HttpMethodHint,
      Examples,
      AutoHelpEnabled,
      Category,
      ShortDesc,
      HiddenFromList,
      Priority,
      Group,
      ShowVersionInHelp,
    }

    struct CommandDefinitionVisitor;

    impl< 'de > Visitor< 'de > for CommandDefinitionVisitor
    {
      type Value = CommandDefinition;

      fn expecting( &self, formatter : &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
      {
        formatter.write_str( "struct CommandDefinition" )
      }

      #[ allow( clippy::too_many_lines ) ]
      fn visit_map< V >( self, mut map : V ) -> Result< CommandDefinition, V::Error >
      where
        V : MapAccess< 'de >,
      {
        let mut name : Option< CommandName > = None;
        let mut description : Option< String > = None;
        let mut arguments : Option< Vec< ArgumentDefinition > > = None;
        let mut routine_link : Option< Option< String > > = None;
        let mut namespace : Option< String > = None;
        let mut hint : Option< String > = None;
        let mut status : Option< CommandStatus > = None;
        let mut version : Option< VersionType > = None;
        let mut tags : Option< Vec< String > > = None;
        let mut aliases : Option< Vec< String > > = None;
        let mut permissions : Option< Vec< String > > = None;
        let mut idempotent : Option< bool > = None;
        let mut deprecation_message : Option< String > = None;
        let mut http_method_hint : Option< String > = None;
        let mut examples : Option< Vec< String > > = None;
        let mut auto_help_enabled : Option< bool > = None;
        let mut category : Option< String > = None;
        let mut short_desc : Option< String > = None;
        let mut hidden_from_list : Option< bool > = None;
        let mut priority : Option< i32 > = None;
        let mut group : Option< String > = None;
        let mut show_version_in_help : Option< bool > = None;

        while let Some( key ) = map.next_key()?
        {
          match key
          {
            Field::Name =>
            {
              if name.is_some()
              {
                return Err( de::Error::duplicate_field( "name" ) );
              }
              name = Some( map.next_value()? );
            },
            Field::Description =>
            {
              if description.is_some()
              {
                return Err( de::Error::duplicate_field( "description" ) );
              }
              description = Some( map.next_value()? );
            },
            Field::Arguments =>
            {
              if arguments.is_some()
              {
                return Err( de::Error::duplicate_field( "arguments" ) );
              }
              arguments = Some( map.next_value()? );
            },
            Field::RoutineLink =>
            {
              if routine_link.is_some()
              {
                return Err( de::Error::duplicate_field( "routine_link" ) );
              }
              routine_link = Some( map.next_value()? );
            },
            Field::Namespace =>
            {
              if namespace.is_some()
              {
                return Err( de::Error::duplicate_field( "namespace" ) );
              }
              namespace = Some( map.next_value()? );
            },
            Field::Hint =>
            {
              if hint.is_some()
              {
                return Err( de::Error::duplicate_field( "hint" ) );
              }
              hint = Some( map.next_value()? );
            },
            Field::Status =>
            {
              if status.is_some()
              {
                return Err( de::Error::duplicate_field( "status" ) );
              }
              status = Some( map.next_value()? );
            },
            Field::Version =>
            {
              if version.is_some()
              {
                return Err( de::Error::duplicate_field( "version" ) );
              }
              version = Some( map.next_value()? );
            },
            Field::Tags =>
            {
              if tags.is_some()
              {
                return Err( de::Error::duplicate_field( "tags" ) );
              }
              tags = Some( map.next_value()? );
            },
            Field::Aliases =>
            {
              if aliases.is_some()
              {
                return Err( de::Error::duplicate_field( "aliases" ) );
              }
              aliases = Some( map.next_value()? );
            },
            Field::Permissions =>
            {
              if permissions.is_some()
              {
                return Err( de::Error::duplicate_field( "permissions" ) );
              }
              permissions = Some( map.next_value()? );
            },
            Field::Idempotent =>
            {
              if idempotent.is_some()
              {
                return Err( de::Error::duplicate_field( "idempotent" ) );
              }
              idempotent = Some( map.next_value()? );
            },
            Field::DeprecationMessage =>
            {
              if deprecation_message.is_some()
              {
                return Err( de::Error::duplicate_field( "deprecation_message" ) );
              }
              deprecation_message = Some( map.next_value()? );
            },
            Field::HttpMethodHint =>
            {
              if http_method_hint.is_some()
              {
                return Err( de::Error::duplicate_field( "http_method_hint" ) );
              }
              http_method_hint = Some( map.next_value()? );
            },
            Field::Examples =>
            {
              if examples.is_some()
              {
                return Err( de::Error::duplicate_field( "examples" ) );
              }
              examples = Some( map.next_value()? );
            },
            Field::AutoHelpEnabled =>
            {
              if auto_help_enabled.is_some()
              {
                return Err( de::Error::duplicate_field( "auto_help_enabled" ) );
              }
              auto_help_enabled = Some( map.next_value()? );
            },
            Field::Category =>
            {
              if category.is_some()
              {
                return Err( de::Error::duplicate_field( "category" ) );
              }
              category = Some( map.next_value()? );
            },
            Field::ShortDesc =>
            {
              if short_desc.is_some()
              {
                return Err( de::Error::duplicate_field( "short_desc" ) );
              }
              short_desc = Some( map.next_value()? );
            },
            Field::HiddenFromList =>
            {
              if hidden_from_list.is_some()
              {
                return Err( de::Error::duplicate_field( "hidden_from_list" ) );
              }
              hidden_from_list = Some( map.next_value()? );
            },
            Field::Priority =>
            {
              if priority.is_some()
              {
                return Err( de::Error::duplicate_field( "priority" ) );
              }
              priority = Some( map.next_value()? );
            },
            Field::Group =>
            {
              if group.is_some()
              {
                return Err( de::Error::duplicate_field( "group" ) );
              }
              group = Some( map.next_value()? );
            },
            Field::ShowVersionInHelp =>
            {
              if show_version_in_help.is_some()
              {
                return Err( de::Error::duplicate_field( "show_version_in_help" ) );
              }
              show_version_in_help = Some( map.next_value()? );
            },
          }
        }

        // Required fields
        let name = name.ok_or_else( || de::Error::missing_field( "name" ) )?;
        let description = description.ok_or_else( || de::Error::missing_field( "description" ) )?;

        // Optional fields with defaults
        let namespace = namespace.unwrap_or_default();

        // Validate namespace using NamespaceType validation rules
        NamespaceType::new( &namespace ).map_err( de::Error::custom )?;
        let hint = hint.unwrap_or_default();
        let status = status.unwrap_or( CommandStatus::Active );
        let version = version.unwrap_or_else( || VersionType::new( "1.0.0" ).expect( "default version valid" ) );
        let arguments = arguments.unwrap_or_default();
        let routine_link = routine_link.unwrap_or( None );
        let tags = tags.unwrap_or_default();
        let aliases = aliases.unwrap_or_default();
        let permissions = permissions.unwrap_or_default();
        let idempotent = idempotent.unwrap_or( true );
        let deprecation_message = deprecation_message.unwrap_or_default();
        let http_method_hint = http_method_hint.unwrap_or_else( || "GET".to_string() );
        let examples = examples.unwrap_or_default();
        let auto_help_enabled = auto_help_enabled.unwrap_or( true );
        let category = category.unwrap_or_default();
        let short_desc = short_desc.unwrap_or_default();
        let hidden_from_list = hidden_from_list.unwrap_or( false );
        let priority = priority.unwrap_or( 0 );
        let group = group.unwrap_or_default();
        let show_version_in_help = show_version_in_help.unwrap_or( true );

        Ok( CommandDefinition
        {
          name,
          description,
          arguments,
          routine_link,
          namespace,
          hint,
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
          short_desc,
          hidden_from_list,
          priority,
          group,
          show_version_in_help,
        })
      }
    }

    const FIELDS : &[ &str ] = &[
      "name",
      "description",
      "arguments",
      "routine_link",
      "namespace",
      "hint",
      "status",
      "version",
      "tags",
      "aliases",
      "permissions",
      "idempotent",
      "deprecation_message",
      "http_method_hint",
      "examples",
      "auto_help_enabled",
      "category",
      "short_desc",
      "hidden_from_list",
      "priority",
      "group",
      "show_version_in_help",
    ];

    deserializer.deserialize_struct( "CommandDefinition", FIELDS, CommandDefinitionVisitor )
  }
}
