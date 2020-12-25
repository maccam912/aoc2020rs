use num::bigint;

lazy_static! {
    static ref B7: bigint::BigUint = bigint::BigUint::from(7usize);
    static ref B20201227: bigint::BigUint = bigint::BigUint::from(20201227usize);
}

fn find_loop_size(pubkey: &usize) -> bigint::BigUint {
    //let mut n = bigint::BigUint::from(17580933usize);
    let mut n = bigint::BigUint::from(1usize);
    let bpubkey = bigint::BigUint::from(*pubkey);
    loop {
        if bigint::BigUint::modpow(&B7, &n, &B20201227) == bpubkey {
            return n;
        }
        n += 1usize;
    }
}

pub fn day25a() -> i64 {
    let door_pubkey = 11562782usize;
    let card_pubkey = 18108497usize;
    let door_loop_size = find_loop_size(&door_pubkey);
    let enc_key = bigint::BigUint::modpow(
        &bigint::BigUint::from(card_pubkey),
        &door_loop_size,
        &B20201227,
    );
    enc_key.to_string().parse::<i64>().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day25;
    use num::bigint;

    #[test]
    fn test_find_loop_size() {
        assert_eq!(
            day25::find_loop_size(&5764801),
            bigint::BigUint::from(8usize)
        );
        assert_eq!(
            day25::find_loop_size(&17807724),
            bigint::BigUint::from(11usize)
        );
    }
}
