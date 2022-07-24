module Main where

import Text.Read
import System.IO
import System.Environment

data Error = ParseError String
  | TypeError String
  | RuntimeError String
  deriving Show

type Ident = String

data Types = Str String | Num Int | Nil deriving Show

data Ops = Def Ident
  | Set Ident Int
  | Call Ident [Types]
  | Return
  deriving (Show)


-------------------------
-- HELPERS FOR STRINGS --
-------------------------

-- See if char is a quote, only supports double quotes
isQuote :: Char -> Bool
isQuote '"' = True -- ONLY SUPPORTING DOUBLE QUOTES
isQuote _ = False

-- Counts numbers of quotes in string
countQuote :: String -> Int
countQuote str = countQuoteHelper 0 str
  where
    countQuoteHelper :: Int -> String -> Int
    countQuoteHelper num "" = num
    countQuoteHelper num (x:xs) = if isQuote x then (countQuoteHelper (num+1) xs) else countQuoteHelper num xs

-- Returns first character, or Nothing if empty
getFirstChar :: String -> Maybe Char
getFirstChar "" = Nothing
getFirstChar (x:_) = Just x

-- Returns last character, or Nothing if empty
getLastChar :: String -> Maybe Char
getLastChar str = recToLast str
  where
    recToLast :: String -> Maybe Char
    recToLast "" = Nothing
    recToLast (x:xs) = case length (x:xs) of
      1 -> Just x
      _ -> recToLast xs

-- Returns True if an input string is properly a string:
-- i.e., double quotes around text: "Hello, World!", not 'Hello, World!'
isString :: String -> Bool
isString "" = False
isString str = case getFirstChar str of
  Nothing -> False
  Just c -> (isQuote c) && case getLastChar str of
    Nothing -> False
    Just c' -> isQuote c' && (countQuote str == 2)


-------------------------
-- HELPERS FOR PARSING --
-------------------------

-- https://stackoverflow.com/questions/4978578/how-to-split-a-string-in-haskell
-- Splits a string on a given condition
splitStringWhen :: (Char -> Bool) -> String -> [String]
splitStringWhen cond s = case dropWhile cond s of
                     "" -> []
                     s' -> w : splitStringWhen cond s''
                               where (w, s'') = break cond s'

cleanParams :: [String] -> [Types]
cleanParams [] = []
cleanParams (p:ps) = case (readMaybe p :: Maybe Int) of
  Just num -> (Num num) : cleanParams ps
  Nothing -> (Str (tail (init p))) : cleanParams ps

parseLine :: String -> Either Error Ops
parseLine str = parseHelper (splitStringWhen (==' ') str)
  where
    parseHelper :: [String] -> Either Error Ops
    parseHelper [] = Left $ ParseError "Error in parsing empty string"
    parseHelper (o:os) = case (head o == '{' && last o == '}') of
      True -> Right $ Def (tail (init o))
      False -> case (head o == '(' && last o == ')') of
        True -> case (readMaybe (last os) :: Maybe Int) of
          Just n -> Right $ Set (tail (init o)) n
          Nothing -> Left $ TypeError "Must be an integer"
        False -> case (head o == '[' && last o == ']') of
          True -> Right $ Call (tail (init o)) (cleanParams os)
          False -> Left $ ParseError $ "Unknown token: " ++ o

parseLines :: [String] -> Either Error [Ops]
parseLines [] = Right []
parseLines (o:os) = case parseLine o of
  Right o' -> case parseLines os of
    Right os' -> Right $ o':os'
    Left e -> Left e
  Left e' -> Left e'

