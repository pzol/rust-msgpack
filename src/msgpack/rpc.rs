use super::{Value,Encoder,Decoder};
use serialize;
use serialize::{Encodable,Decodable};

pub enum RpcMessage {
  RpcRequest      {msgid: u32, method: ~str, params: ~[Value]}, // 0
  RpcResponse     {msgid: u32, error: Value, result: Value}, // 1
  RpcNotification {method: ~str, params: ~[Value]} // 2
}

impl<'a> serialize::Encodable<Encoder<'a>> for RpcMessage {
  fn encode(&self, s: &mut Encoder<'a>) {
    match *self {
      RpcRequest {msgid, ref method, ref params} => {
        (0u, msgid, method, params).encode(s); 
      }
      RpcResponse {msgid, ref error, ref result} => {
        (1u, msgid, error, result).encode(s); 
      }
      RpcNotification {ref method, ref params} => {
        (2u, method, params).encode(s);
      }
    }
  }
}

impl<'a> serialize::Decodable<Decoder<'a>> for RpcMessage {
  fn decode(s: &mut Decoder<'a>) -> RpcMessage {
    let len = s._read_vec_len();
    if len != 3 && len != 4 {
      fail!("Invalid msgpack-rpc message array length")
    }
    let ty: uint = Decodable::decode(s);

    match ty {
      0 => {
        assert!(len == 4);
        let msgid = Decodable::decode(s);
        let method = Decodable::decode(s);
        let params = Decodable::decode(s);
        RpcRequest {msgid: msgid, method: method, params: params}
      }
      1 => {
        assert!(len == 4);
        let msgid = Decodable::decode(s);
        let error = Decodable::decode(s);
        let result = Decodable::decode(s);
        RpcResponse {msgid: msgid, error: error, result: result}
      }
      2 => {
        assert!(len == 3);
        let method = Decodable::decode(s);
        let params = Decodable::decode(s);
        RpcNotification {method: method, params: params}
      }
      _ => {
        fail!("Invalid msgpack-rpc message type");
      }
    }

  }
}


