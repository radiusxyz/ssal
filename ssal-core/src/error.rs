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
        write!(f, "{:?}: {}", self.context, self.source)
    }
}

impl std::error::Error for Error {}

impl Error {
    pub fn none_type<C>(context: C) -> Self
    where
        C: std::fmt::Debug + 'static,
    {
        Self {
            context: Box::new(context),
            source: ErrorKind::NoneType,
        }
    }

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
}

pub enum ErrorKind {
    NoneType,
    Boxed(Box<dyn std::error::Error>),
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoneType => write!(f, "The value returned None"),
            Self::Boxed(error) => write!(f, "{}", error),
        }
    }
}
