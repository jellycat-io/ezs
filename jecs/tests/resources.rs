use jecs::World;

#[test]
fn create_and_get_resource_immutably() {
    let world = initialize_world();
    let fps = world.get_resource::<FPSResource>().unwrap();
    assert_eq!(fps.0, 60);
}

#[test]
fn get_resource_mutably() {
    let mut world = initialize_world();
    {
        let fps = world.get_resource_mut::<FPSResource>().unwrap();
        fps.0 += 10;
    }
    let fps = world.get_resource::<FPSResource>().unwrap();
    assert_eq!(fps.0, 70);
}

#[test]
fn delete_resource() {
    let mut world = initialize_world();
    world.delete_resource::<FPSResource>();
    let deleted_resource = world.get_resource::<FPSResource>();
    assert!(deleted_resource.is_none());
}

fn initialize_world() -> World {
    let mut world = World::new();
    world.add_resource(FPSResource::new(60));
    world
}

#[derive(Debug)]
struct FPSResource(pub u32);

impl FPSResource {
    pub fn new(fps: u32) -> Self {
        Self(fps)
    }
}
