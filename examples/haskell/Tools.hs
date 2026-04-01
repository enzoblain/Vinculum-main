module Tools where

import Data.Int (Int64)

concatStrings :: String -> String -> String
concatStrings s1 s2 = s1 ++ s2

addIfJust :: Maybe Int64 -> Int64 -> Int64
addIfJust Nothing x = x
addIfJust (Just n) x = n + x

tryGetString :: Int64 -> Maybe String
tryGetString n
    | n == 0 = Nothing
    | n > 0 = Just "positive"
    | otherwise = Just "negative"

addToAll :: Int64 -> [Int64] -> [Int64]
addToAll n xs = map (+ n) xs