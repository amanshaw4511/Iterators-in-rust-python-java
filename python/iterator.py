from functools import reduce

def Digits(num) :
    while num > 0 :
        yield num % 10
        num //= 10

if __name__ == '__main__' :
    # loop through for loop 
    for digit in Digits(14562) :
        print(digit, end=" ")
    print()

    # collect digits in list
    digit_list = list(Digits(14562))
    print("digit_list:", digit_list)

    # count digit, max and min of digits
    digit_count = reduce(lambda a,_ : a + 1, Digits(14562), 0)
    digit_max = max(Digits(14562))
    digit_min = min(Digits(14562))
    print("digit_count:", digit_count)
    print("digit_min:", digit_min)

    # sum and product of digits 
    digit_sum = sum(Digits(14562))
    digit_product = reduce(lambda a,x : a * x, Digits(14562), 1)
    print("digit_sum:", digit_sum)
    print("digit_product:", digit_product)

    # abs differnce between sum of even digit and sum of odd digit
    digit_even_sum = sum(filter(lambda e : e % 2 == 0, Digits(14562)))
    digit_odd_sum = sum(filter(lambda e : e % 2 != 0, Digits(14562)))
    diff = abs(digit_even_sum - digit_odd_sum)
    print("digit_abs_diff_even_odd:", diff)

    # number in sorted digit
    digit_sorted = reduce(lambda a,x : a * 10 + x ,sorted(Digits(14562)), 0)
    print("digit_sorted:", digit_sorted)