-------------------------
-- EVALUATOR FUNCTIONS --
-------------------------
evaluateProgram :: [Ops] -> (IO ())
evaluateProgram ops = case runWithStack [] ops of
  Right types -> printResultTypeList types
  Left err -> print err
  where
    runWithStack :: [(String, Types)] -> [Ops] -> Either Error [Types]
    runWithStack _ [] = Right [Nil]
    runWithStack st (o:os) = case o of
      Def name -> runWithStack ((name, Nil):st) os
      Set name value -> case stackHasVar st name of
        False -> Left $ RuntimeError "Cannot set undefined variables"
        True -> runWithStack (replaceStackVariable st (name, Num value)) os
      Call name (p:ps:[]) -> case name of
        "add" -> case p of
          Str nm -> case (getStackVarValue st nm) of
            Nil -> Left $ ParseError $ "Unknown variable name " ++ nm
            Num i -> case runWithStack st os of
              Right ts -> Right $ (Num (i + (getIntFromNum ps))) : ts
              Left e -> Left e
          Num nm -> case ps of
            Num nm' -> case runWithStack st os of
              Right ts -> Right $ (Num (nm + nm')):ts
              Left e -> Left e
            Str str -> case (getStackVarValue st str) of
              Nil -> Left $ ParseError $ "Unknown variable name " ++ str
              Num i -> case runWithStack st os of
                Right ts -> Right $ (Num (nm + i)):ts
                Left e -> Left e
              _ -> Left $ TypeError "Must add only ints"
            _ -> Left $ TypeError "Must add only ints"
          _ -> Left $ TypeError "Must add only ints"
        "sub" -> case p of
          Str nm -> case (getStackVarValue st nm) of
            Nil -> Left $ ParseError $ "Unknown variable name " ++ nm
            Num i -> case runWithStack st os of
              Right ts -> Right $ (Num (i - (getIntFromNum ps))) : ts
              Left e -> Left e
          Num nm -> case ps of
            Num nm' -> case runWithStack st os of
              Right ts -> Right $ (Num (nm - nm')):ts
              Left e -> Left e
            Str str -> case (getStackVarValue st str) of
              Nil -> Left $ ParseError $ "Unknown variable name " ++ str
              Num i -> case runWithStack st os of
                Right ts -> Right $ (Num (nm - i)):ts
                Left e -> Left e
              _ -> Left $ TypeError "Must add only ints"
            _ -> Left $ TypeError "Must add only ints"
          _ -> Left $ TypeError "Must add only ints"
        _ -> Left $ ParseError "I only know add"
      _ -> Left $ ParseError "I can't do the rest of my ops"

    stackHasVar :: [(String, Types)] -> String -> Bool
    stackHasVar [] _ = False
    stackHasVar ((v,_):vs) var = v == var || stackHasVar vs var

printResultType :: [Types] -> String
printResultType [] = ""
printResultType (t:ts) = printtype ++ ('\n' : printtypes)
  where
    printtype = case t of
      Num n -> (show n)
      Str s -> s
      Nil -> ""
    printtypes = printResultType ts

printResultTypeList :: [Types] -> (IO ())
printResultTypeList ts = putStr $ printResultType ts

replaceStackVariable :: [(String, Types)] -> (String, Types) -> [(String, Types)]
replaceStackVariable [] _ = []
replaceStackVariable ((sname, st):ss) (name, t) = if sname == name then (name, t):ss else (sname,st):(replaceStackVariable ss (name, t))

getStackVarValue :: [(String, Types)] -> String -> Types
getStackVarValue [] _ = Nil
getStackVarValue ((sn, st):ss) name = if sn == name then st else getStackVarValue ss name

getIntFromNum :: Types -> Int
getIntFromNum (Num num) = num


-----------------
-- TEST INPUTS --
-----------------
testinput :: String
testinput = "{myvar}\n(myvar) 5\n[add] [myvar] 2"

testinput2 :: String
testinput2 = "{a}\n{b}\n{c}\n(a) 12\n(b) 3\n(c) 7\n[add] [c] -2\n[sub] [a] 3"

----------
-- MAIN --
----------
main :: IO ()
main =
  do
    args <- getArgs
    case length args of
      1 -> do
        file <- readFile (head args)
        let lns = splitStringWhen (=='\n') file
        let parsed = parseLines lns
        case parsed of
          Right ops -> evaluateProgram ops
          Left err -> print err
      _ -> print $ "Bad"
