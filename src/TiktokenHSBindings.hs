module TiktokenHSBindings (encodeNative) where


{-# LANGUAGE ForeignFunctionInterface #-}

import Foreign
import Foreign.C.Types
import Foreign.ForeignPtr
import Foreign.C (CString, withCString)

-- Wrapping the fact.rs module...
foreign import ccall "encode_native"
  encode_native :: CString -> Ptr (Ptr Word64) -> IO Word64


encodeNative :: String -> IO [Word64]
encodeNative s = do
  withCString s $ \cstr -> do
    ptr <- malloc
    resLen <- encode_native cstr ptr
    resPtr <- peek ptr
    res <- peekArray (fromIntegral resLen) resPtr
    free ptr
    pure res