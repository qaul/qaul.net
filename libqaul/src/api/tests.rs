//! libqaul API tests
//!
//! Run some simple tests against the service API to check that users
//! can be created, modified, deleted, etc

use crate::*;

#[test]
fn user_lifecycle() {
    let qaul = Qaul::start();

    // can we make a user?
    let auth = qaul.user_create("password").unwrap();

    // does something get added to the correct parts of state?
    {
        assert_eq!(qaul.auth.lock().unwrap().len(), 1);
        assert_eq!(qaul.keys.lock().unwrap().len(), 1);
        assert_eq!(qaul.users.lock().unwrap().len(), 1);
    }

    // are we trusted?
    let (id, key) = qaul.user_authenticate(auth.clone()).unwrap();

    // can we login as that user?
    let auth2 = qaul.user_login(id.clone(), "password").unwrap();

    // is the state updated appropriately?
    {
        assert_eq!(qaul.auth.lock().unwrap().len(), 1);
        assert_eq!(qaul.keys.lock().unwrap().len(), 2);
        assert_eq!(qaul.users.lock().unwrap().len(), 1);
    }

    // are we trusted?
    let (id2, key2) = qaul.user_authenticate(auth2.clone()).unwrap();

    // do we get back the same id?
    assert_eq!(id.clone(), id2);
    // do we get a different key?
    assert_ne!(key.clone(), key2);

    // can we log out?
    qaul.user_logout(auth2.clone()).unwrap();

    // is the state updated appropriately
    {
        assert_eq!(qaul.auth.lock().unwrap().len(), 1);
        assert_eq!(qaul.keys.lock().unwrap().len(), 1);
        assert_eq!(qaul.users.lock().unwrap().len(), 1);
    }

    // are we not trusted now?
    assert!(qaul.user_authenticate(auth2).is_err());

    // does logging in with the wrong password actually error?
    assert!(qaul.user_login(id.clone(), "not password").is_err());
}
