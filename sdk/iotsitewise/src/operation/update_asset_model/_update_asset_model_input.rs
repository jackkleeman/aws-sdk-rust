// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateAssetModelInput {
    /// <p>The ID of the asset model to update. This can be either the actual ID in UUID format, or else <code>externalId:</code> followed by the external ID, if it has one. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/object-ids.html#external-id-references">Referencing objects with external IDs</a> in the <i>IoT SiteWise User Guide</i>.</p>
    pub asset_model_id: ::std::option::Option<::std::string::String>,
    /// <p>A unique, friendly name for the asset model.</p>
    pub asset_model_name: ::std::option::Option<::std::string::String>,
    /// <p>A description for the asset model.</p>
    pub asset_model_description: ::std::option::Option<::std::string::String>,
    /// <p>The updated property definitions of the asset model. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/asset-properties.html">Asset properties</a> in the <i>IoT SiteWise User Guide</i>.</p>
    /// <p>You can specify up to 200 properties per asset model. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/quotas.html">Quotas</a> in the <i>IoT SiteWise User Guide</i>.</p>
    pub asset_model_properties: ::std::option::Option<::std::vec::Vec<crate::types::AssetModelProperty>>,
    /// <p>The updated hierarchy definitions of the asset model. Each hierarchy specifies an asset model whose assets can be children of any other assets created from this asset model. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/asset-hierarchies.html">Asset hierarchies</a> in the <i>IoT SiteWise User Guide</i>.</p>
    /// <p>You can specify up to 10 hierarchies per asset model. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/quotas.html">Quotas</a> in the <i>IoT SiteWise User Guide</i>.</p>
    pub asset_model_hierarchies: ::std::option::Option<::std::vec::Vec<crate::types::AssetModelHierarchy>>,
    /// <p>The composite models that are part of this asset model. It groups properties (such as attributes, measurements, transforms, and metrics) and child composite models that model parts of your industrial equipment. Each composite model has a type that defines the properties that the composite model supports. Use composite models to define alarms on this asset model.</p><note>
    /// <p>When creating custom composite models, you need to use <a href="https://docs.aws.amazon.com/iot-sitewise/latest/APIReference/API_CreateAssetModelCompositeModel.html">CreateAssetModelCompositeModel</a>. For more information, see
    /// <link>.</p>
    /// </note>
    pub asset_model_composite_models: ::std::option::Option<::std::vec::Vec<crate::types::AssetModelCompositeModel>>,
    /// <p>A unique case-sensitive identifier that you can provide to ensure the idempotency of the request. Don't reuse this client token if a new idempotent request is required.</p>
    pub client_token: ::std::option::Option<::std::string::String>,
    /// <p>An external ID to assign to the asset model. The asset model must not already have an external ID. The external ID must be unique within your Amazon Web Services account. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/object-ids.html#external-ids">Using external IDs</a> in the <i>IoT SiteWise User Guide</i>.</p>
    pub asset_model_external_id: ::std::option::Option<::std::string::String>,
}
impl UpdateAssetModelInput {
    /// <p>The ID of the asset model to update. This can be either the actual ID in UUID format, or else <code>externalId:</code> followed by the external ID, if it has one. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/object-ids.html#external-id-references">Referencing objects with external IDs</a> in the <i>IoT SiteWise User Guide</i>.</p>
    pub fn asset_model_id(&self) -> ::std::option::Option<&str> {
        self.asset_model_id.as_deref()
    }
    /// <p>A unique, friendly name for the asset model.</p>
    pub fn asset_model_name(&self) -> ::std::option::Option<&str> {
        self.asset_model_name.as_deref()
    }
    /// <p>A description for the asset model.</p>
    pub fn asset_model_description(&self) -> ::std::option::Option<&str> {
        self.asset_model_description.as_deref()
    }
    /// <p>The updated property definitions of the asset model. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/asset-properties.html">Asset properties</a> in the <i>IoT SiteWise User Guide</i>.</p>
    /// <p>You can specify up to 200 properties per asset model. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/quotas.html">Quotas</a> in the <i>IoT SiteWise User Guide</i>.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.asset_model_properties.is_none()`.
    pub fn asset_model_properties(&self) -> &[crate::types::AssetModelProperty] {
        self.asset_model_properties.as_deref().unwrap_or_default()
    }
    /// <p>The updated hierarchy definitions of the asset model. Each hierarchy specifies an asset model whose assets can be children of any other assets created from this asset model. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/asset-hierarchies.html">Asset hierarchies</a> in the <i>IoT SiteWise User Guide</i>.</p>
    /// <p>You can specify up to 10 hierarchies per asset model. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/quotas.html">Quotas</a> in the <i>IoT SiteWise User Guide</i>.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.asset_model_hierarchies.is_none()`.
    pub fn asset_model_hierarchies(&self) -> &[crate::types::AssetModelHierarchy] {
        self.asset_model_hierarchies.as_deref().unwrap_or_default()
    }
    /// <p>The composite models that are part of this asset model. It groups properties (such as attributes, measurements, transforms, and metrics) and child composite models that model parts of your industrial equipment. Each composite model has a type that defines the properties that the composite model supports. Use composite models to define alarms on this asset model.</p><note>
    /// <p>When creating custom composite models, you need to use <a href="https://docs.aws.amazon.com/iot-sitewise/latest/APIReference/API_CreateAssetModelCompositeModel.html">CreateAssetModelCompositeModel</a>. For more information, see
    /// <link>.</p>
    /// </note>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.asset_model_composite_models.is_none()`.
    pub fn asset_model_composite_models(&self) -> &[crate::types::AssetModelCompositeModel] {
        self.asset_model_composite_models.as_deref().unwrap_or_default()
    }
    /// <p>A unique case-sensitive identifier that you can provide to ensure the idempotency of the request. Don't reuse this client token if a new idempotent request is required.</p>
    pub fn client_token(&self) -> ::std::option::Option<&str> {
        self.client_token.as_deref()
    }
    /// <p>An external ID to assign to the asset model. The asset model must not already have an external ID. The external ID must be unique within your Amazon Web Services account. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/object-ids.html#external-ids">Using external IDs</a> in the <i>IoT SiteWise User Guide</i>.</p>
    pub fn asset_model_external_id(&self) -> ::std::option::Option<&str> {
        self.asset_model_external_id.as_deref()
    }
}
impl UpdateAssetModelInput {
    /// Creates a new builder-style object to manufacture [`UpdateAssetModelInput`](crate::operation::update_asset_model::UpdateAssetModelInput).
    pub fn builder() -> crate::operation::update_asset_model::builders::UpdateAssetModelInputBuilder {
        crate::operation::update_asset_model::builders::UpdateAssetModelInputBuilder::default()
    }
}

