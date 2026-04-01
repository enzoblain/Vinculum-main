module Codec where

import qualified Data.ByteString as BS
import qualified Data.ByteString.Char8 as C8
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
    | VString String
    | VBytes BS.ByteString
    | VOption (Maybe Value)
    | VVec [Value]

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

            | tag == 12 ->
                let (lenBytes, afterLen) = BS.splitAt 8 rest
                    strLen = fromIntegral (bytesToWord64 lenBytes)
                    (payload, remaining) = BS.splitAt strLen afterLen
                 in (VString (C8.unpack payload), remaining)

            | tag == 13 ->
                let (lenBytes, afterLen) = BS.splitAt 8 rest
                    rawLen = fromIntegral (bytesToWord64 lenBytes)
                    (payload, remaining) = BS.splitAt rawLen afterLen
                 in (VBytes payload, remaining)

            | tag == 14 ->
                case BS.uncons rest of
                    Nothing -> error "Incomplete Option encoding"
                    Just (optTag, remaining)
                        | optTag == 0 -> (VOption Nothing, remaining)
                        | optTag == 1 ->
                            let (value, finalRemaining) = decodeOne remaining
                             in (VOption (Just value), finalRemaining)
                        | otherwise -> error ("Invalid Option tag: " ++ show optTag)

            | tag == 15 ->
                let (lenBytes, afterLen) = BS.splitAt 8 rest
                    vecLen = fromIntegral (bytesToWord64 lenBytes)
                    (vecElements, vecRemaining) = decodeNValues vecLen afterLen
                 in (VVec vecElements, vecRemaining)

            | otherwise ->
                error ("Invalid type tag: " ++ show tag)

decodeNValues :: Int -> BS.ByteString -> ([Value], BS.ByteString)
decodeNValues 0 bs = ([], bs)
decodeNValues n bs
    | n > 0 =
        let (value, rest) = decodeOne bs
            (restValues, finalRemaining) = decodeNValues (n - 1) rest
         in (value : restValues, finalRemaining)
    | otherwise = error "Invalid vector length"

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

encodeString :: String -> BS.ByteString
encodeString s =
    let bytes = C8.pack s
        lenBytes = word64ToBytes (fromIntegral (BS.length bytes))
     in BS.concat [BS.pack [12], BS.pack lenBytes, bytes]

encodeBytes :: BS.ByteString -> BS.ByteString
encodeBytes b =
    let lenBytes = word64ToBytes (fromIntegral (BS.length b))
     in BS.concat [BS.pack [13], BS.pack lenBytes, b]

encodeOptionNothing :: BS.ByteString
encodeOptionNothing =
    BS.pack [14, 0]

encodeOptionJust :: BS.ByteString -> BS.ByteString
encodeOptionJust value =
    BS.concat [BS.pack [14, 1], value]

encodeOption :: Maybe Value -> BS.ByteString
encodeOption Nothing =
    encodeOptionNothing
encodeOption (Just value) =
    encodeOptionJust (encodeValue value)

encodeOptionWith :: (a -> Value) -> Maybe a -> BS.ByteString
encodeOptionWith _ Nothing = encodeOptionNothing
encodeOptionWith f (Just x) = encodeOptionJust (encodeValue (f x))

decodeOptionWith :: (Value -> a) -> Maybe Value -> Maybe a
decodeOptionWith _ Nothing = Nothing
decodeOptionWith f (Just v) = Just (f v)

encodeVec :: [Value] -> BS.ByteString
encodeVec values =
    let lenBytes = word64ToBytes (fromIntegral (length values))
        encodedValues = BS.concat (map encodeValue values)
     in BS.concat [BS.pack [15], BS.pack lenBytes, encodedValues]

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
    VString s -> encodeString s
    VBytes b -> encodeBytes b
    VOption opt -> encodeOption opt
    VVec v -> encodeVec v

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

fromValueInt8 :: Value -> Int8
fromValueInt8 (VInt8 x) = x
fromValueInt8 _ = error "Expected Int8"

fromValueInt16 :: Value -> Int16
fromValueInt16 (VInt16 x) = x
fromValueInt16 _ = error "Expected Int16"

fromValueInt32 :: Value -> Int32
fromValueInt32 (VInt32 x) = x
fromValueInt32 _ = error "Expected Int32"

fromValueInt64 :: Value -> Int64
fromValueInt64 (VInt64 x) = x
fromValueInt64 _ = error "Expected Int64"

fromValueWord8 :: Value -> Word8
fromValueWord8 (VWord8 x) = x
fromValueWord8 _ = error "Expected Word8"

fromValueWord16 :: Value -> Word16
fromValueWord16 (VWord16 x) = x
fromValueWord16 _ = error "Expected Word16"

fromValueWord32 :: Value -> Word32
fromValueWord32 (VWord32 x) = x
fromValueWord32 _ = error "Expected Word32"

fromValueWord64 :: Value -> Word64
fromValueWord64 (VWord64 x) = x
fromValueWord64 _ = error "Expected Word64"

fromValueFloat32 :: Value -> Float
fromValueFloat32 (VFloat32 x) = x
fromValueFloat32 _ = error "Expected Float32"

fromValueFloat64 :: Value -> Double
fromValueFloat64 (VFloat64 x) = x
fromValueFloat64 _ = error "Expected Float64"

fromValueBool :: Value -> Bool
fromValueBool (VBool x) = x
fromValueBool _ = error "Expected Bool"

fromValueChar :: Value -> Char
fromValueChar (VChar x) = x
fromValueChar _ = error "Expected Char"

fromValueString :: Value -> String
fromValueString (VString x) = x
fromValueString _ = error "Expected String"

fromValueBytes :: Value -> BS.ByteString
fromValueBytes (VBytes x) = x
fromValueBytes _ = error "Expected Bytes"