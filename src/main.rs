// use pnet::datalink::{
//     Channel as DataLinkChannel, DataLinkReceiver, DataLinkSender, NetworkInterface,
// };
// use pnet::datalink::Channel::Ethernet;
// use pnet::packet::icmp::{IcmpPacket, IcmpTypes};
// use pnet::packet::icmpv6::{Icmpv6Packet, Icmpv6Types};
// use pnet::packet::{Packet, self};
// use pnet::packet::arp::ArpPacket;
// use pnet::packet::ethernet::{EthernetPacket, EtherTypes};
// use pnet::packet::ip::{IpNextHeaderProtocols, IpNextHeaderProtocol};
// use pnet::packet::ipv4::Ipv4Packet;
// use pnet::packet::ipv6::Ipv6Packet;
// use pnet::packet::tcp::{TcpPacket, TcpFlags};
// //name表示网络接口的名称。这是一个标识符，用于在操作系统中唯一标识网络接口。
// //description 网络接口的描述
// //index代表网络接口的索引。这是一个操作系统特定的值，用于内部标识和管理网络接口。
// //mac表示网络接口的 MAC 地址
// //ips包含了网络接口的 IP 地址及其对应的网络掩码
// //flags: 这个字段的类型根据目标操作系统的不同而不同,例如，一个标志可能表示接口是否处于活动状态，另一个标志可能表示是否支持广播



// //获取和返回系统中所有可用的网络接口，并按照它们拥有的 IP 地址数量降序排序
// pub fn sorted_usable_interfaces() -> Vec<NetworkInterface> {
//     let mut interfaces = pnet::datalink::interfaces()
//         .into_iter()
//         .filter(|iface| iface.mac.is_some() && !iface.ips.is_empty())//过滤掉mac和ips为空的网络接口
//         .collect::<Vec<NetworkInterface>>();

//     // Sort interfaces by descending number of IPs.
//     interfaces.sort_unstable_by_key(|iface| iface.ips.len());
//     interfaces.reverse();

//     interfaces
// }

// fn get_network_interface(name: &str) -> Option<NetworkInterface> {
//     pnet::datalink::interfaces().into_iter().find(|iface| iface.name == name)
// }




// fn main (){
//     println!("{:?}",sorted_usable_interfaces());
//     let interface = get_network_interface("eth0").expect("Network interface 'eth0' not found");
//     let (_, mut rx) = match pnet::datalink::channel(&interface, Default::default()) {
//         Ok(Ethernet(tx, rx)) => (tx, rx),
//         Ok(_) => panic!("Unhandled channel type"),
//         Err(e) => panic!("An error occurred when creating the datalink channel: {}", e),
//     };

//     loop {
//         // next 抓包，send， send_to 发包
//         // 启用混杂，启用fanout 
//         match rx.next() {
//             Ok(packet) => {
//                 handle_packet(packet);
//             }
//             Err(e) => {
//                 // 处理错误
//             }
//         }
//     }
// }

// fn handle_packet(packet: &[u8]) {
//     //处理以太网数据包 
//     if let Some(ethernet) = EthernetPacket::new(packet) {
//         match ethernet.get_ethertype() {
//             EtherTypes::Ipv4 => handle_ipv4_packet(ethernet.payload()),
//             EtherTypes::Ipv6 => handle_ipv6_packet(ethernet.payload()),
//             EtherTypes::Arp => handle_arp_packet(ethernet.payload()),
//             _ => {} // 忽略非 IPv4 数据包
//         }
//     }
// }

// fn handle_ipv6_packet(packet_data: &[u8]) {
//     if let Some(packet) = Ipv6Packet::new(packet_data) {
//         match packet.get_next_header() {
//             IpNextHeaderProtocols::Tcp => handle_tcp_packet(packet.payload()),
//             IpNextHeaderProtocols::Icmpv6=> handle_icmpv6_packet(packet.payload()),
//             _ => {} // 忽略非 TCP 数据包
//         }
//     }
// }


// //将网络层地址（如 IPv4 地址）映射到链路层地址（如 MAC 地址）局域网内广播一个 ARP 请求,设备发现 ARP 请求中的IP与自己IP匹配，发送一个 ARP 响应，告知其 MAC 地址
// fn handle_arp_packet(packet_data: &[u8]) {
//     if let Some(packet) = ArpPacket::new(packet_data) {
//         // 在这里处理 ARP 数据包
//         println!("捕获到一个 ARP 数据包");
//         // 更多的分析和处理可以在这里进行
//     }
// }


