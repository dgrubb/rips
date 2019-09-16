//! TODO: Complete this documentation
use std::result::Result::{self, Ok, Err};

mod rips {
    pub struct PatchEntry {
        offset: u32,
        length: u32,
        is_rle: bool,
        payload: Vec<u8>
    }

    /// Determines the index of an IPS header:
    /// "PATCH" (50, 41, 54, 43, 48)
    /// Returns an index of 0 if no header was found.
    pub fn verify_header_index(_data: Vec<u8>) -> u32 {
        0
    }

    /// Determines the index of an IPS end-of-file marker:
    /// "EOF" (45 4f 46)
    /// Returns an index of 0 if no header was found.
    pub fn verify_eof_marker(_data: Vec<u8>) -> u32 {
        0
    }

    /// Scans raw patch file data (list of bytes) and converts
    /// each encountered patch instruction sequence into a struct
    /// representing the change.
    /// Returns a list of patches.
    pub fn get_patch_list(_data: Vec<u8>) -> Result<Vec<PatchEntry>, String> {
        Err("This function requires completion".to_string())
    }

    /// Applies a list of patches to raw patch file data.
    /// Returns a copy of the input data with patches applied.
    pub fn apply_patch_list(_data: Vec<u8>, _patches: Vec<PatchEntry>) -> Result<Vec<u8>, String> {
        Err("This function requires completion".to_string())
    }

    /// Pretty prints a list of PatchEntry instances.
    pub fn print_patch_list(_data: Vec<PatchEntry>) {

    }

    fn apply_patch(_data: Vec<u8>, _patch: PatchEntry) {

    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

