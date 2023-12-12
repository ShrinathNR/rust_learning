mod structs;
mod enums;
mod common_collection;
mod error_handling;

use structs::structs;
use enums::enums;
use common_collection::common_collection;
use error_handling::handle_err;
fn main() {
    structs();
    enums();
    common_collection();
    handle_err();
}
