use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderFly;

#[derive(Serialize)]
struct DataCertData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    app: PrimField<String>,
    hostname: PrimField<String>,
}

struct DataCert_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataCertData>,
}

#[derive(Clone)]
pub struct DataCert(Rc<DataCert_>);

impl DataCert {
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

    #[doc= "Get a reference to the value of field `app` after provisioning.\nName of app that is attacjed"]
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

impl Datasource for DataCert {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for DataCert {
    fn extract_ref(&self) -> String {
        Datasource::extract_ref(self)
    }
}

impl ToListMappable for DataCert {
    type O = ListRef<DataCertRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataCert_ {
    fn extract_datasource_type(&self) -> String {
        "fly_cert".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCert {
    pub tf_id: String,
    #[doc= "Name of app that is attacjed"]
    pub app: PrimField<String>,
    #[doc= "hostname"]
    pub hostname: PrimField<String>,
}

impl BuildDataCert {
    pub fn build(self, stack: &mut Stack) -> DataCert {
        let out = DataCert(Rc::new(DataCert_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataCertData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                app: self.app,
                hostname: self.hostname,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCertRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCertRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataCertRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `app` after provisioning.\nName of app that is attacjed"]
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
