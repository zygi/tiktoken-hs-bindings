module Main where

import TiktokenHSBindings (encodeNative)


main :: IO ()
main = do
  putStrLn "Hello, Haskell!"
  res <- encodeNative "Hello, Haskell!"
  print res
