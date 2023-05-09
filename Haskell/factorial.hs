fact::Int->Int
fact 1 = 1
fact x = x * fact(x-1)

main::IO()
main=do
    putStr "Enter the number :"
    input<-getLine
    let n = read input :: Int
    putStr "\nFactorial = "
    print(fat n)
