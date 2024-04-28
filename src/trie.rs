pub mod trie {
    use std::collections::HashMap;

    pub struct TrieNode{
        pub is_end_of_word: bool,
        pub children: HashMap<String, TrieNode>,
        pub character: String,
    }

    pub fn insert(root: &mut TrieNode, word: &str){
        if word.is_empty(){
            //println!("no more letters");
            root.is_end_of_word = true;
        } else {
            //println!("{}", &word[0..1]);
            if let Some(mut child) = root.children.get_mut(&word[0..1].to_string()){
                insert(&mut child, &word[1..]);
            } else {
                let mut new_Node = TrieNode{
                    is_end_of_word: false,
                    children: HashMap::new(),
                    character: (&word[0..1]).to_string(),
                };

                insert(&mut new_Node, &word[1..]);                
                root.children.insert((&word[0..1]).to_string(), new_Node);

            }
        };
    }

    pub fn isWord(root: &TrieNode, word: &str) -> bool{
        if word.is_empty(){
            return root.is_end_of_word;
        } 
        if let Some(child) = root.children.get(&word[0..1].to_string()){
            return isWord(child, &word[1..]);
        } else {
            return false;
        };
    }

    pub fn isPrefix(root: &TrieNode, pref: &str) -> bool{
        if pref.is_empty(){
            if root.children.len() > 0 {
                return true;
            } else {
                return false;
            }
        }        
        if let Some(child) = root.children.get(&pref[0..1].to_string()){
            return isPrefix(child, &pref[1..]);
        } else {
            return false;
        };
    }

}

