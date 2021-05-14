
macro_rules! dprint {
    ($input:expr) => {
        println!("{} = {:?}", stringify!($input), $input);
    };
}

fn main() {
    println!("Hello, world!");
    // loop through for loop 
    for digit in Digits::new(14562) {
        print!("{} ", digit);
    }
    println!();

    // collect digits in: list
    let digit_list: Vec<i32> = Digits::new(14562).into_iter().collect();
    dprint!(digit_list);

    // count digit, max and min of digits
    let digit_count = Digits::new(14562).into_iter().count();
    let digit_max = Digits::new(14562).into_iter().max().unwrap();
    let digit_min = Digits::new(14562).into_iter().min().unwrap();
    dprint!(digit_count);
    dprint!(digit_max);
    dprint!(digit_min);

    // sum and product of digits 
    let digit_sum: i32 = Digits::new(14562).into_iter().sum();
    let digit_product: i32 = Digits::new(14562).into_iter().product();
    dprint!(digit_sum);
    dprint!(digit_product);

    // abs differnce between sum of even digit and sum of odd digit
    let digit_even_sum: i32 = Digits::new(14562).into_iter()
                            .filter(|e| e % 2 == 0)
                            .sum();
    let digit_odd_sum: i32 = Digits::new(14562).into_iter()
                            .filter(|e| e % 2 != 0)
                            .sum();
    let diff = i32::abs(digit_even_sum - digit_odd_sum);
    dprint!(diff);

    // number in sorted digit
    let mut digit_sorted = Digits::new(14562).into_iter()
                            .collect::<Vec<_>>();
    digit_sorted.sort();
    let digit_sorted = digit_sorted.into_iter().fold(0, |a,x| a * 10 + x);
    dprint!(digit_sorted);
    

}


struct Digits {
    num : i32,
}

impl Digits {
    fn new(num: i32) -> Self {
        Digits {num}
    }
}

struct DigitIter {
    num : i32,
}

impl Iterator for DigitIter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.num == 0 {
            return None;
        } 
        let digit = self.num % 10;
        self.num /= 10;
        Some(digit)
    }
}

impl IntoIterator for Digits {
    type Item = i32;

    type IntoIter = DigitIter;

    fn into_iter(self) -> Self::IntoIter {
        DigitIter {
            num : self.num,
        }
    }
}

impl IntoIterator for &Digits {
    type Item = i32;

    type IntoIter = DigitIter;

    fn into_iter(self) -> Self::IntoIter {
        DigitIter {
            num : self.num,
        }
    }
}


