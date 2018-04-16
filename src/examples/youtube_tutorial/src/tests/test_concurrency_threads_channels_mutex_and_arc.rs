

mod TestConcurrencyThreadsChannelsMutexAndArc {

    use std::thread;
    use std::sync::mpsc;

    #[test]
    #[ignore]
    fn test_threads_one() {
        let mut c = vec![];

        for i in 0..10 {
            c.push(thread::spawn(move || {
                println!("thread number {}", i);
            }));
        }

        /*for i in 0..10 {*/
            //println!("main thread");
        //}

        for j in c {
            j.join();
        }
        
    }

    #[test]
    #[ignore]
    fn test_threads_two() {

        let v = vec![1, 2, 3];

        let handle = thread::spawn( move || {
            println!("vector: {:?}", v);
        });
        // 스레드에 이미 move 되어지기 때문에 scope 에서 사라짐
        // println!("{:?}", v);
        handle.join();
    }

    #[test]
    fn test_channels() {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            tx.send(42).unwrap();
        });

        println!("got {}", rx.recv().unwrap());
    }
} /* TestConcurrencyThreadsChannelsMutexAndArc */