// fn handle_ipv4_packet(packet_data: &[u8]) {
//     if let Some(packet) = Ipv4Packet::new(packet_data) {
//         match packet.get_next_level_protocol() {
//             IpNextHeaderProtocols::Tcp => handle_tcp_packet(packet.payload()),
//             IpNextHeaderProtocols::Icmp=> handle_icmp_packet(packet.payload()),
//             _ => {} // 忽略非 TCP 数据包
//         }
//     }
// }

// fn handle_icmp_packet(packet_data: &[u8]) {
//     if let Some(icmp) = IcmpPacket::new(packet_data) {
//         println!("捕获到一个 ICMP 数据包：类型：{:?}", icmp.get_icmp_type());

//         // 根据 ICMP 类型执行进一步操作
//         match icmp.get_icmp_type() {
//             IcmpTypes::EchoRequest => {
//                 // 处理 ICMP 回显请求
//             },
//             IcmpTypes::EchoReply => {
//                 // 处理 ICMP 回显应答
//             },
//             // 添加其他你感兴趣的 ICMP 类型
//             _ => {}
//         }
//     }
// }
// //邻居发现（ICMPv6）：在 IPv6 网络中替代 ARP 功能，解析 IP 地址到 MAC 地址。
// fn handle_icmpv6_packet(packet_data: &[u8]) {
//     if let Some(icmpv6) = Icmpv6Packet::new(packet_data) {
//         println!("捕获到一个 ICMPv6 数据包：类型：{:?}", icmpv6.get_icmpv6_type());
        
//         // 根据 ICMPv6 类型执行进一步操作
//         match icmpv6.get_icmpv6_type() {
//             Icmpv6Types::EchoRequest => {
//                 // 处理 ICMPv6 回显请求
//             },
//             Icmpv6Types::EchoReply => {
//                 // 处理 ICMPv6 回显应答
//             },
//             // 添加其他你感兴趣的 ICMPv6 类型
//             _ => {}
//         }
//     }
// }

// // （3 + 数据传输 + 4）* 2 
// fn handle_tcp_packet(packet_data: &[u8]) {
//     if let Some(tcp) = TcpPacket::new(packet_data) {
//         let flags = tcp.get_flags();

//         // 检查是否为握手或挥手包（SYN, FIN）
//         if flags & TcpFlags::SYN != 0 || flags & TcpFlags::FIN != 0 {
//             // 这是握手或挥手包，忽略处理
//             return;
//         }
        
//         // 在这里处理 TCP 数据包
//         println!("捕获到一个 TCP 数据包：源端口：{}，目的端口：{}", tcp.get_source(), tcp.get_destination());
//         // 更多的分析和处理可以在这里进行
        
//     }
// }



extern crate pnet;

use std::collections::{HashMap, BTreeMap};
use std::io::{self, Write};
use std::net::SocketAddr;
use std::{process, option};
use std::sync::{Arc, Mutex};

