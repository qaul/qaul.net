/**
 * Qaul Community Router
 * 
 * This module implements all the tables and logic of the 
 * qaul router.
 */

pub mod neighbours;
pub mod users;
pub mod flooder;
use neighbours::Neighbours;
use users::Users;
use flooder::Flooder;


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
    }
}
