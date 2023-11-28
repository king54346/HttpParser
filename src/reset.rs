

pub fn send_reset(source: SocketAddrV4, destination: SocketAddrV4, sequence_number: u32, acknowledge_number: u32) -> io::Result<usize> {

    let protocol = Layer3(IpNextHeaderProtocols::Tcp);
    // l3 层发出
    let (mut tx, _) = match transport_channel(4096, protocol) {
        Ok((tx, _)) => (tx, _),
        Err(e) => panic!("An error occurred when creating the transport channel: {}", e),
    };
    // 不包含选项的ip头20byte，不包含tcp的头20byte
    let mut packet = [0u8; 40]; // 20 bytes for IPv4 header + 20 bytes for TCP header

    // 创建ip头
    let mut ipv4_packet = MutableIpv4Packet::new(&mut packet[..]).unwrap();
    ipv4_packet.set_version(4);
    ipv4_packet.set_header_length(5); // No IP options
    ipv4_packet.set_total_length(40); //reset包没有payload
    ipv4_packet.set_ttl(64);
    
    ipv4_packet.set_flags(IPv4Flags::DontFragment); //不分片
    ipv4_packet.set_fragment_offset(0);
    ipv4_packet.set_identification(257u16.to_be());//保证唯一性

    ipv4_packet.set_next_level_protocol(IpNextHeaderProtocols::Tcp);

    ipv4_packet.set_source(source.ip().clone());
    ipv4_packet.set_destination(destination.ip().clone());
    ipv4_packet.set_checksum(ip::checksum(&Ipv4Packet::new(ipv4_packet.packet()).unwrap()));

    // 创建tcp头
    let mut tcp_packet = MutableTcpPacket::new(&mut packet[20..]).unwrap();
    tcp_packet.set_source(source.port());
    tcp_packet.set_destination(destination.port());
    tcp_packet.set_sequence(sequence_number);
    tcp_packet.set_acknowledgement(acknowledge_number);
    tcp_packet.set_data_offset(5); // 没有选项和内容
    tcp_packet.set_flags(TcpFlags::RST);
    tcp_packet.set_window(0);//滑动窗口
    tcp_packet.set_checksum(pnet::packet::tcp::ipv4_checksum(
        &tcp_packet.to_immutable(),
        &source.ip(),
        &destination.ip(),
    ));

    //发送到指定ip
    tx.send_to(ipv4_packet, destination)
}
