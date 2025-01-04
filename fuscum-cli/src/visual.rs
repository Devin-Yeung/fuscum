use rinja::Template;
use crate::summary::Summary;

#[derive(Template)]
#[template(path = "network.html")]
pub struct NetworkTemplate<'a> {
    data: &'a Vec<Summary>,
    threshold: f32,
}

impl NetworkTemplate<'_> {
    pub fn new(data: &Vec<Summary>, threshold: f32) -> NetworkTemplate {
        NetworkTemplate { data, threshold }
    }
}