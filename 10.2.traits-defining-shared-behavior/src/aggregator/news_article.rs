use super::summary::Summary;

pub struct NewsArticle {
    pub headline: String,
    pub locatoin: String,
    pub author: String,
    pub content: String,
}

/// trait 혹은 type 둘 중 하나는 현재 crate 에 포함되어 있어야 impl 할 수 있음.
/// 이유: 그렇지 않은 경우 하나의 type 에 똑같은 impl trait 이 여러개가 될 수 있음.
/// 이러면 컴파일러가 어떤걸 선택해야 할지 컴파일 타임에 알 수 없음.
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.locatoin)
    }
}
