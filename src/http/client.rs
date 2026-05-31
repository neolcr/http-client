use crate::http::model::Invocation;
use crate::model::productagreement;
use crate::model::productagreement::model::AgreementItem;
use crate::model::productagreement::model::PARoot;
use crate::model::productagreement::model::Response;
use reqwest::blocking::ClientBuilder;
use serde::{Deserialize, Serialize};
use serde_json::json;

pub fn saludar() {
    println!("saludar");
}

#[allow(unused)]
pub fn caller<I>(invocation: Invocation<I>) -> reqwest::Result<Response> {
    let method = invocation.method.as_deref().unwrap_or("GET");
    match method {
        "GET" => get(invocation),
        "POST" => post(invocation),
        _ => get(invocation),
    }
}

#[allow(unused)]
fn get<I>(invocation: Invocation<I>) -> reqwest::Result<Response> {
    let url = invocation.url.as_ref().unwrap().trim();
    log::info!("Llamar a url {}", &url);
    let client = ClientBuilder::new()
        .danger_accept_invalid_hostnames(true)
        .danger_accept_invalid_certs(true)
        .build()?;

    let response_obj = invocation.response_object.as_deref().unwrap();
    let resp = client
        .get(url)
        .header(reqwest::header::ACCEPT, "application/json")
        .send();

    let final_response = match resp {
        Ok(mut r) => {
            let text = r.text().unwrap();
            log::info!("RAW response: {}", text);
            let response = match response_obj {
                "AgreementItem" => {
                    let parsed: AgreementItem = serde_json::from_str(&text).unwrap();
                    Response::AgreementItem(parsed)
                }
                "PARoot" => {
                    let parsed: PARoot = serde_json::from_str(&text).unwrap();
                    Response::PARoot(parsed)
                }
                _ => {
                    log::warn!("Response object unknown");
                    Response::Unknown()
                }
            };

            match &response {
                Response::AgreementItem(a) => {
                    log::info!("{:?}", a);
                }
                Response::PARoot(p) => {
                    log::info!("{:?}", p);
                }
                Response::Unknown() => {
                    log::warn!("Unknown object");
                }
            }
            response
        }
        Err(e) => {
            log::error!("Error response {}", e);
            Response::Unknown()
        }
    };

    Ok(final_response)
}

#[allow(unused)]
fn post<I>(invocation: Invocation<I>) -> reqwest::Result<Response> {
    Ok(Response::Unknown())
}
