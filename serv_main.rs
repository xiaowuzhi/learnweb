use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    println!("UDP Char App");

    let mut cnt = 0;
    let (mut src,) = (None,);
    let mut name = String::from("user_default");
    for arg in std::env::args() {
        if 1 == cnt {
            src = Some(arg);
        } else if 2 == cnt {
            name = String::from(arg);
        }
        cnt += 1;
    }

    if let (Some(v1),) = (src,) {
        func_create(v1, name);
    }
}

fn func_create<A: ToSocketAddrs>(src: A, name: String) {
    println!("[func_create] create a char app");
    let socket = UdpSocket::bind(src).unwrap();

    let ab: Arc<Mutex<Vec<SocketAddr>>> = Arc::new(Mutex::new(Vec::new()));
    let ab1 = ab.clone();

    let lis_tk = socket.try_clone().unwrap();
    let n1 = name.clone();

    let handle_lis = thread::spawn(move || func_listen(lis_tk, n1, ab1));

    handle_lis.join().unwrap();
}

fn func_listen(socket: UdpSocket, name: String, kk: Arc<Mutex<Vec<SocketAddr>>>) {
    loop {
        let mut buf = [0u8; 256];
        match socket.recv_from(&mut buf) {
            Ok((amt, src)) => {
                let buf = &mut buf[..amt];
                let mut info = String::new();
                for v in buf {
                    if '\n' == (*v as char) || '\r' == (*v as char) {
                        continue;
                    }
                    info.push(*v as char);
                }
                println!("[{}]recv info ==> {}", name, info);

                let _info_u: i32 = match info.trim().parse() {
                    Ok(num) => num,
                    Err(_) => 0,
                };

                let mut mm = kk.lock().unwrap();
                (*mm).push(src.clone());

                println!("runing socketaddr arr: {:?}", *mm);
                if (*mm).get(0).is_some() && (*mm).get(1).is_some() {
                    let res_str0 = format!("{},{}", (*mm).get(0).unwrap(), (*mm).get(1).unwrap());
                    let res_str1 = format!("{},{}", (*mm).get(1).unwrap(), (*mm).get(0).unwrap());
                    socket
                        .send_to(res_str0.as_bytes(), (*mm).get(0).unwrap())
                        .expect("error1");
                    socket
                        .send_to(res_str1.as_bytes(), (*mm).get(1).unwrap())
                        .expect("error1");

                    (*mm) = Vec::new();
                    println!("init socketaddr arr: {:?}", *mm);
                };
            }
            Err(err) => {
                println!("error: {:?}", err);
            }
        }
    }
}
