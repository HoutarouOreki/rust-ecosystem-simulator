pub enum OrganismResult {
    None,
    AteOtherOrganism { other_organism_id: u64 },
    HadChildren { amount: u64 },
}
