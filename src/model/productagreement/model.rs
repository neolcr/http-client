use serde::Deserialize;

#[derive(Debug)]
pub enum Response {
    PARoot(PARoot),
    AgreementItem(AgreementItem),
    Unknown(),
}

#[derive(Deserialize, Debug, Default, Clone)]
#[allow(non_snake_case)]
#[allow(unused)]
pub struct PARoot {
    productAgreements: ProductAgreements,
}

#[derive(Deserialize, Debug, Default, Clone)]
#[allow(non_snake_case)]
#[allow(unused)]
pub struct ProductAgreements {
    data: Vec<AgreementItem>,
}

#[derive(Deserialize, Debug, Default, Clone)]
#[allow(non_snake_case)]
#[allow(unused)]
pub struct AgreementItem {
    lifeCycleStatusType: String,
    productType: String,
    name: String,
    identifiers: Vec<Identifier>,
    involvedPartyRelationships: InvolvedPartyRelationshipData,
}

#[derive(Deserialize, Debug, Default, Clone)]
#[allow(non_snake_case)]
#[allow(unused)]
pub struct Identifier {
    #[serde(rename = "type")]
    pub kind: String,
    pub value: String,
}

#[derive(Deserialize, Debug, Default, Clone)]
#[allow(non_snake_case)]
#[allow(unused)]
pub struct InvolvedPartyRelationshipData {
    data: Vec<InvolvedPartyRelationship>,
}

#[derive(Deserialize, Debug, Default, Clone)]
#[allow(non_snake_case)]
#[allow(unused)]
pub struct InvolvedPartyRelationship {
    involvedParty: InvolvedPartyId,
    #[serde(rename = "type")]
    kind: String,
}

#[derive(Deserialize, Debug, Default, Clone)]
#[allow(non_snake_case)]
#[allow(unused)]
pub struct InvolvedPartyId {
    id: String,
}
