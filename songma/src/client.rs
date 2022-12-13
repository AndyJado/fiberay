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
                "👋 \n键盘输入<试验代码> + 空格 + <材料参数>:\nT90; C0; C90; V; M; A; L; SS; F\n回车 确认 或 Ctrl C 退出👇"
            }
            AppState::Ask => "🙋问吧",
            AppState::Tell => "📖报告地址?",
            AppState::DarkCorner => "你不该来这的,回去吧",
            AppState::Walking => "成功,请继续",
        };
        std::write!(f, "{s}")
    }
}
