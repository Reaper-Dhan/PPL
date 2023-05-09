main::IO()
main = do
    putStr "Enter the word : "
    s <- getLine
    putStr "1st character : "
    print(take 1 s)
