use std::{io, net::TcpListener, thread::spawn};

/// 연결을 수락해서 전담 스레드를 생성하는 일을 반복한다.
fn echo_main(addr: &str) -> Result<(), io::Error> {
    let listener = TcpListener::bind(addr)?;
    println!("listening on {}", addr);
    loop {
        // 클라이언트의 연결을 기다린다.
        let (mut stream, addr) = listener.accept()?;
        println!("connection received from {}", addr);

        // 이 클라이언트를 전담할 스레드를 생성한다.
        let mut write_stream = stream.try_clone()?;
        spawn(move || {
            // `stream`으로 받은 내용을 돌려 보낸다.
            io::copy(&mut stream, &mut write_stream).expect("error in client thread: ");
            println!("connection closed");
        });
    }
}

fn main() {
    echo_main("127.0.0.1:17007").expect("error: ");
}
