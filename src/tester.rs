
mod tester{
    use std::collections::HashMap;    
    pub struct TrieNode{
        pub is_end_of_word: bool,
        pub children: HashMap<String, TrieNode>,
        pub character: String,
    }


    

    pub fn insert(root: TrieNode, word: String){
        if word.is_empty(){
            root.is_end_of_word = true;
        } else if root.children.contains_key(word){
            insert(root.children.get)
        }
    }

}

fn main(){
    use std::collections::HashMap;    

    let root = tester::TrieNode{
        is_end_of_word: false,
        children: HashMap::new(),
        character: '0',
    };

    println!("{}", root.is_end_of_word);
}

