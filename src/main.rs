use atty::Stream;
use chrono::Local;
use once_cell::sync::Lazy;
use oracle::{Connection, Error};
use regex::Regex;
use reqwest::blocking::ClientBuilder;
use serde::{Deserialize, Serialize};
use serde_json::json;
use simplelog::*;
use std::fmt::format;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::model::productagreement::model::Response;
use crate::output::model::Output;
mod api;
mod http;
mod model;
mod output;

static PERSON_ID_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^0*(\d{7,10})$").unwrap());
static LCR_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[A-Za-z0-9+/]{20,}={0,2}$").unwrap());
static DNI_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\d{8}[A-Z]$").unwrap());
static UUID_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[a-fA-F0-9]{8}-([a-fA-F0-9]{4}-){3}[a-fA-F0-9]{12}$").unwrap());
static CID_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\d{11}$").unwrap());
// static NADA: Lazy<String> = Lazy::new(|| String::from("NADA"));

fn main() {
    let now = Local::now();
    let formatted = now.format("%Y-%m-%d").to_string();
    let log_file = format!("output-{}.log", formatted);
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Debug,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(log_file)
                .expect("Fallo al crear log -> writelogger"),
        ),
    ])
    .expect("Fallo al crear log");

    log::info!("inicio");

    let mut input = String::new();
    let mut trim_input: &str = "";
    let mut ask_for_input = true;
    let mut output = Output::default();
    loop {
        if ask_for_input {
            log::info!(
                "Introducir valor (involved party, personId, dni, agreement, cid, local card reference, q -> salir)"
            );
            std::io::stdout().flush().expect("Fallo de flush");
            std::io::stdin()
                .read_line(&mut input)
                .expect("Fallo de readline");

            log::info!("Valor introducido: {}", input);
            trim_input = input.trim();
        }
        match trim_input {
            input if UUID_RE.is_match(input) => {
                (output, ask_for_input) = look_for_uuid(input, output);
                log::info!("Final de flujo-> {:?}", output);
            }
            input if PERSON_ID_RE.is_match(input) => {
                look_for_personid();
            }
            _ => {
                log::info!("Valor de tipo no reconocido");
            }
        }
    }
}

fn look_for_uuid(input: &str, mut output: Output) -> (Output, bool) {
    // buscar involved party
    let response =
        api::productagreement::calls::by_invparty(input).unwrap_or_else(|_| Response::Unknown());
    match response {
        Response::PARoot(p) => {
            output.agreements = p.productAgreements.data;
            (output, false)
        }
        _ => {
            log::warn!("No se recuperan agreements");
            (output, true)
        }
    }
}

fn look_for_invparty() {}

fn look_for_agreement() {}

fn look_for_lcr() {}

fn look_for_dni() {}

fn look_for_cid() {}

fn look_for_personid() {}
