#[cfg(test)]
pub mod day17tests {
    use adventofcode24::week3::day17::*;

    #[test]
    fn simple_bst_test() {
        let mut computer = Computer::new(0, 0, 9);
        computer.compute(2, 6);
        assert_eq!(computer.b_val, 1);
    }

    #[test]
    fn full_test_one() {
        let mut computer = Computer::new(10, 0, 0);
        let instructions = vec![5, 0, 5, 1, 5, 4];
        computer.run(instructions);
        assert_eq!(computer.read_out(), "0,1,2");
    }

    #[test]
    fn test_output_idea() {
        for i in 0..1000 {
            for j in 0..=3 {
                let mut computer = Computer::new(i, 0, 0);
                let instructions = vec![5, j];
                computer.run(instructions);
                assert_eq!(computer.read_out(), j.to_string());
            }
        }
    }
}
