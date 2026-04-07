pub mod manager;
pub mod test;

use std::collections::{BTreeSet, HashMap};

use manager::build_subdomain;
use xlm_ns_common::validation::{parse_fqdn, validate_owner};
use xlm_ns_common::CommonError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParentDomain {
    pub owner: String,
    pub controllers: BTreeSet<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubdomainRecord {
    pub parent: String,
    pub owner: String,
    pub created_at: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubdomainError {
    Validation(CommonError),
    ParentNotFound,
    AlreadyExists,
    NotFound,
    Unauthorized,
}

impl core::fmt::Display for SubdomainError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Validation(error) => write!(f, "{error}"),
            Self::ParentNotFound => f.write_str("parent domain was not registered for subdomain issuance"),
            Self::AlreadyExists => f.write_str("subdomain already exists"),
            Self::NotFound => f.write_str("subdomain was not found"),
            Self::Unauthorized => f.write_str("caller is not authorized for this parent domain"),
        }
    }
}

impl std::error::Error for SubdomainError {}

impl From<CommonError> for SubdomainError {
    fn from(value: CommonError) -> Self {
        Self::Validation(value)
    }
}

#[derive(Debug, Default)]
pub struct SubdomainContract {
    parents: HashMap<String, ParentDomain>,
    subdomains: HashMap<String, SubdomainRecord>,
}

impl SubdomainContract {
    pub fn register_parent(&mut self, parent: &str, owner: &str) -> Result<(), SubdomainError> {
        parse_fqdn(parent)?;
        validate_owner(owner)?;
        self.parents.insert(
            parent.to_string(),
            ParentDomain {
                owner: owner.to_string(),
                controllers: BTreeSet::new(),
            },
        );
        Ok(())
    }

    pub fn add_controller(
        &mut self,
        parent: &str,
        caller: &str,
        controller: &str,
    ) -> Result<(), SubdomainError> {
        validate_owner(controller)?;
        let parent_record = self.parents.get_mut(parent).ok_or(SubdomainError::ParentNotFound)?;
        if parent_record.owner != caller {
            return Err(SubdomainError::Unauthorized);
        }

        parent_record.controllers.insert(controller.to_string());
        Ok(())
    }

    pub fn create(
        &mut self,
        label: &str,
        parent: &str,
        caller: &str,
        owner: &str,
        now_unix: u64,
    ) -> Result<String, SubdomainError> {
        validate_owner(owner)?;
        let parent_record = self.parents.get(parent).ok_or(SubdomainError::ParentNotFound)?;
        if parent_record.owner != caller && !parent_record.controllers.contains(caller) {
            return Err(SubdomainError::Unauthorized);
        }

        let fqdn = build_subdomain(label, parent)?;
        if self.subdomains.contains_key(&fqdn) {
            return Err(SubdomainError::AlreadyExists);
        }

        self.subdomains.insert(
            fqdn.clone(),
            SubdomainRecord {
                parent: parent.to_string(),
                owner: owner.to_string(),
                created_at: now_unix,
            },
        );
        Ok(fqdn)
    }

    pub fn transfer(
        &mut self,
        fqdn: &str,
        caller: &str,
        new_owner: &str,
    ) -> Result<(), SubdomainError> {
        validate_owner(new_owner)?;
        let record = self.subdomains.get_mut(fqdn).ok_or(SubdomainError::NotFound)?;
        if record.owner != caller {
            return Err(SubdomainError::Unauthorized);
        }

        record.owner = new_owner.to_string();
        Ok(())
    }

    pub fn exists(&self, fqdn: &str) -> bool {
        self.subdomains.contains_key(fqdn)
    }

    pub fn record(&self, fqdn: &str) -> Option<&SubdomainRecord> {
        self.subdomains.get(fqdn)
    }
}
