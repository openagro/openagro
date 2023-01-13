/// Copyright (c) 2023 OpenAgro Developers
///
/// This file under Lesser General Public License v3.0, please read accompanying file
/// copy or, read on https://www.gnu.org/licenses/lgpl-3.0.html

pub struct ExtensionMetaData {
    pub module_id: Option<String>,
    pub name: Option<String>,
    pub author: Option<String>,
    pub data: Vec<String>,
    pub depends: Vec<String>,
}

pub trait ExtensionMeta {
    fn auto_migration(&self);
}

impl ExtensionMeta for ExtensionMetaData {
    fn auto_migration(&self) {
        let c = include_str!("../LICENSE");
        println!("{c:?}")
    }
}