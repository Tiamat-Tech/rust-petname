mod lib;
use lib as petname;


fn main() {
    let adjectives = petname::WordList::load(
        petname::Adjective, petname::Large);
    let adverbs = petname::WordList::load(
        petname::Adverb, petname::Medium);
    let names = petname::WordList::load(
        petname::Name, petname::Small);

    if !adjectives.is_empty() {
        println!("Adjectives: {}", adjectives.len());
    }
    if !adverbs.is_empty() {
        println!("Adverbs: {}", adverbs.len());
    }
    if !names.is_empty() {
        println!("Names: {}", names.len());
    }

    println!("--");

    let some_names = names.iter().take(5).collect::<Vec<_>>().join(" ");
    println!("Names: {}", some_names);

    let some_names = names.iter_random().take(5).collect::<Vec<_>>().join(" ");
    println!("Random names: {}", some_names);

    println!(
        "Random petname: {}-{}-{}", adverbs.random(),
        adjectives.random(), names.random());
}