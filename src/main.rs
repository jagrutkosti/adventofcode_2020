// Allowing dead code in whole application as parts will never be run.
// Another alternate would be to create a library, but that's an overkill for the purpose.
#![allow(dead_code)]

mod puzzles;
mod utils;

use puzzles::day10;

fn main() {
    day10::day10();
}
