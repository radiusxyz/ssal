use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub trait WrapError {
    type Output;

    fn wrap<C>(self, context: C) -> Self::Output
    where
        C: std::fmt::Debug + 'static;
}

impl<T, E> WrapError for Result<T, E>
where
    E: std::error::Error + 'static,
{
    type Output = Result<T, Error>;

    fn wrap<C>(self, context: C) -> Self::Output
    where
        C: std::fmt::Debug + 'static,
    {
        self.map_err(|error| Error::boxed_error(context, error))
    }
}

impl<T> WrapError for Option<T> {
    type Output = Result<T, Error>;

    fn wrap<C>(self, context: C) -> Self::Output
    where
        C: std::fmt::Debug + 'static,
    {
        self.ok_or(Error::none_type(context))
    }
}

pub struct Error {
    context: Box<dyn std::fmt::Debug>,
    source: ErrorKind,
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} ({})", self.context, self.source)
    }
}

impl std::error::Error for Error {}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self {
            context: Box::new(value.to_string()),
            source: ErrorKind::PlainString,
        }
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Self {
            context: Box::new(value),
            source: ErrorKind::PlainString,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}

impl Error {
    pub fn boxed_error<C, E>(context: C, source: E) -> Self
    where
        C: std::fmt::Debug + 'static,
        E: std::error::Error + 'static,
    {
        Self {
            context: Box::new(context),
            source: ErrorKind::Boxed(Box::new(source)),
        }
    }

    pub fn none_type<C>(context: C) -> Self
    where
        C: std::fmt::Debug + 'static,
    {
        Self {
            context: Box::new(context),
            source: ErrorKind::NoneType,
        }
    }

    pub fn is_none_type(&self) -> bool {
        match &self.source {
            ErrorKind::NoneType => true,
            _others => false,
        }
    }
}

pub enum ErrorKind {
    Boxed(Box<dyn std::error::Error>),
    PlainString,
    NoneType,
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Boxed(error) => write!(f, "{}", error),
            Self::PlainString => write!(f, ""),
            Self::NoneType => write!(f, "The value returned None"),
        }
    }
}
