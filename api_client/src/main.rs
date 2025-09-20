use serde::Deserialize;

// TODO: 1. Define a `Todo` struct.
// The JSON from the API will look like this:
// { "userId": 1, "id": 1, "title": "...", "completed": false }
// Make sure to handle the `userId` field with a `serde` attribute.
#[derive(Deserialize, Debug)]
struct Todo {
    id: u32,
    title: String,
    completed: bool,
    #[serde(rename = "userId")]
    user_id: u32,
}


#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let api_url = "https://jsonplaceholder.typicode.com/todos";

    // TODO: 2. Make an async GET request to the `api_url`.
    let response = reqwest::get(api_url).await?;

    

    // TODO: 3. Deserialize the JSON response into a `Vec<Todo>`.
    let todo : Vec<Todo> = response.json().await?;
    

    // TODO: 4. Use an iterator chain on the vector of todos to:
    //    - Find the first todo that is `completed`.
    //    - The `.find()` method on an iterator is perfect for this.

    let first_completed = todo.iter().find(|todo| todo.completed);
    

    // TODO: 5. Print the title of the first completed todo.
    //    - If a completed todo was found, print its title.
    //    - If no completed todos were found, print a message saying so.
    match first_completed {
        Some(todo) => println!("First completed todo: {}", todo.title),
        None => println!("No completed todos were found."),
    }
    

    Ok(())
}