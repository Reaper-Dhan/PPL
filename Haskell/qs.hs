qs::[Int]->[Int]
qs[] = []
qs(x:xs) = qs smaller ++ [x] ++ qs larger
 where smaller = [a|a<-xs,a<=x]
       larger = [b|b<-xs,b>x]
