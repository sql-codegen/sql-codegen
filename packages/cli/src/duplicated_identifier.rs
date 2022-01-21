#[derive(Debug)]
pub struct DuplicatedIdentifierError {
    // pub file_name: String,
    pub identifier: String,
}

impl DuplicatedIdentifierError {
    pub fn new(
        // file_name: String,
        identifier: String,
    ) -> DuplicatedIdentifierError {
        DuplicatedIdentifierError {
            // file_name,
            identifier,
        }
    }
}
