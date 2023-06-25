//! The api for creating, getting, updating, and deleting accounts.

use {
    super::RestApi,
    common::{
        internationalization::Language,
        types::{
            account,
            account::Account,
            email, password, username,
            validation::{Validate, Validator, Violations},
            Email, Password, Username,
        },
    },
    derive_more::Display,
    hyper::Method,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct CreateAccountRequest {
    pub username: Username,
    pub email: Email,
    pub password: Password,
    pub language: Language,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum CreateAccountResponse {
    Created { account: Account },
    UsernameUnavailable,
    EmailUnavailable,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Display)]
pub enum CreateAccountRequestViolation {
    Username(Violations<username::Violation>),
    Email(Violations<email::Violation>),
    Password(Violations<password::Violation>),
}

impl Validate for CreateAccountRequest {
    type Violation = CreateAccountRequestViolation;

    fn validate(&self) -> Result<(), Violations<Self::Violation>> {
        Validator::new()
            .validate_field(&self.username, CreateAccountRequestViolation::Username)
            .validate_field(&self.email, CreateAccountRequestViolation::Email)
            .validate_field(&self.password, CreateAccountRequestViolation::Password)
            .into()
    }
}

impl RestApi for CreateAccountRequest {
    type Request = CreateAccountResponse;

    const METHOD: Method = Method::POST;
    const PATH: &'static str = "/account";
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct UpdateAccountRequest {
    pub id: account::Id,
    pub username: Option<Username>,
    pub email: Option<Email>,
    pub password: Option<Password>,
    pub language: Option<Language>,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum UpdateAccountResponse {
    Updated,
    AccountNotFound,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Display)]
pub enum UpdateAccountRequestViolation {
    Username(Violations<username::Violation>),
    Email(Violations<email::Violation>),
    Password(Violations<password::Violation>),
}

impl Validate for UpdateAccountRequest {
    type Violation = UpdateAccountRequestViolation;

    fn validate(&self) -> Result<(), Violations<Self::Violation>> {
        Validator::new()
            .validate_field(&self.username, UpdateAccountRequestViolation::Username)
            .validate_field(&self.email, UpdateAccountRequestViolation::Email)
            .validate_field(&self.password, UpdateAccountRequestViolation::Password)
            .into()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct DeleteAccountRequest {
    pub id: account::Id,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum DeleteAccountResponse {
    Deleted,
    AccountNotFound,
}
