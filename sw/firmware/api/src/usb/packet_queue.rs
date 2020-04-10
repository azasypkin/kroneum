use super::{descriptors::MAX_PACKET_SIZE, endpoint::EndpointType, SUPPORTED_ENDPOINTS};
use array::Array;

const PACKET_QUEUE_CAPACITY: usize = 2;

/// Represents queue of the packets to send over USB.
#[derive(Copy, Clone)]
pub struct PacketQueue {
    buffer: [Option<Array<u8>>; PACKET_QUEUE_CAPACITY * SUPPORTED_ENDPOINTS.len()],
}

impl PacketQueue {
    /// Creates empty queue.
    pub fn new() -> Self {
        PacketQueue {
            buffer: [None; PACKET_QUEUE_CAPACITY * 3],
        }
    }

    /// Adds raw data for the specified endpoint to the queue, if the data cannot fit into one
    /// packet it will be split into multiple packets each of the `MAX_PACKET_SIZE`. If data is
    /// larger than the capacity of the queue it will be trimmed.
    pub fn enqueue(&mut self, endpoint: EndpointType, raw_packet: &[u8]) {
        // Split data into separate packets of `MAX_PACKET_SIZE` size, take only first
        // `PACKET_QUEUE_CAPACITY` packets and reverse them.
        let mut raw_packets = raw_packet
            .chunks(MAX_PACKET_SIZE)
            .take(PACKET_QUEUE_CAPACITY)
            .rev();
        let start_index = Self::start_index(endpoint);
        for index in 0..PACKET_QUEUE_CAPACITY {
            self.buffer[start_index + index] = raw_packets.next().map(Array::from);
        }
    }

    /// Returns next packet for the specified endpoint if any.
    pub fn dequeue(&mut self, endpoint: EndpointType) -> Option<Array<u8>> {
        let start_index = Self::start_index(endpoint);
        self.buffer[start_index..start_index + PACKET_QUEUE_CAPACITY]
            .iter()
            .rposition(|packet| packet.is_some())
            .and_then(|index| self.buffer[start_index + index].take())
    }

    /// Clears the queue for the specified endpoint.
    pub fn clear(&mut self, endpoint: EndpointType) {
        let start_index = Self::start_index(endpoint);
        for index in 0..PACKET_QUEUE_CAPACITY {
            self.buffer[start_index + index] = None;
        }
    }

    /// Determines the start index of the queue portion reserved for a specified endpoint.
    fn start_index(endpoint: EndpointType) -> usize {
        Into::<u8>::into(endpoint) as usize * PACKET_QUEUE_CAPACITY
    }
}

#[cfg(test)]
mod tests {
    use super::{super::endpoint::DeviceEndpoint, *};
    use std::prelude::v1::*;
    use usb::descriptors::MAX_PACKET_SIZE;

    #[test]
    fn empty_queue() {
        let mut queue = PacketQueue::new();
        assert_eq!(queue.dequeue(EndpointType::Control), None);
        assert_eq!(
            queue.dequeue(EndpointType::Device(DeviceEndpoint::System)),
            None
        );
        assert_eq!(
            queue.dequeue(EndpointType::Device(DeviceEndpoint::Keyboard)),
            None
        );
    }

    #[test]
    fn queue_with_one_endpoint() {
        let mut queue = PacketQueue::new();
        queue.enqueue(EndpointType::Control, &[1, 2, 3]);

        let packet = queue.dequeue(EndpointType::Control);
        assert_eq!(packet.is_some(), true);
        assert_eq!(packet.unwrap().as_ref(), &[1, 2, 3]);

        assert_eq!(queue.dequeue(EndpointType::Control), None);
        assert_eq!(
            queue.dequeue(EndpointType::Device(DeviceEndpoint::System)),
            None
        );
        assert_eq!(
            queue.dequeue(EndpointType::Device(DeviceEndpoint::Keyboard)),
            None
        );
    }

    #[test]
    fn queue_with_all_endpoints() {
        let mut queue = PacketQueue::new();
        queue.enqueue(EndpointType::Control, &[1, 2, 3]);
        queue.enqueue(EndpointType::Device(DeviceEndpoint::System), &[4, 5, 6]);
        queue.enqueue(EndpointType::Device(DeviceEndpoint::Keyboard), &[7, 8, 9]);

        let packet = queue.dequeue(EndpointType::Control);
        assert_eq!(packet.is_some(), true);
        assert_eq!(packet.unwrap().as_ref(), &[1, 2, 3]);
        assert_eq!(queue.dequeue(EndpointType::Control), None);

        let packet = queue.dequeue(EndpointType::Device(DeviceEndpoint::System));
        assert_eq!(packet.is_some(), true);
        assert_eq!(packet.unwrap().as_ref(), &[4, 5, 6]);
        assert_eq!(
            queue.dequeue(EndpointType::Device(DeviceEndpoint::System)),
            None
        );

        let packet = queue.dequeue(EndpointType::Device(DeviceEndpoint::Keyboard));
        assert_eq!(packet.is_some(), true);
        assert_eq!(packet.unwrap().as_ref(), &[7, 8, 9]);
        assert_eq!(
            queue.dequeue(EndpointType::Device(DeviceEndpoint::Keyboard)),
            None
        );
    }

