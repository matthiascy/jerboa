pub enum JerboaError {
    /// An error occurred while reading/writing an image.
    ImageIo(ImageError),
}
