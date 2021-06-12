function* Digits(num) {
    while (num > 0) {
        yield num % 10;
        num = Math.floor(num / 10);
    }
}

function main() {
    // loop through for loop
    for (const x of Digits(14562)) {
        process.stdout.write(x + " ");
    }
    console.log();

    // collect digit in list
    digit_list = [...Digits(14562)]
    console.log("digit_list:", digit_list);

    // count digit, max and min of digits
    digit_count = [...Digits(14562)].length;
    digit_max = Math.max(...Digits(14562));
    digit_min = Math.min(...Digits(14562));
    console.log("digit_count:", digit_count);
    console.log("digit_max:", digit_max);
    console.log("digit_min:", digit_min);

    // sum and product of digits 
    digit_sum = [...Digits(14562)].reduce((a,b) => a + b);
    digit_product = [...Digits(14562)].reduce((a,b) => a * b);
    console.log("digit_sum:", digit_sum)
    console.log("digit_product:", digit_product)

    // abs differnce between sum of even digit and sum of odd digit
    digit_even_sum = [...Digits(14562)]
        .filter(x => x % 2 == 0).
        reduce((a,x) => a + x);
    digit_odd_sum = [...Digits(14562)]
        .filter(x => x % 2 != 0)
        .reduce((a,x) => a + x);
    diff = Math.abs(digit_even_sum - digit_odd_sum)
    console.log("digit_abs_diff_even_odd:", diff)

    // number in sorted digit
    digit_sorted = [...Digits(14562)]
        .sort()
        .reduce((a,x) => a * 10 + x);
    console.log("digit_sorted:", digit_sorted)
}

main()
