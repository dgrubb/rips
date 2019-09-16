//! TODO: Complete this documentation

pub struct PatchEntry<> {
    offset: u32,
    length: u32,
    is_rle: bool,
    payload: Vec<u8>
}

/// Determines the index of an IPS header:
/// "PATCH" (50, 41, 54, 43, 48)
/// Returns an index of 0 if no header was found.
pub fn verify_header_index(data: Vec<u8>) -> u32 {
    0
}

/// Determines the index of an IPS end-of-file marker:
/// "EOF" (45 4f 46)
/// Returns an index of 0 if no header was found.
pub fn verify_eof_marker(data: Vec<u8>) -> u32 {
    0
}

/// Scans raw patch file data (list of bytes) and converts 
/// each encountered patch instruction sequence into a struct
/// representing the change.
/// Returns a list of patches.
pub fn get_patch_list(data: Vec<u8>) -> Vec<PatchEntry> {
}

/// Applies a list of patches to raw patch file data.
/// Returns a copy of the input data with patches applied.
pub fn apply_patch_list(data: Vec<u8>, patches: Vec<PatchEntry>) -> Vec<u8> {
}

/// Pretty prints a list of PatchEntry instances.
pub fn print_patch_list(data: Vec<PatchEntry>) {
    
}

fn apply_patch(data: Vec<u8>, patch: PatchEntry) {

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
