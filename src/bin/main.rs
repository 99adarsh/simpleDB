use core::time;
use std::{cell::RefCell, str::from_utf8};

use simpleDB::engine::{engine::Engine, mem_table::red_black_tree::red_black_tree::{RedBlackTree, Status}};

fn main() {
    let engine = RefCell::new(Engine::new(
        "/home/adarsh/my_files/personal/lsm-database-engine/sstable".to_owned(), 
        200
    ));
    // eprintln!("Enigner result {:?}",engine);
    // println!("{}","hello".as_bytes().to_vec().len() + "world".as_bytes().to_vec().len());
    
    // // for i in 1.. {
    // //     println!("Iteration {}",i);
    // // }
    // engine.borrow_mut().as_mut().unwrap().set("hello".as_bytes().to_vec(), "world".as_bytes().to_vec()).unwrap();
    // let get = engine.borrow().as_ref().unwrap().get("hello".as_bytes().to_vec());
    // println!("Get value for hello is {:?}",from_utf8(&get.unwrap()));
    
    // engine.borrow_mut().as_mut().unwrap().set("hello1".as_bytes().to_vec(), "world".as_bytes().to_vec()).unwrap();
    // let get = engine.borrow().as_ref().unwrap().get("hello".as_bytes().to_vec());
    // println!("Get value for hello is {:?}",from_utf8(&get.unwrap()));
    // let get = engine.borrow().as_ref().unwrap().get("hello1".as_bytes().to_vec());
    // println!("Get value for hello1 is {:?}",from_utf8(&get.unwrap()));

    // engine.borrow_mut().as_mut().unwrap().set("hello2".as_bytes().to_vec(), "world".as_bytes().to_vec()).unwrap();
    // let get = engine.borrow().as_ref().unwrap().get("hello".as_bytes().to_vec());
    // println!("Get value for hello is {:?}",from_utf8(&get.unwrap()));
    // let get = engine.borrow().as_ref().unwrap().get("hello1".as_bytes().to_vec());
    // println!("Get value for hello1 is {:?}",from_utf8(&get.unwrap()));
    // let get = engine.borrow().as_ref().unwrap().get("hello2".as_bytes().to_vec());
    // println!("Get value for hello2 is {:?}",from_utf8(&get.unwrap()));

    // engine.borrow_mut().as_mut().unwrap().set("hello3".as_bytes().to_vec(), "world".as_bytes().to_vec()).unwrap();
    // let get = engine.borrow().as_ref().unwrap().get("hello3".as_bytes().to_vec());
    // println!("Get value for hello3 is {:?}",from_utf8(&get.unwrap()));
    
    // engine.borrow_mut().as_mut().unwrap().set("hello4".as_bytes().to_vec(), "world".as_bytes().to_vec()).unwrap();
    // let get = engine.borrow().as_ref().unwrap().get("hello3".as_bytes().to_vec());
    // println!("Get value for hello3 is {:?}",from_utf8(&get.unwrap()));
    // let get = engine.borrow().as_ref().unwrap().get("hello4".as_bytes().to_vec());
    // println!("Get value for hello4 is {:?}",from_utf8(&get.unwrap()));
    
    // engine.borrow_mut().as_mut().unwrap().set("hello5".as_bytes().to_vec(), "world".as_bytes().to_vec()).unwrap();
   
    // let get = engine.borrow().as_ref().unwrap().get("hello3".as_bytes().to_vec());
    // println!("Get value for hello3 is {:?}",from_utf8(&get.unwrap()));
    // let get = engine.borrow().as_ref().unwrap().get("hello4".as_bytes().to_vec());
    // println!("Get value for hello4 is {:?}",from_utf8(&get.unwrap()));
    // let get = engine.borrow().as_ref().unwrap().get("hello5".as_bytes().to_vec());
    // println!("Get value for hello5 is {:?}",from_utf8(&get.unwrap()));

    // engine.borrow_mut().as_mut().unwrap().set("hello6".as_bytes().to_vec(), "world".as_bytes().to_vec()).unwrap();
    
    // let get = engine.borrow().as_ref().unwrap().get("hello6".as_bytes().to_vec());
    // println!("Get value for hello6 is {:?}",from_utf8(&get.unwrap()));

    // engine.borrow_mut().as_mut().unwrap().set("hello7".as_bytes().to_vec(), "world".as_bytes().to_vec()).unwrap();

    // let get = engine.borrow().as_ref().unwrap().get("hello7".as_bytes().to_vec());
    // println!("Get value for hello7 is {:?}",from_utf8(&get.unwrap()));

    // let mut engine = Engine::new(
    //     "/home/adarsh/my_files/personal/lsm-database-engine/sstable".to_owned(), 
    //     200
    // ).unwrap();
    
    // for i in 1..60 {
    //     std::thread::sleep(time::Duration::from_secs(1));
    //     let _ = engine.set(i.to_string().as_bytes().to_vec(), (i+1).to_string().as_bytes().to_vec());
    //     // let response = engine.borrow_mut().as_mut().unwrap().set(i.to_string().as_bytes().to_vec(), (i+1).to_string().as_bytes().to_vec());
    // }

    let mut rb = RedBlackTree::<u8,u8>::new();
    for i in 1..60 {
        rb.insert_or_replace(i, i+1, 0, Status::Available);
    }
    println!("h");
    
    for i in 1..60 {
        rb.insert_or_replace(i, i+5, 0, Status::Available);
    }
    println!("d");
    let mut iter = rb.root.into_iter();
    print!("s");
    for i in 1..60 {
        println!("insi");
        let x = iter.next().unwrap();
        let key = x.key().unwrap();
        let value = x.value().unwrap();
        println!("Key = {} , value = {}",key,value);
        // assert_eq!(Some(i+1),value);
    }
}
