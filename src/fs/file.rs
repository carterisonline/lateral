pub enum FileType<'a> {
    Directory(Directory<'a>),
    File(File<'a>),
}

/// A label with a pointer to the directory's contents located in the __index__.
pub struct Directory<'a> {
    pub label: &'a [u8],
    pub data: usize,
}

// A label with a pointer to the file's contents located in the __data table__.
pub struct File<'a> {
    pub label: &'a [u8],
    pub data: usize,
}
