#[derive(Debug)]
enum CardSuit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

#[derive(Debug)]
struct Card {
    rank: String,
    suit: CardSuit,
}

pub fn index(show: bool) {
    if show {
        let first_card = CardSuit::Hearts;
        println!("first_card: {:?}", first_card);

        let mut second_card = CardSuit::Diamonds;
        println!("second_card: {:?}", second_card);

        second_card = CardSuit::Spades;
        println!("second_card: {:?}", second_card);

        second_card = CardSuit::Clubs;
        println!("second_card: {:?}", second_card);

        let card = Card {
            rank: "Hearts".to_string(),
            suit: CardSuit::Hearts,
        };
        println!("card value: {:?}", card);
        println!("card rank: {:?}", card.rank);
        println!("card suit: {:?}", card.suit);
    }
}
