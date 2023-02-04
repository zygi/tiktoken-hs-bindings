module Main (main) where

import TiktokenHSBindings (encodeNative)
import Test.HUnit

testEmpty = TestCase $ do
    res <- encodeNative ""
    assertEqual "Empty string" [] res


testReference = TestCase $ do
    res <- encodeNative "A quick brown fox jumps over the lazy dog."
    assertEqual "Reference sentence" [32, 2068, 7586, 21831, 18045, 625, 262, 16931, 3290, 13] res

main :: IO ()
main = do
    runTestTT $ TestList [testEmpty, testReference]
    return ()