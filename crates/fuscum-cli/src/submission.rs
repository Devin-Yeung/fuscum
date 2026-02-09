use anyhow::Result;
use fuscum::doc::{Doc, MultiDoc};
use fuscum::fingerprint::{self, FingerPrintConfig};
use fuscum::kgram::default_rolling_kgram;
use fuscum::preprocess::{CPreprocessor, Preprocessor, PythonPreprocessor};
use std::fs::File;
use std::io::{Cursor, Read};
use std::path::Path;
use tempfile::{tempdir, TempDir};
use walkdir::WalkDir;

pub struct Submission {
    name: String,
    location: TempDir,
}

impl Submission {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let dir = tempdir()?;
        // extract the zip file
        println!("extracting: {:?} to {:?}", path.as_ref(), dir.as_ref());
        let mut archive = File::open(&path)?;
        let mut buf = Vec::new();
        archive.read_to_end(&mut buf)?;
        zip_extract::extract(Cursor::new(buf), dir.as_ref(), false)?;
        // get the name of the file
        let name = path
            .as_ref()
            .file_stem()
            .expect("should get file stem")
            .to_str()
            .expect("should convert to str")
            .to_string();
        Ok(Self {
            location: dir,
            name,
        })
    }
}

impl From<Submission> for MultiDoc {
    fn from(val: Submission) -> Self {
        let mut docs = MultiDoc::new(val.name);

        WalkDir::new(val.location.as_ref())
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .filter_map(|entry| {
                let name = entry
                    .path()
                    .file_name()
                    .expect("should get file name")
                    .to_str()
                    .expect("should convert to str")
                    .to_string();

                let preprocessor: Option<Box<dyn Preprocessor>> = match entry
                    .path()
                    .extension()
                    .map(|ext| ext.to_str().expect("should convert to str"))
                {
                    Some("py") => Some(Box::new(PythonPreprocessor::default())),
                    Some("c") => Some(Box::new(CPreprocessor::default())),
                    _ => None,
                };

                let content = std::fs::read_to_string(entry.path()).ok()?;
                let gen = fingerprint::FingerPrintGenerator {
                    config: FingerPrintConfig::default(),
                    preprocessor: preprocessor.unwrap(),
                    kgram: default_rolling_kgram(),
                };

                let fingerprint = gen.generate(&content);
                Some(Doc::new(name, fingerprint))
            })
            .for_each(|doc| docs.add_doc(doc));

        docs
    }
}
