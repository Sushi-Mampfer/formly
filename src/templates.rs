use askama::Template;

use crate::datatypes::{FieldKind, FieldValue, FormDefinition, FormShort, FormSubmission};

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexPage {}

#[derive(Template)]
#[template(path = "signup.html")]
pub struct SignUpPage {
    pub error: Option<String>,
}

#[derive(Template)]
#[template(path = "login.html")]
pub struct LogInPage {
    pub error: Option<String>,
    pub token: String,
    pub challenge: String,
}

#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct DashboardPage {
    pub username: String,
    pub forms: Vec<FormShort>,
}

#[derive(Template)]
#[template(path = "create.html")]
pub struct CreateFormPage {}

#[derive(Template)]
#[template(path = "edit.html")]
pub struct EditFormPage {
    pub id: String,
    pub data: FormDefinition,
}

#[derive(Template)]
#[template(path = "form.html")]
pub struct FormPage {
    pub id: String,
    pub data: FormDefinition,
}

#[derive(Template)]
#[template(path = "submissions.html")]
pub struct SubmissionsPage {
    pub name: String,
    pub data: Vec<FormSubmission>,
}
