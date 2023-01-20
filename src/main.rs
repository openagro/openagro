/// Copyright (c) 2023 OpenAgro Developers
///
/// This file under Lesser General Public License v3.0, please read accompanying file
/// copy or, read on https://www.gnu.org/licenses/lgpl-3.0.html

use openagro_extension::extension::ExtensionMetadata;
use openagro_test_extension::metadata as test_extension;
use openagro_extension_sales::metadata as sales_extension;

use serde_json;

fn main() {

    let list_extension : Vec<ExtensionMetadata> = vec![test_extension(), sales_extension()];

    for extensions in list_extension {
        let json = serde_json::to_string_pretty(&extensions).unwrap();
        println!("{}", json);
    }

}