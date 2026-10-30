use std::{collections::HashMap, fmt::Error, ops::Add};

use bytes::BytesMut;
use futures::stream::StreamExt;
use futures_core::Stream;

use crate::codecs::PacketFrame;

pub trait TryIntoFrom<T, D>: Sized {
    fn try_into_from(value: T) -> Result<D, Error>;
}



#[derive(Debug)]
pub struct EnvMapper {}

impl TryIntoFrom<String, HashMap<String, String>> for EnvMapper {
    fn try_into_from(value: String) -> Result<HashMap<String, String>, Error> {
        let mut bar: HashMap<String, String> = HashMap::new();

        let foo: Vec<&str> = value.split(';').collect();
        
        if foo.len() % 2 != 0 {
            return Err(Error::default());
        }
        
        let mut last = "";
        for v in foo.into_iter() {
            if (last.is_empty()) {
                last = v;
            } else {
                if (last == v) {
                    return Err(Error::default());
                }
                bar.insert(String::from(last), String::from(v));
                last = "";
            }
        }

        Ok(bar)
    }
}













pub trait TryMutAsyncFrom<T, D>: Sized {
    async fn try_mut_async_from(value: &mut T) -> Result<D, Error>;
}


pub struct RoamingMapper {}

pub struct RoamingResult {
    pub domain: String,
    pub bytes: BytesMut
}

impl<T> TryMutAsyncFrom<T, RoamingResult> for RoamingMapper 
where 
    T: Stream<Item = Result<PacketFrame, std::io::Error>> + Unpin
{
    async fn try_mut_async_from(mut value: &mut T) -> Result<RoamingResult, Error> {
        //6
        //1037
        if let Some(Ok(mut frame)) = value.next().await {
            if (frame.bytes.len() < 6 || frame.bytes.len() > 1037) {return Err(Error::default());}
            let offset = frame.offset + 1 + 2;
            
            let src = &mut frame.bytes[offset..];


            let mut position = 0;
            let mut full_lenght: u32 = 0;
            loop {
                let current_byte = src[position];
                full_lenght |= ((current_byte & 0x7F) as u32) << (position * 7);

                position += 1;
                if (current_byte & 0x80) == 0 {
                    break;
                }

                if position >= 5 {
                    return Err(Error::default());
                }
            }

            let src = &mut frame.bytes[offset + position..offset + position + full_lenght as usize];
            let mut src = String::from_utf8(src.to_vec()).unwrap();

            src += ":";
            src += &u16::from_be_bytes(frame.bytes[offset + position + full_lenght as usize..offset + position + full_lenght as usize + 2].try_into().unwrap()).to_string();

            return Ok(RoamingResult { domain: src, bytes: frame.bytes });
            
        }

        return Err(Error::default());
    }
}