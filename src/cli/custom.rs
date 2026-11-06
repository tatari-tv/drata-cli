//! clap types for `drata custom-field ...` (custom field definitions) and
//! `drata custom-connection ...`.

use clap::{Subcommand, ValueEnum};

/// The resource a custom field is placed on (spec `CustomFieldsEntityTypeEnum`).
#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CustomFieldEntityType {
    Risk,
    Control,
    Asset,
    Personnel,
    Vendor,
    FrameworkRequirements,
    Policy,
}

/// The data type of a field `custom-field create` can make: the spec's
/// `fieldType` enum, which leaves out `FORMULA` (computed by the platform).
#[derive(ValueEnum, Clone, Debug, PartialEq, Eq)]
#[clap(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CustomFieldType {
    Text,
    Number,
    Currency,
    Options,
    OptionsNumeric,
    LongText,
    Url,
    Date,
}

#[derive(Subcommand, Debug)]
pub enum CustomFieldAction {
    /// List custom field definitions
    List {
        /// Only fields that can be set on this resource type
        #[arg(long, value_enum, ignore_case = true)]
        entity_type: Option<CustomFieldEntityType>,
        /// Only hidden (true) or visible (false) fields
        #[arg(long)]
        hidden: Option<bool>,
    },
    /// Get a custom field definition by ID, or by `name:<exact field name>`
    Get {
        /// Custom field ID, or `name:<field name>`
        custom_field_id: String,
    },
    /// Create a custom field definition (placements and non-text options go through --data)
    Create {
        /// Field name, unique in the account (max 191 chars; required unless --data/--example)
        #[arg(long, conflicts_with = "data", required_unless_present_any = ["data", "example"])]
        name: Option<String>,
        /// What the field is for (max 30000 chars)
        #[arg(long, conflicts_with = "data")]
        description: Option<String>,
        /// Data type (required unless --data/--example)
        #[arg(
            long,
            value_enum,
            ignore_case = true,
            conflicts_with = "data",
            required_unless_present_any = ["data", "example"]
        )]
        field_type: Option<CustomFieldType>,
        /// Hide the field
        #[arg(long, conflicts_with = "data")]
        hidden: bool,
        /// Require the field when creating a resource it is placed on
        #[arg(long, conflicts_with = "data")]
        required: bool,
        /// Selectable values of an OPTIONS field, in order (space-separated, 2 to 100)
        #[arg(long, num_args = 1.., conflicts_with = "data")]
        option: Vec<String>,
        /// Full request body: inline JSON, @file, or - for stdin (overrides individual flags)
        #[arg(long)]
        data: Option<String>,
        /// Print a JSON skeleton and exit (no API call)
        #[arg(long)]
        example: bool,
    },
}

#[derive(Subcommand, Debug)]
pub enum CustomConnectionAction {
    /// The fields of a Custom Connection resource that custom monitoring test logic can evaluate
    ResourceAttribute {
        /// Workspace ID
        workspace_id: String,
        /// Custom Connection ID, or `name:<clientAlias>`
        connection_id: String,
        /// Custom Resource ID, or `name:<resource name>`
        resource_id: String,
    },
}
