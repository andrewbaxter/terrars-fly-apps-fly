use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderFly;

#[derive(Serialize)]
struct VolumeData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    app: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    internalid: Option<PrimField<String>>,
    name: PrimField<String>,
    region: PrimField<String>,
    size: PrimField<f64>,
}

struct Volume_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<VolumeData>,
}

#[derive(Clone)]
pub struct Volume(Rc<Volume_>);

impl Volume {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Dependable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderFly) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    pub fn set_create_before_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.create_before_destroy = v;
        self
    }

    pub fn set_prevent_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.prevent_destroy = v;
        self
    }

    pub fn ignore_changes_to_all(self) -> Self {
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `id`.\nID of volume"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `internalid`.\nInternal ID"]
    pub fn set_internalid(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().internalid = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `app` after provisioning.\nName of app to attach"]
    pub fn app(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nID of volume"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `internalid` after provisioning.\nInternal ID"]
    pub fn internalid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.internalid", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nname"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\nregion"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `size` after provisioning.\nSize of volume in gb"]
    pub fn size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size", self.extract_ref()))
    }
}

impl Resource for Volume {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for Volume {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for Volume {
    type O = ListRef<VolumeRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Resource::extract_ref(&self))
    }
}

impl Resource_ for Volume_ {
    fn extract_resource_type(&self) -> String {
        "fly_volume".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildVolume {
    pub tf_id: String,
    #[doc= "Name of app to attach"]
    pub app: PrimField<String>,
    #[doc= "name"]
    pub name: PrimField<String>,
    #[doc= "region"]
    pub region: PrimField<String>,
    #[doc= "Size of volume in gb"]
    pub size: PrimField<f64>,
}

impl BuildVolume {
    pub fn build(self, stack: &mut Stack) -> Volume {
        let out = Volume(Rc::new(Volume_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(VolumeData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                app: self.app,
                id: core::default::Default::default(),
                internalid: core::default::Default::default(),
                name: self.name,
                region: self.region,
                size: self.size,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct VolumeRef {
    shared: StackShared,
    base: String,
}

impl Ref for VolumeRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl VolumeRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `app` after provisioning.\nName of app to attach"]
    pub fn app(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nID of volume"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `internalid` after provisioning.\nInternal ID"]
    pub fn internalid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.internalid", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nname"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\nregion"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `size` after provisioning.\nSize of volume in gb"]
    pub fn size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size", self.extract_ref()))
    }
}
