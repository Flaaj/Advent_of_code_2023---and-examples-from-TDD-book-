fn remove_nb(m: i32) -> Vec<(i32, i32)> {
    let m = m as i64;
    let sum = m * (m + 1) / 2;

    ((m / 2)..m)
        .filter(|a| (sum - a) % (a + 1) == 0)
        .map(|a| (a as i32, ((sum - a) / (a + 1)) as i32))
        .collect()
}

#[cfg(test)]
mod test {
    use crate::codewars::is_my_friend_cheating::remove_nb;

    fn testing(n: i32, exp: Vec<(i32, i32)>) -> () {
        assert_eq!(remove_nb(n), exp)
    }

    #[test]
    fn basics_remove_nb() {
        testing(26, vec![(15, 21), (21, 15)]);
        testing(100, vec![]);
        testing(101, vec![(55, 91), (91, 55)]);
        testing(102, vec![(70, 73), (73, 70)]);
        testing(110, vec![(70, 85), (85, 70)]);
        testing(
            1000003,
            vec![
                (550320, 908566),
                (559756, 893250),
                (893250, 559756),
                (908566, 550320),
            ],
        );
    }
}
