mod bindings;

use crate::bindings::exports::golem::component::api::*;
use std::cell::RefCell;

use rand::prelude::*;

struct Component;

/**
 * This is one of any number of data types that our application
 * uses. Golem will take care to persist all application state,
 * whether that state is local to a function being executed or
 * global across the entire program.
 */
struct State {
    user_id: String,
    character: Character,
}

impl Character {
    fn new() -> Self {
        Self {
            name: String::from("John Doe"),
            high_concept: String::from("Brave Adventurer"),
            trouble: String::from("Afraid of the dark"),
            aspects: vec![
                String::from("Clever"),
                String::from("Resourceful"),
                String::from("Quick Thinker"),
            ],
            skills: vec![
                SkillRating {
                    rating: 4,
                    skills: vec![String::from("Contacts") ],
                },
                SkillRating {
                    rating: 3,
                    skills: vec![
                        String::from("Deceive"),
                        String::from("Provoke"),
                    ],
                },
                SkillRating {
                    rating: 2,
                    skills: vec![
                        String::from("Physique"),
                        String::from("Will"),
                        String::from("Burglary"),
                    ],
                },
                SkillRating {
                    rating: 1,
                    skills: vec![
                        String::from("Athletics"),
                        String::from("Combat"),
                        String::from("Empathy"),
                        String::from("Notice"),
                    ],
                },
            ],
            stunts: vec![
                String::from("Combat Expertise: Gain +2 to attack rolls when using a sword."),
                String::from("Athletic Sprint: Once per scene, you may move two zones for free instead of one."),
                String::from("Smooth Talker: Once per session, reroll any failed Deceive or Rapport check."),
            ],
            skill_list: [
                "Athletics", "Burglary", "Contacts", "Crafts", "Deceive", "Drive", "Empathy", "Fight", "Investigate",
                "Lore", "Notice", "Physique", "Provoke", "Rapport", "Shoot", "Stealth", "Will",
            ].map(String::from).to_vec(),
        }
    }
}

thread_local! {
    /**
     * This holds the state of our application, which is always bound to
     * a given user.
     */
    static STATE: RefCell<State> = RefCell::new(State {
        user_id: String::new(),
        character: Character::new(),
    });
}

// Here, we declare a Rust implementation of the `ShoppingCart` trait.
impl Guest for Component {
    fn get_character() -> Character {
        STATE.with_borrow(|s| s.character.clone())
    }

    fn update_character(character: Character) {
        STATE.with(|s| s.borrow_mut().character = character)
    }

    fn toggle_editable() -> bool {
        todo!()
    }

    fn submit_character(character: Character) {
        todo!()
    }
}

bindings::export!(Component with_types_in bindings);
