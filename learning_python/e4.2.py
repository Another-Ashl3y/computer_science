import random

numbers = [random.randrange(1,11) for i in range(10000)]

#for i in numbers: 
#    print(i)
print(sum(numbers)/10000)



