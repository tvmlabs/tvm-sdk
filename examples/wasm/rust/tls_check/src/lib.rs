//#![no_std]
#[allow(warnings)]
mod bindings;
mod tls_session;

use bindings::exports::docs::tlschecker::tls_check_interface::Guest;

struct Component;

impl Guest for Component {
    fn tlscheck(kwargs: Vec<u8>) -> Vec<u8> {
        let public_key_data = tls_session::extract_json_public_key_from_tls(kwargs);
        public_key_data
    }
}

bindings::export!(Component with_types_in bindings);
