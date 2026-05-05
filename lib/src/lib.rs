//! # OmniLynceus 插件开发库
//! 
//! 该库提供了与 OmniLynceusLoader 通信的核心 API。
//! 插件通过共享内存（Shared Memory）将计算后的坐标数据实时传递给 Loader。

use std::error::Error;
use shared_memory::{Shmem, ShmemConf};

/// 共享内存中存储的原始数据结构
/// 
/// 采用 `#[repr(C)]` 以确保内存布局在跨语言或跨版本时的一致性。
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MemData {
    /// Unix 时间戳（秒）
    pub timestamp: u64,      // 0x00-0x07
    /// 坐标数据
    /// 
    /// 低 32 位存储 X 坐标，高 32 位存储 Y 坐标。
    pub position: u64,       // 0x08-0x0F
}

/// 共享内存句柄，用于管理与 Loader 的通信
pub struct MemHandle {
    mem: Shmem,
}

impl MemHandle {
    /// 初始化共享内存句柄。
    /// 
    /// # 错误
    /// 
    /// * 如果环境变量 `OLL_SHMEM_ID` 未设置（说明插件未通过 Loader 启动）。
    /// * 如果无法根据 UUID 打开指定的共享内存空间。
    pub fn init() -> Result<Self, Box<dyn Error>> {
        let uuid = std::env::var("OLL_SHMEM_ID")
            .map_err(|_| "未找到环境变量 OLL_SHMEM_ID，请确保插件由 OmniLynceusLoader 启动")?;

        let shmem = ShmemConf::new()
            .os_id(uuid)
            .open()?;
        
        Ok(Self { 
            mem: shmem 
        })
    }

    /// 将指定的鼠标坐标写入共享内存。
    /// 
    /// `x` 和 `y` 会被自动拼接并附加当前时间戳写入内存。
    /// 
    /// # 示例
    /// 
    /// ```rust
    /// # use omnilynceus_lib::MemHandle;
    /// # fn fake_run(mut mem: MemHandle) {
    /// mem.write(1920, 1080).unwrap();
    /// # }
    /// ```
    pub fn write(&mut self, x: i32, y: i32) -> Result<(), Box<dyn Error>> {
        let data = MemData {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs(),
            position: ((y as u64) << 32) | (x as u32 as u64),
        };
        
        // 使用 volatile 写入确保编译器不会优化掉这次内存修改
        unsafe {
            std::ptr::write_volatile(self.mem.as_ptr() as *mut MemData, data);
        }
        Ok(())
    }
}