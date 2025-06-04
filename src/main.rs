use mimalloc::MiMalloc;

/// 更换全局内存管理器
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() {
    serv::run();
}