    #[test]
    fn queue_with_multiple_packets() {
        // Data represents 2 packets (0..64), (64..100).
        let data: Vec<u8> = (0u8..100).into_iter().collect();
        let mut data_ranges = vec![(0..MAX_PACKET_SIZE), (MAX_PACKET_SIZE..data.len())];

        let mut queue = PacketQueue::new();
        queue.enqueue(EndpointType::Control, &data);
        queue.enqueue(EndpointType::Device(DeviceEndpoint::System), &data);
        queue.enqueue(EndpointType::Device(DeviceEndpoint::Keyboard), &data);

        for range in data_ranges.drain(..) {
            let data_range = &data[range];
            let packet = queue.dequeue(EndpointType::Control);
            assert_eq!(packet.is_some(), true);
            assert_eq!(packet.unwrap().as_ref(), data_range);

            let packet = queue.dequeue(EndpointType::Device(DeviceEndpoint::System));
            assert_eq!(packet.is_some(), true);
            assert_eq!(packet.unwrap().as_ref(), data_range);

            let packet = queue.dequeue(EndpointType::Device(DeviceEndpoint::Keyboard));
            assert_eq!(packet.is_some(), true);
            assert_eq!(packet.unwrap().as_ref(), data_range);
        }

        assert_eq!(queue.dequeue(EndpointType::Control), None);
        assert_eq!(
            queue.dequeue(EndpointType::Device(DeviceEndpoint::System)),
            None
        );
        assert_eq!(
            queue.dequeue(EndpointType::Device(DeviceEndpoint::Keyboard)),
            None
        );
    }

    #[test]
    fn queue_with_overflow() {
        // Data represents 4 packets, but queue will only process 2 packets: (0..64), (64..128),
        // (128..192), (192..256).
        let data: Vec<u8> = (0u8..=u8::max_value()).into_iter().collect();
        let mut data_ranges = vec![(0..MAX_PACKET_SIZE), (MAX_PACKET_SIZE..MAX_PACKET_SIZE * 2)];

        let mut queue = PacketQueue::new();
        queue.enqueue(EndpointType::Control, &data);
        queue.enqueue(EndpointType::Device(DeviceEndpoint::System), &data);
        queue.enqueue(EndpointType::Device(DeviceEndpoint::Keyboard), &data);

        for range in data_ranges.drain(..) {
            let data_range = &data[range];
            let packet = queue.dequeue(EndpointType::Control);
            assert_eq!(packet.is_some(), true);
            assert_eq!(packet.unwrap().as_ref(), data_range);

            let packet = queue.dequeue(EndpointType::Device(DeviceEndpoint::System));
            assert_eq!(packet.is_some(), true);
            assert_eq!(packet.unwrap().as_ref(), data_range);

            let packet = queue.dequeue(EndpointType::Device(DeviceEndpoint::Keyboard));
            assert_eq!(packet.is_some(), true);
            assert_eq!(packet.unwrap().as_ref(), data_range);
        }

        assert_eq!(queue.dequeue(EndpointType::Control), None);
        assert_eq!(
            queue.dequeue(EndpointType::Device(DeviceEndpoint::System)),
            None
        );
        assert_eq!(
            queue.dequeue(EndpointType::Device(DeviceEndpoint::Keyboard)),
            None
        );
    }

    #[test]
    fn clear_empty_queue() {
        let mut queue = PacketQueue::new();
        assert_eq!(queue.dequeue(EndpointType::Control), None);
        assert_eq!(
            queue.dequeue(EndpointType::Device(DeviceEndpoint::System)),
            None
        );
        assert_eq!(
            queue.dequeue(EndpointType::Device(DeviceEndpoint::Keyboard)),
            None
        );

        queue.clear(EndpointType::Control);
        queue.clear(EndpointType::Device(DeviceEndpoint::System));
        queue.clear(EndpointType::Device(DeviceEndpoint::Keyboard));

        assert_eq!(queue.dequeue(EndpointType::Control), None);
        assert_eq!(
            queue.dequeue(EndpointType::Device(DeviceEndpoint::System)),
            None
        );
        assert_eq!(
            queue.dequeue(EndpointType::Device(DeviceEndpoint::Keyboard)),
            None
        );
    }

    #[test]
    fn clear_non_empty_queue() {
        println!("Size: {:?}", core::mem::size_of::<PacketQueue>());
        let data: Vec<u8> = (0u8..=u8::max_value()).into_iter().collect();

        let mut queue = PacketQueue::new();
        queue.enqueue(EndpointType::Control, &data);
        queue.enqueue(EndpointType::Device(DeviceEndpoint::System), &data);
        queue.enqueue(EndpointType::Device(DeviceEndpoint::Keyboard), &data);

        queue.clear(EndpointType::Control);
        queue.clear(EndpointType::Device(DeviceEndpoint::System));
        queue.clear(EndpointType::Device(DeviceEndpoint::Keyboard));

        assert_eq!(queue.dequeue(EndpointType::Control), None);
        assert_eq!(
            queue.dequeue(EndpointType::Device(DeviceEndpoint::System)),
            None
        );
        assert_eq!(
            queue.dequeue(EndpointType::Device(DeviceEndpoint::Keyboard)),
            None
        );
    }
}
