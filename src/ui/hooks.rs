use std::rc::Rc;
use yew::prelude::*;


#[hook]
pub fn use_tokenstore() -> UseTokenStoreHandle {
    let storage = Rc::new(web_sys::window().unwrap().local_storage().unwrap().unwrap());

    UseTokenStoreHandle { storage }
}

#[derive(Clone)]
pub struct UseTokenStoreHandle {
    storage: Rc<web_sys::Storage>
}

impl UseTokenStoreHandle {
    const STORAGE_KEY: &'static str = "ohkami-yew-todo-demo-token";

    pub fn get(&self) -> Option<Rc<String>> {
        (&*self.storage).get(Self::STORAGE_KEY).unwrap().map(Rc::new)
    }

    pub fn store(&self, token: &str) {
        (&*self.storage).set(Self::STORAGE_KEY, &token).unwrap();
    }
}
