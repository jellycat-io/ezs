# World

## Resources

- arena_size
    - x
    - y
- background_color
- clicked_location Option(x, y)
- entity_size
- entity_mesh
- sight_range

## Systems

- Draw entities
- Draw the arena
- Move humans randomly
- Move humans towards each other
- Move humans away from each other
- Move humans towards the average velocity of the nearby humans
- Stop humans when reaching arena edges
- Create zombie
- Move zombie towards the nearest human
- Handle zombies biting humans
- Handle dead humans
- Move humans away from zombies

## Entities

### Human
```
location(x, y)
velocity(x, y)
acceleration(x, y)
speed
color
```
### Dead human
```
location(x, y)
color
decay_time
```

### Zombie
```
location(x, y)
velocity(x, y)
acceleration(x, y)
speed
color
```

# Stories
- [x] Setup
    - [x] Arena is drawn
- [ ] Humans
    - [x] Humans move randomly around the arena
    - [ ] Humans are attracted to each other
    - [x] Humans are repelled from each other
    - [ ] Humans are attracted to the average velocity of the group
    - [x] Humans stop at the edge of the arena
    - [ ] Humans move away from zombies they can see
- [ ] Zombies
    - [ ] A zombie can be created with a mouse click or a keyboard press
    - [ ] A zombie will move towards the nearest human unless there are no more humans
    - [ ] Zombies move slower than humans
    - [ ] When a zombie touch a human, the human dies
    - [ ] After 10 seconds the dead human disappears
    - [ ] When the dead human is removed, there is a chance that a zombie will spawn where the human was

# Crates

- [rand](https://crates.io/crates/rand)
- [eyre](https://crates.io/crates/eyre)
- [ggez](https://crates.io/crates/ggez)
- [jecs](https://crates.io/crates/jecs)