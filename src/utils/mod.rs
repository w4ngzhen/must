/// 判断一个unicode字符是否是宽体的字符
/// 我们认为一个中文字符算宽体字符
pub fn is_wide_char(c: char) -> bool {
    (c >= '\u{4E00}' && c <= '\u{9FFF}')  // 基本多文种平面的CJK统一汉字块
        || (c >= '\u{3400}' && c <= '\u{4DBF}') // 其他补充区汉字块
}
