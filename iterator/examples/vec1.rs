fn main() {
    let player_scores = [
        ("Jack".to_string(), 20), ("Jane".to_string(), 23), ("Jill".to_string(), 18), ("John".to_string(), 19),
    ];

    let players = player_scores
        .iter()
        .map(|(player, _score)| {
            player
        })
        .collect::<Vec<_>>();

    assert_eq!(players, ["Jack", "Jane", "Jill", "John"]);
}
