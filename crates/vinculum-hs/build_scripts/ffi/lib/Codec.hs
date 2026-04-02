module Codec where

import qualified Data.ByteString as BS
import Data.Word
import Data.Int
import Data.Bits
import Unsafe.Coerce (unsafeCoerce)

data Value
    = VInt8 Int8
    | VInt16 Int16
    | VInt32 Int32
    | VInt64 Int64
    | VWord8 Word8
    | VWord16 Word16
    | VWord32 Word32
    | VWord64 Word64
    | VFloat32 Float
    | VFloat64 Double
    | VBool Bool
    | VChar Char

decodeValues :: BS.ByteString -> [Value]
decodeValues bs
    | BS.null bs = []
    | otherwise =
        let (value, rest) = decodeOne bs
         in value : decodeValues rest

decodeOne :: BS.ByteString -> (Value, BS.ByteString)
decodeOne bs =
    case BS.uncons bs of
        Nothing -> error "Empty input"
        Just (tag, rest)
            | tag == 0 ->
                let (payload, remaining) = BS.splitAt 1 rest
                 in (VInt8 (fromIntegral (bytesToWord8 payload)), remaining)

            | tag == 1 ->
                let (payload, remaining) = BS.splitAt 2 rest
                 in (VInt16 (fromIntegral (bytesToWord16 payload)), remaining)

            | tag == 2 ->
                let (payload, remaining) = BS.splitAt 4 rest
                 in (VInt32 (fromIntegral (bytesToWord32 payload)), remaining)

            | tag == 3 ->
                let (payload, remaining) = BS.splitAt 8 rest
                 in (VInt64 (fromIntegral (bytesToWord64 payload)), remaining)

            | tag == 4 ->
                let (payload, remaining) = BS.splitAt 1 rest
                 in (VWord8 (bytesToWord8 payload), remaining)

            | tag == 5 ->
                let (payload, remaining) = BS.splitAt 2 rest
                 in (VWord16 (bytesToWord16 payload), remaining)

            | tag == 6 ->
                let (payload, remaining) = BS.splitAt 4 rest
                 in (VWord32 (bytesToWord32 payload), remaining)

            | tag == 7 ->
                let (payload, remaining) = BS.splitAt 8 rest
                 in (VWord64 (bytesToWord64 payload), remaining)

            | tag == 8 ->
                let (payload, remaining) = BS.splitAt 4 rest
                 in (VFloat32 (unsafeCoerce (bytesToWord32 payload)), remaining)

            | tag == 9 ->
                let (payload, remaining) = BS.splitAt 8 rest
                 in (VFloat64 (unsafeCoerce (bytesToWord64 payload)), remaining)

            | tag == 10 ->
                let (payload, remaining) = BS.splitAt 1 rest
                 in case BS.unpack payload of
                        [b] -> (VBool (b /= 0), remaining)
                        _ -> error "Invalid Bool encoding"

            | tag == 11 ->
                let (payload, remaining) = BS.splitAt 4 rest
                    code = bytesToWord32 payload
                 in case toEnum (fromIntegral code) of
                        c -> (VChar c, remaining)

            | otherwise ->
                error ("Invalid type tag: " ++ show tag)

encodeInt8 :: Int8 -> BS.ByteString
encodeInt8 x =
    BS.pack (0 : word8ToBytes (fromIntegral x))

encodeInt16 :: Int16 -> BS.ByteString
encodeInt16 x =
    BS.pack (1 : word16ToBytes (fromIntegral x))

encodeInt32 :: Int32 -> BS.ByteString
encodeInt32 x =
    BS.pack (2 : word32ToBytes (fromIntegral x))

encodeInt64 :: Int64 -> BS.ByteString
encodeInt64 x =
    BS.pack (3 : word64ToBytes (fromIntegral x))

encodeWord8 :: Word8 -> BS.ByteString
encodeWord8 x =
    BS.pack (4 : word8ToBytes x)

encodeWord16 :: Word16 -> BS.ByteString
encodeWord16 x =
    BS.pack (5 : word16ToBytes x)

encodeWord32 :: Word32 -> BS.ByteString
encodeWord32 x =
    BS.pack (6 : word32ToBytes x)

