# Jelly ECS

A simple but functional ECS. It was created mostly for game development but should be applicable to other projects.\
Feel free to open issues if bugs are encountered and even contribute with pull requests.

## Example usage

```
use jelly_ecs::World;

struct Position(pub f32, pub f32);
struct Health(pub u32);

fn main() -> Result<()> {
    let mut world = World::new();
    world.register_component::<Position>();
    world.register_component::<Health>();
    
    world.create_entity()
        .with_component(Position(10.0, 20.0))?
        .with_component(Health(100))?;
        
    let query = world
        .query()
        .with_component::<Position>()?
        .with_component::<Health>()?
        .run();
        
    let player = &query.1[0];
}
```