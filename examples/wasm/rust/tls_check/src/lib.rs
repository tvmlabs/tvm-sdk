
mod bindings;

mod tls_session;

struct Component;

impl Guest for Component {
    fn add(kwargs: Vec<u8>) -> Vec<u8> {
        // let digits: u64 = (kwargs[0]) << 2 + kwargs[1];
        //let number = u64::from_be_bytes([0, 0, 0, 0, 0, 0, kwargs[0], kwargs[1]]);

        let public_key_data = tls_session::extract_json_public_key_from_tls(kwargs);
        public_key_data //calc::calc::pi(number).as_bytes().to_vec()
        //[kwargs[0] + kwargs[1]].to_vec()
    }
}