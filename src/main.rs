use dict::dict_set::DictSet;

pub mod adt;
pub mod linked_list;
pub mod dict;

fn main() {
    
    // let set = ["gamete", "game", "household", "hold", "horizon", "endgame"];
    // let dict_set = DictSet::aho_corasick(&set).unwrap();

    // let pattern = "The soldiers could barely hold off the orcs until they saw the cavalry charging in from the horizon. The endgame was near.";
    // dict_set.search(&pattern, |index, word| {
    //     println!("Found word {:?} at &pattern[{}..{}]", word, index, index + word.len());
    //     println!("Word in slice -> {:?}", &pattern[index..(index + word.len())]);
    // }).expect("Failed to search string through dictionary");

    let mut dict_set = DictSet::new();
    let set = ["gamete", "game", "household", "hold", "horizon", "endgame"];

    dict_set.insert_all(&set);

    for word in set {
        println!("{:?} -> {}", word, dict_set.contains(word));
    }

    dict_set.remove("game").unwrap();

    println!("\nRemoved: \"game\"\n");

    for word in set {
        println!("{:?} -> {}", word, dict_set.contains(word));
    }

    println!("\n");

    dict_set.display();

}
