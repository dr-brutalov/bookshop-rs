use log::{error, warn};
use regex::Regex;

// Takes a String and removes unnecessary white space.
// This will reduce customer and user confusion in many
// scenarios.
pub fn standardize_whitespace(input_string: String) -> String {
    // Trim spaces from each end of the String
    let tmp_string = input_string.trim().to_string();

    // Trim any excess spaces from the String internals
    let inner_space_reg = Regex::new(r"\s+").unwrap_or_else(|error| {
        error!(target: "error", "Regex error: {}", error);
        panic!("Regex error: {}", error);
    });

    return inner_space_reg
        .replace_all(tmp_string.as_str(), " ")
        .to_string();
}

// Restrict input to alphanumeric characters. Doing so
// removes support for emoji-characters (😔) and also
// troublesome characters such as the escape char (\)
// and common HTML formatting, such as (<>)

pub fn standardize_price(input_price: f64, input_method: String) -> Result<(), String> {
    if input_price <= 0.00 {
        error!(target: "error", "An invalid price of 0.00 or less was submitted in the {} method.", input_method);
        let user_error_message =
            format!("Please provide a value greater than \"0.00\" for the price.");
        return Err(user_error_message);
    }

    match input_price.to_string().contains(".") {
        true => (),
        false => input_price.to_string().push('.'),
    };

    let num_format_regex = Regex::new(r"^\d{1,5}\.\d{0,2}$").unwrap();

    let valid_price = num_format_regex.is_match(&input_price.to_string());

    if !valid_price {
        error!(target: "error", "Invalid price of \"{}\" in {} method.", input_price, input_method);
        let user_error_message = "Please format your price as \"12345.67\". \n\t Valid prices must be between 0.01 and 99999.99".to_string();
        return Err(user_error_message);
    } else {
        return Ok(());
    }
}

pub fn check_for_alphanumeric_input(
    input_string: String,
    input_field: String,
    input_method: String,
) -> Result<(), String> {
    if string_is_all_spaces(input_string.clone()) || string_is_empty(input_string.clone()) {
        // Format a message for the user so they can adjust their input.
        let user_error_message = format!(
            "Please do not provide empty space for \"{}\". \n\t Alphanumeric characters form a valid input.",
            input_field
        );
        warn!(target: "warn", "Empty or all space input given for field \"{}\" in \" {}\" functionality.", input_field, input_method);
        return Err(user_error_message);
    }

    // Check input for alphanumeric, common punctuation, and spaces.
    // let string_is_valid = input_string.chars().all(|x| (x.is_alphanumeric() || x == '.' || x == ',' || x.is_whitespace()));
    if !input_string
        .chars()
        .all(|x| (x.is_alphanumeric() || x == '.' || x == ',' || x.is_whitespace()))
    {
        let user_error_message = format!(
            "Please only provide alphanumeric characters, \",\", and \".\" as input for {}.",
            input_field
        );
        warn!(target: "warn", "Invalid input in \"{}\": {}", input_field, input_string.clone());
        return Err(user_error_message);
    } else {
        return Ok(());
    }
}

fn string_is_empty(input_string: String) -> bool {
    let mut is_empty = false;
    if input_string.is_empty() {
        is_empty = true;
        warn!(target: "warn", "An input string was found to be empty.")
    }
    return is_empty;
}

fn string_is_all_spaces(input_string: String) -> bool {
    let mut is_empty = false;
    if input_string.chars().all(char::is_whitespace) {
        is_empty = true;
        warn!(target: "warn", "An input string was found to be all spaces.")
    }
    return is_empty;
}
