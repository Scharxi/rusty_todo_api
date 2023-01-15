use std::fmt::{Display, Formatter};
use serde::Serialize;
use crate::response::GenericResponse;

pub trait ToResponse<R: Serialize> {
    fn to_response(&self, message: impl Into<String>) -> R;
}

pub enum ActionStatus {
    Success,
    Failure
}

impl ToResponse<GenericResponse> for ActionStatus {
    fn to_response(&self, message: impl Into<String>) -> GenericResponse {
        GenericResponse {
            status: format!("{self}"),
            message: message.into()
        }
    }
}

impl Display for ActionStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Failure => writeln!(f, "fail"),
            Self::Success => writeln!(f, "success")
        }
    }
}