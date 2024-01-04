module Endomorph.Lexer.Operator where

import Endomorph.Lexer.Common (Parser)
import Endomorph.Lexer.Util (symbolToToken)
import Endomorph.Token (Operator (..), Token (Operator))

operator :: Parser Token
operator =
  symbolToToken
    [ ("<<=", Operator LShiftAssign),
      ("**=", Operator PowerAssign),
      (">>=", Operator RShiftAssign),
      ("...", Operator Spread),
      ("&=", Operator AndAssign),
      ("->", Operator Arrow),
      ("/=", Operator DivideAssign),
      ("==", Operator Equality),
      (">=", Operator GreaterEquals),
      ("<=", Operator LessEquals),
      ("<<", Operator LShift),
      ("-=", Operator MinusAssign),
      ("%=", Operator ModuloAssign),
      ("!=", Operator NotEquals),
      ("|=", Operator OrAssign),
      ("+=", Operator PlusAssign),
      ("**", Operator Power),
      (">>", Operator RShift),
      ("*=", Operator TimesAssign),
      ("^=", Operator XorAssign),
      ("&", Operator And),
      ("=", Operator Assign),
      ("/", Operator Divide),
      (".", Operator Dot),
      (">", Operator Greater),
      ("<", Operator Less),
      ("-", Operator Minus),
      ("%", Operator Modulo),
      ("!", Operator Not),
      ("|", Operator Or),
      ("+", Operator Plus),
      ("*", Operator Times)
    ]