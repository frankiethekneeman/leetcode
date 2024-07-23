use std::collections::HashMap;
/*
 *  From the leetcode: https://leetcode.com/problems/two-sum/description/
 *  I did not get to choose the signature here.  I believe it should return (usize, usize)...
 *  or even Option<(usize, usize)>.
 */
pub fn two_sum(nums: Vec<i32>, target: i32) -> Option<(usize, usize)>{
    /*
     *  Per the Constraints of the problem, there will only ever be exactly one solution.
     *  incidentally, this means that either there will only be exactly _two_ of a number, or
     *  that the number, doubled, does not equal the target.  In either case, I only then need
     *  a single index for a given value, as during sum finding, it either won't matter, or
     *  whichever one I _don't_ store will just be found pointing to the one I _did store.
     */
    let lookup: HashMap<i32, usize> = nums.iter().copied().enumerate().map(swap).collect();

    let (idx, first) = nums
        .into_iter()
        .enumerate()
        .find(|(i, x)| lookup.get(&(target - x)).map_or(false, |j| j != i))?;

    lookup.get(&(target - first))
        .map(|i| (idx, *i))
}

// I cannot believe this doesn't exist already
fn swap<L, R>((l, r): (L, R)) -> (R, L) {
    (r, l)
}

#[cfg(test)]
mod test_two_sum {
    use super::*;

    fn case(nums: Vec<i32>, target: i32, expected: Option<(usize, usize)>) {
        let actual = two_sum(nums, target);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_case_1() {
        case(vec![2, 7, 11, 15], 9, Some((0, 1)));
    }

    #[test]
    fn test_case_2() {
        case(vec![3, 2, 4], 6, Some((1, 2)))
    }

    #[test]
    fn test_case_3() {
        case(vec![3, 3], 6, Some((0, 1)))
    }
}
