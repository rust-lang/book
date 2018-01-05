pub struct Context<'a>(&'a str);

pub struct Parser<'a> {
    pub context: &'a Context<'a>,
}

impl<'a> Parser<'a> {
    pub fn parse(&self) -> Result<(), &str> {
        Err(&self.context.0[1..])
    }
}

fn main() {}
