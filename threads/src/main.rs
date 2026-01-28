use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
    time::Duration,
};

fn main() {
    // ! If a main thread ends spawned threas stopped no matter it is finished its executing or not
    // let hanldle =  thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("hi number {} from the spawned thread!", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // ! Small details, such as where join is called, can affect whether or not your threads run at the same time.
    // hanldle.join().unwrap();

    // for i in 1..5 {
    //     println!("hi number {} from the main thread!", i);
    //     thread::sleep(Duration::from_millis(1));
    // }

    // hanldle.join().unwrap();

    // ! Using move Closures with Threads
    // let v = vec![1, 2, 3];

    // //? Threads must own the data they use, not borrow it.
    // let handle = thread::spawn(move || {
    //     println!("Here's a vector: {v:?}");
    // });

    // handle.join().unwrap();

    // ! Transfer Data Between Threads with Message Passing
    //? Move data between threads, don’t let threads fight over the same data

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let msg = String::from("hello");
        tx.send(msg).unwrap();

        // ! Transferring Ownership Through Channels
        // println!("msg is {}", msg); // # This line give us error
    });

    let received = rx.recv().unwrap(); //? recv() will block the main thread's execution and waiting while its waits for a message to be sent down the channel. It returns a result type
    // let received = rx.try_recv().unwrap(); //? try_recv() will not block the main thread's execution instead it will return result type immediately

    println!("Got: {}", received);

    // ! Sending Multiple Values
    // let (tx, rx) = mpsc::channel();

    // thread::spawn(move || {
    //     let vals = vec![
    //         String::from("hi"),
    //         String::from("from"),
    //         String::from("the"),
    //         String::from("thread"),
    //     ];

    //     for val in vals {
    //         tx.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });

    // for received in rx {
    //     println!("Got: {received}");
    // }

    // ! Creating Multiple Producers
    // let (tx, rx) = mpsc::channel();
    // let tx2 = tx.clone();

    // thread::spawn(move || {
    //     let vals = vec![
    //         String::from("hi 1"),
    //         String::from("from 1"),
    //         String::from("the 1"),
    //         String::from("thread 1"),
    //     ];

    //     for val in vals {
    //         tx.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });

    // thread::spawn(move || {
    //     let vals = vec![
    //         String::from("hi 2"),
    //         String::from("from 2"),
    //         String::from("the 2"),
    //         String::from("thread 2"),
    //     ];

    //     for val in vals {
    //         tx2.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });

    // for received in rx {
    //     println!("Got: {received}");
    // }

    // ! Controlling Access with Mutexes
    // # The API of Mutex<T>
    //? Mutex<T> forces you to lock before access and unlocks automatically when the guard goes out of scope
    let m = Mutex::new(7);

    {
        let mut num = m.lock().unwrap();
        *num = 9;
    }

    println!("m = {:?}", m);

    // # Shared Access to Mutex<T>
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
