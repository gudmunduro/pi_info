#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn cpu_temp_test() {
        println!("{}", cpu::cpu_temp());
        assert_eq!(cpu::cpu_temp(), Ok(value));
    }
}
