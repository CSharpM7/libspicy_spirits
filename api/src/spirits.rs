pub const RULESET_TIME: i32 = 0;
pub const RULESET_STOCK: i32 = 1;
pub const RULESET_STAMINA: i32 = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SpiritEnemy {
    pub kind: i32,
    pub color: i32,
}
impl SpiritEnemy {
    pub(crate) fn new() -> Self {
        Self {
            kind: 0,
            color: 0,
        }
    }
}
impl PartialEq for SpiritEnemy {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind &&
        (self.color == other.color ||
        (self.color * other.color == -1))
    }
}
fn do_vecs_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
    let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
    matching == a.len() && matching == b.len()
}
fn do_enemies_match(a: &Vec<SpiritEnemy>, b: &Vec<SpiritEnemy>) -> bool {
    if a.len() != b.len() {return false;}
    for i in 0..a.len()-1 {
        if (a[i] != b[i]) {
            return false;
        }
    }

    true
}
const EPSILON: f32 = 0.001;
fn approximate_eq(n1: f32, n2: f32) -> bool {
  (n1 - n2).abs() < EPSILON
}

#[repr(C)]
#[derive(Clone)]
pub struct SpiritBattle {
    pub battle_id: u64,
    pub battle_type: i32,
    pub basic_init_hp: f32,
    pub basic_stock: u64,
    pub stage_id: i32,
    pub enemies: Vec<SpiritEnemy>
}
impl SpiritBattle {
    pub(crate) fn new() -> Self {
        Self {
            battle_id: 0,
            battle_type: 0,
            basic_init_hp: 0.0,
            basic_stock: 0,
            stage_id: 0,
            enemies: vec![]
        }
    }
}
impl PartialEq for SpiritBattle {
    fn eq(&self, other: &Self) -> bool {
        let rule_match = self.battle_type == other.battle_type &&
        approximate_eq(self.basic_init_hp,other.basic_init_hp) &&
        self.basic_stock == other.basic_stock;
        let stage_match = self.stage_id == other.stage_id;
        let enemy_match = do_enemies_match(&self.enemies,&other.enemies);
        //println!("R: {rule_match} E: {enemy_match} S: {stage_match}");
        return rule_match && stage_match && enemy_match; 
    }
}