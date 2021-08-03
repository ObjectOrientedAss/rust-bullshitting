use std::vec::Vec;
use super::vector2::Vector2;

pub struct Health(pub i32, pub i32); //max and starting
pub struct Data(pub &'static str, pub &'static str); //name and tag
pub struct Transform(pub Vector2); //position

trait ComponentVec
{
    fn push_none(& mut self);
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

impl<T: 'static> ComponentVec for Vec<Option<T>>
{
    //push a null component into the vector
    fn push_none(& mut self)
    {
        self.push(None);
    }

    //converts T to &Any
    fn as_any(&self) -> &dyn std::any::Any
    {
        self as &dyn std::any::Any
    }

    //converts T to &mut Any
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any
    {
        self as &mut dyn std::any::Any
    }
}

pub struct World
{
    entities_count: usize,
    component_vecs: Vec<Box<dyn ComponentVec>>,
}

impl World
{
    //create an instance of World, it returns the instance
    pub fn new() -> Self //Self capital means "of the type of self"
    {
        Self
        {
            entities_count: 0, //the number of entities
            component_vecs: Vec::new(), //the components containers
        } //<- return this
    }

    pub fn new_entity(&mut self) -> usize //this function returns the created entity id
    {
        let entity_id = self.entities_count; //set this entity id to be the the entities count
        for component_vec in self.component_vecs.iter_mut() //iterate all the component containers
        {
            component_vec.push_none(); //add a null value into each component container
        }
        self.entities_count += 1; //increase the entities count
        entity_id //<- no ";" : it means this value is returned
    }

    pub fn add_component<ComponentType: 'static>(&mut self, entity_id: usize, component: ComponentType)
    {
        for component_vec in self.component_vecs.iter_mut()
        {
            //convert the component vector to &mut any, then downcast to &mut <Vec<Option<ComponentType>>>
            if let Some(component_vec) = component_vec.as_any_mut().downcast_mut::<Vec<Option<ComponentType>>>()
            {
                //access to the component vector of this type and add the component at the entity id index.
                component_vec[entity_id] = Some(component);
                return;
            }
        }

        //--- if we are here, it means that a component vec of the given component type, doesn't exist
            
        //create a new component vector of the given componeny type
        let mut new_component_vec : Vec<Option<ComponentType>> = Vec::with_capacity(self.entities_count);
        for _entity in 0..self.entities_count
        {
            //add a null component for each existing entity
            new_component_vec.push(None);
        }

        //now give the component to this entity.
        new_component_vec[entity_id] = Some(component);
        self.component_vecs.push(Box::new(new_component_vec));
    }
}