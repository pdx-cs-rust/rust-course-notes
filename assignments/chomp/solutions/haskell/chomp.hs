-- Copyright © 2013 Bart Massey
-- [This program is licensed under the "MIT License"]
-- Please see the file COPYING in the source
-- distribution of this software for license terms.

-- Chomp player in Haskell

import Data.Array
import Data.List (foldl1')
import Text.Printf

data Move = Move { coords :: (Int, Int) }

newtype Board = Board { squares :: Array (Int, Int) Bool }

rows :: Int
rows = 3
cols :: Int
cols = 4

newBoard :: Board
newBoard = Board $
           listArray ((0, 0), (rows - 1, cols - 1)) $
           repeat True

instance Show Board where
  show b =
    unlines $ [ concat $ [ boardChar b (i, j) | j <- [0 .. cols - 1] ] |
                i <- [0 .. rows - 1] ]
    where
      boardChar b c
        | not (squares b ! c) = "."
        | c == (0, 0) = "*"
        | otherwise = "o"

instance Show Move where
  show move =
    show (adjust $ coords move)
    where
      adjust (r, c) = (r + 1, c + 1)

getMove :: IO Move
getMove =
  fmap (Move . adjust . read) getLine
  where
    adjust (r, c) = (r - 1, c - 1)

makeMove :: Board -> Move -> Board
makeMove b (Move (r0, c0)) =
  Board $
  squares b // [ ((r, c), False) |
                 r <- [r0 .. rows - 1],
                 c <- [c0 .. cols - 1] ]

negamax :: Board -> Either Move Move
negamax b =
  tryMoves $ map fst $ filter snd $ assocs $ squares b
  where
    tryMoves [] =
      Right undefined
    tryMoves ms =
      foldl1' firstRight $ map (Left . Move) ms
      where
        firstRight (Right m) _ =
          Right m
        firstRight _ (Left cmove) =
          case negamax $ makeMove b cmove of
            Left _ -> Right cmove
            Right _ -> Left cmove
        firstRight _ (Right _) =
          error "internal error: Right cmove"

type Action = Board -> IO Board

humanTurn :: Action
humanTurn b =
  fmap (makeMove b) getMove

computerTurn :: Action
computerTurn b = do
  case negamax b of
    Right m -> do
      _ <- printf "%s :-)\n" $ show m
      return $ makeMove b m
    Left m
      | coords m == (0, 0) -> do
        _ <- printf "%s :-(\n" $ show m
        return $ makeMove b m
      | otherwise -> do
        _ <- printf "%s :-P\n" $ show m
        return $ makeMove b m

playGame :: Board -> (Action, String) -> (Action, String) -> IO ()
playGame b p1@(a1, s1) p2 = do
  case (squares b) ! (0, 0) of
    True -> do
      putStr $ show b
      b' <- a1 b
      putStrLn ""
      playGame b' p2 p1
    False -> putStrLn s1

main :: IO ()
main = playGame newBoard (humanTurn, "I lose.") (computerTurn, "You lose.")
