A simple basic wrapper of the Tiktoken tokenizer in Haskell.

Currently only supports Linux and only exposes the basic Encode function with the gpt2 tokenizer config, but exposing other functions and configs should be trivial.

Requires `cargo` to compile. `cabal test` should just work.