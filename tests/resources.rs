use ezs::World;
use std::ops::Deref;

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
        let fps: &mut FPSResource = world.get_resource_mut::<FPSResource>().unwrap();
        fps.0 += 10;
    }
    let fps = world.get_resource::<FPSResource>().unwrap();
    assert_eq!(fps.0, 70);
}

fn initialize_world() -> World {
    let mut world = World::new();
    world.add_resource(FPSResource(60));
    world
}

#[derive(Debug)]
struct FPSResource(pub u32);

impl Deref for FPSResource {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
