/**
 * Qaul Community Router
 * 
 * This module implements all the tables and logic of the 
 * qaul router.
 */

pub mod neighbours;
pub mod users;
pub mod flooder;
pub mod table;
pub mod connections;
use neighbours::Neighbours;
use users::Users;
use flooder::Flooder;
use table::Table;
use connections::ConnectionTable;


pub struct Router {

}

impl Router {
    pub fn init() {
        // initialize direct neighbours table
        Neighbours::init();

        // initialize users table
        Users::init();

        // initialize flooder queue
        Flooder::init();

        // initialize the global routing table
        Table::init();

        // initialize the routing information collection
        // tables per connection module
        ConnectionTable::init();
    }
}
