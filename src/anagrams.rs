use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for str in strs {
        let mut hash = str.chars().map(|c| c as u32).collect::<Vec<u32>>();
        hash.sort();

        let entry = map.entry(hash.into_iter().map(|s| s.to_string()).collect::<String>());

        match entry {
            Entry::Vacant(e) => {
                e.insert(vec![str]);
            }
            Entry::Occupied(e) => {
                e.into_mut().push(str);
            }
        }
    }

    let mut answ = vec![];
    for (_, v) in map {
        answ.push(v);
    }

    answ
}

#[test]
fn test_anagrams() {
    // let anagrams = vec![
    //     "eat".to_string(),
    //     "tea".to_string(),
    //     "tan".to_string(),
    //     "ate".to_string(),
    //     "nat".to_string(),
    //     "bat".to_string(),
    // ];

    // let answer = group_anagrams(anagrams);

    // let a = vec!["ate", "eat", "tea"];
    // let b = vec!["nat", "tan"];
    // let c = vec!["bat"];
    // let mut out = Vec::new();
    // out.push(a);
    // out.push(b);
    // out.push(c);

    // assert_eq!(answer, out);

    // let anagrams = vec![
    //     "cab".to_string(),
    //     "tin".to_string(),
    //     "pew".to_string(),
    //     "duh".to_string(),
    //     "may".to_string(),
    //     "ill".to_string(),
    //     "buy".to_string(),
    //     "bar".to_string(),
    //     "max".to_string(),
    //     "doc".to_string(),
    // ];

    // let answer = group_anagrams(anagrams);

    // let a = vec!["ate", "eat", "tea"];
    // let b = vec!["nat", "tan"];
    // let c = vec!["bat"];
    // let mut out = Vec::new();
    // out.push(a);
    // out.push(b);
    // out.push(c);

    // assert_eq!(answer, out);
}
