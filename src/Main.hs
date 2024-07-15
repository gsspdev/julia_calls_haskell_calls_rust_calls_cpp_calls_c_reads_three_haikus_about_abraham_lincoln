{-# LANGUAGE ForeignFunctionInterface #-}

module Main where

-- Import the Rust function
foreign import ccall "callRustFunction" callRustFunction :: IO ()

-- Haskell function to call the Rust function
callRust :: IO ()
callRust = callRustFunction

-- Main function to be called from Julia
foreign export ccall callRust :: IO ()

main :: IO ()
main = callRust
