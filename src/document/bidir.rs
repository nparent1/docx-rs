use hard_xml::{XmlRead, XmlWrite};
use std::borrow::{Cow};

use super::Run;

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:dir")]
pub struct BiDirectionalEmbedding<'a> {
    // A BiDirectionalEmbedding can have a number of rich text runs
    #[xml(child = "w:r")]
    pub runs: Vec<Run<'a>>,
    // A BiDirectionalEmbedding can include nested embedding layers
    #[xml(child = "w:dir")]
    pub nested_levels: Vec<BiDirectionalEmbedding<'a>>
}

impl<'a> BiDirectionalEmbedding<'a> {
    pub fn iter_text(&self) -> Box<dyn Iterator<Item = &Cow<'a, str>> + '_> {
        Box::new(
            self.runs.iter().flat_map(|run| run.iter_text()).chain(
                self.nested_levels.iter().flat_map(|nested| nested.iter_text())
            )
        )
    }

    pub fn iter_text_mut(&mut self) -> Box<dyn Iterator<Item = &mut Cow<'a, str>> + '_> {
        Box::new(self.runs.iter_mut()
            .flat_map(|run| run.iter_text_mut())
            .chain(
                self.nested_levels.iter_mut().flat_map(|nested| nested.iter_text_mut())
            )
        )
    }
}
