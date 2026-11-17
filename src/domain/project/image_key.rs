/// 作品の画像を指すキー。ドメインは画像の置き場を知らず、キーだけを持つ。
/// ページ側がキーを画像に引き当てる（v1 は `asset!` の定数、管理画面を作ったあとはストレージの URL）。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImageKey(String);

impl From<String> for ImageKey {
    fn from(key: String) -> Self {
        Self(key)
    }
}
