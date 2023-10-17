// if3.rs
//
// Execute `rustlings hint if3` or use the `hint` watch subcommand for a hint.

pub fn animal_habitat(animal: &str) -> &'static str {
    let identifier = if animal == "crab" {
        1
    } else if animal == "gopher" {
        2 // Fix type to use integer
    } else if animal == "snake" {
        3
    } else {
        -1 // Return value that is not matched in `habitat` test cases below
    };

    // DO NOT CHANGE THIS STATEMENT BELOW
    let habitat = if identifier == 1 {
        "Beach"
    } else if identifier == 2 {
        "Burrow"
    } else if identifier == 3 {
        "Desert"
    } else {
        "Unknown"
    };

    habitat

    /*
    Horrible code BTW. Here's a version that uses `match`, keeping the identifier use:
    
    let identifier = match animal {
        "crab" => 1,
        "gopher" => 2,
        "snake" => 3,
        _ => -1,
    };

    let habitat = match identifier {
        1 => "Beach",
        2 => "Burrow",
        3 => "Desert",
        _ => "Unknown",
    };

    habitat
    */

    /*
    Or using only one `match`:
    
    match animal {
        "crab" => "Beach",
        "gopher" => "Burrow",
        "snake" => "Desert",
        _ => "Unknown",
    }
    */
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
