use serde::Deserialize;
use std::collections::hash_set;

//Ran into lifetimes issues, need to actually understand what is going on. 
fn suggest<'a>(list: &Vec<Book>, database: &'a Vec<Vec<Book>>) -> hash_set::HashSet<&'a Book> {
    let min_threshold: u8 = 2;
    let mut result : hash_set::HashSet<&Book> = hash_set::HashSet::new();
    for entry in database {
        let mut intersect_counter: u8 = 0;
        'outer: for book in list {
            for db_book in entry {
                if book == db_book {
                    intersect_counter += 1;
                }
                if intersect_counter >= min_threshold {
                    break 'outer; //all of the db_books in entry are of interest at this point,
                                  //stop looking
                }
            }
        }
        result.extend(entry.iter());
    }

    //we don't want any of the original list to be re-recommended either
    result.retain(|v| !list.contains(v));
    result
}

#[derive(Deserialize, Debug, PartialEq, Clone, Eq, Hash)]
struct Book {
    title : String,
    author : String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //load database, currently being served as a raw JSON file
    //TODO: parse these into specific structs rather than Value
    let database : Vec<Vec<Book>> = reqwest::blocking::get("http://localhost:8000/database.json")?
        .json()?;

    let data = r#"
        [
              {
                "title": "Gone Girl",
                "author": "Gillian Flynn"
              },
              {
                "title": "Pride and Prejudice",
                "author": "Jane Austen"
              }
        ]"#;
    let sample_user_list: Vec<Book> = serde_json::from_str(data)?;
    let intersection = suggest(&sample_user_list, &database);
    println!("{:#?}", intersection); 

    //println!("{:#?}", database); //useful debug string, but ultimately should be removed.
    Ok(())
}