encodeWord64 :: Word64 -> BS.ByteString
encodeWord64 x =
    BS.pack (7 : word64ToBytes x)

encodeFloat32 :: Float -> BS.ByteString
encodeFloat32 x =
    BS.pack (8 : word32ToBytes (unsafeCoerce x))

encodeFloat64 :: Double -> BS.ByteString
encodeFloat64 x =
    BS.pack (9 : word64ToBytes (unsafeCoerce x))

encodeBool :: Bool -> BS.ByteString
encodeBool b =
    BS.pack [10, if b then 1 else 0]

encodeChar :: Char -> BS.ByteString
encodeChar c =
    BS.pack (11 : word32ToBytes (fromIntegral (fromEnum c)))

encodeValue :: Value -> BS.ByteString
encodeValue val = case val of
    VInt8 x -> encodeInt8 x
    VInt16 x -> encodeInt16 x
    VInt32 x -> encodeInt32 x
    VInt64 x -> encodeInt64 x
    VWord8 x -> encodeWord8 x
    VWord16 x -> encodeWord16 x
    VWord32 x -> encodeWord32 x
    VWord64 x -> encodeWord64 x
    VFloat32 x -> encodeFloat32 x
    VFloat64 x -> encodeFloat64 x
    VBool b -> encodeBool b
    VChar c -> encodeChar c

word8ToBytes :: Word8 -> [Word8]
word8ToBytes w = [w]

word16ToBytes :: Word16 -> [Word8]
word16ToBytes w =
    [ fromIntegral (w .&. 0xff)
    , fromIntegral ((w `shiftR` 8) .&. 0xff)
    ]

word32ToBytes :: Word32 -> [Word8]
word32ToBytes w =
    [ fromIntegral (w .&. 0xff)
    , fromIntegral ((w `shiftR` 8) .&. 0xff)
    , fromIntegral ((w `shiftR` 16) .&. 0xff)
    , fromIntegral ((w `shiftR` 24) .&. 0xff)
    ]

word64ToBytes :: Word64 -> [Word8]
word64ToBytes w =
    [ fromIntegral ( w              .&. 0xff)
    , fromIntegral ((w `shiftR` 8)  .&. 0xff)
    , fromIntegral ((w `shiftR` 16) .&. 0xff)
    , fromIntegral ((w `shiftR` 24) .&. 0xff)
    , fromIntegral ((w `shiftR` 32) .&. 0xff)
    , fromIntegral ((w `shiftR` 40) .&. 0xff)
    , fromIntegral ((w `shiftR` 48) .&. 0xff)
    , fromIntegral ((w `shiftR` 56) .&. 0xff)
    ]

bytesToWord8 :: BS.ByteString -> Word8
bytesToWord8 bs =
    case BS.unpack bs of
        [b0] -> fromIntegral b0
        _ -> error "Expected exactly 1 byte"

bytesToWord16 :: BS.ByteString -> Word16
bytesToWord16 bs =
    case BS.unpack bs of
        [b0, b1] ->
              fromIntegral b0
            .|. (fromIntegral b1 `shiftL` 8)
        _ -> error "Expected exactly 2 bytes"

bytesToWord32 :: BS.ByteString -> Word32
bytesToWord32 bs =
    case BS.unpack bs of
        [b0, b1, b2, b3] ->
              fromIntegral b0
            .|. (fromIntegral b1 `shiftL` 8)
            .|. (fromIntegral b2 `shiftL` 16)
            .|. (fromIntegral b3 `shiftL` 24)
        _ -> error "Expected exactly 4 bytes"

bytesToWord64 :: BS.ByteString -> Word64
bytesToWord64 bs =
    case BS.unpack bs of
        [b0, b1, b2, b3, b4, b5, b6, b7] ->
              fromIntegral b0
            .|. (fromIntegral b1 `shiftL` 8)
            .|. (fromIntegral b2 `shiftL` 16)
            .|. (fromIntegral b3 `shiftL` 24)
            .|. (fromIntegral b4 `shiftL` 32)
            .|. (fromIntegral b5 `shiftL` 40)
            .|. (fromIntegral b6 `shiftL` 48)
            .|. (fromIntegral b7 `shiftL` 56)
        _ -> error "Expected exactly 8 bytes"