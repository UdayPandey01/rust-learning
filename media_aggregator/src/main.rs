// TODO: 1. Define a trait named `Summarizable`
// It should have one method signature: `summary(&self) -> String`
trait Summarizable {
    fn summary(&self) -> String;
}


// TODO: 2. Define two structs: `BlogPost` and `WeatherForecast`
// `BlogPost` should have fields: `author: String` and `content: String`
// `WeatherForecast` should have fields: `location: String` and `temperature: f64`
struct BlogPost {
    author : String,
    content : String
}

struct WeatherForecast {
    location : String,
    temperature : f64
}

impl Summarizable for BlogPost {
    fn summary(&self) -> String {
        format!("New post by {}", self.author)
    }
}

impl Summarizable for WeatherForecast {
    fn summary(&self) -> String {
        format!("Weather is {} : {}", self.location, self.temperature)
    }
}

// TODO: 3. Implement the `Summarizable` trait for both structs.
// For `BlogPost`, `summary()` should return a string like: "New post by [author]".
// For `WeatherForecast`, `summary()` should return a string like: "Weather in [location]: [temp]C".


// TODO: 4. Implement this generic function.
// It should accept any item that implements the `Summarizable` trait.
fn notify(item: &impl Summarizable) {
    // Inside the function, print the item's summary.
    println!("Notification {}",item.summary());
    // e.g., "Notification: [summary text]"
}

fn main() {
    // TODO: 5. Create an instance of `BlogPost` and `WeatherForecast`.
    let mut blogPost = BlogPost {
        author : String::from("Uday"),
        content : String::from("Hello")
    };

    let mut weatherForecast = WeatherForecast {
        location : String::from("Kanpur"),
        temperature : 37.6
    };

    notify(&blogPost);
    notify(&weatherForecast);
    // Then, call the `notify` function with each of them to test your code.
}