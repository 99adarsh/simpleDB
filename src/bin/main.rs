use std::{cell::RefCell, str::from_utf8};

use simpleDB::engine::engine::Engine;

fn main() {
    let engine = RefCell::new(Engine::new(
        "/home/adarsh/my_files/personal/lsm-database-engine/sstable".to_owned(), 
        200
    ));
    eprintln!("Enigner result {:?}",engine);
    println!("{}","hello".as_bytes().to_vec().len() + "world".as_bytes().to_vec().len());
    
    // for i in 1.. {
    //     println!("Iteration {}",i);
    // }
    engine.borrow_mut().as_mut().unwrap().set("hello".as_bytes().to_vec(), "world".as_bytes().to_vec()).unwrap();
    let get = engine.borrow().as_ref().unwrap().get("hello".as_bytes().to_vec());
    println!("Get value for hello is {:?}",from_utf8(&get.unwrap()));
    
    engine.borrow_mut().as_mut().unwrap().set("hello1".as_bytes().to_vec(), "world".as_bytes().to_vec()).unwrap();
    let get = engine.borrow().as_ref().unwrap().get("hello".as_bytes().to_vec());
    println!("Get value for hello is {:?}",from_utf8(&get.unwrap()));
    let get = engine.borrow().as_ref().unwrap().get("hello1".as_bytes().to_vec());
    println!("Get value for hello1 is {:?}",from_utf8(&get.unwrap()));

    engine.borrow_mut().as_mut().unwrap().set("hello2".as_bytes().to_vec(), "world".as_bytes().to_vec()).unwrap();
    let get = engine.borrow().as_ref().unwrap().get("hello".as_bytes().to_vec());
    println!("Get value for hello is {:?}",from_utf8(&get.unwrap()));
    let get = engine.borrow().as_ref().unwrap().get("hello1".as_bytes().to_vec());
    println!("Get value for hello1 is {:?}",from_utf8(&get.unwrap()));
    let get = engine.borrow().as_ref().unwrap().get("hello2".as_bytes().to_vec());
    println!("Get value for hello2 is {:?}",from_utf8(&get.unwrap()));

    engine.borrow_mut().as_mut().unwrap().set("hello3".as_bytes().to_vec(), "world".as_bytes().to_vec()).unwrap();
    let get = engine.borrow().as_ref().unwrap().get("hello3".as_bytes().to_vec());
    println!("Get value for hello3 is {:?}",from_utf8(&get.unwrap()));
    
    engine.borrow_mut().as_mut().unwrap().set("hello4".as_bytes().to_vec(), "world".as_bytes().to_vec()).unwrap();
    let get = engine.borrow().as_ref().unwrap().get("hello3".as_bytes().to_vec());
    println!("Get value for hello3 is {:?}",from_utf8(&get.unwrap()));
    let get = engine.borrow().as_ref().unwrap().get("hello4".as_bytes().to_vec());
    println!("Get value for hello4 is {:?}",from_utf8(&get.unwrap()));
    
    engine.borrow_mut().as_mut().unwrap().set("hello5".as_bytes().to_vec(), "world".as_bytes().to_vec()).unwrap();
   
    let get = engine.borrow().as_ref().unwrap().get("hello3".as_bytes().to_vec());
    println!("Get value for hello3 is {:?}",from_utf8(&get.unwrap()));
    let get = engine.borrow().as_ref().unwrap().get("hello4".as_bytes().to_vec());
    println!("Get value for hello4 is {:?}",from_utf8(&get.unwrap()));
    let get = engine.borrow().as_ref().unwrap().get("hello5".as_bytes().to_vec());
    println!("Get value for hello5 is {:?}",from_utf8(&get.unwrap()));

    engine.borrow_mut().as_mut().unwrap().set("hello6".as_bytes().to_vec(), "world".as_bytes().to_vec()).unwrap();
    
    let get = engine.borrow().as_ref().unwrap().get("hello6".as_bytes().to_vec());
    println!("Get value for hello6 is {:?}",from_utf8(&get.unwrap()));

    engine.borrow_mut().as_mut().unwrap().set("hello7".as_bytes().to_vec(), "world".as_bytes().to_vec()).unwrap();

    let get = engine.borrow().as_ref().unwrap().get("hello7".as_bytes().to_vec());
    println!("Get value for hello7 is {:?}",from_utf8(&get.unwrap()));
}
