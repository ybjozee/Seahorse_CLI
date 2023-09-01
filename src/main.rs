use std::env;

use colored::Colorize;
use dotenv::dotenv;
use seahorse::{App, Command, Context, Flag, FlagType};
use seahorse::error::FlagError;

use cli_response::CLIResponse;

mod caesar;
mod bacon;
mod input_filter;
mod notification;

mod cli_response;

fn main() {
    dotenv().ok();

    let args: Vec<String> = env::args().collect();
    let app = App::new("cipher_cli")
        .description("Encrypt and decrypt secret messages in seconds!!!")
        .usage("cipher_cli [command] [arg]")
        .action(base_action())
        .command(decrypt_command())
        .command(encrypt_command());

    app.run(args);
}

fn base_action() -> fn(&Context) {
    |_c: &Context| {
        println!("{}", r#"
        This application encrypts and decrypts secret messages with ease.
        Try out the Caesar and Bacon Cipher options to generate secret messages and share with your inner circle
        "#.yellow()
        );
        println!("{}", r#"
        Example Usage

        cipher_cli encrypt Welcome to the hallowed chAmbers!  --algorithm=caesar --rotations=54

        cipher_cli encrypt Welcome to the hallowed chambers! --algorithm=bacon

        cipher_cli decrypt Ygneqog vq vjg jcnnqygf ejCodgtu! --algorithm=caesar --rotations=54

        "#.blue())
    }
}

fn decrypt_command() -> Command {
    Command::new("decrypt")
        .description("decrypt input using a specified algorithm")
        .usage("cipher_cli decrypt(de, d) [input] -r=[rotations] -a=[algorithm]")
        .alias("d")
        .alias("de")
        .action(decrypt_action)
        .flag(
            Flag::new("rotations", FlagType::Int)
                .description("rotations flag")
                .alias("r"),
        )
        .flag(
            Flag::new("algorithm", FlagType::String)
                .description("algorithm flag")
                .alias("a")
                .alias("al"),
        )
}

fn decrypt_action(c: &Context) {
    let input = &c.args.join(" ");
    let mut response = CLIResponse::new();

    if let Ok(algorithm) = c.string_flag("algorithm") {
        match algorithm.to_lowercase().as_str() {
            "bacon" => {
                let plaintext = bacon::decrypt(input);
                response.success_update(format!("Ciphertext: {input} \nPlaintext: {plaintext}"));
            }
            "caesar" => match c.int_flag("rotations") {
                Ok(rotations) => if let Some(rotations) = input_filter::number_of_rotations(rotations) {
                    let plaintext = caesar::decrypt(input, rotations);
                    response.success_update(format!("Ciphertext: {input} \nRotations: {rotations} \nPlaintext: {plaintext}"));
                } else {
                    response.error_update(format!("Rotations cannot be less than 0, {rotations} provided"));
                },
                Err(e) => match e {
                    FlagError::NotFound | FlagError::Undefined => {
                        response.error_update(String::from("Required flag \"rotations\" not provided"));
                    }
                    _ => {
                        response.error_update(String::from("Invalid value provided for \"rotations\""));
                    }
                }
            },
            _ => {
                response.error_update(String::from("Unknown algorithm provided"));
            }
        }
    } else {
        response.error_update(String::from("Required flag \"algorithm\" not provided"));
    }
}

fn encrypt_command() -> Command {
    Command::new("encrypt")
        .description("encrypt command")
        .usage("cipher_cli encrypt [input]")
        .action(encrypt_action)
        .flag(
            Flag::new("rotations", FlagType::Int)
                .description("rotations flag")
                .alias("r")
                .alias("ro"),
        )
        .flag(
            Flag::new("algorithm", FlagType::String)
                .description("algorithm flag")
                .alias("a")
                .alias("al"),
        )
        .flag(
            Flag::new("recipient", FlagType::Int)
                .description("recipient flag")
                .alias("re"),
        )
}

fn encrypt_action(c: &Context) {
    let input = &c.args.join(" ");
    let mut response = CLIResponse::new();

    if let Ok(algorithm) = c.string_flag("algorithm") {
        let mut ciphertext = String::new();
        match algorithm.to_lowercase().as_str() {
            "bacon" => {
                ciphertext = bacon::encrypt(input);
                response.success_update(format!("Plaintext: {input} \nCiphertext: {ciphertext}\n"));
            }
            "caesar" => match c.int_flag("rotations") {
                Ok(rotations) => if let Some(rotations) = input_filter::number_of_rotations(rotations) {
                    ciphertext = caesar::encrypt(input, rotations);
                    response.success_update(format!("Plaintext: {input} \nRotations: {rotations} \nCiphertext: {ciphertext}\n"));
                } else {
                    response.error_update(format!("Rotations cannot be less than 0, {rotations} provided"));
                },
                Err(e) => match e {
                    FlagError::NotFound | FlagError::Undefined => {
                        response.error_update(String::from("Required flag \"rotations\" not provided"));
                    }
                    _ => {
                        response.error_update(String::from("Invalid value provided for \"rotations\""));
                    }
                }
            },
            _ => {
                response.error_update(String::from("Unknown algorithm provided"));
            }
        }

        if response.is_success() {
            if let Ok(recipient) = c.int_flag("recipient") {
                let (notification_response, is_successful) = handle_notification(ciphertext, recipient);
                if is_successful {
                    response.success_update(notification_response);
                } else {
                    response.error_update(notification_response);
                }
            }
        }
    } else {
        response.error_update(String::from("Required flag \"algorithm\" not provided"));
    }
}

fn handle_notification(ciphertext: String, recipient: isize) -> (String, bool) {
    if let Some(recipient) = input_filter::recipient_phone_number(format!("+{recipient}").as_str()) {
        return notification::send_whats_app_message(format!("From your partner in mischief: {ciphertext}"), recipient);
    }
    (String::from("Invalid phone number provided for \"recipient\""), false)
}