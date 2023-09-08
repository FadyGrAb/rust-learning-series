#[derive(Debug)]
enum Superhero {
    Batman,
    Superman,
    WonderWoman,
    Flash,
    GreenLantern,
    CatWoman,
}

#[derive(Debug)]
enum SuperheroWithWeapon {
    Batman(FavoriteWeapon),
    Superman,
    WonderWoman(FavoriteWeapon),
    Flash,
    GreenLantern(FavoriteWeapon),
    CatWoman,
}

#[derive(Debug)]
enum FavoriteWeapon {
    LassoOfTruth,
    GreenLanternRing,
    Batarang,
}

fn main() {
    // Basic matching.
    let hero = Superhero::Batman;
    let mut is_batman = false;
    let super_power = match hero {
        Superhero::Batman => {
            is_batman = true;
            "Rich"
        }
        Superhero::Superman => "Flying, Bulletproof, Heat vision, Cold breath",
        Superhero::WonderWoman => "Flying, Strength, Divine weapons",
        Superhero::CatWoman => "Stealth, Agility",
        Superhero::Flash => "Speed, Connection with the speed-force",
        Superhero::GreenLantern => "Flying, Instant light constructs, Lantern Corps",
    };
    println!("{hero:?}: {super_power} -- is batman {is_batman}");

    // Matching with patterns that bind to values with a catch-all.
    let hero_with_weapon_1 = SuperheroWithWeapon::WonderWoman(FavoriteWeapon::LassoOfTruth);
    match hero_with_weapon_1 {
        SuperheroWithWeapon::Batman(weapon) => {
            println!("Batman: {weapon:?}")
        }
        SuperheroWithWeapon::WonderWoman(weapon) => {
            println!("Wonder Woman: {weapon:?}")
        }
        SuperheroWithWeapon::GreenLantern(weapon) => {
            println!("Green Lantern: {weapon:?}")
        }
        other => println!("{other:?} doesn't usually use weapons."),
    };
    
    // Matching with patterns that bind to values with _ placeholder.
    let hero_with_weapon_2 = SuperheroWithWeapon::Superman;
    match hero_with_weapon_2 {
        SuperheroWithWeapon::Batman(weapon) => {
            println!("Batman: {weapon:?}")
        }
        SuperheroWithWeapon::WonderWoman(weapon) => {
            println!("Wonder Woman: {weapon:?}")
        }
        SuperheroWithWeapon::GreenLantern(weapon) => {
            println!("Green Lantern: {weapon:?}")
        }
        _ => println!("Doesn't usually use weapons."),
    };
}
