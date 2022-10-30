use vos_error::for_3rd::EmailAddress;

pub trait Validator {}

pub fn valid_email(email: &str) -> bool {
    EmailAddress::is_valid(email)
}
