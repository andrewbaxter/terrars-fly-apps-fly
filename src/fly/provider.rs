use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct ProviderFlyData {
    #[serde(skip_serializing_if = "Option::is_none")]
    alias: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fly_api_token: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fly_http_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    internaltunnelorg: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    internaltunnelregion: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    useinternaltunnel: Option<PrimField<bool>>,
}

struct ProviderFly_ {
    data: RefCell<ProviderFlyData>,
}

pub struct ProviderFly(Rc<ProviderFly_>);

impl ProviderFly {
    pub fn provider_ref(&self) -> String {
        let data = self.0.data.borrow();
        if let Some(alias) = &data.alias {
            format!("{}.{}", "fly", alias)
        } else {
            "fly".into()
        }
    }

    pub fn set_alias(self, alias: impl ToString) -> Self {
        self.0.data.borrow_mut().alias = Some(alias.to_string());
        self
    }

    #[doc= "Set the field `fly_api_token`.\nfly.io api token. If not set checks env for FLY_API_TOKEN"]
    pub fn set_fly_api_token(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().fly_api_token = Some(v.into());
        self
    }

    #[doc= "Set the field `fly_http_endpoint`.\nWhere the provider should look to find the fly http endpoint"]
    pub fn set_fly_http_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().fly_http_endpoint = Some(v.into());
        self
    }

    #[doc= "Set the field `internaltunnelorg`.\n"]
    pub fn set_internaltunnelorg(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().internaltunnelorg = Some(v.into());
        self
    }

    #[doc= "Set the field `internaltunnelregion`.\n"]
    pub fn set_internaltunnelregion(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().internaltunnelregion = Some(v.into());
        self
    }

    #[doc= "Set the field `useinternaltunnel`.\n"]
    pub fn set_useinternaltunnel(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().useinternaltunnel = Some(v.into());
        self
    }
}

impl Provider for ProviderFly_ {
    fn extract_type_tf_id(&self) -> String {
        "fly".into()
    }

    fn extract_provider_type(&self) -> serde_json::Value {
        serde_json::json!({
            "source": "fly-apps/fly",
            "version": "0.0.20",
        })
    }

    fn extract_provider(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildProviderFly {}

impl BuildProviderFly {
    pub fn build(self, stack: &mut Stack) -> ProviderFly {
        let out = ProviderFly(Rc::new(ProviderFly_ { data: RefCell::new(ProviderFlyData {
            alias: None,
            fly_api_token: core::default::Default::default(),
            fly_http_endpoint: core::default::Default::default(),
            internaltunnelorg: core::default::Default::default(),
            internaltunnelregion: core::default::Default::default(),
            useinternaltunnel: core::default::Default::default(),
        }) }));
        stack.add_provider(out.0.clone());
        out
    }
}
