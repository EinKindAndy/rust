use std;
import std::arc;
fn main() {
    let x = ~arc::rw_arc(1);
    let mut y = none;
    do x.write_downgrade |write_mode| {
        y = some(x.downgrade(write_mode));
        //~^ ERROR cannot infer an appropriate lifetime
    }
    // Adding this line causes a method unification failure instead
    // do (&option::unwrap(y)).read |state| { assert *state == 1; }
}
