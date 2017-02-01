// Copyright 2016 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under (1) the MaidSafe.net
// Commercial License, version 1.0 or later, or (2) The General Public License
// (GPL), version 3, depending on which licence you accepted on initial access
// to the Software (the "Licences").
//
// By contributing code to the SAFE Network Software, or to this project
// generally, you agree to be bound by the terms of the MaidSafe Contributor
// Agreement, version 1.0.
// This, along with the Licenses can be found in the root directory of this
// project at LICENSE, COPYING and CONTRIBUTOR.
//
// Unless required by applicable law or agreed to in writing, the SAFE Network
// Software distributed under the GPL Licence is distributed on an "AS IS"
// BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or
// implied.
//
// Please review the Licences for the specific language governing permissions
// and limitations relating to use of the SAFE Network Software.


use ffi_utils::{ReprC, vec_into_raw_parts};
use nfs::errors::NfsError;
use nfs::ffi::File as FfiFile;
use routing::XorName;
use rustc_serialize::{Decodable, Decoder, Encodable, Encoder};
use std::slice;
use time::{self, Timespec, Tm};

/// Representation of a File to be put into the network. Could be any kind of
/// file: text, music, video, etc.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct File {
    size: u64,
    created: Tm,
    modified: Tm,
    user_metadata: Vec<u8>,
    data_map_name: XorName,
}

impl File {
    /// Create a new instance of FileMetadata
    pub fn new(user_metadata: Vec<u8>) -> File {
        File {
            size: 0,
            created: time::now_utc(),
            modified: time::now_utc(),
            user_metadata: user_metadata,
            data_map_name: XorName::default(),
        }
    }

    /// Construct FFI wrapper for the native rust `File`, consuming the file.
    pub fn into_repr_c(self) -> FfiFile {
        // TODO: move the metadata, not clone.
        println!("pre meta");
        let user_metadata = self.user_metadata().to_vec();
        println!("post meta");
        let (user_metadata_ptr, user_metadata_len, user_metadata_cap) =
            vec_into_raw_parts(user_metadata);
        println!("in parts");

        FfiFile {
            size: self.size(),
            created: *self.created_time(),
            modified: *self.modified_time(),
            user_metadata_ptr: user_metadata_ptr,
            user_metadata_len: user_metadata_len,
            user_metadata_cap: user_metadata_cap,
            data_map_name: self.data_map_name().0,
        }
    }

    /// Get time of creation
    pub fn created_time(&self) -> &Tm {
        &self.created
    }

    /// Get time of modification
    pub fn modified_time(&self) -> &Tm {
        &self.modified
    }

    /// Get the network name of the data containing the data-map of the File
    pub fn data_map_name(&self) -> &XorName {
        &self.data_map_name
    }

    /// Get size information
    pub fn size(&self) -> u64 {
        self.size
    }

    /// Get user setteble custom metadata
    pub fn user_metadata(&self) -> &[u8] {
        &self.user_metadata
    }

    /// Set the data-map name of the File
    pub fn set_data_map_name(&mut self, datamap_name: XorName) {
        self.data_map_name = datamap_name;
    }

    /// Set the size of file
    pub fn set_size(&mut self, size: u64) {
        self.size = size;
    }

    /// Set time of creation
    pub fn set_created_time(&mut self, created_time: Tm) {
        self.created = created_time
    }

    /// Set time of modification
    pub fn set_modified_time(&mut self, modified_time: Tm) {
        self.modified = modified_time
    }

    /// User setteble metadata for custom metadata
    pub fn set_user_metadata(&mut self, user_metadata: Vec<u8>) {
        self.user_metadata = user_metadata;
    }
}

impl ReprC for File {
    type C = *const FfiFile;
    type Error = NfsError;

    /// Convert to the native rust equivalent by cloning the internal data, preserving self.
    #[allow(unsafe_code)]
    #[cfg_attr(feature="clippy", allow(not_unsafe_ptr_arg_deref))]
    fn from_repr_c_cloned(repr_c: *const FfiFile) -> Result<File, NfsError> {
        let user_metadata = unsafe {
                slice::from_raw_parts((*repr_c).user_metadata_ptr, (*repr_c).user_metadata_len)
            }
            .to_vec();

        let mut file = File::new(user_metadata);
        unsafe {
            file.set_size((*repr_c).size);
            file.set_created_time((*repr_c).created);
            file.set_modified_time((*repr_c).modified);
            file.set_data_map_name(XorName((*repr_c).data_map_name));
        }
        Ok(file)
    }
}

impl Encodable for File {
    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        let created_time = self.created.to_timespec();
        let modified_time = self.modified.to_timespec();

        e.emit_struct("File", 7, |e| {
            e.emit_struct_field("size", 0, |e| self.size.encode(e))?;
            e.emit_struct_field("created_time_sec", 1, |e| created_time.sec.encode(e))?;
            e.emit_struct_field("created_time_nsec", 2, |e| created_time.nsec.encode(e))?;
            e.emit_struct_field("modified_time_sec", 3, |e| modified_time.sec.encode(e))?;
            e.emit_struct_field("modified_time_nsec", 4, |e| modified_time.nsec.encode(e))?;
            e.emit_struct_field("user_metadata", 5, |e| self.user_metadata.encode(e))?;
            e.emit_struct_field("data_map_name", 6, |e| self.data_map_name.encode(e))?;

            Ok(())
        })
    }
}

impl Decodable for File {
    fn decode<D: Decoder>(d: &mut D) -> Result<File, D::Error> {
        d.read_struct("File", 7, |d| {
            Ok(File {
                size: d.read_struct_field("size", 0, Decodable::decode)?,
                created: ::time::at_utc(Timespec {
                    sec: d.read_struct_field("created_time_sec", 1, Decodable::decode)?,
                    nsec: d.read_struct_field("created_time_nsec", 2, Decodable::decode)?,
                }),
                modified: ::time::at_utc(Timespec {
                    sec: d.read_struct_field("modified_time_sec", 3, Decodable::decode)?,
                    nsec: d.read_struct_field("modified_time_nsec", 4, Decodable::decode)?,
                }),
                user_metadata: d.read_struct_field("user_metadata", 5, Decodable::decode)?,
                data_map_name: d.read_struct_field("data_map_name", 6, Decodable::decode)?,
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use maidsafe_utilities::serialisation::{deserialise, serialise};

    #[test]
    fn serialise_deserialise() {
        let obj_before = File::new("{mime:\"application/json\"}"
            .to_string()
            .into_bytes());
        let serialised_data = unwrap!(serialise(&obj_before));
        let obj_after = unwrap!(deserialise(&serialised_data));
        assert_eq!(obj_before, obj_after);
    }
}
