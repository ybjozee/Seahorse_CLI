use std::env;

use reqwest::{blocking::Client, StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct SuccessResponse {
    api_version: String,
    body: String,
    date_created: String,
    date_sent: Option<String>,
    date_updated: String,
    direction: String,
    from: String,
    messaging_service_sid: Option<String>,
    price: Option<String>,
    price_unit: Option<String>,
    sid: String,
    status: String,
    to: String,
    uri: String,
}

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    code: u16,
    message: String,
    more_info: String,
    status: u16,
}

fn error_message(body: String) -> String {
    let error_response: ErrorResponse = serde_json::from_str(&body).expect("Unable to deserialise JSON error response.");
    format!("There was a problem sending your message :( \nWhatsApp message was not sent because: {:?}.", error_response.message)
}

fn success_message(body: String) -> String {
    let response: SuccessResponse = serde_json::from_str(&body).expect("Unable to deserialise JSON success response.");
    format!("Notification handled successfully!! \nYour WhatsApp message with content {:?} is now {:?}.", response.body, response.status)
}

pub fn send_whats_app_message(message: String, recipient_phone_number: &str) -> (String, bool) {
    let twilio_account_sid =
        env::var("TWILIO_ACCOUNT_SID").expect("Twilio Account SID could not be retrieved.");
    let twilio_auth_token =
        env::var("TWILIO_AUTH_TOKEN").expect("Twilio Auth Token could not be retrieved.");
    let twilio_phone_number =
        env::var("TWILIO_WHATSAPP_NUMBER").expect("The Twilio phone number could not be retrieved.");

    let request_url =
        format!("https://api.twilio.com/2010-04-01/Accounts/{twilio_account_sid}/Messages.json");

    let formatted_phone_number = format!("whatsapp:{recipient_phone_number}");
    let formatted_twilio_phone_number = format!("whatsapp:{twilio_phone_number}");
    let client = Client::new();
    let request_params = [
        ("To", formatted_phone_number),
        ("From", formatted_twilio_phone_number),
        ("Body", message.to_string()),
    ];

    let response = client
        .post(request_url)
        .basic_auth(twilio_account_sid, Some(twilio_auth_token))
        .form(&request_params)
        .send();

    match response {
        Ok(response) => {
            let status = response.status();
            let body = match response.text() {
                Ok(result) => result,
                Err(error) => panic!(
                    "Problem extracting the JSON body content. Reason: {:?}",
                    error
                ),
            };
            match status {
                StatusCode::BAD_REQUEST => (error_message(body), false),
                StatusCode::OK | StatusCode::CREATED => (success_message(body), true),
                _ => (format!("Received status code: {status}"), false),
            }
        }
        Err(error) => { panic!("Problem executing request. Reason: {:?}", error) }
    }
}