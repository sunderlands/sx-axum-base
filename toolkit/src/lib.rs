pub mod time {
    use time::{
        UtcOffset,
        format_description::BorrowedFormatItem,
        macros::{format_description, offset},
    };

    /// 北京时间偏移量
    pub const OFFSET_BEIJING: UtcOffset = offset!(+8);

    /// 本项目通用时间格式化：yyyy-MM-dd hh-mm-ss
    pub const FORMAT_UTIL: &[BorrowedFormatItem] =
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
}

pub mod to_string {
    pub fn err_chain(err: anyhow::Error) -> String {
        let mut rs = String::new();
        for (i, casue) in err.chain().enumerate() {
            rs.push_str(&format!(
                "----【错误 {}】------------------------------\n",
                i + 1
            ));
            rs.push_str(&format!("{}\n\n", casue));
        }
        rs
    }
}
