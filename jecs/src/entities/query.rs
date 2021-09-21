use super::Entities;
use crate::entities::Component;
use crate::errors::JecsError;
use eyre::Result;
use std::any::{Any, TypeId};

pub type QueryIndexes = Vec<usize>;
pub type QueryComponents = Vec<Vec<Component>>;

pub struct QueryResult {
    pub indexes: QueryIndexes,
    pub result: QueryComponents
}

impl QueryResult {
    pub fn new(indexes: QueryIndexes, result: QueryComponents) -> Self { Self { indexes, result } }
}

#[derive(Debug)]
pub struct Query<'a> {
    bitmap: u32,
    entities: &'a Entities,
    type_ids: Vec<TypeId>,
}

impl<'a> Query<'a> {
    pub fn new(entities: &'a Entities) -> Self {
        Self {
            entities,
            bitmap: 0,
            type_ids: vec![],
        }
    }

    pub fn with_component<T: Any>(&mut self) -> Result<&mut Self> {
        let type_id = TypeId::of::<T>();
        if let Some(bit_mask) = self.entities.get_bit_mask(&type_id) {
            self.bitmap |= bit_mask;
            self.type_ids.push(type_id);
        } else {
            return Err(JecsError::ComponentNotRegistered.into());
        }

        Ok(self)
    }

    pub fn run(&self) -> QueryResult {
        let indexes: Vec<usize> = self
            .entities
            .bitmap
            .iter()
            .enumerate()
            .filter_map(|(index, entity_map)| {
                if entity_map & self.bitmap == self.bitmap {
                    Some(index)
                } else {
                    None
                }
            })
            .collect();

        let mut result = vec![];

        for type_id in &self.type_ids {
            let entity_components = self.entities.components.get(type_id).unwrap();
            let mut components_to_keep = vec![];
            for index in &indexes {
                components_to_keep.push(entity_components[*index].as_ref().unwrap().clone());
            }

            result.push(components_to_keep);
        }

        QueryResult::new(indexes, result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn query_mask_updating_with_component() -> Result<()> {
        let mut entities = Entities::default();
        entities.register_component::<u32>();
        entities.register_component::<f32>();

        let mut query = Query::new(&entities);
        query.with_component::<u32>()?.with_component::<f32>()?;

        assert_eq!(query.bitmap, 3);
        assert_eq!(TypeId::of::<u32>(), query.type_ids[0]);
        assert_eq!(TypeId::of::<f32>(), query.type_ids[1]);
        Ok(())
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn run_query() -> Result<()> {
        let mut entities = Entities::default();
        entities.register_component::<u32>();
        entities.register_component::<f32>();

        entities
            .create_entity()
            .with_component(10_u32)?
            .with_component(16.0_f32)?;
        entities.create_entity().with_component(20_u32)?;
        entities.create_entity().with_component(32.0_f32)?;
        entities
            .create_entity()
            .with_component(16_u32)?
            .with_component(64.0_f32)?;

        let query = Query::new(&entities)
            .with_component::<u32>()?
            .with_component::<f32>()?
            .run();

        let u32s = &query.result[0];
        let f32s = &query.result[1];
        let indexes = &query.indexes;

        assert!(u32s.len() == f32s.len() && u32s.len() == indexes.len());
        assert_eq!(u32s.len(), 2);

        let borrowed_first_u32 = u32s[0].borrow();
        let first_u32 = borrowed_first_u32.downcast_ref::<u32>().unwrap();
        assert_eq!(*first_u32, 10);

        let borrowed_first_f32 = f32s[0].borrow();
        let first_f32 = borrowed_first_f32.downcast_ref::<f32>().unwrap();
        assert_eq!(*first_f32, 16.0);

        let borrowed_second_u32 = u32s[1].borrow();
        let second_u32 = borrowed_second_u32.downcast_ref::<u32>().unwrap();
        assert_eq!(*second_u32, 16);

        let borrowed_second_f32 = f32s[1].borrow();
        let second_f32 = borrowed_second_f32.downcast_ref::<f32>().unwrap();
        assert_eq!(*second_f32, 64.0);

        assert_eq!(indexes[0], 0);
        assert_eq!(indexes[1], 3);

        Ok(())
    }
}
