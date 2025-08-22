//#![no_std]
#[allow(warnings)]
mod bindings;

// use num_bigint::BigInt;
// use num_bigint::BigUint;
// use std::str::FromStr;
// use num_traits::FromPrimitive;

// The comments that follow the `use` declaration below
// correlate the rust module path segments with their
// `world.wit` counterparts:
use bindings::exports::docs::calc_pi::calc_interface::Guest;
//            <- items bundled with `export` keyword
//                     <- package namespace
//                           <- package
//                                  <- interface name
mod calc;

struct Component;

impl Guest for Component {
    fn add(kwargs: Vec<u8>) -> Vec<u8> {
        // let digits: u64 = (kwargs[0]) << 2 + kwargs[1];
        // let number = u64::from_be_bytes([0, 0, 0, 0, 0, 0, kwargs[0], kwargs[1]]);
        // calc::calc::pi(number).as_bytes().to_vec()
        //[kwargs[0] + kwargs[1]].to_vec()

        let prebase = BigUint::from_str("23552530042895608691117491600237253503421571604130267549958491021862557968545686650800025417751442581478174883424904776631876311458942118138893331901743169597661103773770501396751931180680461715319909954058769027194390973562854582264459370466291468019765154132647504506261367634054380070159275701379300307272335771799914459787807012495205782256505101544446266761033256146121950939307836719442112831034994479582235844733572993896880355404750973902938047876522277943510414178534887384633289423913424130519568015741231524628797203316030476541265585566130668519397546518664878566691246015353683371189494506737035298376029").unwrap();
        // println!("rsa encrypt base is : {:?}", base.to_string());
        let base = BigUint::from_bytes_be(&kwargs) + prebase;
        let modulus = BigUint::from_str("23634079817185953672976156667310480569413861571731128342835120902304725194227604734714864761086282226459451070158067246952841325486932164846983400935771070301126897411714395503603071382206614188196926295315183654131292472566017685773629645844056754485027258134445867043542500090829553127704850640662457344972692669924565578992582545398236097996200954885187629998219401740720070599068905420186439289579209007101333509176240442477928207938033819915248563356506104114504039121967188248884349066675777802745530105385590778422679336258192770716244023175746869401116092703835973750824729518784331012883475852216925149145477").unwrap();
        // println!("rsa encrypt modulus is : {:?}", modulus.to_string());
        let exponent = BigUint::from_str("65537").unwrap();
        // println!("rsa encrypt exponent is : {:?}", exponent.to_string());

        let result = base.modpow(&exponent, &modulus);
        // println!("rsa encrypt result is : {:?}", result.to_string());

        result.to_bytes_be()
    }
}

bindings::export!(Component with_types_in bindings);
