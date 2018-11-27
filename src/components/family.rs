///Family describes a group of entities who know who is next in line
//TODO: Probably don't actually need Family any more, Walker is probably an equiv set
#[derive(Debug, Clone, Default)]
pub struct Family;

///Matriarch is the head of the family, there should only be one per family
#[derive(Debug, Clone, Default)]
pub struct Matriarch;