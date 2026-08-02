use rand::prelude::SliceRandom;

/// Semi delle carte. Nel gioco di Scoundrel:
/// - ♠ Spade e ♣ Fiori (neri) sono i MOSTRI (valore 2-14, Ace = 14)
/// - ♦ Quadri sono le ARMI (valore 2-10)
/// - ♥ Cuori sono le POZIONI (valore 2-10)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Suit {
    Spades,
    Clubs,
    Diamonds,
    Hearts,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Card {
    pub suit: Suit,
    pub value: u8,
}

impl Card {
    pub fn new(suit: Suit, value: u8) -> Card {
        Card { suit, value }
    }

    pub fn is_monster(&self) -> bool {
        matches!(self.suit, Suit::Spades | Suit::Clubs)
    }

    pub fn is_weapon(&self) -> bool {
        self.suit == Suit::Diamonds
    }

    pub fn is_potion(&self) -> bool {
        self.suit == Suit::Hearts
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Weapon {
    pub value: u8,
    /// Valore dell'ultimo mostro ucciso con quest'arma: da quel momento
    /// l'arma è "legata" e può combattere solo mostri di forza <= a questo.
    pub last_monster_value: Option<u8>,
}

impl Weapon {
    pub fn new(value: u8) -> Self {
        Self {
            value,
            last_monster_value: None,
        }
    }
}

pub const MAX_HP: i8 = 20;
pub const ROOM_SIZE: usize = 4;

#[derive(Clone)]
pub struct GameState {
    pub hp: i8,
    pub turn: u32,
    pub weapon: Option<Weapon>,
    pub dungeon_deck: Vec<Card>,
    pub current_room: [Option<Card>; ROOM_SIZE],
    pub last_action_was_avoid: bool,
    pub have_healed: bool,
}

impl GameState {
    pub fn new() -> Self {
        let mut state = Self {
            hp: MAX_HP,
            turn: 1,
            weapon: None,
            dungeon_deck: Self::spawn_deck(),
            current_room: [None; ROOM_SIZE],
            last_action_was_avoid: false,
            have_healed: false,
        };
        state.draw_room();
        state
    }

    /// Partita persa: punti vita a zero o sotto.
    pub fn is_dead(&self) -> bool {
        self.hp <= 0
    }

    /// Partita vinta: mazzo esaurito e stanza completamente svuotata.
    pub fn has_won(&self) -> bool {
        self.dungeon_deck.is_empty() && self.current_room.iter().all(|c| c.is_none())
    }

    /// Equipaggia un'arma (♦) o beve una pozione (♥).
    /// Una sola pozione per stanza; la vita non può superare MAX_HP.
    pub fn equip(&mut self, card: Card) -> Result<(), String> {
        if card.is_weapon() {
            self.weapon = Some(Weapon::new(card.value));
        } else if card.is_potion() {
            if self.have_healed {
                return Err("You can only drink one potion per room".to_string());
            }
            self.hp = (self.hp + card.value as i8).min(MAX_HP);
            self.have_healed = true;
        } else {
            return Err("The selected card is not a weapon".to_string());
        }

        self.remove_card(card);
        Ok(())
    }

    /// Fuga: le 4 carte visibili tornano nel mazzo (rimischiato) e
    /// viene pescata una nuova stanza. Non si può fuggire due volte di fila.
    pub fn flee(&mut self) -> Result<(), String> {
        if self.last_action_was_avoid {
            return Err("You cannot avoid two rooms in a row".to_string());
        }

        for slot in self.current_room.iter_mut() {
            if let Some(card) = slot.take() {
                self.dungeon_deck.push(card);
            }
        }
        self.dungeon_deck.shuffle(&mut rand::rng());
        self.draw_room();

        self.last_action_was_avoid = true;
        self.have_healed = false;
        Ok(())
    }

    /// Attacca un mostro (♠♣). Senza arma subisci il danno pieno;
    /// con un'arma il danno è ridotto del valore dell'arma (minimo 0) e
    /// l'arma si lega all'ultimo mostro ucciso (weapon binding).
    pub fn attack(&mut self, card: Card) -> Result<(), String> {
        if !card.is_monster() {
            return Err("The selected card is not a monster".to_string());
        }

        let damage = match self.weapon {
            Some(weapon) => {
                if let Some(last) = weapon.last_monster_value {
                    if card.value > last {
                        return Err(
                            "Your weapon is bound: it can only fight monsters of equal or lower \
                             strength than the last one it defeated"
                                .to_string(),
                        );
                    }
                }
                card.value.saturating_sub(weapon.value)
            }
            None => card.value,
        };

        self.hp -= damage as i8;

        if let Some(weapon) = self.weapon.as_mut() {
            weapon.last_monster_value = Some(card.value);
        }

        self.remove_card(card);
        Ok(())
    }

    /// Nuovo turno: si può iniziare solo quando nella stanza è rimasta
    /// una sola carta; pesca 3 nuove carte per riportarla a 4.
    pub fn new_turn(&mut self) -> Result<(), String> {
        let empty = self.current_room.iter().filter(|c| c.is_none()).count();
        if empty != ROOM_SIZE - 1 {
            return Err(
                "You can start a new turn only when a single card is left in the room".to_string(),
            );
        }

        for slot in self.current_room.iter_mut() {
            if slot.is_none() {
                *slot = self.dungeon_deck.pop();
            }
        }

        self.turn += 1;
        self.last_action_was_avoid = false;
        self.have_healed = false;
        Ok(())
    }

    /// Pesa una stanza completa di 4 carte.
    fn draw_room(&mut self) {
        for slot in self.current_room.iter_mut() {
            *slot = self.dungeon_deck.pop();
        }
    }

    /// Mazzo di 44 carte: nere 2-14 (figure e assi inclusi), rosse 2-10
    /// (le figure rosse e gli assi rossi vengono rimossi dalle regole).
    fn spawn_deck() -> Vec<Card> {
        let mut result = Vec::with_capacity(44);

        // Mostri: 13 valori (2..=14) x 2 semi
        for value in 2..=14 {
            result.push(Card::new(Suit::Clubs, value));
            result.push(Card::new(Suit::Spades, value));
        }
        // Armi e pozioni: 9 valori (2..=10)
        for value in 2..=10 {
            result.push(Card::new(Suit::Diamonds, value));
            result.push(Card::new(Suit::Hearts, value));
        }

        let mut rng = rand::rng();
        result.shuffle(&mut rng);
        result
    }

    fn remove_card(&mut self, card: Card) {
        for slot in self.current_room.iter_mut() {
            if *slot == Some(card) {
                *slot = None;
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deck_is_44_cards_and_room_is_dealt() {
        let state = GameState::new();
        assert_eq!(state.dungeon_deck.len(), 40); // 44 - 4 pescate
        assert_eq!(
            state.current_room.iter().filter(|c| c.is_some()).count(),
            4
        );
        assert_eq!(state.hp, MAX_HP);
    }

    #[test]
    fn barehanded_attack_deals_full_damage() {
        let mut state = GameState::new();
        state.current_room = [Some(Card::new(Suit::Spades, 7)), None, None, None];
        state.attack(Card::new(Suit::Spades, 7)).unwrap();
        assert_eq!(state.hp, MAX_HP - 7);
        assert!(state.current_room[0].is_none());
    }

    #[test]
    fn weapon_reduces_damage() {
        let mut state = GameState::new();
        state.weapon = Some(Weapon::new(4));
        state.current_room = [Some(Card::new(Suit::Clubs, 12)), None, None, None];
        state.attack(Card::new(Suit::Clubs, 12)).unwrap();
        assert_eq!(state.hp, MAX_HP - 8); // 12 - 4
    }

    #[test]
    fn weapon_binds_to_last_monster() {
        let mut state = GameState::new();
        state.weapon = Some(Weapon::new(10));
        state.current_room = [Some(Card::new(Suit::Clubs, 13)), None, None, None];
        state.attack(Card::new(Suit::Clubs, 13)).unwrap();
        assert_eq!(state.weapon.unwrap().last_monster_value, Some(13));

        // Legata a 13: puoi ancora attaccare un 10...
        state.current_room = [Some(Card::new(Suit::Spades, 10)), None, None, None];
        state.attack(Card::new(Suit::Spades, 10)).unwrap();
        // ...ma un Ace (14) no.
        state.current_room = [Some(Card::new(Suit::Spades, 14)), None, None, None];
        assert!(state.attack(Card::new(Suit::Spades, 14)).is_err());
    }

    #[test]
    fn cannot_attack_red_cards() {
        let mut state = GameState::new();
        state.current_room = [Some(Card::new(Suit::Hearts, 5)), None, None, None];
        assert!(state.attack(Card::new(Suit::Hearts, 5)).is_err());
    }

    #[test]
    fn potion_heals_and_caps_at_max_hp() {
        let mut state = GameState::new();
        state.hp = 10;
        state.current_room = [Some(Card::new(Suit::Hearts, 8)), None, None, None];
        state.equip(Card::new(Suit::Hearts, 8)).unwrap();
        assert_eq!(state.hp, 18);

        // nuova stanza: il limite "una pozione per stanza" si azzera
        state.have_healed = false;
        state.hp = 19;
        state.current_room = [Some(Card::new(Suit::Hearts, 8)), None, None, None];
        state.equip(Card::new(Suit::Hearts, 8)).unwrap();
        assert_eq!(state.hp, MAX_HP);
    }

    #[test]
    fn only_one_potion_per_room() {
        let mut state = GameState::new();
        state.have_healed = true;
        state.current_room = [Some(Card::new(Suit::Hearts, 4)), None, None, None];
        assert!(state.equip(Card::new(Suit::Hearts, 4)).is_err());
    }

    #[test]
    fn flee_shuffles_cards_back_and_forbids_double_avoid() {
        let mut state = GameState::new();
        let deck_before = state.dungeon_deck.len();
        state.flee().unwrap();
        // le 4 carte sono tornate nel mazzo e ne sono state pescate 4 nuove
        assert_eq!(state.dungeon_deck.len(), deck_before);
        assert_eq!(
            state.current_room.iter().filter(|c| c.is_some()).count(),
            4
        );
        assert!(state.last_action_was_avoid);
        assert!(state.flee().is_err());
    }

    #[test]
    fn flee_stays_blocked_until_room_completed() {
        let mut state = GameState::new();
        state.flee().unwrap();

        // giocare una carta (equip/attack) NON sblocca la fuga
        state.current_room[0] = Some(Card::new(Suit::Diamonds, 5));
        state.equip(Card::new(Suit::Diamonds, 5)).unwrap();
        assert!(
            state.flee().is_err(),
            "non si può fuggire due stanze di fila, anche dopo aver giocato una carta"
        );

        // completare la stanza (new turn) sblocca la fuga
        state.current_room = [Some(Card::new(Suit::Spades, 3)), None, None, None];
        state.new_turn().unwrap();
        assert!(state.flee().is_ok());
    }

    #[test]
    fn new_turn_requires_single_card_left() {
        let mut state = GameState::new();
        assert!(state.new_turn().is_err()); // stanza piena

        state.current_room = [Some(Card::new(Suit::Spades, 3)), None, None, None];
        let deck_before = state.dungeon_deck.len();
        state.new_turn().unwrap();
        assert_eq!(
            state.current_room.iter().filter(|c| c.is_some()).count(),
            4
        );
        assert_eq!(state.dungeon_deck.len(), deck_before - 3);
        assert_eq!(state.turn, 2);
    }

    #[test]
    fn win_and_loss_conditions() {
        let mut won = GameState::new();
        won.dungeon_deck.clear();
        won.current_room = [None; ROOM_SIZE];
        assert!(won.has_won());
        assert!(!won.is_dead());

        let mut lost = GameState::new();
        lost.hp = 0;
        assert!(lost.is_dead());
        assert!(!lost.has_won());
    }
}
