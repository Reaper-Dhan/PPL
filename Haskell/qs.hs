qs::[Int]->[Int]
qs[]=[]
qs(x:xs)= 
 let sm=[a|a<-xs,a<=x]      
 bi=[a|a<-xs,a>x] in qs sm++[x]++qs bi 

main::IO()
main=do  
putStrLn "Enter the element seperated by space"  
input<-getLine  
let nums= map read $ words input :: [Int]  
putStrLn $ "sorted list:" ++ show (qs nums)
