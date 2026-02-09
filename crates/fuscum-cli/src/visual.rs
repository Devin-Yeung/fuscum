use crate::summary::Summaries;
use rinja::Template;

#[derive(Template)]
#[template(path = "network.html")]
pub struct NetworkTemplate<'a> {
    data: &'a Summaries,
    threshold: f32,
}

impl NetworkTemplate<'_> {
    pub fn new(data: &Summaries, threshold: f32) -> NetworkTemplate<'_> {
        NetworkTemplate { data, threshold }
    }
}
