use nipa_macros::letter;
use nipa_traits::Letter;

letter!(H);
letter!(E);
letter!(L);
letter!(O);
letter!(N);
letter!(I);
letter!(P);
letter!(A);

fn main() {
    let h = H::new();
    let e = E::new();
    let l1 = L::new();
    let l2 = L::new();
    let o = O::new();
    let n = N::new();
    let i = I::new();
    let p = P::new();
    let a = A::new();
    A::print_upper_other(h);
    e.print_lower();
    I::print_lower_other(l2);
    l1.print_lower();
    O::print_lower_space_other(o);
    n.print_upper();
    E::print_upper_other(i);
    p.print_upper();
    A::println_upper_other(a);
}
