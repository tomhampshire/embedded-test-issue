#![no_std]
#![no_main]

#[cfg(test)]
#[embedded_test::tests]
mod tests {
    use defmt_rtt as _;
    use embassy_stm32 as _;

    #[test]
    fn is_ok() {
        let _config = embassy_stm32::Config::default();
        assert!(true);
    }
}
