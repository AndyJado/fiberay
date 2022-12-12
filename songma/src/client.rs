use std::{io::Stdin, mem::swap};

pub enum AppState {
    Welcome,
    Ask,
    Tell,
    DarkCorner,
    Walking,
}

impl AppState {
    pub fn home(&mut self) {
        swap(self, &mut Self::Welcome)
    }
    pub fn ask(&mut self) {
        swap(self, &mut Self::Ask)
    }
    pub fn lost(&mut self) {
        swap(self, &mut Self::DarkCorner)
    }
    pub fn walking(&mut self) {
        swap(self, &mut Self::Walking)
    }
}

impl std::fmt::Display for AppState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            AppState::Welcome => {
                "👋,你好哇,又见面了\n按1问我我知道的\n按2告诉我我不知道的\n Ctrl C 退出"
            }
            AppState::Ask => "🙋问吧",
            AppState::Tell => "📖报告地址?",
            AppState::DarkCorner => "你不该来这的,回去吧",
            AppState::Walking => "好了,现在呢?",
        };
        std::write!(f, "{s}")
    }
}
