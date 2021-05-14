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


fn main() {
    println!("Hello, world!");
    let digits = Digits::new(14235);
    
    // let mut v: Vec<_> = digits.into_iter().collect();
    // v.sort_by(|a,b| b.cmp(a));
    // v.iter().for_each(|e| print!("{}",e));
    for digit in &digits {
        println!("{}", digit);
    }
    for digit in digits {
        print!("{}", digit);
    }
    println!(".................");
    // println!("{}", digits.into_iter().max().unwrap());
    println!("{}", Digits::new(24111).into_iter().fold(0, |a,x| a + x));

}
