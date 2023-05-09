pa :: [Char]->[Char]
pa []=[]
pa (x:xs) = (pa xs) ++ [x]

pal :: [Char]->Bool
pal x=(x==(pa x))

main::IO()
main=do
    putStr "Enter thw word :"
    line<-getLine
    if (pal line) then putStrLn "\nIt is a Palindrome" else putStrLn "\nIt is not a Palindrome"
