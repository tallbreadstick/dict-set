use dict::dict_set::DictSet;

pub mod adt;
pub mod linked_list;
pub mod dict;

fn main() {
    
    let set = ["gamete", "game", "household", "hold", "horizon", "endgame"];
    let dict_set = DictSet::aho_corasick(&set).unwrap();

    let pattern = "The soldiers could barely hold off the orcs until they saw the cavalry charging in from the horizon. The endgame was near.";
    dict_set.search(&pattern, |index, word| {
        println!("Found word {:?} at &pattern[{}..{}]", word, index - word.len(), index + 1);
        println!("Word in slice -> {:?}", &pattern[(index - word.len())..(index + 1)]);
    }).expect("Failed to search string through dictionary");

}
