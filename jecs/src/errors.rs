use thiserror::Error;

#[derive(Debug, Error)]
pub enum JecsError {
    #[error("Attempted to reference a resource that wasn't registered")]
    ResourceNotFound,
    #[error("Attempted to add a component to an entity without calling create_entity first")]
    CreateEntityNeverCalled,
    #[error("Attempted to reference a component that wasn't registered")]
    ComponentNotRegistered,
    #[error("Attempted to reference an entity that doesn't exist")]
    EntityDoesNotExist,
    #[error("Error downcasting type")]
    DowncastError
}
