mod test_first;
mod test_vector;
mod test_number;

fn main() {

    test_first::add_two_integers::test();

    // test - Vector
    test_vector::get_concatenation::test();
    test_vector::build_array::test();
    test_vector::running_sum::test();
    test_vector::maximum_wealth::test();
    test_vector::shuffle::test();

    // test - Numbers
    test_number::minimum_sum::test();
    test_number::num_identical_pairs::test();
    test_number::kids_with_candies::test();
    test_number::subtract_product_and_sum::test();
    test_number::smaller_numbers_than_current::test();

    // TODO test - String
    // TODO test - Encoding/Decoding
    // TODO test - Iterations


}
