use reqwest::blocking::*;

/*
use reqwest::tls::Identity;

use std::fs::File;
use std::io::Read;
*/

pub fn get_uri_html_content(uri: &str) -> String {
    /* Build the pem buffer (Contains the x509 certificate and key) */
    /*
    let mut pem_buf: Vec<u8> = Vec::new();
    File::open("/home/dev/certif/certif.pem")
        .unwrap()
        .read_to_end(&mut pem_buf)
        .unwrap();

    /* Build the x509 identity */
    let identity: Identity = Identity::from_pem(&pem_buf).unwrap();
    */
    /* Create the client builder */
    let client_builder: ClientBuilder = ClientBuilder::new().danger_accept_invalid_certs(true);

    /* Build the client */
    let client: Client = client_builder.build().unwrap();

    /* Sends the get request and returns the html content as a String */
    client.get(uri).send().unwrap().text().unwrap()
}
