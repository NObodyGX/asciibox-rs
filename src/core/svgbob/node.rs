use crate::core::utils::cn_length;
use std::{fmt, ops::Not};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ADirect {
    None,
    Double,
    Left,
    Right,
    Up,
    Down,
    LeftUp,
    LeftDown,
    RightUp,
    RightDown,
    // UpLeft,
    // UpRight,
    // DownLeft,
    // DownRight,
}

impl ToString for ADirect {
    fn to_string(&self) -> String {
        match self {
            ADirect::None => String::from("none"),
            ADirect::Double => String::from("double"),
            ADirect::Left => String::from("left"),
            ADirect::Right => String::from("right"),
            ADirect::Up => String::from("up"),
            ADirect::Down => String::from("down"),
            ADirect::LeftUp => String::from("leftup"),
            ADirect::LeftDown => String::from("leftdown"),
            ADirect::RightUp => String::from("rightup"),
            ADirect::RightDown => String::from("rightdown"),
        }
    }
}

impl Not for ADirect {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            ADirect::None => ADirect::None,
            ADirect::Double => ADirect::Double,
            ADirect::Left => ADirect::Right,
            ADirect::Right => ADirect::Left,
            ADirect::Up => ADirect::Down,
            ADirect::Down => ADirect::Up,
            ADirect::LeftUp => ADirect::RightDown,
            ADirect::LeftDown => ADirect::RightUp,
            ADirect::RightUp => ADirect::LeftDown,
            ADirect::RightDown => ADirect::LeftUp,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct RBox {
    pub w_left: usize,
    pub w_right: usize,
    pub h_up: usize,
    pub h_down: usize,
    pub left: ADirect,
    pub right: ADirect,
    pub up: ADirect,
    pub down: ADirect,
    pub left_down: ADirect,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ASharp {
    Round,
    Square,
    Circle,
}

#[derive(Clone, Debug, Eq, Hash)]
pub struct ANode {
    // 节点排序用序号
    pub idx: usize,
    // 节点 id
    pub id: String,
    // 节点展示内容原始值
    pub name: String,
    // 横坐标，对应水平行上的位置
    pub x: usize,
    // 纵坐标，对应垂直列上的位置
    pub y: usize,
    // 内容宽度
    pub w: usize,
    // 内容高度
    pub h: usize,
    // 具体每行内容
    words: Vec<String>,
    // 周围可用的箭头
    pub arrows: Vec<AEdge>,
    pub arrows_no_render: Vec<AEdge>,
    // render 用形状
    sharp: ASharp,
}

impl ANode {
    #[must_use]
    pub fn new(id: &str, name: String, x: usize, y: usize) -> Self {
        let nid = String::from(id).trim().to_string();
        let nname: String = name.trim().to_string();
        let pwords: Vec<&str> = nname.split('\n').collect();
        let mut words = Vec::new();
        let h: usize = pwords.len() as usize;
        let mut w: usize = 0;
        for word in pwords {
            w = std::cmp::max(w, cn_length(word) as usize);
            words.push(word.to_string());
        }
        Self {
            id: nid,
            name: nname,
            x,
            y,
            w,
            h,
            words,
            arrows: Vec::new(),
            arrows_no_render: Vec::new(),
            idx: 0,
            sharp: ASharp::Round,
        }
    }

    pub fn set_sharp(&mut self, sharp: ASharp) {
        self.sharp = sharp;
    }

    pub fn render(
        &self,
        i: usize,
        _ch: usize,
        cw: usize,
        lw: usize,
        rw: usize,
        expand_mode: bool,
    ) -> String {
        let lb: usize = (cw - self.content_w() + 1) / 2;
        let rb: usize = cw - self.content_w() - lb;

        if i == 0 || i == self.h + 1 {
            let spc = if self.sharp == ASharp::Square {
                "+"
            } else {
                if i == 0 {
                    "."
                } else {
                    "'"
                }
            };

            if expand_mode {
                let lstr = " ".repeat(lw);
                let rstr = " ".repeat(rw);
                let cstr = "-".repeat(cw);
                return format!("{}{}{}{}{}", spc, lstr, cstr, spc, rstr);
            }
            let lstr = " ".repeat(lb + lw);
            let rstr = " ".repeat(rb + rw);
            let cstr = "-".repeat(self.content_w());
            return format!("{}{}{}{}{}", lstr, spc, cstr, spc, rstr);
        } else if i >= self.h + 2 {
            // 超出行
            return format!("{}", " ".repeat(lw + cw + rw));
        }
        // 内容行
        match self.words.get(i as usize - 1) {
            Some(cword) => {
                let lbank = (self.w as usize + 2 - cn_length(cword) + 1) / 2;
                let rbank = self.w as usize + 2 - cn_length(cword) - lbank;
                if expand_mode {
                    let lstr = " ".repeat(lb + lbank);
                    let rstr = " ".repeat(rb + rbank);
                    return format!("|{}{}{}|", lstr, cword, rstr);
                }
                let lstr = " ".repeat(lbank);
                let rstr = " ".repeat(rbank);
                return format!(
                    "{}|{}{}{}|{}",
                    " ".repeat(lb),
                    lstr,
                    cword,
                    rstr,
                    " ".repeat(rb)
                );
            }
            None => {
                let ww: usize = self.w as usize + 2;
                return format!(
                    "{}|{}|{}",
                    " ".repeat(lb + lw),
                    " ".repeat(ww),
                    " ".repeat(rb + rw),
                );
            }
        }
    }

    pub fn content_w(&self) -> usize {
        return self.w as usize + 2;
    }

    pub fn content_h(&self) -> usize {
        return self.h as usize + 2;
    }
}

impl fmt::Display for ANode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GNode({})", self.id)
    }
}

impl PartialEq for ANode {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.name == other.name
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AEdge {
    pub direct: ADirect,
    pub src: String,
    pub dst: String,
    pub text: String,
}

impl AEdge {
    pub fn new(direct: ADirect, from: String, to: String, text: String) -> Self {
        Self {
            direct,
            src: from,
            dst: to,
            text,
        }
    }
}

impl fmt::Display for AEdge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GArrow({} -{:?}- {})", self.src, self.direct, self.dst)
    }
}
