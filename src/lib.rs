pub mod input;

mod tests {
    #[test]
    fn it_works() {
        assert!(true);
    }

    #[test]
    fn splitter() {
        let split_comma: Vec<&str> = "450,602:".split(',').collect();
        println!("{:?}", split_comma);
        let split: Vec<u32> = split_comma.iter().map(|s| s.trim_right_matches(':').parse().unwrap()).collect();

        for i in split {
            println!("{}", i);
        }
    }
}
