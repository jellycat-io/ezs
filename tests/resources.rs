use ezs::World;
use std::ops::Deref;

#[test]
fn create_and_get_resource_immutably() {
    let mut world = World::default();
    world.add_resource(FPSResource(60));
    let fps = world.get_resource::<FPSResource>().unwrap();
    assert_eq!(fps.0, 60);
}

struct FPSResource(pub u32);

impl Deref for FPSResource {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
