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
pub fn caller<T, I>(invocation: Invocation<I>) -> reqwest::Result<Box<dyn std::fmt::Debug>> {
    let method = invocation.method.as_deref().unwrap_or("GET");
    let response = match method {
        "GET" => get::<T, _>(invocation),
        "POST" => post::<T, _>(invocation),
        _ => get::<T, _>(invocation),
    };
    Ok(Box::new(response))
}

#[allow(unused)]
fn get<T, I>(invocation: Invocation<I>) -> reqwest::Result<Box<dyn std::fmt::Debug>> {
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

    Ok(Box::new(final_response))
}

#[allow(unused)]
fn post<T, I>(invocation: Invocation<I>) -> reqwest::Result<Box<dyn std::fmt::Debug>> {
    // let trim_value = invocation.value.trim();
    // let method = invocation.method;

    let paroot = productagreement::model::Identifier {
        kind: "algo".to_string(),
        value: "algo".to_string(),
    };
    Ok(Box::new(paroot))
}
