/* This code will cause compile time error since here
 * we are trying to modify an immnutable variable.
 */ 
/* 
fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    // Modifying x viz. an immutable variable
    x = 9;
    println!("The value of x is: {x}");
}
*/

/* This code will run fine since the updated variable 
 * is mutable.
 */
/*
 fn main() {
    let mut var: i32 = 7;
    println!("The value of var is: {var}");
    // Modifying var viz. mutable
    var = 9;
    println!("var is updated to: {var}");
}
*/

/* Demonstration of constants. */
/*
fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("THREE_HOURS_IN_SECONDS: {THREE_HOURS_IN_SECONDS}");
}
*/

/* Demonstration of SHADOWING. */
/*
fn main() {
    let x = 5;
    let x = x + 1;
    println!("The value of x before entering the inner scope is: {x}");
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x after exiting the inner scope is: {x}");
}
*/

fn main() {
    /* This fashion of code will make the first immutable variable
     * unused and will result in compilation warning with a suggestion
     * to use '_' (_immutable_var) before the var name which is a convention to indaicate
     * the compiler that the variable is intentionally unused.
     */

/*  let immutable_var: u32 = 7;
    let immutable_var: u32 = 9;

    println!("immutable_var: {immutable_var}");
*/

    /* This fashion will let you redefine the immuatable variable.
     * Notice the use of 'let' for reassigning the immutable variable;
     * it's in contrast to the mutable varibale where a variable can
     * be redefined without the use of 'let' 
     */
    let immutable_var: u32 = 7;
    println!("immutable_var: {immutable_var}");
 
    let immutable_var: u32 = 9;
    println!("immutable_var: {immutable_var}");
}