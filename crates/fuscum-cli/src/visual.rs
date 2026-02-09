use crate::summary::Summary;
use rinja::Template;

#[derive(Template)]
#[template(path = "network.html")]
pub struct NetworkTemplate<'a> {
    data: &'a Vec<Summary>,
    threshold: f32,
}

impl NetworkTemplate<'_> {
    pub fn new(data: &Vec<Summary>, threshold: f32) -> NetworkTemplate<'_> {
        NetworkTemplate { data, threshold }
    }
}
