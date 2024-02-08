struct Joke {
    content: String,
}

struct DadJoke {
    content: String,
}

enum Name {
    Kiril,
    Josh,
    Megan,
    Pavel,
    Anne
}

struct Person {
    name: Name,
    statement: String,
}

impl Person {
    fn new(name: Name, statement: &str) -> Self {
        Person {
            name,
            statement: statement.to_string(),
        }
    }
}

impl TryFrom<Person> for Joke {
    type Error = DadJoke;

    fn try_from(person: Person) -> Result<Self, Self::Error> {
        match person.name {
            Name::Kiril => Err(DadJoke { content: person.statement }),
            _ => Ok(Joke { content: person.statement }),
        }
    }
}

fn main() {
    let person = Person::new(Name::Kiril, "Joke::try_from(beer)");

    let joke = match Joke::try_from(person) {
        Ok(joke) => joke.content,
        Err(dad_joke) => format!("Dad joke alarm! {}", dad_joke.content),
    };

    println!("{}", joke);
}