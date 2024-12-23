#[cfg(test)]
pub mod day22tests {
    use adventofcode24::week4::day22::*;

    #[test]
    fn secret_number_test() {
        let mut secret = 123;
        let next_secret_numbers = vec![
            15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
            5908254,
        ];
        for expected_secret in next_secret_numbers {
            secret = next_random(secret);
            assert_eq!(secret, expected_secret);
        }
    }

    #[test]
    fn four_secrets_2000() {
        let secrets = vec![1, 10, 100, 2024];
        let expected = vec![8685429, 4700978, 15273692, 8667524];
        let mut sum = 0;
        for (secret_idx, mut secret) in secrets.into_iter().enumerate() {
            for _ in 0..2000 {
                secret = next_random(secret);
            }
            assert_eq!(secret, *expected.get(secret_idx).unwrap());
            sum += secret;
        }

        assert_eq!(sum, 37327623);
    }
}
