#[derive(Debug, Default)]
#[allow(unused)]
pub struct Invocation<'a, T> {
    pub value: Option<String>,
    pub values: Option<Vec<&'a str>>,
    pub headers: Option<Vec<&'a str>>,
    pub method: Option<&'a str>,
    pub host: Option<String>,
    pub url: Option<String>,
    pub payload: Option<T>,
    pub response_object: Option<String>,
}
