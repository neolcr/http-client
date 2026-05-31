use crate::model::productagreement::model::AgreementItem;

#[derive(Default, Debug)]
pub struct Output {
    pub involved_party: String,
    pub person_id: String,
    pub dni: String,
    pub birth_date: String,
    pub agreements: Vec<AgreementItem>,
}
