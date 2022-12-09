pub struct Path {
    pub prefix: String,
}

impl Path {
    pub fn define(&self, follow_on: String) -> String {
        self.prefix.to_owned() + &follow_on
    }
}