use pnet::packet::Packet;
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::{TcpPacket, TcpFlags, TcpOptionNumbers, TcpOption, TcpOptionPacket};

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
    use std::collections::hash_map;
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

                let mut hash_map=HashMap::new();
                loop {
                    match rx.next() {
                        Ok(_packet) => {
                            
                            writeln!(
                                io::stdout(),
                                "Received packet on thread {:?}",
                                handle.name()
                            )
                            .unwrap();
                            handle_packet(_packet,&mut hash_map);
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


struct TcpStreamTracker {
    packets: BTreeMap<u32, Vec<u8>>, // 存储数据包的有效载荷
    expected_seq: u32,               // 期望的下一个序列号
    reassembled_data: Vec<u8>,       // 重组的数据流
}

impl TcpStreamTracker {
    fn new(initial_seq: u32) -> TcpStreamTracker {
        TcpStreamTracker {
            packets: BTreeMap::new(),
            expected_seq: initial_seq,
            reassembled_data: Vec::new(),
        }
    }

    fn add_packet(&mut self, packet: &TcpPacket) {
        let seq = packet.get_sequence();
        if seq == self.expected_seq {
            // self.packets.insert(seq, packet.payload().to_vec());
            self.reassembled_data.extend_from_slice(packet.payload());
            self.expected_seq += packet.payload().len() as u32;
        }
    }

    fn update_expected_seq(&mut self) {
        while let Some((&seq, payload)) = self.packets.iter().next() {
            if seq == self.expected_seq {
                // 将有效载荷添加到重组的数据流
                self.reassembled_data.extend_from_slice(payload);

                // 更新期望的序列号
                self.expected_seq += payload.len() as u32;
                
                // 移除已处理的包
                self.packets.remove(&seq);
            } else {
                break;
            }
        }
    }

    // 获取重组的数据流
    fn get_reassembled_data(&self) -> &[u8] {
        &self.reassembled_data
    }

    // 其他方法...
}

fn handle_packet(packet: &[u8], connections: &mut HashMap<String, TcpStreamTracker>) {
    let ethernet = EthernetPacket::new(packet).unwrap();
    if let EtherTypes::Ipv4 = ethernet.get_ethertype() {
        let header = Ipv4Packet::new(ethernet.payload());
        if let Some(header) = header {
            // println!("捕获到一个 IP 数据包：源IP：{}，目的IP：{}",header.get_source() , header.get_destination());
            match header.get_next_level_protocol() {
                IpNextHeaderProtocols::Tcp => {
                    let tcp = TcpPacket::new(header.payload());
                    if let Some(tcp) = tcp {
                        // 检查是否为HTTP端口
                        if tcp.get_destination() == 80 || tcp.get_source() == 80 {
                            // 检查TCP标志位以去除握手和挥手包
                            let flags = tcp.get_flags();
                            if flags & (TcpFlags::SYN | TcpFlags::FIN) == 0 {
                                // 组包逻辑: 这里简单地将TCP有效载荷作为HTTP数据
                                let payload = tcp.payload();
                                if !payload.is_empty() {
                                    // 构建一个用于标识TCP流的唯一键
                                        let stream_key = format!("{}:{}->{}:{}",
                                        header.get_source(), tcp.get_source(),
                                        header.get_destination(), tcp.get_destination());
                                        // 提取序列号
                                        let seq = tcp.get_sequence();
                                        println!("{}",stream_key);
                                        connections.entry(stream_key.clone())
                                            .or_insert_with(|| TcpStreamTracker::new(seq))
                                            .add_packet(&tcp);
                                        println!("{:?}",String::from_utf8(connections.get(&stream_key).unwrap().reassembled_data.clone()));
                                    }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}



// // 定义一个用于跟踪TCP连接状态的结构
// struct TcpConnection {
//     last_seq: u32,
// }

// // 创建一个全局的连接状态表
// let connections: Arc<Mutex<HashMap<SocketAddr, TcpConnection>>> = Arc::new(Mutex::new(HashMap::new()));

// fn handle_packet(packet: &[u8], connections: Arc<Mutex<HashMap<SocketAddr, TcpConnection>>>) {
//     let tcp = TcpPacket::new(packet).unwrap();

//     // 获取源和目的Socket地址
//     let source_ip = ...; // 解析源IP地址
//     let dest_ip = ...; // 解析目的IP地址
//     let source_port = tcp.get_source();
//     let dest_port = tcp.get_destination();
//     let source_addr = SocketAddr::new(IpAddr::V4(source_ip), source_port);
//     let dest_addr = SocketAddr::new(IpAddr::V4(dest_ip), dest_port);

//     // 获取当前包的序列号
//     let seq = tcp.get_sequence();

//     let mut conn_table = connections.lock().unwrap();
//     let conn_key = (source_addr, dest_addr);

//     // 检查是否是新连接或旧连接的新数据包
//     let is_new_or_non_duplicate = match conn_table.get(&conn_key) {
//         Some(conn) => seq > conn.last_seq,
//         None => true, // 新连接
//     };

//     if is_new_or_non_duplicate {
//         // 处理数据包...

//         // 更新连接状态表
//         conn_table.insert(conn_key, TcpConnection { last_seq: seq });
//     } else {
//         // 忽略重传或重复的包
//         println!("Duplicate or retransmitted packet ignored.");
//     }
// }



// 移除相同的数据包(序列号和长度相同)
// 如果一个带有PSH标志位的数据包排在一组数据包的前面，这可能意味着发送方已经发送了所有当前可用的数据，并希望这些数据被立即处理
// 有psh标识标识非空的tcp包
// 一次tcp请求是连续的序列号
// 通过seq+消息的长度获取是否是完整的tcp
// 