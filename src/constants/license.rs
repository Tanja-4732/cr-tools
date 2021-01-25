// This file contains a license notice, see the full AGPL license in LICENSE.md

use super::meta;

/// The licence notice (AGPL 3) of the application
pub const LICENSE_SHORT: &'static str = concat![
    "Copyright 2021 Bernd-L; All rights reserved.\n",
    "Licensed under the AGPL 3.0"
];

/// The licence URL (AGPL 3) of the application
pub const LICENSE_URL: &'static str = "https://github.com/Bernd-L/cr-tools/blob/main/LICENSE.md";

/// The title of the license notice
pub fn license_notice_title() -> String {
    format!(
        "{name} v{version} license notice",
        name = meta::NAME,
        version = meta::VERSION
    )
}

/// The text body of the license notice
pub fn license_notice_body() -> String {
    format!(
        "
{name} is free software: you can redistribute it and/or modify it under the terms
of the GNU Affero General Public License as published by the Free Software Foundation,
either version 3 of the License, or (at your option) any later version.

{name} is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
See the GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License along with {name}.
If not, see https://www.gnu.org/licenses/.
\n",
        name = meta::NAME
    )
}

// This file contains a license notice, see the full AGPL license in LICENSE.md
