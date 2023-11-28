extern crate pnet;
extern crate pnet_datalink;

use std::io::{self, Write};
use std::process;

#[cfg(not(target_os = "linux"))]
fn main() {
    writeln!(io::stderr(), "fanout is only supported on Linux").unwrap();
    process::exit(1);
}
// fanout: 多个线程同时捕获一张网卡上的流量，对一任何一个数据包保证发送到每个线程中。
#[cfg(target_os = "linux")]
fn main() {
    use pnet::datalink::Channel::Ethernet;
    use pnet::datalink::{self, Config, FanoutOption, FanoutType, NetworkInterface};
    use std::env;
    use std::thread;

    let iface_name = match env::args().nth(1) {
        Some(n) => n,
        None => {
            writeln!(io::stderr(), "USAGE: fanout <NETWORK INTERFACE> [hash|*round-robin*|cpu|rollover|rnd|qm|cbpf|ebpf] [group-id:123]").unwrap();
            process::exit(1);
        }
    };
    let interface_names_match = |iface: &NetworkInterface| iface.name == iface_name;

    // 获取指定名称的网卡
    let interfaces = datalink::linux::interfaces();
    let interface = interfaces
        .into_iter()
        .filter(interface_names_match)
        .next()
        .unwrap();
    // fanout的类型
    let fanout_type = match env::args().nth(2) {
        Some(n) => match n.to_lowercase().as_str() {
            "hash" => FanoutType::HASH,       //hash模式，基于每个数据包的哈希值
            "round-robin" => FanoutType::LB,  //轮询模式
            "cpu" => FanoutType::CPU,
            "rollover" => FanoutType::ROLLOVER,
            "rnd" => FanoutType::RND,
            "qm" => FanoutType::QM,
            "cbpf" => FanoutType::CBPF,
            "ebpf" => FanoutType::EBPF,
            _ => panic!("Unsupported fanout type, use one of hash, round-robin, cpu, rollover, rnd, qm, cbpf or ebpf")
        },
        None => FanoutType::LB,
    };
    // 同一个group_id的packet socket实例将共享接收到的网络数据包，以允许多个packet socket实例高效地共享对网络接口层数据包的访问通过使用相同的group_id
    // 获取第3个命令行参数为group_id 如果没有则设置为123
    let group_id = match env::args().nth(3) {
        Some(n) => n.parse::<u16>().unwrap(),
        None => 123,
    };
    //配置
    let mut config: Config = Default::default();
    config.linux_fanout = Some(FanoutOption {
        group_id: group_id,
        fanout_type: fanout_type,
        defrag: true,
        rollover: false,
    });

    let mut threads = vec![];
    for x in 0..3 {
        let itf = interface.clone();
        let thread = thread::Builder::new()
            .name(format!("thread{}", x))
            .spawn(move || {
                // 创建 packet socket实例 数据包轮询的分发给3个线程中的一个，每个线程单独处理一个数据包
                let (_, mut rx) = match datalink::channel(&itf, config) {
                    Ok(Ethernet(tx, rx)) => (tx, rx),
                    Ok(_) => panic!("packetdump: unhandled channel type"),
                    Err(e) => panic!("packetdump: unable to create channel: {}", e),
                };

                let handle = thread::current();

                loop {
                    match rx.next() {
                        Ok(_packet) => {
                            writeln!(
                                io::stdout(),
                                "Received packet on thread {:?}",
                                handle.name()
                            )
                            .unwrap();
                        }
                        Err(e) => panic!("packetdump: unable to receive packet: {}", e),
                    }
                }
            })
            .unwrap();
        threads.push(thread);
    }

    for t in threads {
        t.join().unwrap();
    }
}