fn main() {
    println!("{}", is_palindrome(0));
}

pub fn is_palindrome(x: i32) -> bool {
    let mut reversed: i32 = 0;
    let mut original: i32 = x;
    if x < 0 {
        return false;
    }

    if x % 10 == 0 {
        return false;
    }

    if x == 0 {
        return true;
    }

    while x > reversed {
        let digit = original % 10;
        reversed = reversed * 10 + digit;
        println!("reversed {}", reversed);
        original = original / 10;
        println!("original {}", original)
    }

    if x == reversed {
        return true;
    }

    false
}
