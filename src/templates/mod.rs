use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub daily_wisdom: String,
}

#[derive(Template)]
#[template(path = "about.html")]
pub struct AboutTemplate {}

#[derive(Template)]
#[template(path = "wisdom.html")]
pub struct WisdomTemplate {
    pub daily_wisdom: String,
}

#[derive(Template)]
#[template(path = "quantum_field.html")]
pub struct QuantumFieldTemplate {}
