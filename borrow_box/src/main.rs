use borrow_box::*;

fn main() {
    let mut game = GameSession::new(0, String::from("Joao"), String::from("Susana"), 5);
    println!("{:?}", game.read_winner());

    game.update_score("Joao");
    game.update_score("Joao");
    game.update_score("Susana");
    game.update_score("Susana");
    println!("{:?}", game.read_winner());

    game.update_score("Joao");
    // This one will not count because it already 5 games played, the `nb_games`
    game.update_score("Susana");

    println!("{:?}", game.read_winner());

    println!("{:?}", game.delete());

    // game.read_winner();
    // This will give an error as the game was dropped with `delete` and no longer exists
}

// use borrow_box::*;

#[inline]
fn games() -> [GameSession; 3] {
    [
        GameSession::new(0, "player1".to_owned(), "player2".to_owned(), 1),
        GameSession::new(1, "Alice".to_owned(), "Mark".to_owned(), 3),
        GameSession::new(2, "Jack".to_owned(), "Miller".to_owned(), 5),
    ]
}

#[test]
fn test_update_and_read() {
    let mut games = games();
    games[0].update_score("player1");
    assert_eq!(games[0].read_winner(), Some(&("player1".to_owned(), 1)));

    games[0].update_score("player2");
    // this will stay the same because the nb_games is 1 so if one
    // of the players wins just once it will no longer increment the score
    assert_eq!(games[0].read_winner(), Some(&("player1".to_owned(), 1)));

    games[2].update_score("Jack");
    games[2].update_score("Jack");
    games[2].update_score("Miller");
    games[2].update_score("Miller");
    assert_eq!(games[2].read_winner(), None);

    games[2].update_score("Jack");
    assert_eq!(games[2].read_winner(), Some(&("Jack".to_owned(), 3)));
}

#[test]
fn test_stop_updating() {
    let mut games = games();
    games[0].update_score("player1");
    games[0].update_score("player1");
    assert_eq!(games[0].read_winner(), Some(&("player1".to_owned(), 1)));

    games[2].update_score("Jack");
    games[2].update_score("Jack");
    games[2].update_score("Jack");
    games[2].update_score("Jack");
    games[2].update_score("Jack");
    assert_eq!(games[2].read_winner(), Some(&("Jack".to_owned(), 3)));
}

#[test]
fn test_delete() {
    let game = GameSession::new(0, "Alice".to_owned(), "Mark".to_owned(), 3);
    let game1 = GameSession::new(23, "Jack".to_owned(), "Miller".to_owned(), 1);

    assert_eq!(game.delete(), String::from("game deleted: id -> 0"));
    assert_eq!(game1.delete(), String::from("game deleted: id -> 23"));
}

#[test]
fn test_different_name() {
    let mut game = GameSession::new(0, "Alice".to_owned(), "Mark".to_owned(), 3);

    game.update_score("Mark");
    assert_eq!(game.read_winner(), Some(&("Mark".to_owned(), 1)));

    game.update_score("Miller");
    assert_eq!(game.read_winner(), Some(&("Mark".to_owned(), 1)));
}
