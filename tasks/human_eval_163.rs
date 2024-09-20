/*
### ID
HumanEval/163
*/
/*
### VERUS BEGIN
*/
use vstd::prelude::*;

verus! {

fn min(a: u32, b: u32) -> (m: u32)
    ensures
        m == a || m == b,
        m <= a && m <= b,
{
    if a < b {
        a
    } else {
        b
    }
}

fn max(a: u32, b: u32) -> (m: u32)
    ensures
        m == a || m == b,
        m >= a && m >= b,
{
    if a > b {
        a
    } else {
        b
    }
}

#[verifier::loop_isolation(false)]
fn generate_integers(a: u32, b: u32) -> (result: Vec<u32>)
    ensures
        forall|i: int| 0 <= i < result.len() ==> result[i] % 2 == 0,
        forall|i: int|
            0 <= i < result.len() ==> (result[i] == 2 || result[i] == 4 || result[i] == 6
                || result[i] == 8),
        forall|i: int, j: int| 0 <= i < j < result.len() ==> result[i] <= result[j],
        forall|x: int|
            a <= x <= b && 2 <= x <= 8 && x % 2 == 0 ==> #[trigger] result@.contains(x as u32),
{
    let left = min(a, b);
    let right = max(a, b);

    if right < 2 || left > 8 {
        return vec![];
    }
    let lower = max(2, left);
    let upper = min(8, right);

    assert(2 <= lower);
    assert(upper <= 8);

    let mut result: Vec<u32> = vec![];
    let mut i = lower;
    while i <= upper
        invariant
            lower <= i <= upper + 1,
            forall|i: int| 0 <= i < result.len() ==> result[i] % 2 == 0,
            forall|i: int|
                0 <= i < result.len() ==> (result[i] == 2 || result[i] == 4 || result[i] == 6
                    || result[i] == 8),
            forall|j: int| 0 <= j < result.len() ==> result[j] <= i,
            forall|i: int, j: int| 0 <= i < j < result.len() ==> result[i] <= result[j],
    {
        if i % 2 == 0 {
            result.push(i);
        }
    }
    result
}

} // verus!
fn main() {}

/*
### VERUS END
*/

/*
### PROMPT

def generate_integers(a, b):
    """
    Given two positive integers a and b, return the even digits between a
    and b, in ascending order.

    For example:
    generate_integers(2, 8) => [2, 4, 6, 8]
    generate_integers(8, 2) => [2, 4, 6, 8]
    generate_integers(10, 14) => []
    """

*/

/*
### ENTRY POINT
generate_integers
*/

/*
### CANONICAL SOLUTION
    lower = max(2, min(a, b))
    upper = min(8, max(a, b))

    return [i for i in range(lower, upper+1) if i % 2 == 0]

*/

/*
### TEST
def check(candidate):

    # Check some simple cases
    assert candidate(2, 10) == [2, 4, 6, 8], "Test 1"
    assert candidate(10, 2) == [2, 4, 6, 8], "Test 2"
    assert candidate(132, 2) == [2, 4, 6, 8], "Test 3"
    assert candidate(17,89) == [], "Test 4"

    # Check some edge cases that are easy to work out by hand.
    assert True, "This prints if this assert fails 2 (also good for debugging!)"


*/
