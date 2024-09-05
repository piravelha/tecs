use uuid::Uuid;
use std::collections::HashMap;

struct Entity<C> {
  id: Uuid,
  component: C,
}

impl<C> Entity<C> {
  fn new(component: C) -> Self {
    Self {
      id: Uuid::new_v4(),
      component,
    }
  }
}

struct World<C> {
  entities: Vec<Uuid>,
  components: HashMap<Uuid, C>,
}

impl<C> World<C> {
  fn new() -> Self {
    Self {
      entities: vec![],
      components: HashMap::new(),
    }
  }

  fn spawn(&mut self, entity: Entity<C>) {
    self.entities.push(entity.id);
    self.components.insert(entity.id, entity.component);
  }
}

// Usage

#[derive(Debug, Clone)]
struct Position(i32, i32);

#[derive(Debug, Clone)]
struct Name(String);

struct Component {
  name: Option<Name>,
  position: Option<Position>,
}

impl Component {
  fn new() -> Self {
    Self {
      name: None,
      position: None,
    }
  }

  fn with_position(self, position: Position) -> Self {
    Self {
      position: Some(position),
      ..self
    }
  }

  fn with_name(self, name: Name) -> Self {
    Self {
      name: Some(name),
      ..self
    }
  }
}

macro_rules! query {
  ( $world:ident , $($comp:ident),+ $(,)? ) => {
    $world.entities.iter().filter_map(|entity| {
      let comps = $world.components.get(entity)?;
      Some((
        entity,
        $(comps.$comp.clone()?,)+
      ))
    })
  }
}

fn movement_system(world: &World<Component>) {
  let entities = query!(world, position);
  for (id, pos) in entities {
    println!("(ID: {})", id);
    println!("[MOVEMENT] {:?}", pos);
  }
}

fn greet_system(world: &World<Component>) {
  let entities = query!(world, name);
  for (id, name) in entities {
    println!("(ID: {})", id);
    println!("[NAME] {:?}", name);
  }
}

fn render_system(world: &World<Component>) {
  let entities = query!(world, position, name);
  for (id, pos, name) in entities {
    println!("(ID: {})", id);
    println!("[RENDER] {:?} at {:?}", name, pos);
  }
}

fn main() {
  let mut world = World::new();

  let point = Entity::new(
    Component::new()
      .with_position(Position(3, 4))
  );

  let label = Entity::new(
    Component::new()
      .with_name(Name(String::from("Label")))
  );

  let player = Entity::new(
    Component::new()
      .with_position(Position(0, 0))
      .with_name(Name(String::from("Ian")))
  );

  world.spawn(point);
  world.spawn(label);
  world.spawn(player);
  
  println!("{}", vec!["-"; 50].join(""));
  
  movement_system(&world);
  println!("{}", vec!["-"; 50].join(""));

  greet_system(&world);
  println!("{}", vec!["-"; 50].join(""));

  render_system(&world);
  println!("{}", vec!["-"; 50].join(""));
}
