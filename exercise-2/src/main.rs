use reqwest;
use serde;
use std::collections::HashMap;

#[derive(serde::Deserialize, Debug)]
struct GitHubUser {
    login: String,
}

fn main() {
    let sublime_slack_quotes: HashMap<&str, &str> = [
        ("celinekurpershoek", "happy days; http://cultofthepartyparrot.com/"),
        ("Frankwarnaar", "Except Anne. He always joins lunch when you’re about to start cleaning the table"),
        ("jbmoelker", "Random thought: is dat een versienummer in de nieuwe naam van UNITiD? Die designers leren het ook nooit hè. Voor je het weet heten ze Hike Def Final."),
        ("petergoes", "zijn jullie front-end devs of niet? `npx serve`"),
        ("phortuin", "dude put his lifesavings in bitcoin. not sure if he even is allowed to dish out ‘advice’"),
    ]
         .iter().cloned().collect();

    reqwest::get("https://api.github.com/orgs/voorhoede/members")
        .unwrap()
        .json::<Vec<GitHubUser>>()
        .unwrap()
        .iter()
        // Filter out `None` quotes that have no matching users
        // Find a useful method on `Iterator` to replace the `map`
        // https://doc.rust-lang.org/std/iter/trait.Iterator.html
        .map(|user| {
            sublime_slack_quotes
                // https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.get
                .get(user.login.as_str())
                // Use `map` in combination with: https://doc.rust-lang.org/std/macro.format.html
        })
        .for_each(|message| println!("{:?}", message));
}
