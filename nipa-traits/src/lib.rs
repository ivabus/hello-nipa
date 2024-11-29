pub trait Letter {
    fn print_upper(&self);

    fn print_lower(&self);

    fn print_upper_space(&self);

    fn print_lower_space(&self);

    fn println_upper(&self);

    fn println_lower(&self);

    fn print_upper_dot(&self);

    fn print_lower_dot(&self);

    fn print_upper_space_dot(&self);

    fn print_lower_space_dot(&self);

    fn print_upper_space_dot_after(&self);

    fn print_lower_space_dot_after(&self);

    fn println_upper_dot(&self);

    fn println_lower_dot(&self);

    fn print_upper_comma(&self);

    fn print_lower_comma(&self);

    fn print_upper_space_comma(&self);

    fn print_lower_space_comma(&self);

    fn print_upper_space_comma_after(&self);

    fn print_lower_space_comma_after(&self);

    fn println_upper_comma(&self);

    fn println_lower_comma(&self);

    fn print_upper_other(other: impl Letter) {
        other.print_upper()
    }

    fn print_lower_other(other: impl Letter) {
        other.print_lower()
    }

    fn print_upper_space_other(other: impl Letter) {
        other.print_upper_space()
    }

    fn print_lower_space_other(other: impl Letter) {
        other.print_lower_space()
    }

    fn println_upper_other(other: impl Letter) {
        other.println_upper()
    }

    fn println_lower_other(other: impl Letter) {
        other.println_lower()
    }

    fn print_upper_dot_other(other: impl Letter) {
        other.print_upper_dot()
    }

    fn print_lower_dot_other(other: impl Letter) {
        other.print_lower_dot()
    }

    fn print_upper_space_dot_other(other: impl Letter) {
        other.print_upper_space_dot()
    }

    fn print_lower_space_dot_other(other: impl Letter) {
        other.print_lower_space_dot()
    }

    fn print_upper_space_dot_after_other(other: impl Letter) {
        other.print_upper_space_dot_after()
    }

    fn print_lower_space_dot_after_other(other: impl Letter) {
        other.print_lower_space_dot_after()
    }

    fn println_upper_dot_other(other: impl Letter) {
        other.print_upper_dot()
    }

    fn println_lower_dot_other(other: impl Letter) {
        other.print_lower_dot()
    }

    fn print_upper_comma_other(other: impl Letter) {
        other.print_upper_comma()
    }

    fn print_lower_comma_other(other: impl Letter) {
        other.print_lower_comma()
    }

    fn print_upper_space_comma_other(other: impl Letter) {
        other.print_upper_space_comma()
    }

    fn print_lower_space_comma_other(other: impl Letter) {
        other.print_lower_space_comma()
    }

    fn print_upper_space_comma_after_other(other: impl Letter) {
        other.print_upper_space_comma_after()
    }

    fn print_lower_space_comma_after_other(other: impl Letter) {
        other.print_lower_space_comma_after()
    }

    fn println_upper_comma_other(other: impl Letter) {
        other.print_upper_comma()
    }

    fn println_lower_comma_other(other: impl Letter) {
        other.print_lower_comma()
    }
}
