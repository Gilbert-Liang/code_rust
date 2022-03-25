#[derive(Debug)]
enum PokerCard {
    Clubs(u8),
    Spades(u8),
    Diamonds(char),
    Hearts(char),
}

fn main() {
    // let c1 = PokerCard {
    //     suit: PokerSuit::Clubs,
    //     value: 1,
    // };
    // let c2 = PokerCard {
    //     suit: PokerSuit::Diamonds,
    //     value: 12,
    // };
    let c1 = PokerCard::Spades(5);
    let c2 = PokerCard::Diamonds('A');

    println!("{:?}",c1);
    println!("{:?}",c2);
}