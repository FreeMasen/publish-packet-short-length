#[cfg(test)]
mod tests {
    use mqtt::{packet::{PublishPacket, publish::QoSWithPacketIdentifier}, topic_name::TopicName, Encodable};
    #[test]
    fn it_fails() {
        let mut packet = PublishPacket::new(
            TopicName::new("topic").unwrap(),
            QoSWithPacketIdentifier::Level0,
            Vec::new(),
        );
        packet.set_dup(true);
        packet.set_qos(QoSWithPacketIdentifier::Level1(1));
        let mut bytes = vec![0u8;50];
        packet.encode(&mut bytes.as_mut_slice()).unwrap();
        assert_eq!(
            bytes[1], 
            2 // topic length u16
            + 5 // topic bytes
            + 2 // packet id
        )
    }
}
