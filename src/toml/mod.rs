use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct InfoToml {
    pub name: String,
    pub authors: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
pub struct SubmissionToml {
    pub submission_type: String,
    pub binary: Option<String>,
    pub compiler: Option<String>,
    pub flags: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
pub struct TestToml {
    pub test_type: String,
    pub binary: Option<String>,
    pub args: Option<Vec<Vec<String>>>,
}

#[derive(Deserialize, Debug)]
pub struct ModuleToml {
    pub info: InfoToml,
    pub submission: SubmissionToml,
    pub test: TestToml,
}
