// bin/perf-vumeter.rs

use log::*;
use std::{io, io::Write, net};
use structopt::StructOpt;
use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};

use tcp_server::*;
const BUF_SZ: usize = 8192;

fn main() -> anyhow::Result<()> {
    let opts = OptsCommon::from_args();
    opts.start_pgm(env!("CARGO_BIN_NAME"));

    let runtime = tokio::runtime::Runtime::new()?;
    runtime.block_on(async move { run_server(&opts).await })
}

async fn run_server(opts: &OptsCommon) -> anyhow::Result<()> {
    let addr = &opts.listen;
    let listener = TcpListener::bind(addr).await?;
    info!("Listening on {addr}");
    let mut i: u64 = 0;
    loop {
        let (socket, c_addr) = listener.accept().await?;
        let cn = i;
        i += 1;

        tokio::spawn(async move { process_conn(socket, c_addr, cn).await });
    }
}

async fn process_conn(mut socket: TcpStream, addr: net::SocketAddr, cn: u64) -> anyhow::Result<()> {
    info!("New conn #{cn} from {addr:?}");
    let mut buf = vec![0; BUF_SZ];

    loop {
        let n = socket.read(&mut buf).await?;
        if n == 0 {
            info!("Conn #{cn} closed.");
            return Ok(());
        }
        {
            let mut w = io::stdout().lock();
            w.write_all(format!("[{cn}] ").as_bytes())?;
            w.write_all(&buf[0..n])?;
            w.flush()?;
        }
    }
}
// EOF
