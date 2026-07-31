use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("JSON invalide : {0}")]
    InvalidJson(String),

    #[error("Échec de compilation Typst : {0}")]
    TypstCompilation(String),

    #[error("Ressource introuvable : {0}")]
    ResourceNotFound(String),

    #[error("Entrée invalide : {0}")]
    InvalidInput(String),
}

pub type Result<T> = std::result::Result<T, AppError>;