/// A builder for [`UpdateAssetModelInput`](crate::operation::update_asset_model::UpdateAssetModelInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct UpdateAssetModelInputBuilder {
    pub(crate) asset_model_id: ::std::option::Option<::std::string::String>,
    pub(crate) asset_model_name: ::std::option::Option<::std::string::String>,
    pub(crate) asset_model_description: ::std::option::Option<::std::string::String>,
    pub(crate) asset_model_properties: ::std::option::Option<::std::vec::Vec<crate::types::AssetModelProperty>>,
    pub(crate) asset_model_hierarchies: ::std::option::Option<::std::vec::Vec<crate::types::AssetModelHierarchy>>,
    pub(crate) asset_model_composite_models: ::std::option::Option<::std::vec::Vec<crate::types::AssetModelCompositeModel>>,
    pub(crate) client_token: ::std::option::Option<::std::string::String>,
    pub(crate) asset_model_external_id: ::std::option::Option<::std::string::String>,
}
impl UpdateAssetModelInputBuilder {
    /// <p>The ID of the asset model to update. This can be either the actual ID in UUID format, or else <code>externalId:</code> followed by the external ID, if it has one. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/object-ids.html#external-id-references">Referencing objects with external IDs</a> in the <i>IoT SiteWise User Guide</i>.</p>
    /// This field is required.
    pub fn asset_model_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.asset_model_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the asset model to update. This can be either the actual ID in UUID format, or else <code>externalId:</code> followed by the external ID, if it has one. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/object-ids.html#external-id-references">Referencing objects with external IDs</a> in the <i>IoT SiteWise User Guide</i>.</p>
    pub fn set_asset_model_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.asset_model_id = input;
        self
    }
    /// <p>The ID of the asset model to update. This can be either the actual ID in UUID format, or else <code>externalId:</code> followed by the external ID, if it has one. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/object-ids.html#external-id-references">Referencing objects with external IDs</a> in the <i>IoT SiteWise User Guide</i>.</p>
    pub fn get_asset_model_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.asset_model_id
    }
    /// <p>A unique, friendly name for the asset model.</p>
    /// This field is required.
    pub fn asset_model_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.asset_model_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A unique, friendly name for the asset model.</p>
    pub fn set_asset_model_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.asset_model_name = input;
        self
    }
    /// <p>A unique, friendly name for the asset model.</p>
    pub fn get_asset_model_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.asset_model_name
    }
    /// <p>A description for the asset model.</p>
    pub fn asset_model_description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.asset_model_description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A description for the asset model.</p>
    pub fn set_asset_model_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.asset_model_description = input;
        self
    }
    /// <p>A description for the asset model.</p>
    pub fn get_asset_model_description(&self) -> &::std::option::Option<::std::string::String> {
        &self.asset_model_description
    }
    /// Appends an item to `asset_model_properties`.
    ///
    /// To override the contents of this collection use [`set_asset_model_properties`](Self::set_asset_model_properties).
    ///
    /// <p>The updated property definitions of the asset model. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/asset-properties.html">Asset properties</a> in the <i>IoT SiteWise User Guide</i>.</p>
    /// <p>You can specify up to 200 properties per asset model. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/quotas.html">Quotas</a> in the <i>IoT SiteWise User Guide</i>.</p>
    pub fn asset_model_properties(mut self, input: crate::types::AssetModelProperty) -> Self {
        let mut v = self.asset_model_properties.unwrap_or_default();
        v.push(input);
        self.asset_model_properties = ::std::option::Option::Some(v);
        self
    }
    /// <p>The updated property definitions of the asset model. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/asset-properties.html">Asset properties</a> in the <i>IoT SiteWise User Guide</i>.</p>
    /// <p>You can specify up to 200 properties per asset model. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/quotas.html">Quotas</a> in the <i>IoT SiteWise User Guide</i>.</p>
    pub fn set_asset_model_properties(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::AssetModelProperty>>) -> Self {
        self.asset_model_properties = input;
        self
    }
    /// <p>The updated property definitions of the asset model. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/asset-properties.html">Asset properties</a> in the <i>IoT SiteWise User Guide</i>.</p>
    /// <p>You can specify up to 200 properties per asset model. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/quotas.html">Quotas</a> in the <i>IoT SiteWise User Guide</i>.</p>
    pub fn get_asset_model_properties(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::AssetModelProperty>> {
        &self.asset_model_properties
    }
    /// Appends an item to `asset_model_hierarchies`.
    ///
    /// To override the contents of this collection use [`set_asset_model_hierarchies`](Self::set_asset_model_hierarchies).
    ///
    /// <p>The updated hierarchy definitions of the asset model. Each hierarchy specifies an asset model whose assets can be children of any other assets created from this asset model. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/asset-hierarchies.html">Asset hierarchies</a> in the <i>IoT SiteWise User Guide</i>.</p>
    /// <p>You can specify up to 10 hierarchies per asset model. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/quotas.html">Quotas</a> in the <i>IoT SiteWise User Guide</i>.</p>
    pub fn asset_model_hierarchies(mut self, input: crate::types::AssetModelHierarchy) -> Self {
        let mut v = self.asset_model_hierarchies.unwrap_or_default();
        v.push(input);
        self.asset_model_hierarchies = ::std::option::Option::Some(v);
        self
    }
    /// <p>The updated hierarchy definitions of the asset model. Each hierarchy specifies an asset model whose assets can be children of any other assets created from this asset model. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/asset-hierarchies.html">Asset hierarchies</a> in the <i>IoT SiteWise User Guide</i>.</p>
    /// <p>You can specify up to 10 hierarchies per asset model. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/quotas.html">Quotas</a> in the <i>IoT SiteWise User Guide</i>.</p>
    pub fn set_asset_model_hierarchies(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::AssetModelHierarchy>>) -> Self {
        self.asset_model_hierarchies = input;
        self
    }
    /// <p>The updated hierarchy definitions of the asset model. Each hierarchy specifies an asset model whose assets can be children of any other assets created from this asset model. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/asset-hierarchies.html">Asset hierarchies</a> in the <i>IoT SiteWise User Guide</i>.</p>
    /// <p>You can specify up to 10 hierarchies per asset model. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/quotas.html">Quotas</a> in the <i>IoT SiteWise User Guide</i>.</p>
    pub fn get_asset_model_hierarchies(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::AssetModelHierarchy>> {
        &self.asset_model_hierarchies
    }
    /// Appends an item to `asset_model_composite_models`.
    ///
    /// To override the contents of this collection use [`set_asset_model_composite_models`](Self::set_asset_model_composite_models).
    ///
    /// <p>The composite models that are part of this asset model. It groups properties (such as attributes, measurements, transforms, and metrics) and child composite models that model parts of your industrial equipment. Each composite model has a type that defines the properties that the composite model supports. Use composite models to define alarms on this asset model.</p><note>
    /// <p>When creating custom composite models, you need to use <a href="https://docs.aws.amazon.com/iot-sitewise/latest/APIReference/API_CreateAssetModelCompositeModel.html">CreateAssetModelCompositeModel</a>. For more information, see
    /// <link>.</p>
    /// </note>
    pub fn asset_model_composite_models(mut self, input: crate::types::AssetModelCompositeModel) -> Self {
        let mut v = self.asset_model_composite_models.unwrap_or_default();
        v.push(input);
        self.asset_model_composite_models = ::std::option::Option::Some(v);
        self
    }
    /// <p>The composite models that are part of this asset model. It groups properties (such as attributes, measurements, transforms, and metrics) and child composite models that model parts of your industrial equipment. Each composite model has a type that defines the properties that the composite model supports. Use composite models to define alarms on this asset model.</p><note>
    /// <p>When creating custom composite models, you need to use <a href="https://docs.aws.amazon.com/iot-sitewise/latest/APIReference/API_CreateAssetModelCompositeModel.html">CreateAssetModelCompositeModel</a>. For more information, see
    /// <link>.</p>
    /// </note>
    pub fn set_asset_model_composite_models(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::AssetModelCompositeModel>>) -> Self {
        self.asset_model_composite_models = input;
        self
    }
    /// <p>The composite models that are part of this asset model. It groups properties (such as attributes, measurements, transforms, and metrics) and child composite models that model parts of your industrial equipment. Each composite model has a type that defines the properties that the composite model supports. Use composite models to define alarms on this asset model.</p><note>
    /// <p>When creating custom composite models, you need to use <a href="https://docs.aws.amazon.com/iot-sitewise/latest/APIReference/API_CreateAssetModelCompositeModel.html">CreateAssetModelCompositeModel</a>. For more information, see
    /// <link>.</p>
    /// </note>
    pub fn get_asset_model_composite_models(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::AssetModelCompositeModel>> {
        &self.asset_model_composite_models
    }
    /// <p>A unique case-sensitive identifier that you can provide to ensure the idempotency of the request. Don't reuse this client token if a new idempotent request is required.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.client_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A unique case-sensitive identifier that you can provide to ensure the idempotency of the request. Don't reuse this client token if a new idempotent request is required.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.client_token = input;
        self
    }
    /// <p>A unique case-sensitive identifier that you can provide to ensure the idempotency of the request. Don't reuse this client token if a new idempotent request is required.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.client_token
    }
    /// <p>An external ID to assign to the asset model. The asset model must not already have an external ID. The external ID must be unique within your Amazon Web Services account. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/object-ids.html#external-ids">Using external IDs</a> in the <i>IoT SiteWise User Guide</i>.</p>
    pub fn asset_model_external_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.asset_model_external_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>An external ID to assign to the asset model. The asset model must not already have an external ID. The external ID must be unique within your Amazon Web Services account. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/object-ids.html#external-ids">Using external IDs</a> in the <i>IoT SiteWise User Guide</i>.</p>
    pub fn set_asset_model_external_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.asset_model_external_id = input;
        self
    }
    /// <p>An external ID to assign to the asset model. The asset model must not already have an external ID. The external ID must be unique within your Amazon Web Services account. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/object-ids.html#external-ids">Using external IDs</a> in the <i>IoT SiteWise User Guide</i>.</p>
    pub fn get_asset_model_external_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.asset_model_external_id
    }
    /// Consumes the builder and constructs a [`UpdateAssetModelInput`](crate::operation::update_asset_model::UpdateAssetModelInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::update_asset_model::UpdateAssetModelInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::update_asset_model::UpdateAssetModelInput {
            asset_model_id: self.asset_model_id,
            asset_model_name: self.asset_model_name,
            asset_model_description: self.asset_model_description,
            asset_model_properties: self.asset_model_properties,
            asset_model_hierarchies: self.asset_model_hierarchies,
            asset_model_composite_models: self.asset_model_composite_models,
            client_token: self.client_token,
            asset_model_external_id: self.asset_model_external_id,
        })
    }
}
