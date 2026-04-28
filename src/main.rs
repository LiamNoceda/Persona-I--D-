use std::fmt::Display;

struct Persona<I, D> {
    uiid: I,
    name: String,
    full_name: String,
    age: f64,
    datalife: D,
}

impl<I: Display, D: Display> Persona<I, D> {
    pub fn new(uiid: I, name: String, full_name: String, age: f64, datalife: D) -> Self {
        Self {
            uiid,
            name,
            full_name,
            age,
            datalife,
        }
    }
}

fn create_persona<I: Display, D: Display>(uiid: I, name: String, full_name: String, age: f64, datalife: D) -> Persona<I, D> {
    Persona::new(uiid, name, full_name, age, datalife)
}

fn main() {
    let persona = create_persona("#11151".to_string(), "Alice".to_string(), "Alice Smith".to_string(), 30.6, "Engineer".to_string());
    println!("Persona: uiid = {}, name = {}, full_name = {}, age = {}, datalife = {}", 
        persona.uiid,
        persona.name,
        persona.full_name,
        persona.age,
        persona.datalife
    );
}
