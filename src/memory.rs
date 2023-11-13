use super::errors::WgseEngineError;
use anyhow::{anyhow, Result};
use binrw::{binrw, BinRead, BinWrite};
use std::io::Cursor;
use wgse_kernel::types::common::Handle;

#[binrw]
#[brw(little)]
#[repr(C, align(1))]
#[derive(Debug, Default)]
pub struct Memory {
    #[bw(calc = buff.len() as u64)]
    size: u64,
    #[br(count = size)]
    buff: Vec<u8>,
}

impl Memory {
    pub fn with_buff(buff: Vec<u8>) -> Self {
        Self { buff }
    }

    #[cfg(debug_assertions)]
    pub fn get_object<T>(&mut self, handle: Handle<T>) -> Result<T>
    where
        T: for<'a> BinRead<Args<'a> = ()> + for<'a> BinWrite<Args<'a> = ()> + Default,
    {
        use binrw::NullString;
        #[binrw]
        #[brw(little)]
        #[repr(C, align(1))]
        struct ProxyObject<T>
        where
            T: for<'a> BinRead<Args<'a> = ()> + for<'a> BinWrite<Args<'a> = ()> + Default,
        {
            pub type_info: NullString,
            pub object: T,
        }

        let mut cursor = Cursor::new(self.buff.as_mut_slice());
        cursor.set_position(handle.address as u64);

        let proxy_object = ProxyObject::<T>::read_le(&mut cursor)?;
        if proxy_object.type_info == handle.type_info {
            Ok(proxy_object.object)
        } else {
            Err(anyhow!(WgseEngineError::InconsistentTypes {
                expect: handle.type_info.to_string(),
                found: proxy_object.type_info.to_string()
            }))
        }
    }

    #[cfg(not(debug_assertions))]
    pub fn get_object<T>(&mut self, handle: Handle<T>) -> Result<T>
    where
        T: for<'a> BinRead<Args<'a> = ()> + for<'a> BinWrite<Args<'a> = ()> + Default,
    {
        let mut cursor = Cursor::new(self.buff.as_mut_slice());
        cursor.set_position(handle.address as u64);

        Ok(T::read_le(&mut cursor)?)
    }
}
