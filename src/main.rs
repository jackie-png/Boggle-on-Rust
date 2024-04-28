#![allow(non_snake_case,non_camel_case_types,dead_code)]
use std::{collections::HashMap, env};

use trie::TrieNode;

use crate::trie::{isPrefix, isWord};
pub mod trie {
    use std::collections::HashMap;

    pub struct TrieNode{
        pub is_end_of_word: bool,
        pub children: HashMap<String, TrieNode>,
        pub character: String,
    }

    pub fn insert(root: &mut TrieNode, word: &str){
        if word.is_empty(){
            println!("no more letters");
            root.is_end_of_word = true;
        } else {
            if let Some(currChar) = word.chars().nth(0){
                println!("{}",word);
                if let Some(mut child) = root.children.get_mut(&currChar.to_string()){
                    insert(&mut child, &word[1..]);
                } else {
                    let mut new_Node = TrieNode{
                        is_end_of_word: false,
                        children: HashMap::new(),
                        character: currChar.to_string(),
                    };

                    if let Some(after) = word.get(1..){
                        insert(&mut new_Node, after);                
                        root.children.insert(currChar.to_string(), new_Node);
                    };


                }                
            } else{
                println!("bruh");
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
            if root.children.len() > 0 || root.is_end_of_word{
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

/*
    Fill in the boggle function below. Use as many helpers as you want.
    Test your code by running 'cargo test' from the tester_rs_simple directory.
    
    To demonstrate how the HashMap can be used to store word/coord associations,
    the function stub below contains two sample words added from the 2x2 board.
*/

fn boggle(board: & [&str], words: & Vec<String>) -> HashMap<String, Vec<(u8, u8)>>
{
    let mut root = trie::TrieNode{ // making the root of the trie
        is_end_of_word: false,
        children: HashMap::new(),
        character: String::from("0"),
    };

    for word in words.iter(){ // adding the list of words into the trie
        //println!("{}", *word);
        trie::insert(&mut root, word);
        //println!("{}",isWord(&root, word));
        //println!("{}",isPrefix(&root, word));
    }

    let mut found: HashMap<String, Vec<(u8,u8)>> = HashMap::new(); // the found words


    for (row, &row_str) in board.iter().enumerate() {
        for (col, letter) in row_str.chars().enumerate() {
            //println!("Letter at row {} and column {}: {}", row, col, letter);
            let path: Vec<(u8,u8)> = Vec::new();
            let visited = createVisitedBoard(board.len(), board[0].len());

            //searchWord, the only thing mutable should be wordList, everything else should be a clone of the original
            searchWords(row as i8, col as i8, board, &mut visited.clone(), &mut (String::from("")).clone(), &root, &mut path.clone(), &mut found);
        }
    }
    return found;
}

fn searchWords(row: i8, col: i8, board: &[&str], visited: &mut Vec<Vec<bool>>, acc: &mut String , root: &TrieNode, path: &mut Vec<(u8,u8)>, wordList: &mut HashMap<String, Vec<(u8,u8)>>) -> (){

    let usize_row = row as usize;
    let usize_col = col as usize;

    if row >= 0 && col >= 0 && usize_row < board.len() && usize_col < board[0].len() && !visited[usize_row][usize_col] {        
        visited[row as usize][col as usize] = true;
        path.push((row as u8,col as u8));

        if let Some(currChar) = board[row as usize].chars().nth(col as usize){
            acc.push(currChar);
        }//this should always fire
        //println!("--------------------\nvisited: {:?}\nPath: {:?}\nAcc: {}\nwordList :{:?}\n--------------------\n",visited,path,acc,wordList);

        if isPrefix(root, &acc){
            //println!("{} is a prefix", acc);
            //println!("isWord? :{}", isWord(root, &acc));
            if isWord(root, &acc){
                //println!("{} is a word", acc);
                wordList.insert(acc.to_string(), path.to_vec());
            }
            //println!("down ({},{})", row, col);
            searchWords(row+1,col,board,&mut visited.clone(),&mut acc.clone(),root,&mut path.clone(),wordList);
            //println!("UP ({},{})", row, col);
            searchWords(row-1,col,board,&mut visited.clone(),&mut acc.clone(),root,&mut path.clone(),wordList);
            //println!("left ({},{})", row, col);
            searchWords(row,col+1,board,&mut visited.clone(),&mut acc.clone(),root,&mut path.clone(),wordList);
            //println!("right ({},{})", row, col);
            searchWords(row,col-1,board,&mut visited.clone(),&mut acc.clone(),root,&mut path.clone(),wordList);
            //println!("down right ({},{})", row, col);
            searchWords(row+1,col+1,board,&mut visited.clone(),&mut acc.clone(),root,&mut path.clone(),wordList);
            //println!("down left ({},{})", row, col);
            searchWords(row+1,col-1,board,&mut visited.clone(),&mut acc.clone(),root,&mut path.clone(),wordList);
            //println!("up right ({},{})", row, col);

            searchWords(row-1,col+1,board,&mut visited.clone(),&mut acc.clone(),root,&mut path.clone(),wordList);
            //println!("up left at ({},{})", row, col);

            searchWords(row-1,col-1,board,&mut visited.clone(),&mut acc.clone(),root,&mut path.clone(),wordList);
            //println!("backtracking at ({},{}) when its a prefix", row, col);

            backTrack(row as usize, col as usize, path, visited, acc);

        } else {
            //println!("backtracking at ({},{}) when not a prefix", row, col);
            backTrack(row as usize, col as usize, path, visited, acc);
        }

    } else {
        return ();
    }
    //acc, path, visited are all clones of the original, when passing into recursion clone them again
    


}


fn backTrack(row: usize, col: usize, path: &mut Vec<(u8,u8)>, visited: &mut Vec<Vec<bool>>, acc: &mut String){
    visited[row][col] = false;
    acc.truncate(acc.len()-1);
    path.pop();
}


fn createVisitedBoard (rowsNeeded: usize, maxRow: usize) -> Vec<Vec<bool>>{
    let mut grid:Vec<Vec<bool>> = Vec::new();

    for _rows in 0..rowsNeeded {
        let mut row:Vec<bool> = Vec::new();
        //println!("{:?}",rows);
        for _col in 0..maxRow {
            row.push(false);
            //println!("Columns {:?}", col)
        }
        grid.push(row);
        //println!("{:?}", grid);
    }
    return grid;
}

// fn main(){
//     let board = [
//         "ABC",
//         "DEF",
//         "GHI",
//     ];

//     let words = vec![
//         "BED".to_string(),
//         "CA’B".to_string(),
//         "HID".to_string(),
//         "FED".to_string(),
//         "AGE".to_string(),
//     ];
//     let wordlist = boggle(&board, &words);
//     println!("{:?}", wordlist);
// }


    
#[cfg(test)]
#[path = "tests.rs"]
mod tests;
