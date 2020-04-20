#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn cpu_temp_test() {
        assert_eq!(cpu::cpu_temp(), Some(value));
    }
}
