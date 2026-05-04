use std::error::Error;
use shared_memory::{Shmem, ShmemConf};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MemData {
    pub timestamp: u64,      // 0x00-0x07: Unix 时间戳（秒）
    pub position: u64,       // 0x08-0x0F: 坐标数据
}

pub struct MemHandle {
    mem: Shmem,
}

impl MemHandle {
    
    pub fn init() -> Result<Self, Box<dyn Error>> {
        let uuid = std::env::var("OLL_SHMEM_ID")?;

        let shmem = ShmemConf::new()
            .os_id(uuid)
            .open()?;
        
        Ok(Self { 
            mem: shmem 
        })
    }

    pub fn write(&mut self, x: i32, y: i32) -> Result<(), Box<dyn std::error::Error>> {
        let data = MemData {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs(),
            position: ((y as u64) << 32) | (x as u32 as u64),
        };
        unsafe {
            std::ptr::write_volatile(self.mem.as_ptr() as *mut MemData, data);
        }
        Ok(())
    }
}