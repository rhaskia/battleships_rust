pub fn get_high_score() -> usize {
    // loading in high score from storage
    let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    let s = local_storage.get_item("highscore");

    // check if it exists
    if let Ok(Some(c)) = s { return c.parse::<usize>().unwrap(); }

    // if it's not real
    set_high_score(200); 
    200
}

pub fn set_high_score(i: usize) {
    // if not a high score
    if get_high_score() < i { return; }

    // setting high score
    let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    let _ = local_storage.set_item("highscore", &format!("{}", i));
}