// if3.rs
//
// Execute `rustlings hint if3` or use the `hint` watch subcommand for a hint.

pub fn animal_habitat(animal: &str) -> &'static str {
    let identifier = if animal == "crab" {
        1.0
    } else if animal == "gopher" {
        2.0
    } else if animal == "snake" {
        3.0
    } else {
        -1.0
    };

    // DO NOT CHANGE THIS STATEMENT BELOW
    let habitat = if identifier == 1.0 {
        "Beach"
    } else if identifier == 2.0 {
        "Burrow"
    } else if identifier == 3.0 {
        "Desert"
    } else {
        "Unknown"
    };

    habitat
}

// No test changes needed.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gopher_lives_in_burrow() {
        assert_eq!(animal_habitat("gopher"), "Burrow")
    }

    #[test]
    fn snake_lives_in_desert() {
        assert_eq!(animal_habitat("snake"), "Desert")
    }

    #[test]
    fn crab_lives_on_beach() {
        assert_eq!(animal_habitat("crab"), "Beach")
    }

    #[test]
    fn unknown_animal() {
        assert_eq!(animal_habitat("dinosaur"), "Unknown")
    }
}
