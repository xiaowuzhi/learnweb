use std::net::{ToSocketAddrs, UdpSocket};

use std::io;
use std::thread;

fn main() {
    println!("UDP Char App");

    let mut cnt = 0;
    let (mut tar,) = (None,);
    let mut name = String::from("user_default");
    for arg in std::env::args() {
        if 1 == cnt {
            tar = Some(arg);
        } else if 2 == cnt {
            name = String::from(arg);
        }

        cnt += 1;
    }

    if let (Some(v1),) = (tar,) {
        func_create(v1, name);
    }
}

fn func_create<A: ToSocketAddrs>(tar: A, name: String) {
    println!("[func_create] create a char app");
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    socket.connect(tar).expect("error 2");
    println!("socket addr {:?}", socket.peer_addr());

    let lis_tk = socket.try_clone().unwrap();
    let n1 = name.clone();

    let rep_tk = socket.try_clone().unwrap();
    let n2 = name.clone();

    let handle_lis = thread::spawn(move || func_listen(lis_tk, n1));
    let handle_rep = thread::spawn(move || func_replys(rep_tk, n2));

    handle_lis.join().unwrap();
    handle_rep.join().unwrap();
}

fn func_listen(socket: UdpSocket, name: String) {
    //loop {
    let mut buf = [0u8; 1024];
    match socket.recv_from(&mut buf) {
        Ok((amt, _src)) => {
            let buf = &mut buf[..amt];
            let mut info = String::new();
            for v in buf {
                if '\n' == (*v as char) || '\r' == (*v as char) {
                    // continue;
                }
                info.push(*v as char);
            }

            let pos: Vec<&str> = info.split(",").collect();
            println!("{:?}", pos);

            func_create_udp(pos[0], pos[1], name);
        }
        Err(err) => {
            println!("error: {:?}", err);
        }
    }
    //}
}

fn func_create_udp<A: ToSocketAddrs>(src: A, tar: A, name: String) {
    println!("[func_create] create a char app");
    let socket = UdpSocket::bind(src).unwrap();
    if socket.connect(tar).is_err() {
        return;
    }

    let lis_tk = socket.try_clone().unwrap();
    let n1 = name.clone();
    let rep_tk = socket.try_clone().unwrap();
    let n2 = name.clone();
    let handle_lis = thread::spawn(move || func_listen_udp(lis_tk, n1));
    let handle_rep = thread::spawn(move || func_replys_udp(rep_tk, n2));

    handle_lis.join().unwrap();
    handle_rep.join().unwrap();
}

fn func_replys(socket: UdpSocket, name: String) {
    //loop {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let sinfo = format!("{},{}", name, input);

    if socket.send(sinfo.as_bytes()).is_err() {
        //continue;
    }
    println!("[{}]send info ==> {}", name, input);
    //}
}

fn func_replys_udp(socket: UdpSocket, name: String) {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let sinfo = format!("[{}]:{}", name, input);

        if socket.send(sinfo.as_bytes()).is_err() {
            continue;
        }
        println!("[{}]send info ==> {}", name, input);
    }
}

fn func_listen_udp(socket: UdpSocket, name: String) {
    loop {
        let mut buf = [0u8; 256];
        match socket.recv_from(&mut buf) {
            Ok((amt, _src)) => {
                let buf = &mut buf[..amt];
                let mut info = String::new();
                for v in buf {
                    if '\n' == (*v as char) || '\r' == (*v as char) {
                        continue;
                    }
                    info.push(*v as char);
                }
                println!("[{}]recv info ==> {}", name, info);
            }
            Err(err) => {
                println!("error: {:?}", err);
            }
        }
    }
}
