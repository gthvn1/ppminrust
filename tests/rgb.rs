#[cfg(test)]
mod tests {

    use ppminrust::ppm::rgb::RGB;

    #[test]
    fn test_un() {
        let color = RGB::new(0x000000);
        assert_eq!(color.to_string(), "0 0 0");
    }
}
