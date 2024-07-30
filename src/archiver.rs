use color_eyre::eyre::Result;
use std::{
    fs::File,
    io::{Read, Write},
    iter::Iterator,
    path::Path,
};

use walkdir::WalkDir;
use zip::{write::FileOptions, ZipWriter};

pub struct ZipArchiver<'a> {
    src: &'a Path,
    dest: &'a Path,
    options: FileOptions,
}

impl<'a> ZipArchiver<'a> {
    #[must_use]
    pub fn new(src: &'a str, dest: &'a str) -> Self {
        let options = FileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated)
            .unix_permissions(0o755);

        Self {
            src: Path::new(src),
            dest: Path::new(dest),
            options,
        }
    }

    fn add_path(&self, zip: &mut ZipWriter<File>, path: &Path) -> Result<()> {
        let name = path.strip_prefix(self.src)?.to_str().unwrap();

        // Write file or directory explicitly
        // Some unzip tools unzip files with directory paths correctly, some do not!
        if path.is_file() {
            log::info!("[+] Adding file {:?} as {:?}...", path, name);
            zip.start_file(name, self.options)?;

            let mut file = File::open(path)?;
            let mut buff = Vec::new();
            file.read_to_end(&mut buff)?;
            zip.write_all(&*buff)?;
            buff.clear();
        } else if !name.is_empty() {
            // Only if not root! Avoids path spec / warning
            // and mapname conversion failed error on unzip
            log::info!("[+] Adding dir {:?} as {:?}...", path, name);
            zip.add_directory(name, self.options)?;
        }

        Ok(())
    }

    pub fn run(&self) -> Result<()> {
        // Individually downloaded files are not placed in the `src` directory,
        // and so their paths are unknown and cannot be added to the zip.
        if !self.src.is_dir() {
            log::warn!("Unable to zip individual files, skipping");
            return Ok(());
        }

        let dest = self.dest;
        let dest_exists = dest.exists();

        let writer = File::options()
            .read(true)
            .write(true)
            .create(true)
            .open(&dest)?;

        let mut zip = if dest_exists {
            zip::ZipWriter::new_append(writer)?
        } else {
            zip::ZipWriter::new(writer)
        };

        for entry in WalkDir::new(&self.src).into_iter().filter_map(Result::ok) {
            self.add_path(&mut zip, entry.path())?;
        }

        zip.finish()?;

        log::info!("[+] Zip archived as {:?}.", &self.dest);

        Ok(())
    }
}
