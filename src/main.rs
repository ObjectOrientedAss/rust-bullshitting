mod vector2;
mod ecs;

fn main()
{
    test_vectors();
    test_ecs();
}

fn test_vectors()
{
    let mut v1 = vector2::Vector2::New(1.0, 2.0);
    let v2 = vector2::Vector2::New(1.0, 2.0);

    v1 += v2;
    println!("v1 += v2 {} {}", v1.x, v1.y);
    v1 -= v2;
    println!("v1 += v2 {} {}", v1.x, v1.y);
    v1 *= 2.0;
    println!("v1 *= 2.0 {} {}", v1.x, v1.y);
    v1 /= 2.0;
    println!("v1 /= 2.0 {} {}", v1.x, v1.y);
    
    let mut v3 = v1 + v2;
    println!("v1 + v2 {} {}", v3.x, v3.y);
    v3 = v1 - v2;
    println!("v1 - v2 {} {}", v3.x, v3.y);
    v3 = v1 * 2.0;
    println!("v1 * 2.0 {} {}", v3.x, v3.y);
    v3 = v1 / 2.0;
    println!("v1 / 2.0 {} {}", v3.x, v3.y);
    
    let dot = vector2::Vector2::Dot(&v1, &v2);
    println!("v1 Dot v2 {}", dot);
}

fn test_ecs()
{
    let mut world_instance = ecs::World::new();
    let new_entity_id = world_instance.new_entity();

    world_instance.add_component(new_entity_id, ecs::Health(100, 100));
    world_instance.add_component(new_entity_id, ecs::Data("Manuel", "Player"));
    world_instance.add_component(new_entity_id, ecs::Transform(vector2::Vector2::New(60.0, 50.0)));
}