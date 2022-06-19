use rand::{distributions::Alphanumeric, Rng};

pub fn rand_str() -> String {
    let s = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect();

    return s;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_is_6_chars() {
        let str = rand_str();
        println!("{}", str);
        assert_eq!(str.chars().count() == 6, true);
    }
}
