mod test_first;
mod test_vector;

fn main() {

    test_first::add_two_integers::test();

    // test - Vector
    test_vector::get_concatenation::test();
    test_vector::build_array::test();
    test_vector::running_sum::test();
    test_vector::maximum_wealth::test();
    test_vector::shuffle::test();

    // TODO test - Numbers
    // TODO test - String
    // TODO test - Encoding/Decoding
    // TODO test - Iterations


}
