f :: Int->Int
f 1 = 0
f 2 = 1
f n = f(n-1)+f(n-2)

main::IO()
main=do
    putStrLn "Enter the number :"
    input<-getLine
    let n = read input :: Int
    putStr "\nFibanocii Number :"
    print(f n)
