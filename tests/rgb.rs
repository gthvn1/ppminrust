#[cfg(test)]
mod tests {

    use ppminrust::ppm::rgb::Rgb;

    #[test]
    fn test_new_hexa() {
        let color = Rgb::new(0xFF0102);
        assert_eq!(color.to_string(), "255 1 2");
    }

    #[test]
    fn test_new_rgb() {
        let color = Rgb::new((255, 0, 0));
        assert_eq!(color.to_string(), "255 0 0");
    }
}
