use eyre::Result;
use jecs::World;

#[test]
fn create_entity() -> Result<()> {
    let mut world = World::new();
    world.register_component::<Location>();
    world.register_component::<Size>();

    world
        .create_entity()
        .with_component(Location(16.0, 64.0))?
        .with_component(Size(10.0))?;

    Ok(())
}

#[test]
#[allow(clippy::float_cmp)]
fn query_for_entities() -> Result<()> {
    let mut world = World::new();
    world.register_component::<Location>();
    world.register_component::<Size>();

    world
        .create_entity()
        .with_component(Location(16.0, 64.0))?
        .with_component(Size(10.0))?;
    world.create_entity().with_component(Size(32.0))?;
    world.create_entity().with_component(Location(20.0, 80.0))?;
    world
        .create_entity()
        .with_component(Location(32.0, 128.0))?
        .with_component(Size(20.0))?;

    let query = world
        .query()
        .with_component::<Location>()?
        .with_component::<Size>()?
        .run();

    let locations = &query.1[0];
    let sizes = &query.1[1];

    assert_eq!(locations.len(), 2);
    assert_eq!(sizes.len(), 2);

    let borrowed_first_location = locations[0].borrow();
    let first_location = borrowed_first_location.downcast_ref::<Location>().unwrap();
    assert_eq!(first_location.0, 16.0);

    let borrowed_first_size = sizes[0].borrow();
    let first_size = borrowed_first_size.downcast_ref::<Size>().unwrap();
    assert_eq!(first_size.0, 10.0);

    let borrowed_second_location = locations[1].borrow();
    let second_location = borrowed_second_location.downcast_ref::<Location>().unwrap();
    assert_eq!(second_location.0, 32.0);

    let mut borrowed_second_size = sizes[1].borrow_mut();
    let second_size = borrowed_second_size.downcast_mut::<Size>().unwrap();
    second_size.0 += 10.0;
    assert_eq!(second_size.0, 30.0);

    Ok(())
}

#[test]
fn delete_component_from_entity() -> Result<()> {
    let mut world = World::new();
    world.register_component::<Location>();
    world.register_component::<Size>();

    world
        .create_entity()
        .with_component(Location(16.0, 64.0))?
        .with_component(Size(10.0))?;
    world
        .create_entity()
        .with_component(Location(32.0, 128.0))?
        .with_component(Size(20.0))?;

    world.delete_component_by_entity_id::<Location>(0)?;

    let query = world
        .query()
        .with_component::<Location>()?
        .with_component::<Size>()?
        .run();

    assert_eq!(query.0.len(), 1);
    assert_eq!(query.0[0], 1);

    Ok(())
}

#[test]
fn add_component_to_entity() -> Result<()> {
    let mut world = World::new();
    world.register_component::<Location>();
    world.register_component::<Size>();

    world.create_entity().with_component(Location(16.0, 64.0))?;

    world.add_component_by_entity_id(Size(20.0), 0)?;

    let query = world
        .query()
        .with_component::<Location>()?
        .with_component::<Size>()?
        .run();

    assert_eq!(query.0.len(), 1);
    Ok(())
}

#[test]
#[allow(clippy::float_cmp)]
fn delete_entity() -> Result<()> {
    let mut world = World::new();
    world.register_component::<Location>();

    world.create_entity().with_component(Location(16.0, 64.0))?;
    world
        .create_entity()
        .with_component(Location(32.0, 128.0))?;

    world.delete_entity_by_id(0)?;

    let query = world.query().with_component::<Location>()?.run();
    assert_eq!(query.0.len(), 1);

    let borrowed_location = query.1[0][0].borrow();
    let location = borrowed_location.downcast_ref::<Location>().unwrap();
    assert_eq!(location.0, 32.0);

    world
        .create_entity()
        .with_component(Location(64.0, 256.0))?;

    let query = world.query().with_component::<Location>()?.run();
    assert_eq!(query.0.len(), 2);

    let borrowed_location = query.1[0][0].borrow();
    let location = borrowed_location.downcast_ref::<Location>().unwrap();
    assert_eq!(location.0, 64.0);
    Ok(())
}

struct Location(pub f32, pub f32);
struct Size(pub f32);
