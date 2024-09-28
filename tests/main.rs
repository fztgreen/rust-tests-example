use rand::Rng;
#[path = "../src/main.rs"] mod main;

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn should_return_added() {
        let mut rng = rand::thread_rng();
        let arg1 = rng.gen_range(1..100);
        let arg2 = rng.gen_range(1..100);
        let expected = arg1 + arg2;

        let result = main::add(arg1, arg2);

        assert_eq!(expected, result);
    }

    #[test_case(1, 2, 3)]
    #[test_case(22, 5, 27)]
    #[test_case(321, 123, 444)]
    #[test_case(1000, 251, 1251)]
    fn should_return_numbers_added(a: i32, b: i32, expected: i32) {
        let actual: i32 = main::add(a, b);

        assert_eq!(expected, actual);
    }

    #[test_case(0, 0 , 0)]
    #[test_case(1, 1 , 1)]
    #[test_case(1, 121 , 121)]
    #[test_case(-5, 5 , -25)]
    #[test_case(-123, -2 , 246)]
    fn should_return_numbers_multiplied(a: i32, b: i32, expected: i32) {
        let actual: i32 = main::multiply(a, b);

        assert_eq!(expected, actual);
    }

    #[test_case(1,1,1)]
    #[test_case(5,-5,-1)]
    #[test_case(-25,-5,5)]
    #[test_case(100,5,20)]
    fn should_return_numbers_divided(a: i32, b: i32, expected: i32) {
        let actual: i32 = main::divide(a, b);

        assert_eq!(expected, actual);
    }

    #[test_case(1,1,0)]
    #[test_case(5,10,-5)]
    #[test_case(100,55,45)]
    #[test_case(10,0,10)]
    fn should_return_numbers_subtracted(a: i32, b: i32, expected: i32) {
        let actual: i32 = main::subtract(a, b);

        assert_eq!(expected, actual);
    }
}