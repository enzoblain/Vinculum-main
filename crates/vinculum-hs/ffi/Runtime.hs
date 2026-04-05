{-# LANGUAGE ForeignFunctionInterface #-}

module Runtime where

import Codec
import Dispatch (dispatchUserFunction)
import qualified Data.ByteString as BS
import qualified Data.ByteString.Unsafe as BSU
import Data.Word
import Foreign
import Foreign.C.Types
import Foreign.Marshal.Alloc (free, mallocBytes)
import Foreign.Marshal.Utils (copyBytes)

data Buffer = Buffer
    { ptr :: Ptr Word8
    , len :: CSize
    }

instance Storable Buffer where
    sizeOf _ = sizeOf (nullPtr :: Ptr Word8) + sizeOf (undefined :: CSize)
    alignment _ = alignment (nullPtr :: Ptr Word8)

    peek p = do
        bufferPtr <- peekByteOff p 0
        bufferLen <- peekByteOff p (sizeOf (nullPtr :: Ptr Word8))
        pure (Buffer bufferPtr bufferLen)

    poke p (Buffer bufferPtr bufferLen) = do
        pokeByteOff p 0 bufferPtr
        pokeByteOff p (sizeOf (nullPtr :: Ptr Word8)) bufferLen

foreign export ccall free_haskell_buffer :: Ptr Word8 -> IO ()

foreign export ccall call_haskell_function
    :: Ptr Word8 -> CSize -> Ptr Word8 -> CSize
    -> Ptr (Ptr Word8) -> Ptr CSize -> IO ()

free_haskell_buffer :: Ptr Word8 -> IO ()
free_haskell_buffer ptr = free ptr

call_haskell_function
    :: Ptr Word8 -> CSize -> Ptr Word8 -> CSize
    -> Ptr (Ptr Word8) -> Ptr CSize -> IO ()
call_haskell_function namePtr nameLen inputPtr inputLen outPtrPtr outLenPtr = do
    functionName <- bytesToByteString namePtr nameLen
    input <- bytesToByteString inputPtr inputLen
    output <- dispatchUserFunction functionName input
    Buffer resultPtr resultLen <- byteStringToBuffer output
    poke outPtrPtr resultPtr
    poke outLenPtr resultLen

bytesToByteString :: Ptr Word8 -> CSize -> IO BS.ByteString
bytesToByteString p n = BS.packCStringLen (castPtr p, fromIntegral n)

byteStringToBuffer :: BS.ByteString -> IO Buffer
byteStringToBuffer bs =
    BSU.unsafeUseAsCStringLen bs $ \(srcPtr, srcLen) -> do
        dstPtr <- mallocBytes srcLen
        copyBytes dstPtr (castPtr srcPtr) srcLen
        pure (Buffer (castPtr dstPtr) (fromIntegral srcLen))