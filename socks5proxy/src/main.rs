fn hand(src_stream: &std::net::TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    println!("src {}", src_stream.peer_addr().unwrap());
    Ok(())
}

fn main() {
    let mut c_listen = String::from("127.0.0.1:8080");
    {
        let mut ap = argparse::ArgumentParser::new();
        ap.set_description("Socks5 Proxy");
        ap.refer(&mut c_listen).add_option(&["-l", "--listen"], argparse::Store, "listen address");
        ap.parse_args_or_exit();
    }

    println!("Listen and server on {}", c_listen);

    let listener = std::net::TcpListener::bind(c_listen.as_str()).unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(data) => {
                std::thread::spawn(move || {
                    if let Err(err) = hand(&data) {
                        println!("error: {}", err)
                    }
                });
            },
            Err(err) => {
                println!("error: {}", err)
            }
        }
    }

}
