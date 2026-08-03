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

        // Se non restano mostri né carte utilizzabili (armi, o pozioni se
        // non ne hai già bevuta una), la stanza è esaurita: avanzamento
        // automatico. Questo copre anche le stanze senza mostri.
        //if !self.has_monsters() && !self.has_usable_cards() {
        //    self.maybe_advance();
        //}
        self.maybe_advance();
        self.have_healed = true;


        Ok(())
    }

    /// C'è almeno un mostro nella stanza?
    fn has_monsters(&self) -> bool {
        self.current_room
            .iter()
            .any(|c| c.is_some_and(|c| c.is_monster()))
    }

    /// Resta qualche carta utilizzabile? Un'arma è sempre equipaggiabile;
    /// una pozione solo se non ne hai già bevuta una nella stanza.
    fn has_usable_cards(&self) -> bool {
        self.current_room.iter().any(|c| {
            c.is_some_and(|c| c.is_weapon() || (c.is_potion() && !self.have_healed))
        })
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

    /// Attacca un mostro (♠♣).
    ///
    /// - `barehanded == false`: usa l'arma equipaggiata (se presente), applicando
    ///   il weapon binding; senza arma subisci il danno pieno.
    /// - `barehanded == true`: combatti a mani nude anche con un'arma equipaggiata
    ///   (danno pieno, l'arma resta e il suo legame non cambia). È l'opzione
    ///   ufficiale per fronteggiare mostri più forti dell'ultimo sconfitto.
    ///
    /// Dopo il colpo la stanza può avanzare automaticamente (vedi `maybe_advance`).
    pub fn attack(&mut self, card: Card, barehanded: bool) -> Result<(), String> {
        if !card.is_monster() {
            return Err("The selected card is not a monster".to_string());
        }

        let damage = if barehanded {
            card.value
        } else {
            match self.weapon {
                Some(weapon) => {
                    if let Some(last) = weapon.last_monster_value {
                        if card.value > last {
                            return Err(
                                "Your weapon is bound: it can only fight monsters of equal or lower \
                                 strength than the last one it defeated. Fight barehanded instead."
                                    .to_string(),
                            );
                        }
                    }
                    card.value.saturating_sub(weapon.value)
                }
                None => card.value,
            }
        };

        self.hp -= damage as i8;

        if !barehanded {
            if let Some(weapon) = self.weapon.as_mut() {
                weapon.last_monster_value = Some(card.value);
            }
        }

        self.remove_card(card);
        self.maybe_advance();
        self.have_healed = false;

        Ok(())
    }

    /// Avanza automaticamente la stanza dopo un'azione che rimuove una carta:
    ///
    /// 1. **Completamento**: se nella stanza non è rimasto alcun mostro, le carte
    ///    residue (pozioso/armi inutilizzate) vengono scartate e si pesca una
    ///    nuova stanza completa. Elimina il deadlock di una stanza composta solo
    ///    da pozioni e un mostro.
    /// 2. **Nuovo turno**: altrimenti, quando è rimasta una sola carta (mostro o
    ///    no), si pescano 3 nuove carte, lasciando quella "passata avanti".
    ///
    /// In entrambi i casi si azzerano i flag di fuga/pozione e si incrementa il turno.
    fn maybe_advance(&mut self) {
        let has_monster = self
            .current_room
            .iter()
            .any(|c| c.is_some_and(|c| c.is_monster()));

        if !has_monster {
            self.current_room = [None; ROOM_SIZE];
            self.draw_room();
        } else if self.current_room.iter().filter(|c| c.is_some()).count() == 1 {
            for slot in self.current_room.iter_mut() {
                if slot.is_none() {
                    *slot = self.dungeon_deck.pop();
                }
            }
        } else {
            return;
        }

        self.turn += 1;
        self.last_action_was_avoid = false;
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

    /// Costruisce una stanza di ROOM_SIZE a partire da una lista di carte.
    fn room_with(cards: &[Option<Card>]) -> [Option<Card>; ROOM_SIZE] {
        let mut r = [None; ROOM_SIZE];
        for (i, c) in cards.iter().enumerate().take(ROOM_SIZE) {
            r[i] = *c;
        }
        r
    }

    fn monster(v: u8) -> Card {
        Card::new(Suit::Spades, v)
    }

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
        // Due mostri: dopo il primo colpo la stanza non avanza subito (resta un mostro)
        state.current_room =
            room_with(&[Some(monster(7)), Some(monster(3)), None, None]);
        state.attack(monster(7), false).unwrap();
        assert_eq!(state.hp, MAX_HP - 7);
        assert!(
            state.current_room.iter().all(|c| *c != Some(monster(7))),
            "il mostro ucciso deve sparire dalla stanza"
        );
    }

    #[test]
    fn weapon_reduces_damage() {
        let mut state = GameState::new();
        state.weapon = Some(Weapon::new(4));
        state.current_room =
            room_with(&[Some(monster(12)), Some(monster(6)), None, None]);
        state.attack(monster(12), false).unwrap();
        assert_eq!(state.hp, MAX_HP - 8); // 12 - 4
    }

    #[test]
    fn weapon_binds_to_last_monster() {
        let mut state = GameState::new();
        state.weapon = Some(Weapon::new(10));
        state.current_room =
            room_with(&[Some(monster(13)), Some(monster(10)), None, None]);
        state.attack(monster(13), false).unwrap();
        assert_eq!(state.weapon.unwrap().last_monster_value, Some(13));

        // Legata a 13: puoi ancora attaccare un 10 (≤ 13)
        state.attack(monster(10), false).unwrap();
        assert_eq!(state.weapon.unwrap().last_monster_value, Some(10));
    }

    #[test]
    fn bound_weapon_blocks_stronger_monster_but_barehanded_works() {
        let mut state = GameState::new();
        state.weapon = Some(Weapon {
            value: 10,
            last_monster_value: Some(3),
        });
        state.current_room =
            room_with(&[Some(monster(14)), Some(monster(3)), None, None]);

        // Con l'arma legata a 3 non puoi colpire un 14...
        assert!(state.attack(monster(14), false).is_err());

        // ... ma puoi combatterlo a mani nude (danno pieno), l'arma resta.
        state.attack(monster(14), true).unwrap();
        assert_eq!(state.hp, MAX_HP - 14);
        assert_eq!(state.weapon.unwrap().last_monster_value, Some(3));
    }

    #[test]
    fn cannot_attack_red_cards() {
        let mut state = GameState::new();
        state.current_room =
            room_with(&[Some(Card::new(Suit::Hearts, 5)), Some(monster(4)), None, None]);
        assert!(state.attack(Card::new(Suit::Hearts, 5), true).is_err());
    }

    #[test]
    fn potion_heals_and_caps_at_max_hp() {
        let mut state = GameState::new();
        state.hp = 10;
        state.current_room =
            room_with(&[Some(Card::new(Suit::Hearts, 8)), Some(monster(3)), None, None]);
        state.equip(Card::new(Suit::Hearts, 8)).unwrap();
        assert_eq!(state.hp, 18);

        // nuova stanza (qui simulata): il limite "una pozione per stanza" si azzera
        state.have_healed = false;
        state.hp = 19;
        state.current_room =
            room_with(&[Some(Card::new(Suit::Hearts, 8)), Some(monster(3)), None, None]);
        state.equip(Card::new(Suit::Hearts, 8)).unwrap();
        assert_eq!(state.hp, MAX_HP);
    }

    #[test]
    fn only_one_potion_per_room() {
        let mut state = GameState::new();
        state.have_healed = true;
        state.current_room = room_with(&[Some(Card::new(Suit::Hearts, 4)), Some(monster(3)), None, None]);
        assert!(state.equip(Card::new(Suit::Hearts, 4)).is_err());
    }

    #[test]
    fn flee_shuffles_cards_back_and_forbids_double_avoid() {
        let mut state = GameState::new();
        let deck_before = state.dungeon_deck.len();
        state.flee().unwrap();
        // le 4 carte + 4 nuove pescate: il mazzo torna alla stessa dimensione
        assert_eq!(state.dungeon_deck.len(), deck_before);
        assert_eq!(
            state.current_room.iter().filter(|c| c.is_some()).count(),
            4
        );
        assert!(state.last_action_was_avoid);
        assert!(state.flee().is_err());
    }

    #[test]
    fn room_completes_when_last_monster_dies() {
        let mut state = GameState::new();
        state.current_room = room_with(&[
            Some(Card::new(Suit::Hearts, 5)),
            Some(Card::new(Suit::Hearts, 6)),
            Some(Card::new(Suit::Hearts, 7)),
            Some(monster(4)),
        ]);
        let deck_before = state.dungeon_deck.len();

        state.equip(Card::new(Suit::Hearts, 5)).unwrap(); // bevi 1 pozione
        state.attack(monster(4), false).unwrap(); // ultimo mostro muore

        // Completamento automatico: le pozzi residue sono scartate,
        // ne vengono pescate 4 nuove, flag azzerati e turn incrementato.
        assert_eq!(
            state.current_room.iter().filter(|c| c.is_some()).count(),
            4
        );
        assert_eq!(state.dungeon_deck.len(), deck_before - 4);
        assert_eq!(state.turn, 2);
        assert!(!state.have_healed);
        assert!(!state.last_action_was_avoid);
    }

    #[test]
    fn auto_advance_when_single_card_left() {
        let mut state = GameState::new();
        let deck_before = state.dungeon_deck.len();
        state.current_room = room_with(&[
            Some(monster(2)),
            Some(monster(3)),
            Some(Card::new(Suit::Diamonds, 4)),
            None,
        ]);

        state.equip(Card::new(Suit::Diamonds, 4)).unwrap(); // resta 2 mostri
        state.attack(monster(2), false).unwrap(); // resta 1 mostro -> auto new turn

        assert_eq!(
            state.current_room.iter().filter(|c| c.is_some()).count(),
            4
        );
        assert_eq!(state.dungeon_deck.len(), deck_before - 3); // 3 nuove pescate
        assert_eq!(state.turn, 2);
    }

    #[test]
    fn flee_stays_blocked_until_room_advances() {
        let mut state = GameState::new();
        state.flee().unwrap();

        // giocare una carta (equip) NON sblocca la fuga
        state.current_room[0] = Some(Card::new(Suit::Diamonds, 5));
        state.equip(Card::new(Suit::Diamonds, 5)).unwrap();
        assert!(
            state.flee().is_err(),
            "non si può fuggire due stanze di fila, anche dopo aver giocato una carta"
        );

        // uccidere l'ultimo mostro fa avanzare la stanza e sblocca la fuga
        state.current_room = room_with(&[Some(monster(3)), Some(monster(4)), None, None]);
        state.attack(monster(3), true).unwrap();
        state.attack(monster(4), true).unwrap(); // ultimo mostro -> completamento
        assert!(state.flee().is_ok());
    }

    #[test]
    fn room_without_monsters_advances_after_usable_cards_gone() {
        let mut state = GameState::new();
        // stanza senza mostri: 1 arma + 3 pozioni
        state.current_room = room_with(&[
            Some(Card::new(Suit::Diamonds, 4)),
            Some(Card::new(Suit::Hearts, 6)),
            Some(Card::new(Suit::Hearts, 8)),
            Some(Card::new(Suit::Hearts, 10)),
        ]);
        let deck_before = state.dungeon_deck.len();

        // bevi la pozione: c'è ancora l'arma da equipaggiare -> nessun avanzamento
        state.equip(Card::new(Suit::Hearts, 6)).unwrap();
        assert_eq!(state.turn, 1);

        // equipaggi l'arma: restano solo pozioni non più bevibili -> avanzamento
        state.equip(Card::new(Suit::Diamonds, 4)).unwrap();
        assert_eq!(state.turn, 2);
        assert_eq!(
            state.current_room.iter().filter(|c| c.is_some()).count(),
            4
        );
        assert_eq!(state.dungeon_deck.len(), deck_before - 4);
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
