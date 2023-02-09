use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderFly;

#[derive(Serialize)]
struct DataAppData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    name: PrimField<String>,
}

struct DataApp_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataAppData>,
}

#[derive(Clone)]
pub struct DataApp(Rc<DataApp_>);

impl DataApp {
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

    #[doc= "Get a reference to the value of field `appurl` after provisioning.\n"]
    pub fn appurl(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.appurl", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `currentrelease` after provisioning.\n"]
    pub fn currentrelease(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.currentrelease", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `deployed` after provisioning.\n"]
    pub fn deployed(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deployed", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `healthchecks` after provisioning.\n"]
    pub fn healthchecks(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.healthchecks", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `hostname` after provisioning.\n"]
    pub fn hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ipaddresses` after provisioning.\n"]
    pub fn ipaddresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ipaddresses", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of app"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }
}

impl Datasource for DataApp {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for DataApp {
    fn extract_ref(&self) -> String {
        Datasource::extract_ref(self)
    }
}

impl ToListMappable for DataApp {
    type O = ListRef<DataAppRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataApp_ {
    fn extract_datasource_type(&self) -> String {
        "fly_app".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataApp {
    pub tf_id: String,
    #[doc= "Name of app"]
    pub name: PrimField<String>,
}

impl BuildDataApp {
    pub fn build(self, stack: &mut Stack) -> DataApp {
        let out = DataApp(Rc::new(DataApp_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataAppData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                name: self.name,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAppRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataAppRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `appurl` after provisioning.\n"]
    pub fn appurl(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.appurl", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `currentrelease` after provisioning.\n"]
    pub fn currentrelease(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.currentrelease", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `deployed` after provisioning.\n"]
    pub fn deployed(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deployed", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `healthchecks` after provisioning.\n"]
    pub fn healthchecks(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.healthchecks", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `hostname` after provisioning.\n"]
    pub fn hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ipaddresses` after provisioning.\n"]
    pub fn ipaddresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ipaddresses", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of app"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }
}
