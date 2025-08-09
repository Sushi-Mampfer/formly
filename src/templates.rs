use askama::Template;

#[derive(Template)]
#[template(path="signup.html")]
pub struct SignUpPage {
    pub error: Option<String>
}

#[derive(Template)]
#[template(path="login.html")]
pub struct LogInPage {
    pub error: Option<String>,
    pub token: String,
    pub challenge: String
}