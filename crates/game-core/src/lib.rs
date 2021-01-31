




//not sure the usefulness of a core but will see later. Might come in use when I think of all systems working together.



pub mod stages {
    pub const INIT: &'static str = "init";
    pub const POST_INIT: &'static str = "post_init";
    pub const PRE_UPDATE: &'static str = "pre_update";
    pub const UPDATE: &'static str = "update";
    pub const POST_UPDATE: &'static str = "post_update";
}

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<InputTimer>();
    }
}

#[derive(Debug)]
pub struct InputTimer(pub Timer);
impl Default for InputTimer {
    fn default() -> Self {
        InputTimer(Timer::from_seconds(0.1, false))
    }
}



















#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
