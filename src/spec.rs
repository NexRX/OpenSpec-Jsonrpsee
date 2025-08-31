//! This module contains just the model for the OpenRPC specification.
//!
//! The contents of this object represent a whole OpenRPC document.
//! How this object is constructed or stored is outside the scope
//! of the OpenRPC Specification.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use typed_builder::TypedBuilder;

type Schema = schemars::Schema;

/// The root object of the OpenRPC document semver **1.3.2**
///
/// The contents of this object represent a whole OpenRPC document.
/// How this object is constructed or stored is outside the scope
/// of the OpenRPC Specification.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(default, setter(strip_option)))]
#[serde(rename_all = "camelCase")]
pub struct OpenRpcSpec {
    /// OpenRPC Specification 1.3.2
    ///
    /// REQUIRED. This string MUST be the semantic version number of the OpenRPC Specification version
    /// that the OpenRPC document uses. The `openrpc` field SHOULD be used by tooling specifications
    /// and clients to interpret the OpenRPC document. This is not related to the API `info.version`.
    #[builder(default = "1.3.2".into())]
    #[builder(setter(skip))]
    openrpc: String,

    /// REQUIRED. Provides metadata about the API. The metadata MAY be used by tooling as required.
    #[builder(default = Info::builder().build(), setter(!strip_option))]
    pub info: Info,

    /// An array of Server Objects, which provide connectivity information to a target server.
    /// If not provided, or is an empty array, the default value would be a Server Object with a url `localhost`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<Server>>,

    /// REQUIRED. The available methods for the API. While it is required, the array may be empty
    /// (to handle security filtering, for example).
    #[builder(setter(!strip_option))]
    pub methods: Vec<Method>,

    /// An element to hold various schemas for the specification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Components>,

    /// Additional external documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentation>,
}

impl OpenRpcSpec {
    /// Creates a JSON string like [`OpenRpcSpec::to_string`] but with pretty formatting.
    /// This is also the same as using the format macro with the `#` flag i.e. `format!("{spec:#}")`.
    pub fn to_string_pretty(&self) -> String {
        format!("{self:#}")
    }
}

impl std::fmt::Display for OpenRpcSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let json_result = if f.alternate() {
            serde_json::to_string_pretty(self)
        } else {
            serde_json::to_string(self)
        };

        match json_result {
            Ok(json) => f.write_str(&json),
            Err(_) => f.write_str("<OpenRpcSpec: serialization failed>"),
        }
    }
}

/// Provides metadata about the API.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(default, setter(strip_option)))]
#[serde(rename_all = "camelCase")]
pub struct Info {
    /// REQUIRED. The title of the application.
    #[builder(default = env!("CARGO_PKG_NAME").to_string(), setter(!strip_option))]
    pub title: String,

    /// A verbose description of the application. Markdown syntax MAY be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// A URL to the Terms of Service for the API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_service: Option<String>,

    /// The contact information for the exposed API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,

    /// The license information for the exposed API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<License>,

    /// REQUIRED. The version of the OpenRPC document
    /// (distinct from the OpenRPC Specification version or the API implementation version).
    #[builder(default = env!("CARGO_PKG_VERSION").to_string(), setter(!strip_option))]
    pub version: String,
}

/// Contact information for the exposed API.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(default, setter(strip_option)))]
pub struct Contact {
    /// The identifying name of the contact person/organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The URL pointing to the contact information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The email address of the contact person/organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

/// License information for the exposed API.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(default, setter(strip_option)))]
pub struct License {
    /// REQUIRED. The license name used for the API.
    #[builder(setter(!strip_option))]
    pub name: String,
    /// A URL to the license used for the API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// An object representing a Server.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(default, setter(strip_option)))]
pub struct Server {
    /// REQUIRED. A name to be used as the canonical name for the server.
    #[builder(setter(!strip_option))]
    pub name: String,
    /// REQUIRED. A URL to the target host. May contain Server Variables and MAY be relative.
    #[builder(setter(!strip_option))]
    pub url: String,
    /// A short summary of what the server is.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// An optional string describing the host designated by the URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A map between a variable name and its value for URL template substitution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<HashMap<String, ServerVariable>>,
}

/// An object representing a Server Variable for server URL template substitution.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct ServerVariable {
    /// An enumeration of string values to be used if the substitution options are from a limited set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#enum: Option<Vec<String>>,
    /// REQUIRED. The default value to use for substitution if an alternate value is not supplied.
    #[builder(setter(!strip_option))]
    pub default: String,
    /// An optional description for the server variable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Describes the interface for a given method name.
/// The method name is used as the `method` field of the JSON-RPC body.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Method {
    /// REQUIRED. The canonical name for the method. Must be unique.
    pub name: String,
    /// A list of tags for API documentation control.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// A short summary of what the method does.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// A verbose explanation of the method behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Additional external documentation for this method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentation>,
    /// REQUIRED. A list of parameters applicable for this method.
    pub params: Vec<ContentDescriptor>,
    /// The description of the result returned by the method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<ContentDescriptor>,
    /// Declares this method to be deprecated. Default is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    /// An alternative servers array to service this method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<Server>>,
    /// A list of custom application-defined errors that MAY be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    /// A list of possible links from this method call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
    /// The expected format of the parameters. Defaults to "either".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param_structure: Option<String>,
    /// Example params-to-result pairings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub examples: Option<Vec<ExamplePairing>>,
}

/// Describes content for parameters or results. Must have a schema.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct ContentDescriptor {
    /// REQUIRED. Name of the content (also used as param key when by-name).
    pub name: String,
    /// A short summary of the content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// A verbose explanation of the content descriptor behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Determines if the content is required. Default = false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    /// REQUIRED. Schema that describes the content.
    pub schema: Schema,
    /// Specifies that the content is deprecated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
}

/// An example object intended to match the schema of a given Content Descriptor.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Example {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_value: Option<String>,
}

/// An example pairing of params and results.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct ExamplePairing {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    pub params: Vec<Example>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Example>,
}

/// Represents a possible design-time link for a result.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct Link {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<Server>,
}

/// Defines an application level error.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct Error {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// Holds a set of reusable objects.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Components {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_descriptors: Option<HashMap<String, ContentDescriptor>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schemas: Option<HashMap<String, Schema>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub examples: Option<HashMap<String, Example>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<HashMap<String, Link>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<HashMap<String, Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example_pairing_objects: Option<HashMap<String, ExamplePairing>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<HashMap<String, Tag>>,
}

/// Metadata for a tag.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentation>,
}

/// Allows referencing an external resource for extended documentation.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct ExternalDocumentation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub url: String,
}
