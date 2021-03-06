#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]

pub enum Token<'a> {
  // Variants that take an argument
  Identifier(&'a str),
  Value(u64),

  // Keywords: input, output, persistent, transient, constant, snippet, and, or, not, bit
  Input,
  Output,
  Const,
  Persistent,
  Transient,
  Snippet,
  BooleanAnd,
  BooleanOr,
  BooleanNot, // XXX: Not used in language yet
  Bit,

  // Separators
  Colon,
  SemiColon,
  Comma,
  Arrow,

  // Grouping operators
  ParenLeft,
  ParenRight,
  BraceLeft,
  BraceRight,
  SquareLeft,
  SquareRight,

  // Binary arithmetic operators + conditional operator
  Plus,
  Minus,
  Mul,
  Div,
  Modulo,
  Cond,

  //BitWise
  ShiftLeft,
  ShiftRight,

  // Comparison operators
  Equal,
  NotEqual,
  LTEQOp,
  GTEQOp,
  LessThan,
  GreaterThan,

  // Assignment
  Assign,

  // Additions for Network Model
  Import,
  Device,
  Packet,
  Global,
  If,
  Else,
  Annotation,

  // Dot operator
  Dot,
}
