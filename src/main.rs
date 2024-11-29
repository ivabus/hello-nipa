use nipa_macros::letter;

letter!(H);
letter!(E);
letter!(L);
letter!(O);
letter!(N);
letter!(I);
letter!(P);
letter!(A);

fn main() {
    H::print_upper();
    E::print_lower();
    L::print_lower();
    L::print_lower();
    O::print_lower_space();
    N::print_upper();
    I::print_upper();
    P::print_upper();
    A::println_upper();
}
