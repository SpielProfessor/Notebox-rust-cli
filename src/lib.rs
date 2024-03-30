/* Spielprofessor/MrVollbart's utils
 * for the rust programming language
 * v1.0
 * 2024
 */

use std::io;
use std::io::Write;

pub fn scan() -> String {
    let mut out=String::new();
    io::stdout().flush().expect("Please insert a number");
    _ = io::stdin().read_line(&mut out);
    return out;
}
pub fn toint(text:String) -> i32{
    text.trim().parse().unwrap()
}
