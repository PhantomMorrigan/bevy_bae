use alloc::collections::VecDeque;
use core::any::TypeId;

use disqualified::ShortName;

use crate::{
    prelude::*,
    task::{
        BaeTask,
        primitive::OperatorId,
        validation::{
            BaeTaskPresent, insert_bae_task_present_on_add, remove_bae_task_present_on_remove,
        },
    },
};

pub mod relationship;
pub mod select;
pub mod sequence;

pub trait CompoundTask: Component {
    fn decompose(
        entity: Entity,
        world: &World,
        props: &mut Props,
        tasks: &mut VecDeque<OperatorId>,
        index: usize,
    );
}

impl<T: CompoundTask> BaeTask for T {}

#[derive(Component)]
struct TypeErasedCompoundTask {
    entity: Entity,
    name: ShortName<'static>,
    type_id: TypeId,
    decompose: for<'a> fn(
        entity: Entity,
        world: &'a World,
        props: &'a mut Props,
        tasks: &'a mut VecDeque<OperatorId>,
        index: usize,
    ),
}

impl TypeErasedCompoundTask {
    #[must_use]
    fn new<C: CompoundTask>(entity: Entity) -> Self {
        Self {
            entity,
            name: ShortName::of::<C>(),
            type_id: TypeId::of::<C>(),
            decompose: C::decompose,
        }
    }
}

pub trait CompoundAppExt {
    fn add_compound_task<C: CompoundTask>(&mut self) -> &mut Self;
}

impl CompoundAppExt for App {
    fn add_compound_task<C: CompoundTask>(&mut self) -> &mut Self {
        self.add_observer(insert_type_erased_task::<C>)
            .add_observer(remove_type_erased_task::<C>)
            .add_observer(insert_bae_task_present_on_add::<C>)
            .add_observer(remove_bae_task_present_on_remove::<C>);
        let _ = self.try_register_required_components::<C, BaeTaskPresent>();
        self
    }
}

fn insert_type_erased_task<C: CompoundTask>(insert: On<Insert, C>, mut commands: Commands) {
    commands
        .entity(insert.entity)
        .try_insert(TypeErasedCompoundTask::new::<C>(insert.entity));
}
fn remove_type_erased_task<C: CompoundTask>(remove: On<Remove, C>, mut commands: Commands) {
    commands
        .entity(remove.entity)
        .try_remove::<TypeErasedCompoundTask>();
}
