use mimalloc::MiMalloc;

/// 更换全局内存管理器
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() {
    // 注册日志模块，保持无阻塞日志模块的守护者
    let _gaurds = log::init();
    serv::run().unwrap();
}
