use std::collections::BTreeSet;
use std::slice::from_raw_parts;
use std::error::Error;
use std::path::Path;
use std::sync::Arc;
use std::{io, mem};

use byteorder::{NativeEndian, WriteBytesExt};
use fst::raw::MmapReadOnly;
use serde::ser::{Serialize, Serializer};

use crate::DocumentId;
use crate::data::Data;

#[derive(Clone)]
pub struct DocIds {
    data: Data,
}

impl DocIds {
    pub unsafe fn from_path<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let mmap = MmapReadOnly::open_path(path)?;
        let data = Data::Mmap(mmap);
        Ok(DocIds { data })
    }

    pub fn from_bytes(vec: Vec<u8>) -> Result<Self, Box<Error>> {
        // FIXME check if modulo DocumentId
        let len = vec.len();
        let data = Data::Shared {
            vec: Arc::new(vec),
            offset: 0,
            len: len
        };
        Ok(DocIds { data })
    }

    pub fn from_document_ids(vec: Vec<DocumentId>) -> Self {
        DocIds::from_bytes(unsafe { mem::transmute(vec) }).unwrap()
    }

    pub fn contains(&self, doc: DocumentId) -> bool {
        // FIXME prefer using the sdset::exponential_search function
        self.doc_ids().binary_search(&doc).is_ok()
    }

    pub fn doc_ids(&self) -> &[DocumentId] {
        let slice = &self.data;
        let ptr = slice.as_ptr() as *const DocumentId;
        let len = slice.len() / mem::size_of::<DocumentId>();
        unsafe { from_raw_parts(ptr, len) }
    }
}

impl Serialize for DocIds {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.data.as_ref().serialize(serializer)
    }
}

pub struct DocIdsBuilder<W> {
    doc_ids: BTreeSet<DocumentId>, // TODO: prefer a linked-list
    wrt: W,
}

impl<W: io::Write> DocIdsBuilder<W> {
    pub fn new(wrt: W) -> Self {
        Self {
            doc_ids: BTreeSet::new(),
            wrt: wrt,
        }
    }

    pub fn insert(&mut self, doc: DocumentId) -> bool {
        self.doc_ids.insert(doc)
    }

    pub fn into_inner(mut self) -> io::Result<W> {
        for id in self.doc_ids {
            self.wrt.write_u64::<NativeEndian>(id)?;
        }
        Ok(self.wrt)
    }
}