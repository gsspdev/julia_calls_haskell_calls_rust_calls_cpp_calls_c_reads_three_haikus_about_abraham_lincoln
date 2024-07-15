# Define the Haskell function
function callHaskellFunction()
    ccall((:callRust, "libhaskell"), Cvoid, ())
end

# Call the Haskell function
callHaskellFunction()
