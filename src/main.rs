use z3::*;
use z3::{
    ast,
    ast::{Ast, Bool, Int},
    Config, Context, DeclKind, FuncDecl, Sort,
};
fn main() {
    // Create a Z3 context
    let config = z3::Config::new();
    let ctx = z3::Context::new(&config);

    // Create an integer constant "x"
    let x = Int::new_const(&ctx, "x");
    let y = Int::new_const(&ctx, "y");
    let z = Int::new_const(&ctx, "z");

    let solver = Solver::new(&ctx);
    solver.assert(&x.gt(&Int::from_i64(&ctx, 0)));
    solver.assert(&y.gt(&Int::from_i64(&ctx, 0)));
    solver.assert(&z.gt(&Int::from_i64(&ctx, 0)));

    // Define the equation x^2 + y^2 == z^2.
    let lhs = x.clone() * x.clone() + y.clone() * y.clone();
    let rhs = z.clone() * z.clone();

    solver.assert(&lhs._eq(&rhs));

    // Check if the solver is satisfiable
    let result = solver.check();

    if result == SatResult::Sat {
        // Get the model from the solver
        let model = solver.get_model().unwrap();

        // Evaluate and print the values of x, y, and z from the model
        let val_x = model.eval(&x, true).unwrap();
        let val_y = model.eval(&y, true).unwrap();
        let val_z = model.eval(&z, true).unwrap();

        println!("Solution: x = {}, y = {}, z = {}", val_x, val_y, val_z);
    } else {
        println!("No solution found.");
    }
}
