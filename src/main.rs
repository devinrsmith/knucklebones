use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Die {
    ONE = 1,
    TWO = 2,
    THREE = 3,
    FOUR = 4,
    FIVE = 5,
    SIX = 6,
}

impl Die {
    const VALUES: [Self; 6] = [
        Self::ONE,
        Self::TWO,
        Self::THREE,
        Self::FOUR,
        Self::FIVE,
        Self::SIX,
    ];
}

trait Hand {
    fn die_1(&self) -> Option<Die>;

    fn die_2(&self) -> Option<Die>;

    fn die_3(&self) -> Option<Die>;

    fn has(&self, die: Die) -> bool;

    fn is_full(&self) -> bool;

    fn len(&self) -> u8;

    fn score(&self) -> u8;
}

fn overlaps<T: Hand>(hand_1: &T, hand_2: &T) -> bool {
    (hand_1.has(Die::ONE) && hand_2.has(Die::ONE))
        || (hand_1.has(Die::TWO) && hand_2.has(Die::TWO))
        || (hand_1.has(Die::THREE) && hand_2.has(Die::THREE))
        || (hand_1.has(Die::FOUR) && hand_2.has(Die::FOUR))
        || (hand_1.has(Die::FIVE) && hand_2.has(Die::FIVE))
        || (hand_1.has(Die::SIX) && hand_2.has(Die::SIX))
}

#[derive(Debug, Copy, Clone)]
struct HandExplicit {
    die_1: Option<Die>,

    // todo: die_2 >= die_1
    die_2: Option<Die>,

    // todo: die_3 >= die_2
    die_3: Option<Die>,
}

impl HandExplicit {
    const fn empty() -> HandExplicit {
        HandExplicit {
            die_1: None,
            die_2: None,
            die_3: None,
        }
    }

    const fn new1(die_1: Die) -> HandExplicit {
        HandExplicit {
            die_1: Some(die_1),
            die_2: None,
            die_3: None,
        }
    }

    const fn new2(die_1: Die, die_2: Die) -> HandExplicit {
        // todo: validate die_1 <= die_2
        HandExplicit {
            die_1: Some(die_1),
            die_2: Some(die_2),
            die_3: None,
        }
    }

    const fn new(die_1: Die, die_2: Die, die_3: Die) -> HandExplicit {
        // todo: validate die_1 <= die_2 <= die_3
        HandExplicit {
            die_1: Some(die_1),
            die_2: Some(die_2),
            die_3: Some(die_3),
        }
    }

    fn val(die: Option<Die>) -> u8 {
        match die {
            None => 0,
            Some(d) => d as u8,
        }
    }

    fn val_1(&self) -> u8 {
        Self::val(self.die_1)
    }

    fn val_2(&self) -> u8 {
        Self::val(self.die_2)
    }

    fn val_3(&self) -> u8 {
        Self::val(self.die_3)
    }
}

impl Hand for HandExplicit {
    fn die_1(&self) -> Option<Die> {
        self.die_1
    }

    fn die_2(&self) -> Option<Die> {
        self.die_2
    }

    fn die_3(&self) -> Option<Die> {
        self.die_3
    }

    fn has(&self, die: Die) -> bool {
        self.die_1 == Some(die) || self.die_2 == Some(die) || self.die_3 == Some(die)
    }

    fn is_full(&self) -> bool {
        self.die_3.is_some()
    }

    fn len(&self) -> u8 {
        if self.die_3.is_some() {
            3
        } else if self.die_2.is_some() {
            2
        } else if self.die_1.is_some() {
            1
        } else {
            0
        }
    }

    fn score(&self) -> u8 {
        let abc = self.val_1() + self.val_2() + self.val_3();
        let ab = if self.die_1 == self.die_2 {
            self.val_1() * self.val_1()
        } else {
            0
        };
        let bc = if self.die_2 == self.die_3 {
            self.val_2() * self.val_2()
        } else {
            0
        };
        let ac = if self.die_1 == self.die_3 {
            self.val_1() * self.val_1()
        } else {
            0
        };
        abc + ab + bc + ac
    }
}

impl fmt::Display for HandExplicit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(")?;
        if self.die_1.is_some() {
            write!(f, "{}", self.die_1.unwrap())?;
        }
        if self.die_2.is_some() {
            write!(f, "{}", self.die_2.unwrap())?;
        }
        if self.die_3.is_some() {
            write!(f, "{}", self.die_3.unwrap())?;
        }
        write!(f, ")")
    }
}

impl fmt::Display for Die {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Die::ONE => write!(f, "1"),
            Die::TWO => write!(f, "2"),
            Die::THREE => write!(f, "3"),
            Die::FOUR => write!(f, "4"),
            Die::FIVE => write!(f, "5"),
            Die::SIX => write!(f, "6"),
        }
    }
}

fn main() {
    let hands = hands();
    println!("Hands: {}", hands.len());

    let hand_pairs = hand_pairs(hands);
    println!("Hand pairs: {}", hand_pairs.len());

    let state_counts = state_counts(hand_pairs);
    println!("Intermediate states: {}", state_counts.0);
    println!("Final states: {}", state_counts.1);
    println!("Total: {}", state_counts.0 + state_counts.1);
}

fn hands() -> Vec<HandExplicit> {
    let mut v = vec![];
    v.push(HandExplicit::empty());
    for (i1, d1) in Die::VALUES.iter().enumerate() {
        v.push(HandExplicit::new1(*d1));
        for (i2, d2) in Die::VALUES.iter().skip(i1).enumerate() {
            v.push(HandExplicit::new2(*d1, *d2));
            for d3 in Die::VALUES.iter().skip(i1 + i2) {
                v.push(HandExplicit::new(*d1, *d2, *d3));
            }
        }
    }
    v
}

fn hand_pairs(hands: Vec<HandExplicit>) -> Vec<(HandExplicit, HandExplicit)> {
    let mut v = vec![];
    for hand_1 in hands.iter() {
        for hand_2 in hands.iter() {
            if !overlaps(hand_1, hand_2) {
                v.push((*hand_1, *hand_2));
            }
        }
    }
    v
}

fn state_counts(hand_pairs: Vec<(HandExplicit, HandExplicit)>) -> (u64, u64) {
    let mut intermediate_states: u64 = 0;
    let mut final_states: u64 = 0;
    for (ix_1, column_1) in hand_pairs.iter().enumerate() {
        for (ix_2, column_2) in hand_pairs.iter().skip(ix_1).enumerate() {
            for column_3 in hand_pairs.iter().skip(ix_1 + ix_2) {
                let p1_full = column_1.0.is_full() && column_2.0.is_full() && column_3.0.is_full();
                let p2_full = column_1.1.is_full() && column_2.1.is_full() && column_3.1.is_full();
                if p1_full && p2_full {
                    // Invalid state, can't have both sides full
                    continue;
                }
                if p1_full ^ p2_full {
                    final_states += 1;
                } else {
                    intermediate_states += 1;
                }
            }
        }
    }
    (intermediate_states, final_states)
}
