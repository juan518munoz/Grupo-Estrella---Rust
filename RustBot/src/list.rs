// List of Tuple String and Integer
#[derive(Debug)]
pub enum List {
    Cons(String, i32, Box<List>),
    Nil,
}

// Iterate with a for loop the list
pub fn print_list(list: &List, requested : i32) {
    if requested == 3 {
        return;
    }

    match list {
        List::Cons(x, y, list) => {
            println!("Song: {}, times played: {}", x, y);
            let req = requested + 1;
            print_list(list, req);
        }
        List::Nil => {}
    }
}

// Add String to list, if already on list increase count
pub fn add_to_list(list: &mut List, string: String) {
    match list {
        List::Cons(string_in_list, int, tail) => {
            if string_in_list == &string {
                *int += 1;
            } else {
                add_to_list(tail, string);
            }
        }
        List::Nil => {
            *list = List::Cons(string, 1, Box::new(List::Nil));
        }
    }
}

// appends all Tuple values from list to a vector
pub fn list_to_vec(list: &List) -> Vec<(String, i32)> {
    let mut vec = Vec::new();
    match list {
        List::Cons(x, y, list) => {
            // copy string x to a new string
            let string = x.clone();
            vec.push((string, *y));
            vec.append(&mut list_to_vec(list));
        }
        List::Nil => {}
    }
    vec
}

// appends all Tuple values from vector to a list
fn vec_to_list(vec: Vec<(String, i32)>) -> List {
    let mut list = List::Nil;
    for (x, y) in vec.iter() {
        list = List::Cons(x.clone(), y.clone(), Box::new(list));
    }
    list
}

// sorts a vector of Tuple String and Integer by its Interger
fn sort_vector(vector: Vec<(String, i32)>) -> Vec<(String, i32)> {
    let mut vector = vector;
    vector.sort_by(|a, b| a.1.cmp(&b.1));
    vector
}

// sorts a list by count
pub fn sort_list(list: List) -> List {
    // transform list to vector
    let vec = list_to_vec(&list);

    // sort vector
    let vec = sort_vector(vec);

    // transform vector to list
    vec_to_list(vec)
}