use crate::http::client::caller;
use crate::http::model::Invocation;

pub fn by_invparty(involved_party: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut invocation: Invocation<()> = Invocation::default();
    invocation.value = Some(String::from(involved_party));
    invocation.method = Some("GET");
    invocation.host = Some(String::from("https://test.pt.ing.net"));
    invocation.response_object = Some(String::from("PARoot"));
    invocation.url = Some(format!(
        "{}{}{}{}",
        invocation.host.as_ref().unwrap(),
        "/v6/agreements/products?involvedPartyId=",
        invocation.value.as_ref().unwrap(),
        "&productAgreementStatus=ALL&agreementInvolvedPartyRelationshipStatus=ALL"
    ));
    return caller::<Invocation<()>, ()>(invocation)
        .map(|res| format!("{:?}", res))
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>);
}
