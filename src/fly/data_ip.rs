use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderFly;

#[derive(Serialize)]
struct DataIpData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    app: PrimField<String>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

struct DataIp_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataIpData>,
}

#[derive(Clone)]
pub struct DataIp(Rc<DataIp_>);

impl DataIp {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Dependable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderFly) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc= "Get a reference to the value of field `address` after provisioning.\nID of volume"]
    pub fn address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `app` after provisioning.\nName of app to attach"]
    pub fn app(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nID of address"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\nregion"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nv4 or v6"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}

impl Datasource for DataIp {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for DataIp {
    fn extract_ref(&self) -> String {
        Datasource::extract_ref(self)
    }
}

impl ToListMappable for DataIp {
    type O = ListRef<DataIpRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Datasource::extract_ref(&self))
    }
}

impl Datasource_ for DataIp_ {
    fn extract_datasource_type(&self) -> String {
        "fly_ip".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataIp {
    pub tf_id: String,
    #[doc= "Name of app to attach"]
    pub app: PrimField<String>,
    #[doc= "v4 or v6"]
    pub type_: PrimField<String>,
}

impl BuildDataIp {
    pub fn build(self, stack: &mut Stack) -> DataIp {
        let out = DataIp(Rc::new(DataIp_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataIpData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                app: self.app,
                type_: self.type_,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataIpRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataIpRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataIpRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `address` after provisioning.\nID of volume"]
    pub fn address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `app` after provisioning.\nName of app to attach"]
    pub fn app(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nID of address"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\nregion"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nv4 or v6"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}
