use thiserror::Error;

#[derive(Debug, Error)]
pub enum JellyEcsError {
    #[error("Attempted to add a component to an entity without calling create_entity first")]
    CreateEntityNeverCalled,
    #[error("Attempted to reference a component that wasn't registered")]
    ComponentNotRegistered,
    #[error("Attempted to reference an entity that doesn't exist")]
    EntityDoesNotExist,
}
