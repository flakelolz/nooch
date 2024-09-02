use crate::prelude::*;

#[derive(Component, Debug, Serialize, Deserialize)]
pub struct ActionData(IndexMap<String, Action>);

impl std::ops::Deref for ActionData {
    type Target = IndexMap<String, Action>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ActionData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ActionData {
    pub fn new(actor: Name) -> Self {
        Self(load_action_data(actor.into()))
    }

    pub fn to_vec(&self) -> Vec<Action> {
        self.0.values().cloned().collect()
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct CharacterData {
    pub name: String,
    pub max_health: i32,
    pub forward_walk: i32,
    pub backward_walk: i32,
    pub jump_velocity: i32,
    pub jump_deceleration: i32,
    pub jump_forward: i32,
    pub jump_backward: i32,
    pub origin: Vec2,
    pub pushbox: Boxes,
}

// Only used for deserializing ActionData
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Actions {
    pub actions: Vec<Action>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Action {
    pub name: String,
    pub total: u32,
    pub looping: bool,
    pub pushboxes: Option<Vec<Pushbox>>,
    pub hurtboxes: Option<Vec<Hurtbox>>,
    pub hitboxes: Option<Vec<Hitbox>>,
    pub modifiers: Option<Modifiers>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Modifiers {
    pub positions: Option<Vec<PositionModifier>>,
    pub cancels: Option<Vec<CancelModifier>>,
    // pub proximity: Option<ProximityBox>,
    // pub meter: Option<MeterModifier>,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq)]
pub struct PositionModifier {
    pub on_frame: u32,
    pub value: IVec2,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq)]
pub struct CancelModifier {
    pub on: Option<Vec<CollisionType>>,
    pub after_frame: u32,
    pub until_frame: Option<u32>,
    pub states: Vec<States>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq)]
pub enum CollisionType {
    #[default]
    Whiff,
    Hit,
    Block,
    Parry,
}

#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize, PartialEq)]
pub struct Pushbox {
    pub start_frame: u32,
    pub duration: u32,
    pub value: Boxes,
}

#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize, PartialEq)]
pub struct Hurtbox {
    pub start_frame: u32,
    pub duration: u32,
    pub height: Height,
    pub invul: Invulnerability,
    pub value: Boxes,
}

#[derive(Debug, Clone, Copy, Default, Eq, Deserialize, Serialize, PartialEq)]
pub enum Height {
    #[default]
    Upper,
    Lower,
}

#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize, PartialEq)]
pub enum Invulnerability {
    #[default]
    None,
    Ground,
    Air,
    Throw,
    Projectile,
    All,
}

#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize, PartialEq)]
pub struct Hitbox {
    pub start_frame: u32,
    pub duration: u32,
    pub properties: HitProperties,
    pub value: Boxes,
}

#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize, PartialEq)]
pub struct HitProperties {
    pub hit_type: HitType,
    pub strength: Strength,
    pub hitstop: u32,
    pub hitstun: u32,
    pub blockstun: u32,
    pub knockback: i32,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize, Serialize)]
pub enum HitType {
    #[default]
    Ground,
    Air,
    Throw,
    Projectile,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize, Serialize)]
pub enum Strength {
    #[default]
    Weak,
    Mid,
    Strong,
    Rising,
    FrontSpin,
    BackSpin,
    // Knockdown,
    // Launch,
}

#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize, PartialEq)]
pub struct Boxes {
    pub top: i32,
    pub bottom: i32,
    pub left: i32,
    pub right: i32,
}
