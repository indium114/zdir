pub fn parse_response(response: &str) -> Vec<(f64, String)> {
    response
        .split("**")
        .filter(|token| !token.is_empty())
        .filter_map(|token| {
            let (rank, path) = token.split_once(':')?;
            Some((rank.parse::<f64>().ok().unwrap(), path.to_string()))
        })
        .collect()
}
