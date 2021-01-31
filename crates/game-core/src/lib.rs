




//not sure the usefulness of a core but will see later. Might come in use when I think of all systems working together.



pub mod stages {
    pub const INIT: &'static str = "init";
    pub const POST_INIT: &'static str = "post_init";
    pub const PRE_UPDATE: &'static str = "pre_update";
    pub const UPDATE: &'static str = "update";
    pub const POST_UPDATE: &'static str = "post_update";
}




















#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
