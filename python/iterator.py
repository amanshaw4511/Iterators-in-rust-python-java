from functools import reduce

def digits(num) :
    while num > 0 :
        yield num % 10
        num //= 10
    # raise StopIteration



for digit in digits(352) :
    print(digit)

length = reduce(lambda a,_ : a+1, digits(3525), 0)
minimum = reduce(lambda minn,x : min(minn,x), digits(3525), 88)
print(sorted(digits(35146)))

print("length: ", length)
print("minimum: ", minimum)
