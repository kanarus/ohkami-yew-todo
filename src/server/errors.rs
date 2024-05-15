use ohkami::prelude::*;


#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    #[error("Error in worker: {0}")]
    Worker(#[from] worker::Error),

    #[error("User(id = {user_id}) requested operation on {resource} whitch is NOT of the user")]
    NotOwner { user_id: String, resource: &'static str },
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        worker::console_error!("{self}");

        match self {
            Self::Worker  {..} => Response::InternalServerError(),
            Self::NotOwner{..} => Response::Forbidden(),
        }
    }
}
