extern crate pnet;

use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::udp::MutableUdpPacket;
use pnet::packet::{MutablePacket, Packet};
use pnet::transport::TransportChannelType::Layer4;
use pnet::transport::TransportProtocol::Ipv4;
use pnet::transport::{transport_channel, udp_packet_iter};

//创建一个基于特定协议的网络数据包处理程序
fn main() {
    // ipv4 协议
    let protocol = Layer4(Ipv4(IpNextHeaderProtocols::Test1));

    // 创建一个发送和接收通道，处理4层的数据包使用一个test1的协议，接收buffer4096个
    // transport_channel 直接于网络层交互,接收发送到所在主机的IP地址的数据包
    let (mut tx, mut rx) = match transport_channel(4096, protocol) {
        Ok((tx, rx)) => (tx, rx),
        Err(e) => panic!(
            "An error occurred when creating the transport channel: {}",
            e
        ),
    };

    // 使用udp_packet_iter创建一个迭代器来迭代接收到的UDP数据包。
    let mut iter = udp_packet_iter(&mut rx);
    loop {
        match iter.next() {
            Ok((packet, addr)) => {
                // 程序创建一个新的UDP数据包
                let mut vec: Vec<u8> = vec![0; packet.packet().len()];
                let mut new_packet = MutableUdpPacket::new(&mut vec[..]).unwrap();

                // 数据包的副本
                new_packet.clone_from(&packet);

                // 交换原始数据包的源和目的端口
                new_packet.set_source(packet.get_destination());
                new_packet.set_destination(packet.get_source());

                // 修改后的数据包发送回原始发送者的地址
                match tx.send_to(new_packet, addr) {
                    Ok(n) => assert_eq!(n, packet.packet().len()),
                    Err(e) => panic!("failed to send packet: {}", e),
                }
            }
            Err(e) => {
                // If an error occurs, we can handle it here
                panic!("An error occurred while reading: {}", e);
            }
        }
    }
}