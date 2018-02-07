// Copyright (c) 2017 University of Glasgow
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
//
// 1. Redistributions of source code must retain the above copyright
//    notice, this list of conditions and the following disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright
//    notice, this list of conditions and the following disclaimer in the
//    documentation and/or other materials provided with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
// LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
// SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
// INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
// CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
// ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF
// THE POSSIBILITY OF SUCH DAMAGE.
//
// SPDX-License-Identifier: BSD-2-Clause






// extern crate bytes;
// 
// use bytes::{Bytes, BytesMut, Buf, BufMut, IntoBuf, BigEndian};
// 
// #[derive(Debug, PartialEq, Eq)]
// struct QuicVersion(u32);
// 
// #[derive(Debug, PartialEq, Eq)]
// struct ConnectionId(u64);
// 
// #[derive(Debug, PartialEq, Eq)]
// enum PacketNumber {
//     OneOctet(u8),
//     TwoOctet(u16),
//     FourOctet(u32)
// }
// 
// #[derive(Debug)]
// enum PacketType {
//     VersionNegotiation,
//     ClientInitial,
//     ServerStatelessRetry,
//     ServerCleartext,
//     ClientCleartext,
//     ZeroRttProtected
// }
// 
// #[derive(Debug)]
// enum QuicHeader {
// 	LongHeader{
// 		packet_type   : PacketType,
// 		connection_id : ConnectionId,
// 		packet_number : PacketNumber,
// 		version       : QuicVersion,
// 		payload       : Vec<u8>,
// 	},
// 	ShortHeader{
//         key_phase     : bool,
// 		connection_id : Option<ConnectionId>,
// 		packet_number : PacketNumber,
// 		payload       : Vec<u8>,
// 	}
// }
// 
// impl QuicHeader {
//     fn encode(self) -> Bytes {
//         let mut buf = BytesMut::with_capacity(1024);
// 
//         match self {
//             QuicHeader::LongHeader{packet_type, connection_id, packet_number, version, payload} => {
//                 // See draft-ietf-quic-transport-07 section 5.1
//                 //
//                 //  0                   1                   2                   3
//                 //  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
//                 // +-+-+-+-+-+-+-+-+
//                 // |1|   Type (7)  |
//                 // +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//                 // |                                                               |
//                 // +                       Connection ID (64)                      +
//                 // |                                                               |
//                 // +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//                 // |                       Packet Number (32)                      |
//                 // +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//                 // |                         Version (32)                          |
//                 // +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//                 // |                          Payload (*)                        ...
//                 // +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//                 match packet_type { 
//                     PacketType::VersionNegotiation   => buf.put_u8(128 + 0x01),
//                     PacketType::ClientInitial        => buf.put_u8(128 + 0x02),
//                     PacketType::ServerStatelessRetry => buf.put_u8(128 + 0x03),
//                     PacketType::ServerCleartext      => buf.put_u8(128 + 0x04),
//                     PacketType::ClientCleartext      => buf.put_u8(128 + 0x05),
//                     PacketType::ZeroRttProtected     => buf.put_u8(128 + 0x06)
//                 }
//                 match connection_id {
//                     ConnectionId(id) => buf.put_u64::<BigEndian>(id)
//                 }
//                 match packet_number {
//                     PacketNumber::OneOctet(_)  => panic!("unsupported"),
//                     PacketNumber::TwoOctet(_)  => panic!("unsupported"),
//                     PacketNumber::FourOctet(num) => buf.put_u32::<BigEndian>(num)
// 
//                 }
//                 match version {
//                     QuicVersion(v) => buf.put_u32::<BigEndian>(v)
//                 }
//                 buf.put_slice(&payload)
//             },
//             QuicHeader::ShortHeader{key_phase, connection_id, packet_number, payload} => {
//                 // See draft-ietf-quic-transport-07 section 5.2
//                 //
//                 //  0                   1                   2                   3
//                 //  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
//                 // +-+-+-+-+-+-+-+-+
//                 // |0|C|K| Type (5)|
//                 // +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//                 // |                                                               |
//                 // +                     [Connection ID (64)]                      +
//                 // |                                                               |
//                 // +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//                 // |                      Packet Number (8/16/32)                ...
//                 // +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//                 // |                     Protected Payload (*)                   ...
//                 // +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//                 let ptype = match packet_number {
//                     PacketNumber::OneOctet(_)  => 0x01,
//                     PacketNumber::TwoOctet(_)  => 0x02,
//                     PacketNumber::FourOctet(_) => 0x03
//                 };
//                 let cid = match connection_id {
//                     Some(_) => 1 << 6,
//                     None    => 0 << 6
//                 };
//                 let kpb = match key_phase {
//                     true  => 1 << 5,
//                     false => 0 << 5
//                 };
// 
//                 buf.put_u8(cid | kpb | ptype);
//                 match connection_id {
//                     Some(ConnectionId(id)) => buf.put_u64::<BigEndian>(id),
//                     None                   => {} // Do nothing
//                 }
//                 match packet_number {
//                     PacketNumber::OneOctet(num)  => buf.put_u8(num),
//                     PacketNumber::TwoOctet(num)  => buf.put_u16::<BigEndian>(num),
//                     PacketNumber::FourOctet(num) => buf.put_u32::<BigEndian>(num)
//                 }
//                 buf.put_slice(&payload);
//             }
//         }
//         buf.freeze()
//     }
// 
//     fn decode(header : Bytes) -> QuicHeader {
//         let mut buf     = header.into_buf();
//         let     initial = buf.get_u8();
//         let mut payload = vec!();
// 
//         if ((initial >> 7) & 0x01) != 0 {
//             // Long Hheader
//             let packet_type = match initial & 0b01111111 {
//                 0x01 => PacketType::VersionNegotiation,
//                 0x02 => PacketType::ClientInitial,
//                 0x03 => PacketType::ServerStatelessRetry,
//                 0x04 => PacketType::ServerCleartext,
//                 0x05 => PacketType::ClientCleartext,
//                 0x06 => PacketType::ZeroRttProtected,
//                 _    => panic!("unsupported")
//             };
//             let connection_id = ConnectionId(buf.get_u64::<BigEndian>());
//             let packet_number = PacketNumber::FourOctet(buf.get_u32::<BigEndian>());
//             let version       = QuicVersion(buf.get_u32::<BigEndian>());
//             payload.extend_from_slice(buf.bytes());
// 
//             QuicHeader::LongHeader {
//                 packet_type,
//                 connection_id,
//                 packet_number,
//                 version,
//                 payload
//             }
//         } else {
//             // Short Header
//             let has_cid   = (initial & 0b01000000) != 0;
//             let key_phase = (initial & 0b00100000) != 0;
//             let ptype     =  initial & 0b00011111;
// 
//             let connection_id = match has_cid {
//                 true  => Some(ConnectionId(buf.get_u64::<BigEndian>())),
//                 false => None
//             };
// 
//             let packet_number = match ptype {
//                 0x01 => PacketNumber::OneOctet(buf.get_u8()),
//                 0x02 => PacketNumber::TwoOctet(buf.get_u16::<BigEndian>()),
//                 0x04 => PacketNumber::FourOctet(buf.get_u32::<BigEndian>()),
//                 _    => panic!("unsupported")
//             };
//             payload.extend_from_slice(buf.bytes());
// 
//             QuicHeader::ShortHeader {
//                 key_phase,
//                 connection_id,
//                 packet_number,
//                 payload
//             }
//         }
//     }
// }
// 
// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
