use std::{
    cell::Cell,
    fs::File,
    io::{Read, Seek, Write},
    iter::Iterator,
    path::Path,
};
use zip::{result::ZipError, write::FileOptions};

use walkdir::{DirEntry, WalkDir};

#[derive(Debug)]
pub struct ZipArchiver {
    m_src_dir: String,
    m_dest_zip_fname: String,
    m_dst_zip_exists: Cell<bool>, // interior mutability
    m_compress_method: zip::CompressionMethod,
}

impl ZipArchiver {
    #[must_use]
    pub fn new(src_dir: &str, dest_zip_fname: &str) -> Self {
        Self {
            m_src_dir: src_dir.to_string(),
            m_dest_zip_fname: dest_zip_fname.to_string(),
            m_dst_zip_exists: Cell::new(false),
            m_compress_method: zip::CompressionMethod::Deflated,
        }
    }

    fn zip_dir<T>(
        &self,
        dir_entries: &mut dyn Iterator<Item = DirEntry>,
        prefix: &str,
        writer: T,
    ) -> zip::result::ZipResult<()>
    where
        T: Write + Seek + Read,
    {
        let mut zip = if self.m_dst_zip_exists.get() {
            zip::ZipWriter::new_append(writer).unwrap()
        } else {
            zip::ZipWriter::new(writer)
        };

        let options = FileOptions::default()
            .compression_method(self.m_compress_method)
            .unix_permissions(0o755);

        let mut file_buf = Vec::new();
        for curr_entry in dir_entries {
            let entry_path = curr_entry.path();
            let entry_name = entry_path
                .strip_prefix(Path::new(prefix))
                .unwrap()
                .to_str()
                .unwrap();

            // Write file or directory explicitly
            // Some unzip tools unzip files with directory paths correctly, some do not!
            if entry_path.is_file() {
                println!("[+] Adding file {:?} as {:?} ...", entry_path, entry_name);
                zip.start_file(entry_name, options)?;
                let mut fh = File::open(entry_path)?;

                fh.read_to_end(&mut file_buf)?;
                zip.write_all(&*file_buf)?;
                file_buf.clear();
            } else if !entry_name.is_empty() {
                // Only if not root! Avoids path spec / warning
                // and mapname conversion failed error on unzip
                println!("[+] Adding dir {:?} as {:?} ...", entry_path, entry_name);
                zip.add_directory(entry_name, options)?;
            }
        }

        zip.finish()?;
        Result::Ok(())
    }

    pub fn run(&self) -> zip::result::ZipResult<()> {
        if !Path::new(&self.m_src_dir).is_dir() {
            return Err(ZipError::FileNotFound);
        }

        let path = Path::new(&self.m_dest_zip_fname);

        if path.exists() {
            self.m_dst_zip_exists.set(true);
        }

        let file = File::options()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)?;

        let dir_content = WalkDir::new(&self.m_src_dir).into_iter();

        self.zip_dir(
            &mut dir_content.filter_map(Result::ok),
            &self.m_src_dir,
            file,
        )?;

        println!("[+] Zip archived: {:?}", &self.m_dest_zip_fname);

        Ok(())
    }
}
