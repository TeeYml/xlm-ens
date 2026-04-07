pub mod expiry;
pub mod pricing;
pub mod test;

use core::fmt;
use std::collections::{HashMap, HashSet};

use expiry::{expiry_from_now, within_grace_period};
use pricing::price_for_label;
use xlm_ns_common::validation::{parse_fqdn, validate_label, validate_owner, validate_registration_years};
use xlm_ns_common::{CommonError, NameRecord, GRACE_PERIOD_SECONDS};

#[derive(Debug, Clone)]
pub struct RegistrationQuote {
    pub fee_stroops: u64,
    pub expiry_unix: u64,
    pub grace_period_ends_at: u64,
}

#[derive(Debug)]
pub enum RegistrarError {
    InsufficientFee,
    NotFound,
    NotRenewable,
    AlreadyRegistered,
    Reserved,
    Unauthorized,
    Validation(CommonError),
}

impl fmt::Display for RegistrarError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InsufficientFee => f.write_str("fee paid is below required amount"),
            Self::NotFound => f.write_str("registration was not found"),
            Self::NotRenewable => f.write_str("registration is no longer eligible for renewal"),
            Self::AlreadyRegistered => f.write_str("name is already registered"),
            Self::Reserved => f.write_str("label is reserved and cannot be directly registered"),
            Self::Unauthorized => f.write_str("caller is not authorized for this registration"),
            Self::Validation(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for RegistrarError {}

impl From<CommonError> for RegistrarError {
    fn from(value: CommonError) -> Self {
        Self::Validation(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegistrationRecord {
    pub record: NameRecord,
    pub fee_paid: u64,
    pub renewed_at: u64,
}

#[derive(Debug, Default)]
pub struct RegistrarContract {
    registrations: HashMap<String, RegistrationRecord>,
    reserved_labels: HashSet<String>,
    treasury_balance: u64,
}

impl RegistrarContract {
    pub fn reserve_label(&mut self, label: &str) -> Result<(), RegistrarError> {
        validate_label(label)?;
        self.reserved_labels.insert(label.to_string());
        Ok(())
    }

    pub fn quote_registration(
        &self,
        label: &str,
        years: u64,
        now_unix: u64,
    ) -> Result<RegistrationQuote, RegistrarError> {
        validate_label(label)?;
        validate_registration_years(years)?;
        Ok(quote_registration(label, years, now_unix))
    }

    pub fn register(
        &mut self,
        label: &str,
        owner: &str,
        years: u64,
        payment_stroops: u64,
        now_unix: u64,
    ) -> Result<(), RegistrarError> {
        validate_label(label)?;
        validate_owner(owner)?;
        validate_registration_years(years)?;

        if self.reserved_labels.contains(label) {
            return Err(RegistrarError::Reserved);
        }

        let quote = quote_registration(label, years, now_unix);
        if payment_stroops < quote.fee_stroops {
            return Err(RegistrarError::InsufficientFee);
        }

        let name = format!("{label}.xlm");
        if let Some(existing) = self.registrations.get(&name) {
            if !existing.record.is_claimable_at(now_unix) {
                return Err(RegistrarError::AlreadyRegistered);
            }
        }

        self.treasury_balance = self.treasury_balance.saturating_add(payment_stroops);
        self.registrations.insert(
            name,
            RegistrationRecord {
                record: NameRecord::new(
                    label,
                    owner,
                    None,
                    now_unix,
                    quote.expiry_unix,
                    quote.grace_period_ends_at,
                ),
                fee_paid: payment_stroops,
                renewed_at: now_unix,
            },
        );
        Ok(())
    }

    pub fn renew(
        &mut self,
        name: &str,
        caller: &str,
        years: u64,
        payment_stroops: u64,
        now_unix: u64,
    ) -> Result<(), RegistrarError> {
        let (label, _) = parse_fqdn(name)?;
        validate_owner(caller)?;
        validate_registration_years(years)?;

        let record = self
            .registrations
            .get_mut(name)
            .ok_or(RegistrarError::NotFound)?;
        if record.record.owner != caller {
            return Err(RegistrarError::Unauthorized);
        }
        if !can_renew(record.record.expires_at, now_unix) {
            return Err(RegistrarError::NotRenewable);
        }

        let annual_fee = price_for_label(&label);
        let fee_due = annual_fee.saturating_mul(years);
        if payment_stroops < fee_due {
            return Err(RegistrarError::InsufficientFee);
        }

        let base_time = record.record.expires_at.max(now_unix);
        let expires_at = expiry_from_now(base_time, years);
        record
            .record
            .extend_expiry(expires_at, expires_at.saturating_add(GRACE_PERIOD_SECONDS));
        record.fee_paid = record.fee_paid.saturating_add(payment_stroops);
        record.renewed_at = now_unix;
        self.treasury_balance = self.treasury_balance.saturating_add(payment_stroops);
        Ok(())
    }

    pub fn registration(&self, name: &str) -> Option<&RegistrationRecord> {
        self.registrations.get(name)
    }

    pub fn is_available(&self, label: &str, now_unix: u64) -> bool {
        let name = format!("{label}.xlm");
        self.registrations
            .get(&name)
            .map(|record| record.record.is_claimable_at(now_unix))
            .unwrap_or(true)
    }

    pub fn treasury_balance(&self) -> u64 {
        self.treasury_balance
    }
}

pub fn quote_registration(label: &str, years: u64, now_unix: u64) -> RegistrationQuote {
    let annual_fee = price_for_label(label);
    let expiry_unix = expiry_from_now(now_unix, years);

    RegistrationQuote {
        fee_stroops: annual_fee.saturating_mul(years),
        expiry_unix,
        grace_period_ends_at: expiry_unix.saturating_add(GRACE_PERIOD_SECONDS),
    }
}

pub fn can_renew(expiry_unix: u64, now_unix: u64) -> bool {
    now_unix <= expiry_unix || within_grace_period(expiry_unix, now_unix)
}
