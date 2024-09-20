/*
### ID
HumanEval/150
*/
/*
### VERUS BEGIN
*/
use vstd::prelude::*;

verus! {

spec fn spec_prime_helper(num: int, limit: int) -> bool {
    forall|j: int| 2 <= j < limit ==> (#[trigger] (num % j)) != 0
}

spec fn spec_prime(num: int) -> bool {
    num >= 2 && spec_prime_helper(num, num)
}

fn is_prime(num: u32) -> (result: bool)
    requires
        num >= 2,
    ensures
        result <==> spec_prime(num as int),
{
    let mut i = 2;
    let mut result = true;
    while i < num
        invariant
            2 <= i <= num,
            result <==> spec_prime_helper(num as int, i as int),
    {
        if num % i == 0 {
            result = false;
        }
        i += 1;
    }
    result
}

fn x_or_y(n: u32, x: i32, y: i32) -> (result: i32)
    ensures
        spec_prime(n as int) ==> result == x,
        !spec_prime(n as int) ==> result == y,
{
    if n >= 2 && is_prime(n) {
        x
    } else {
        y
    }
}

} // verus!
fn main() {}

/*
### VERUS END
*/

/*
### PROMPT

def x_or_y(n, x, y):
    """A simple program which should return the value of x if n is
    a prime number and should return the value of y otherwise.

    Examples:
    for x_or_y(7, 34, 12) == 34
    for x_or_y(15, 8, 5) == 5

    """

*/

/*
### ENTRY POINT
x_or_y
*/

/*
### CANONICAL SOLUTION
    if n == 1:
        return y
    for i in range(2, n):
        if n % i == 0:
            return y
            break
    else:
        return x

*/

/*
### TEST
def check(candidate):

    # Check some simple cases
    assert candidate(7, 34, 12) == 34
    assert candidate(15, 8, 5) == 5
    assert candidate(3, 33, 5212) == 33
    assert candidate(1259, 3, 52) == 3
    assert candidate(7919, -1, 12) == -1
    assert candidate(3609, 1245, 583) == 583
    assert candidate(91, 56, 129) == 129
    assert candidate(6, 34, 1234) == 1234


    # Check some edge cases that are easy to work out by hand.
    assert candidate(1, 2, 0) == 0
    assert candidate(2, 2, 0) == 2


*/
