// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p></p>
#[deprecated(note = "This structure is deprecated.")]
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DataProductItem {
    /// <p></p>
    pub item_id: ::std::option::Option<::std::string::String>,
    /// <p></p>
    pub domain_id: ::std::option::Option<::std::string::String>,
}
impl DataProductItem {
    /// <p></p>
    pub fn item_id(&self) -> ::std::option::Option<&str> {
        self.item_id.as_deref()
    }
    /// <p></p>
    pub fn domain_id(&self) -> ::std::option::Option<&str> {
        self.domain_id.as_deref()
    }
}
impl DataProductItem {
    /// Creates a new builder-style object to manufacture [`DataProductItem`](crate::types::DataProductItem).
    pub fn builder() -> crate::types::builders::DataProductItemBuilder {
        crate::types::builders::DataProductItemBuilder::default()
    }
}

/// A builder for [`DataProductItem`](crate::types::DataProductItem).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DataProductItemBuilder {
    pub(crate) item_id: ::std::option::Option<::std::string::String>,
    pub(crate) domain_id: ::std::option::Option<::std::string::String>,
}
impl DataProductItemBuilder {
    /// <p></p>
    pub fn item_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.item_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p></p>
    pub fn set_item_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.item_id = input;
        self
    }
    /// <p></p>
    pub fn get_item_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.item_id
    }
    /// <p></p>
    pub fn domain_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.domain_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p></p>
    pub fn set_domain_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.domain_id = input;
        self
    }
    /// <p></p>
    pub fn get_domain_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.domain_id
    }
    /// Consumes the builder and constructs a [`DataProductItem`](crate::types::DataProductItem).
    pub fn build(self) -> crate::types::DataProductItem {
        crate::types::DataProductItem {
            item_id: self.item_id,
            domain_id: self.domain_id,
        }
    }
}
