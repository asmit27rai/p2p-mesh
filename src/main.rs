use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, time::Duration};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    time,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Share {
    from: usize,
    content: String,
    timestamp: u64,
}

async fn handle_incoming(mut stream: TcpStream, node_id: usize) {
    let mut buf = vec![0u8; 1024];
    loop {
        match stream.read(&mut buf).await {
            Ok(0) => break, // Connection closed
            Ok(n) => {
                let msg = &buf[..n];
                if let Ok(share) = serde_json::from_slice::<Share>(msg) {
                    tracing::info!(
                        "Node {} received share from Node {}: {}",
                        node_id,
                        share.from,
                        share.content
                    );
                }
            }
            Err(_) => break,
        }
    }
}

async fn node_task(
    node_id: usize,
    addr: SocketAddr,
    peers: Vec<SocketAddr>,
    latency_ms: u64,
) -> anyhow::Result<()> {
    let listener = TcpListener::bind(addr).await?;
    tracing::info!("Node {} listening on {}", node_id, addr);

    // Spawn listener for incoming shares
    tokio::spawn({
        let node_id = node_id.clone();
        async move {
            loop {
                let (stream, _) = listener.accept().await.unwrap();
                tokio::spawn(handle_incoming(stream, node_id));
            }
        }
    });

    // Generate and send shares periodically
    let mut interval = time::interval(Duration::from_secs(2));
    loop {
        interval.tick().await;

        let share = Share {
            from: node_id,
            content: format!("Share from Node {}", node_id),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };

        for peer_addr in &peers {
            let share = share.clone();
            let peer_addr = *peer_addr;
            let latency = Duration::from_millis(latency_ms);

            tokio::spawn(async move {
                time::sleep(latency).await; // Simulate network latency
                match TcpStream::connect(peer_addr).await {
                    Ok(mut stream) => {
                        let msg = serde_json::to_vec(&share).unwrap();
                        stream.write_all(&msg).await.unwrap();
                    }
                    Err(e) => tracing::error!("Node {} failed to connect to {}: {}", node_id, peer_addr, e),
                }
            });
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    const NUM_NODES: usize = 10;
    const BASE_PORT: u16 = 8000;
    let latency_ms = 500; // Hard-coded latency

    // Create addresses for all nodes
    let addrs: Vec<_> = (0..NUM_NODES)
        .map(|i| format!("127.0.0.1:{}", BASE_PORT + i as u16).parse().unwrap())
        .collect();

    // Start all nodes
    for node_id in 0..NUM_NODES {
        let peers = addrs
            .iter()
            .enumerate()
            .filter_map(|(i, addr)| (i != node_id).then_some(*addr))
            .collect(); // Full mesh (all nodes are peers)

        let addr = addrs[node_id];
        tokio::spawn(node_task(node_id, addr, peers, latency_ms));
    }

    // Keep main thread alive
    loop {
        time::sleep(Duration::from_secs(3600)).await;
    }
}