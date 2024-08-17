use crate::prelude::*;

#[derive(Component, Serialize, Deserialize)]
pub struct ActionData(HashMap<String, Action>);

impl std::ops::Deref for ActionData {
    type Target = HashMap<String, Action>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ActionData {
    pub fn new(actor: Name) -> Self {
        Self(load_action_data(actor.into()))
    }
}

#[derive(Serialize, Deserialize)]
pub struct ActorData {
    /// Actor's name
    pub name: String,
    // pub health: i32,
    // pub forward_walk: i32,
    // pub backward_walk: i32,
    // pub jump_velocity: i32,
    // pub jump_deceleration: i32,
    // pub jump_forward: i32,
    // pub jump_backward: i32,
    // pub origin: Vec2,
    // pub pushbox: Boxes,
    /// Used for deserializing ActionData
    pub actions: Vec<Action>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub name: String,
    pub total: u32,
    pub looping: bool,
    // pub pushboxes: Option<Vec<Pushbox>>,
    // pub hurtboxes: Option<Vec<Hurtbox>>,
    // pub hitboxes: Option<Vec<Hitbox>>,
    pub modifiers: Option<Modifiers>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Modifiers {
    pub positions: Option<Vec<PositionModifier>>,
    pub cancels: Option<Vec<CancelModifier>>,
    // pub proximity: Option<ProximityBox>,
    // pub meter: Option<MeterModifier>,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct PositionModifier {
    pub on_frame: u32,
    pub value: IVec2,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelModifier {
    pub on: Option<Vec<CollisionType>>,
    pub after_frame: u32,
    pub until_frame: Option<u32>,
    pub states: Vec<States>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum CollisionType {
    #[default]
    Whiff,
    Hit,
    Block,
    Parry,
}
