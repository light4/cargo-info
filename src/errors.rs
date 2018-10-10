use reqwest;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "requests error {}", _0)]
    Requests(#[cause] reqwest::Error),
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Requests(err)
    }
}
