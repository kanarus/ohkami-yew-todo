use std::rc::Rc;
use yew::prelude::*;


#[hook]
pub fn use_token() -> UseTokenHandle {
    let storage = Rc::new(web_sys::window().unwrap().local_storage().unwrap().unwrap());

    UseTokenHandle {
        storage: Rc::clone(&storage),
        token: use_state(|| storage.get(UseTokenHandle::STORAGE_KEY).unwrap().map(Rc::new))
    }
}

#[derive(Clone)]
pub struct UseTokenHandle {
    storage: Rc<web_sys::Storage>,
    token:   UseStateHandle<Option<Rc<String>>>,
}

impl UseTokenHandle {
    const STORAGE_KEY: &'static str = "ohkami-yew-todo-demo-token";

    pub fn get(&self) -> Option<Rc<String>> {
        (&*self.storage).get(Self::STORAGE_KEY).unwrap().map(Rc::new)
    }

    pub fn set(&self, token: &str) {
        (&*self.storage).set(Self::STORAGE_KEY, &token).unwrap();
    }
}

const _: () = {
    impl std::ops::Deref for UseTokenHandle {
        type Target = Option<Rc<String>>;
        fn deref(&self) -> &Self::Target {
            &*self.token
        }
    }
};
