fs :: Int->[Int]
fs 0 = []
fs n = fs(n-1)++[fs n]

main::IO()
main=do
    putStr "Enter the number :"
    input<-getLine
    let n = read input :: Int
    putStr "\nFibanocii Series :"
    print(fs n)
