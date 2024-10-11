use std::io;

pub trait Saveable {
    fn save_to_file(&self, filename: &str) -> io::Result<()>;
    fn load_from_file(filename: &str) -> io::Result<Self> where Self: Sized;
}