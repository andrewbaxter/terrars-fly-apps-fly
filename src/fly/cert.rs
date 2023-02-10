use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderFly;

#[derive(Serialize)]
struct CertData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    app: PrimField<String>,
    hostname: PrimField<String>,
}

struct Cert_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CertData>,
}

#[derive(Clone)]
pub struct Cert(Rc<Cert_>);

impl Cert {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
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

    #[doc= "Get a reference to the value of field `app` after provisioning.\nName of app to attach"]
    pub fn app(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `check` after provisioning.\ncheck"]
    pub fn check(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.check", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dnsvalidationhostname` after provisioning.\nDnsValidationHostname"]
    pub fn dnsvalidationhostname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dnsvalidationhostname", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dnsvalidationinstructions` after provisioning.\nDnsValidationHostname"]
    pub fn dnsvalidationinstructions(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dnsvalidationinstructions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dnsvalidationtarget` after provisioning.\nDnsValidationTarget"]
    pub fn dnsvalidationtarget(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dnsvalidationtarget", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `hostname` after provisioning.\nhostname"]
    pub fn hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nID of address"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
}

impl Referable for Cert {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for Cert { }

impl ToListMappable for Cert {
    type O = ListRef<CertRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Cert_ {
    fn extract_resource_type(&self) -> String {
        "fly_cert".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCert {
    pub tf_id: String,
    #[doc= "Name of app to attach"]
    pub app: PrimField<String>,
    #[doc= "hostname"]
    pub hostname: PrimField<String>,
}

impl BuildCert {
    pub fn build(self, stack: &mut Stack) -> Cert {
        let out = Cert(Rc::new(Cert_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CertData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                app: self.app,
                hostname: self.hostname,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CertRef {
    shared: StackShared,
    base: String,
}

impl Ref for CertRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CertRef {
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

    #[doc= "Get a reference to the value of field `check` after provisioning.\ncheck"]
    pub fn check(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.check", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dnsvalidationhostname` after provisioning.\nDnsValidationHostname"]
    pub fn dnsvalidationhostname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dnsvalidationhostname", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dnsvalidationinstructions` after provisioning.\nDnsValidationHostname"]
    pub fn dnsvalidationinstructions(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dnsvalidationinstructions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dnsvalidationtarget` after provisioning.\nDnsValidationTarget"]
    pub fn dnsvalidationtarget(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dnsvalidationtarget", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `hostname` after provisioning.\nhostname"]
    pub fn hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nID of address"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
}
