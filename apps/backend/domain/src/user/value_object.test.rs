pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// テストモジュール
#[cfg(test)]
mod tests {
    // テスト対象の関数をインポート
    use super::add;

    // テスト関数
    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
    }
}
