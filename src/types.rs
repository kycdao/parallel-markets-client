use either::Either;
use serde::{Deserialize, Serialize};
use std::fmt;

use json_api_client::types::*;

pub type EntityId = String;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Scope {
    Profile,
    AccreditationStatus,
    Identity,
}

impl fmt::Display for Scope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match self {
            Scope::Profile => "profile",
            Scope::AccreditationStatus => "accreditation_status",
            Scope::Identity => "identity",
        };
        write!(f, "{}", str)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    pub address_one: String,
    pub address_two: String,
    pub city: String,
    pub region: Option<String>,
    pub postal_code: String,
    pub state: Option<String>,
    pub country: CountryCode,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserSession {
    /// False if the user did not use an anonymizing proxy (e.g. TOR, Public VPN) when submitting linked information
    pub maybe_anonymizing_proxy: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum BusinessType {
    /// A public charity as defined in Section 501(c)(3) of the Internal Revenue Code
    #[serde(rename = "Public Charity")]
    PublicCharity,
    /// A private foundation as defined in Section 501(c)(3) of the Internal Revenue Code
    #[serde(rename = "Private Foundation")]
    PrivateFoundation,
    /// A S-Corp as defined in Subchapter S of Chapter 1 of the Internal Revenue Code
    #[serde(rename = "S Corporation")]
    SCorporation,
    /// A C-Corp as defined in the Internal Revenue Code
    #[serde(rename = "C Corporation")]
    CCorporation,
    /// Any irrevocable business or personal trust described in Section 501(a) of the Securities Act of 1933, as amended.
    #[serde(rename = "Irrevocable Trust")]
    IrrevocableTrust,
    /// Any revocable business or personal trust described in Section 501(a) of the Securities Act of 1933, as amended.
    #[serde(rename = "Revocable Trust")]
    RevocableTrust,
    /// A partnership organized as a limited liability company under the laws of its state of formation
    #[serde(rename = "Partnership LLC")]
    PartnershipLLC,
    /// A partnership organized as a limited partnership under the laws of its state of formation
    #[serde(rename = "Partnership LP")]
    PartnershipLP,
    /// A "family office" as defined in Rule 202(a)(11)(G)–1 under the Investment Advisers Act of 1940
    #[serde(rename = "Family Office")]
    FamilyOffice,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BusinessProfile {
    pub name: String,
    pub business_type: BusinessType,
    pub primary_contact: Option<IndividualProfile>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IndividualProfile {
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum DocumentType {
    CertificationLetter,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccreditationDocument {
    pub download_url: String,
    /// Number of seconds until expiration
    pub download_url_expires: u64,
    #[serde(rename = "type")]
    pub document_type: DocumentType,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccreditationStatus {
    /// Accreditation is currently valid
    Current,
    /// Accreditation application is in review
    Pending,
    /// Accreditation application is awaiting documentation from a third party evaluator
    ThirdPartyPending,
    /// Accreditation has expired and is no longer valid
    Expired,
    /// Accreditation attempt was unsuccessful due to a failure to meet requirements
    Rejected,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum BusinessAssertionType {
    /// Accreditation is based on worth (only used for businesses)
    Worth,
    /// Accreditation is based on a third party evaluation (CPA, lawyer, or broker dealer reviewed accreditation status)
    EvaluatorAssertion,
    /// Accreditation is based on all owners being accredited
    AccreditedOwners,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BusinessAccreditation {
    pub id: String,
    pub status: AccreditationStatus,
    #[serde(with = "time::serde::timestamp::option")]
    pub expires_at: Option<DateTime>,
    pub assertion_type: BusinessAssertionType,
    #[serde(with = "time::serde::timestamp")]
    pub created_at: DateTime,
    #[serde(with = "time::serde::timestamp::option")]
    pub certified_at: Option<DateTime>,
    pub name: String,
    pub documents: Vec<AccreditationDocument>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum IndividualAssertionType {
    /// Accreditation is based on the income test
    Income,
    /// Accreditation is based on the net-worth test
    NetWorth,
    /// Accreditation is based on a third party evaluation (CPA, lawyer, or broker dealer reviewed accreditation status)
    EvaluatorAssertion,
    /// The person holds a professional license that qualifies them for accreditation
    ProfessionalLicense,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IndividualAccreditation {
    pub id: String,
    pub status: AccreditationStatus,
    #[serde(with = "time::serde::timestamp::option")]
    pub expires_at: Option<DateTime>,
    pub assertion_type: IndividualAssertionType,
    #[serde(with = "time::serde::timestamp")]
    pub created_at: DateTime,
    #[serde(with = "time::serde::timestamp::option")]
    pub certified_at: Option<DateTime>,
    pub first_name: String,
    pub last_name: String,
    pub documents: Vec<AccreditationDocument>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EntityType {
    Individual,
    Business,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BeneficialOwnerReference {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub reference_type: EntityType,
    pub ownership_percent: Decimal,
    #[serde(with = "either::serde_untagged")]
    pub profile: Either<BusinessProfile, IndividualProfile>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlPersonReference {
    pub id: Option<String>,
    /// Note: Only possible value is Individual
    #[serde(rename = "type")]
    pub reference_type: EntityType,
    pub title: String,
    pub profile: IndividualProfile,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum IdentityDocumentType {
    DriversLicense,
    StateIdCard,
    Passport,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MaritalStatus {
    Single,
    Married,
    Separated,
    Divorced,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IdentityDocument {
    pub download_url: String,
    /// Number of seconds until expiration
    pub download_url_expires: u64,
    #[serde(rename = "type")]
    pub document_type: IdentityDocumentType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BusinessRiskMonitorMatch {
    /// True if the matched entity appears in searched media sources linking it to serious crime (e.g. fraud, money laundering, terrorism, etc.)
    pub adverse_media: bool,
    /// True if the matched entity appears in any monitored sanctions datasets
    pub currently_sanctioned: bool,
    /// True if the matched entity appears in any searched disqualified directors datasets
    pub disqualified_director: bool,
    /// True if the matched entity appears in any searched financial regulator datasets
    pub financial_regulator: bool,
    /// True if the matched entity appears in any insolvency datasets
    pub insolvent: bool,
    /// True if the matched entity appears in any law enforcement datasets
    pub law_enforcement: bool,
    /// The name of the matched entity (likely the same or similar to the business name)
    pub name: String,
    /// True if the matched entity is considered a Politically Exposed Person
    pub pep: bool,
    /// True if the matched entity was sanctioned in the past
    pub previously_sanctioned: bool,
    /// Entity match score 1-100. Higher numbers indicate higher confidence in a positive match
    pub score: u8,
    /// The matched entity's website
    pub website: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IndividualRiskMonitorMatch {
    /// True if the matched individual appears in searched media sources linking them to serious crime (e.g. fraud, money laundering, terrorism, etc.)
    pub adverse_media: bool,
    pub birth_date: Date,
    /// True if the matched individual is currently listed as a sanctioned business by OFAC
    pub currently_sanctioned: bool,
    /// True if the matched individual is deceased
    pub deceased: bool,
    /// True if the matched individual appears in any searched disqualified director datasets
    pub disqualified_director: bool,
    /// True if the matched individual appears in any searched financial regulator datasets
    pub financial_regulator: bool,
    /// True if the matched individual appears in any insolvency datasets
    pub insolvent: bool,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    /// True if the matched individual appears in any law enforcement datasets
    pub law_enforcement: bool,
    /// A description of the matched individual's nationality
    pub nationality: String,
    /// True if the matched individual is considered a Politically Exposed Person
    pub pep: bool,
    /// True if the matched individual was sanctioned in the past
    pub previously_sanctioned: bool,
    /// Entity match score 1-100. Higher numbers indicate higher confidence in a positive match
    pub score: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BusinessIdentityDetails {
    pub business_type: BusinessType,
    #[serde(with = "time::serde::rfc3339")]
    pub completed_at: DateTime,
    pub control_persons: Vec<ControlPersonReference>,
    pub created_by: IndividualProfile,
    pub direct_beneficial_owners: Vec<BeneficialOwnerReference>,
    #[serde(with = "time::serde::rfc3339")]
    pub expires_at: DateTime,
    pub foreign_tax_id: String,
    pub identity_files: Vec<IdentityDocument>,
    pub incorporation_country: CountryCode,
    pub incorporation_state: Option<String>,
    pub name: String,
    pub primary_contact: IndividualProfile,
    pub principal_location: Location,
    pub risk_monitor_matches: Vec<BusinessRiskMonitorMatch>,
    pub us_tax_id: String, // TODO KYC-321 is this optional?
    pub user_session: UserSession,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IndividualIdentityDetails {
    pub birth_date: Date,
    pub citizenship_country: CountryCode,
    #[serde(with = "time::serde::rfc3339")]
    pub completed_at: DateTime,
    pub created_by: IndividualProfile,
    pub domicile_location: Location,
    pub email: String,
    #[serde(with = "time::serde::rfc3339")]
    pub expires_at: DateTime,
    pub first_name: String,
    pub last_name: String,
    pub identity_files: Vec<IdentityDocument>,
    pub marital_status: MaritalStatus,
    /// Phone number is in E.164 format
    pub phone: String,
    pub residence_location: Location,
    pub risk_monitor_matches: Vec<IndividualRiskMonitorMatch>,
    pub us_tax_id: String, // TODO KYC-321 is this optional?
    pub foreign_tax_id: Option<String>,
    pub user_session: UserSession,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum ProvidingFor {
    /// The user has authenticated as themselves and is sharing their own information
    #[serde(rename = "self")]
    AsSelf,
    /// The user has authenticated as a business and is sharing that business' information.
    ControlledBusiness,
    /// The user has authenticated on behalf of another person and is sharing that person's information (this option is only possible with the identity scope)
    OtherIndividual,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RevokeType {
    /// The subject (or an associated individual) requested to revoke access to the subject's data
    Subject,
    /// You (or an associated individual) requested to revoke your own access to the subject's data
    Partner,
    /// Access to this subject's data was revoked by our internal systems
    System,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProfileResponse {
    pub id: EntityId,
    #[serde(rename = "type")]
    pub entity_type: EntityType,
    #[serde(with = "either::serde_untagged")]
    pub profile: Either<IndividualProfile, BusinessProfile>,
    pub user_id: String,
    pub user_profile: IndividualProfile,
    pub user_providing_for: ProvidingFor,
    #[serde(with = "time::serde::rfc3339::option")]
    pub access_expires_at: Option<DateTime>,
    pub access_revoked_by: Option<RevokeType>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccreditationsResponse {
    pub id: EntityId,
    #[serde(rename = "type")]
    pub entity_type: EntityType,
    pub user_id: String,
    #[serde(with = "time::serde::timestamp::option")]
    pub indicated_unaccredited: Option<DateTime>,
    #[serde(with = "either::serde_untagged")]
    pub accreditations: Either<Vec<IndividualAccreditation>, Vec<BusinessAccreditation>>,
    // pub user_providing_for: ProvidingFor, // Missing from docs
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IdentityResponse {
    pub id: EntityId,
    #[serde(rename = "type")]
    pub entity_type: EntityType,
    #[serde(with = "either::serde_untagged")]
    pub identity_details: Either<BusinessIdentityDetails, IndividualIdentityDetails>,
    pub user_id: String,
    pub user_providing_for: ProvidingFor,
    #[serde(with = "time::serde::rfc3339::option")]
    pub access_expires_at: Option<DateTime>,
    pub access_revoked_by: Option<RevokeType>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DependencyIdentityResponse {
    pub id: EntityId,
    #[serde(rename = "type")]
    pub entity_type: EntityType,
    #[serde(with = "either::serde_untagged")]
    pub identity_details: Either<BusinessIdentityDetails, IndividualIdentityDetails>,
    pub user_id: String,
    #[serde(with = "time::serde::rfc3339::option")]
    pub access_expires_at: Option<DateTime>,
    /// Note: Only possible value is Subject
    pub access_revoked_by: Option<RevokeType>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    /// Accreditation Scope: Indicates that there is a change in an entity's accreditation status
    /// Identity Scope: Indicates that a new identity information has been submitted for the entity
    DataUpdate,
    /// Indicates that access revocation has been scheduled and API access will expire after a period of time.
    /// You can use the Profile API to get the access expiration details such as expiration date.
    AccessRevocationScheduled,
    /// The entity appears as a possible match in monitored media sources linking it to serious crime (e.g. fraud, money laundering, terrorism, etc.)
    AdverseMediaRiskMonitorMatch,
    /// The entity appears as a possible match in any monitored sanctions datasets
    CurrentlySanctionedRiskMonitorMatch,
    /// The entity appears as a possible match in any monitored disqualified directors datasets
    DisqualifiedDirectorRiskMonitorMatch,
    /// The entity appears as a possible match in any monitored financial regulator datasets
    FinancialRegulatorRiskMonitorMatch,
    /// The entity appears as a possible match in any monitored insolvency datasets
    InsolventRiskMonitorMatch,
    /// The entity appears as a possible match in any monitored law enforcement datasets
    LawEnforcementRiskMonitorMatch,
    /// The entity is possibly considered a Politically Exposed Person
    PepRiskMonitorMatch,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EntityIdentification {
    pub id: EntityId,
    #[serde(rename = "type")]
    pub entity_type: EntityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebhookData {
    pub entity: EntityIdentification,
    pub event: EventType,
    pub scope: Scope,
    pub connecting_business_id: Option<String>,
}
