#[macro_use]
extern crate lazy_static;

mod token;
mod lexer;
mod lexer_tests;
mod parser;
mod parser_impl;
mod parser_tests;

fn main() {
  let input_program = r"snippet fun(a, b, c, x, y) {
                          static x = 0;
                          if (a >= b) {
                            a = x;
                            b = y;
                          } elif (c >= d) {
                            m == 5;
                          }
                        }";
  parser::parse_prog(lexer::get_tokens(input_program));
}
