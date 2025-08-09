pub fn edit_distance(source: &str, target: &str) -> usize {
    let s = source.len();
    let t = target.len();

    let mut dp = vec![vec![0; t + 1]; s + 1];

    for i in 1..=s {
        dp[i][0] = i as i32;
    }
    for j in 1..=t {
        dp[0][j] = j as i32;
    }

    for i in 1..=s {
        for j in 1..=t {
            if source.chars().nth(i - 1) == target.chars().nth(j - 1) {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = 1 + dp[i][j - 1].min(dp[i - 1][j]).min(dp[i - 1][j - 1]);
            }
        }
    }

    dp[s][t] as usize
}
