extern crate acbidder;
extern crate reqwest;

use reqwest::Client;
use std::io::Read;

mod utils;

#[test]
fn response_404() {
    let server = utils::TestServer::new();
    let client = Client::new();
    let mut url = server.url();
    url.push_str("/bad");
    let resp = client.get(&url).send().unwrap();

    assert!(resp.status().is_client_error());
}

#[test]
fn response_acq_positive() {
    let server = utils::TestServer::new();
    let client = Client::new();

    let mut url = server.url();
    url.push_str("/acq/:nyt.com");
    let mut resp = client.get(&url).send().unwrap();

    let mut parsed_resp = String::new();
    resp.read_to_string(&mut parsed_resp).unwrap();

    assert_eq!(parsed_resp.cmp(&String::from("true")), std::cmp::Ordering::Equal);
}

#[test]
fn response_acq_negative() {
    let server = utils::TestServer::new();
    let client = Client::new();

    let mut url = server.url();
    url.push_str("/acq/:crazypartygirls.ru");
    let mut resp = client.get(&url).send().unwrap();

    let mut parsed_resp = String::new();
    resp.read_to_string(&mut parsed_resp).unwrap();

    assert_eq!(parsed_resp.cmp(&String::from("false")), std::cmp::Ordering::Equal);
}

