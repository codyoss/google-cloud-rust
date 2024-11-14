// Copyright 2024 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

type BoxError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    source: BoxError,
}

impl Error {
    /// Creates a new [Error] with the given [ErrorKind] and source error.
    pub fn new<T: Into<BoxError>>(kind: ErrorKind, source: T) -> Self {
        Error {
            kind,
            source: source.into(),
        }
    }

    /// A helper to create a new serialization or deserialization error.
    pub fn serde<T: Into<BoxError>>(source: T) -> Self {
        Error::new(ErrorKind::Serde, source)
    }

    /// A helper to create a new authentication error.
    pub fn authentication<T: Into<BoxError>>(source: T) -> Self {
        Error::new(ErrorKind::Authentication, source)
    }

    /// A helper to create a new I/O error.
    pub fn io<T: Into<BoxError>>(source: T) -> Self {
        Error::new(ErrorKind::Io, source)
    }

    /// A helper to create a new RPC error.
    pub fn rpc<T: Into<BoxError>>(source: T) -> Self {
        Error::new(ErrorKind::RPC, source)
    }

    /// A helper to create a new uncategorized error.
    pub fn other<T: Into<BoxError>>(source: T) -> Self {
        Error::new(ErrorKind::Other, source)
    }

    /// Returns the [ErrorKind] associated with this error.
    pub fn kind(&self) -> ErrorKind {
        self.kind.clone()
    }

    /// Recuses through the source error chain and returns some reference to the
    /// inner value if it is of type `T`, or `None` if it isn't found.
    pub fn as_inner<T: std::error::Error + Send + Sync + 'static>(&self) -> Option<&T> {
        let mut error = self.source.as_ref() as &(dyn std::error::Error);
        loop {
            match error.downcast_ref::<T>() {
                Some(e) => return Some(e),
                None => error = error.source()?,
            }
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.kind, self.source)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.source.as_ref())
    }
}

// impl From<HttpError> for Error {
//     fn from(err: HttpError) -> Self {
//         Error::new(ErrorKind::RPC, Some(err))
//     }
// }

#[derive(Clone, Debug, PartialEq)]
pub enum ErrorKind {
    /// A serialization or deserialization error.
    Serde,
    /// An authentication error.
    Authentication,
    /// An I/O error.
    Io,
    /// An error related to making a RPC.
    RPC,
    /// A uncategorized error.
    Other,
}

impl Default for ErrorKind {
    fn default() -> Self {
        ErrorKind::Other
    }
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::Serde => write!(
                f,
                "a problem occurred during serialization or deserialization"
            ),
            ErrorKind::Authentication => write!(f, "a problem occurred during authentication"),
            ErrorKind::Io => write!(f, "a problem occurred during I/O"),
            ErrorKind::RPC => write!(f, "a problem occurred while making a RPC"),
            ErrorKind::Other => write!(f, "a problem occurred"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Default)]
    struct LeafError {}
    impl LeafError {
        fn hey(&self) -> &'static str {
            "hey"
        }
    }
    impl std::fmt::Display for LeafError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "other error")
        }
    }
    impl std::error::Error for LeafError {}

    #[derive(Debug)]
    struct MiddleError {
        pub source: Box<dyn std::error::Error + Send + Sync>,
    }
    impl std::fmt::Display for MiddleError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "holder error")
        }
    }
    impl std::error::Error for MiddleError {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            Some(self.source.as_ref())
        }
    }

    #[test]
    fn downcast() -> Result<(), Box<dyn std::error::Error>> {
        let leaf_err = LeafError::default();
        let middle_err = MiddleError {
            source: Box::new(leaf_err),
        };
        let root_err = Error::new(ErrorKind::Other, middle_err);
        let msg = root_err.as_inner::<LeafError>().unwrap().hey();
        assert_eq!(msg, "hey");
        Ok(())
    }
}
