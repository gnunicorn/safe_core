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

//! App-related Testing utilities.

use super::super::App;
use super::super::errors::AppError;
use super::super::test_utils::create_app_with_access;
use ffi_utils::catch_unwind_error_code;
use safe_core::{DIR_TAG, MDataInfo};
use safe_core::ipc::Permission;
use std::collections::HashMap;


/// Create an app instance with some random access through FFI
#[no_mangle]
pub unsafe extern "C" fn gen_testing_app_with_access(o_app: *mut *mut App) -> i32 {
    catch_unwind_error_code(|| -> Result<_, AppError> {
        // Shared container
        let container_info = MDataInfo::random_private(DIR_TAG)?;

        let mut container_permissions = HashMap::new();
        let _ = container_permissions.insert("_test".to_string(),
                                             (container_info.clone(),
                                              btree_set![Permission::Read, Permission::Insert]));

        let app = create_app_with_access(container_permissions.clone(), false);

        *o_app = Box::into_raw(Box::new(app));
        println!("WE are here!");
        Ok(())
    })
}
