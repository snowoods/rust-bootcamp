use std::rc::Rc;

mod models;

mod db;
use db::*;

mod ui;

mod io_utils;
use io_utils::*;

mod navigator;
use navigator::*;

/// < 개발 순서 >
/// DB Modeling (Stage1-Step1)
///  - db.json
///  - models.rs
/// DB READ/WRITE (Stage1-Step2)
///  - db.rs
///  - dependencies (anyhow, serde, serde_json, tempfile)
/// JiraDatabase Epic/Story CRUD (Stage1-Step3)
///  - db.rs
/// Pages and page_helpers (Stage2-Step1)
///  - ui/pages/mod.rs
///  - ui/pages/page_helpers.rs
///  - dependencies (ellipse, itertools)
/// Prompts (Stage2-Step2)
///  - prompt.rs
///  - io_utils.rs
/// Implementing The Navigator (Stage3-Step1)
///  - navigator.rs
/// The Program Loop (Stage3-Step2)
///  - main.rs
/// 
/// Finished!!

fn main() {
    // TODO: create database and navigator
    let db = Rc::new(JiraDatabase::new("./data/db.json".to_owned()));
    let mut navigator = Navigator::new(Rc::clone(&db));

    loop {
        clearscreen::clear().unwrap();

        // TODO: implement the following functionality:
        // 1. get current page from navigator. If there is no current page exit the loop.
        if let Some(page) = navigator.get_current_page() {

            // 2. render page
            if let Err(error) = page.draw_page() {
                println!("Error rendering page: {}\nPress any key to continue...", error);
                wait_for_key_press();
            };

            // 3. get user input
            let user_input = get_user_input();

            // 4. pass input to page's input handler
            match page.handle_input(user_input.trim()) {
                Err(error) => {
                    println!("Error getting user input: {}\nPress any key to continue...", error);
                    wait_for_key_press();
                },
                // 5. if the page's input handler returns an action let the navigator process the action
                Ok(action) => {
                    if let Some(action) = action {
                        if let Err(error) = navigator.handle_action(action) {
                            println!("Error handling processing user input: {}\nPress any key to continue...", error);
                            wait_for_key_press();
                        }
                    }
                }
            }
        } else {
            break;
        }
    }
}