use bevy_ecs::system::SystemId;
use core::any::TypeId;

use disqualified::ShortName;

use crate::{
    plan::Plan,
    prelude::*,
    task::validation::{
        BaeTaskPresent, insert_bae_task_present_on_add, remove_bae_task_present_on_remove,
    },
};

pub mod relationship;
pub mod select;
pub mod sequence;

pub trait CompoundTask: Send + Sync + 'static {
    fn register_decompose(commands: &mut Commands) -> DecomposeId;
}

pub type DecomposeId = SystemId<In<DecomposeInput>, DecomposeResult>;

#[derive(Debug)]
pub struct DecomposeInput {
    pub root: Entity,
    pub compound_task: Entity,
    pub world_state: Props,
    pub plan: Plan,
}

#[derive(Component, Clone)]
pub(crate) struct TypeErasedCompoundTask {
    pub(crate) entity: Entity,
    pub(crate) name: ShortName<'static>,
    pub(crate) type_id: TypeId,
    pub(crate) decompose: DecomposeId,
}

impl TypeErasedCompoundTask {
    #[must_use]
    fn new<C: CompoundTask>(entity: Entity, id: DecomposeId) -> Self {
        Self {
            entity,
            name: ShortName::of::<C>(),
            type_id: TypeId::of::<C>(),
            decompose: id,
        }
    }
}

pub enum DecomposeResult {
    Success { plan: Plan, world_state: Props },
    Rejection,
    Failure,
}

pub trait CompoundAppExt {
    fn add_compound_task<C: CompoundTask>(&mut self) -> &mut Self;
}

impl CompoundAppExt for App {
    fn add_compound_task<C: CompoundTask>(&mut self) -> &mut Self {
        self.add_observer(insert_type_erased_task::<C>)
            .add_observer(remove_type_erased_task::<C>)
            .add_observer(insert_bae_task_present_on_add::<Tasks<C>>)
            .add_observer(remove_bae_task_present_on_remove::<Tasks<C>>);
        let _ = self.try_register_required_components::<Tasks<C>, BaeTaskPresent>();
        self
    }
}

fn insert_type_erased_task<C: CompoundTask>(insert: On<Insert, Tasks<C>>, mut commands: Commands) {
    let system_id = C::register_decompose(&mut commands);
    commands
        .entity(insert.entity)
        .try_insert(TypeErasedCompoundTask::new::<C>(insert.entity, system_id));
}
fn remove_type_erased_task<C: CompoundTask>(remove: On<Remove, Tasks<C>>, mut commands: Commands) {
    commands
        .entity(remove.entity)
        .try_remove::<TypeErasedCompoundTask>();
}
