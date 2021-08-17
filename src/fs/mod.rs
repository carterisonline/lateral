use self::file::{Directory, File, FileType};

pub mod file;

const FS_MAX_SIZE: usize = 0x400 * 0x400 * 10;
const DATA_SEP: u8 = 0x1E;
// const UNIT_SEP: u8 = 0x1F;

const TYPE_FILE: u8 = 0x11;
const TYPE_DIR: u8 = 0x12;

macro_rules! write_unit_filesystem {
    ($self: ident type $ft: ident ... $label: expr, $data: expr) => {
        $self.heading = $self.ptr;
        for i in $label {
            $self.index[$self.heading] = *i;
            $self.heading += 1;
        }

        $self.index[$self.heading] = DATA_SEP;
        $self.heading += 1;
        $self.index[$self.heading] = $ft;

        for i in 0..4 {
            $self.heading += 1;
            $self.index[$self.heading] = ($data & (0x000F << (i * 8))) as u8 >> (i * 8)
        }

        $self.heading += 1;
    };
}

pub struct Filesystem {
    index: [u8; const { FS_MAX_SIZE * (1 / 8) }],
    fs: [u8; const { FS_MAX_SIZE * (7 / 8) }],
    heading: usize,
    ptr: usize,
}

impl Filesystem {
    pub fn new() -> Self {
        Self {
            index: [0; const { FS_MAX_SIZE * (1 / 8) }],
            fs: [0; const { FS_MAX_SIZE * (7 / 8) }],
            heading: 0,
            ptr: 0,
        }
    }

    pub fn read_unit(&mut self) -> FileType {
        self.heading = self.ptr;
        while self.index[self.heading] != DATA_SEP {
            self.heading += 1;
        }

        let end_of_label = self.heading;

        self.heading += 1;
        let filetype = self.index[self.heading];

        let mut data = 0usize;

        for i in 0..4 {
            self.heading += 1;
            data |= (self.index[self.heading] as usize) << (8 * i);
        }

        self.heading += 1;

        if filetype == TYPE_FILE {
            FileType::Directory(Directory {
                label: &self.fs[self.ptr..end_of_label],
                data,
            })
        } else {
            FileType::File(File {
                label: &self.fs[self.ptr..end_of_label],
                data,
            })
        }
    }

    pub fn forward(&mut self) {
        self.ptr = self.heading;
    }

    pub fn write_unit(&mut self, file: FileType) {
        match file {
            FileType::Directory(dir) => {
                write_unit_filesystem!(self type TYPE_DIR ... dir.label, dir.data);
            }
            FileType::File(file) => {
                write_unit_filesystem!(self type TYPE_FILE ... file.label, file.data);
            }
        }
    }
}
