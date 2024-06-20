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

impl RBox {
    pub fn new() -> Self {
        Self {
            left: ADirect::None,
            right: ADirect::None,
            up: ADirect::None,
            down: ADirect::None,
            left_down: ADirect::None,
            w_left: 0,
            w_right: 0,
            h_up: 0,
            h_down: 0,
        }
    }

    pub fn set_left_w(&mut self, w: usize) {
        self.w_left = std::cmp::max(self.w_left, w);
    }
    pub fn set_right_w(&mut self, w: usize) {
        self.w_right = std::cmp::max(self.w_right, w);
    }
    pub fn set_up_h(&mut self, w: usize) {
        self.h_up = std::cmp::max(self.h_up, w);
    }
    pub fn set_down_h(&mut self, w: usize) {
        self.h_down = std::cmp::max(self.h_down, w);
    }
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
    // 是否浮动
    pub floating: usize,
    // render 用 box 解构
    rbox: RBox,
    // render 用形状
    sharp: ASharp,
}

impl ANode {
    #[must_use]
    pub fn new(id: String, name: String, x: usize, y: usize, sharp: ASharp) -> Self {
        let nid: String = id.trim().to_string();
        let nname: String = name.trim().to_string();
        let pwords: Vec<&str> = nname.split('\n').collect();
        let mut words = Vec::new();
        let h: usize = pwords.len() as usize;
        let mut w: usize = 0;
        for word in pwords {
            w = std::cmp::max(w, cn_length(word) as usize);
            words.push(word.to_string());
        }
        let mbox = RBox::new();

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
            rbox: mbox,
            sharp,
            floating: 0,
        }
    }

    /// 向 node 添加 arrow
    /// - arrow: 要添加的 GArrow
    /// - direct: 要添加的方向
    /// - enable_render: 是否需要被绘制
    pub fn add_arrow(&mut self, arrow: &AEdge, direct: ADirect, enable_render: bool) {
        if !enable_render {
            self.arrows_no_render.push(arrow.clone());
            return;
        }
        self.arrows.push(arrow.clone());
        match direct {
            ADirect::Left => {
                self.rbox.left = arrow.direct.clone();
                self.rbox.set_left_w(if arrow.direct == ADirect::Double {
                    4
                } else {
                    3
                });
            }
            ADirect::Right => {
                self.rbox.right = arrow.direct.clone();
                self.rbox.set_right_w(if arrow.direct == ADirect::Double {
                    4
                } else {
                    3
                });
            }
            ADirect::Up => {
                self.rbox.up = arrow.direct.clone();
                self.rbox.set_up_h(2);
            }
            ADirect::Down => {
                self.rbox.down = arrow.direct.clone();
                self.rbox.set_down_h(2);
            }
            ADirect::LeftDown => {
                self.rbox.left_down = arrow.direct.clone();
                self.rbox.set_down_h(2);
                self.rbox.set_left_w(3);
            }
            _ => {}
        }
    }

    fn render_arrow(&self, i: usize) -> (String, String) {
        let mut lcontent = String::new();
        let mut rcontent = String::new();

        if self.rbox.w_left > 0 {
            let v = if i != (self.h + 1) / 2 {
                " ".repeat(self.rbox.w_left)
            } else {
                match self.rbox.left {
                    ADirect::Left => {
                        format!("<{}", "-".repeat(self.rbox.w_left - 1))
                    }
                    ADirect::Right => {
                        format!("{}>", "-".repeat(self.rbox.w_left - 1))
                    }
                    ADirect::Double => {
                        format!("<{}>", "-".repeat(self.rbox.w_left - 2))
                    }
                    // GDirect::LeftDown => {
                    //     format!("-{}-", "-".repeat(self.mbox.w_left - 2))
                    // }
                    _ => " ".repeat(self.rbox.w_left),
                }
            };
            lcontent.push_str(v.as_str());
        }

        if self.rbox.w_right > 0 {
            let v = if i != (self.h + 1) / 2 {
                " ".repeat(self.rbox.w_right)
            } else {
                match self.rbox.right {
                    ADirect::Left => {
                        format!("<{}", "-".repeat(self.rbox.w_right - 1))
                    }
                    ADirect::Right => {
                        format!("{}>", "-".repeat(self.rbox.w_right - 1))
                    }
                    ADirect::Double => {
                        format!("<{}>", "-".repeat(self.rbox.w_right - 2))
                    }
                    _ => " ".repeat(self.rbox.w_right),
                }
            };
            rcontent.push_str(v.as_str());
        }
        (lcontent, rcontent)
    }

    pub fn render(
        &self,
        i: usize,
        _maxh: usize,
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
                let (lastr, rastr) = self.render_arrow(i);
                let lbank = (self.w as usize + 2 - cn_length(cword) + 1) / 2;
                let rbank = self.w as usize + 2 - cn_length(cword) - lbank;
                if expand_mode {
                    let lstr = " ".repeat(lb + lbank);
                    let rstr = " ".repeat(rb + rbank);
                    return format!("{}|{}{}{}|{}", lastr, lstr, cword, rstr, rastr);
                }
                let lstr = " ".repeat(lbank);
                let rstr = " ".repeat(rbank);
                return format!(
                    "{}{}|{}{}{}|{}{}",
                    " ".repeat(lb),
                    lastr,
                    lstr,
                    cword,
                    rstr,
                    rastr,
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

    pub fn render_up(&self, i: usize, _maxh: usize, cw: usize, lw: usize, rw: usize) -> String {
        if self.rbox.h_up <= 0 {
            return format!("{}", " ".repeat(lw + cw + rw));
        }
        // 将 cw 分隔成 lb + 1 + rb
        let lb: usize = (cw - 1) / 2;
        let rb: usize = cw - 1 - lb;
        if i == 0 {
            return format!("{}^{}", " ".repeat(lb + lw), " ".repeat(rb + rw));
        } else if i <= self.rbox.h_up - 1 {
            return format!("{}|{}", " ".repeat(lb + lw), " ".repeat(rb + rw));
        }
        return format!("{}", " ".repeat(lw + cw + rw));
    }

    pub fn render_down(&self, i: usize, _maxh: usize, cw: usize, lw: usize, rw: usize) -> String {
        if self.rbox.h_down <= 0 {
            return format!("{}", " ".repeat(lw + cw + rw));
        }
        // 将 cw 分隔成 lb + 1 + rb
        let lb: usize = (cw - 1) / 2;
        let rb: usize = cw - 1 - lb;
        if i == self.rbox.h_down - 1 {
            return format!("{}v{}", " ".repeat(lb + lw), " ".repeat(rb + rw));
        } else if i < self.rbox.h_down - 1 {
            return format!("{}|{}", " ".repeat(lb + lw), " ".repeat(rb + rw));
        }
        return format!("{}", " ".repeat(lw + cw + rw));
    }

    pub fn content_w(&self) -> usize {
        return self.w as usize + 2;
    }

    pub fn left_w(&self) -> usize {
        return self.rbox.w_left;
    }

    pub fn right_w(&self) -> usize {
        return self.rbox.w_right;
    }

    pub fn total_h(&self) -> usize {
        return self.rbox.h_up + self.h as usize + 2 + self.rbox.h_down;
    }

    pub fn up_h(&self) -> usize {
        return self.rbox.h_up;
    }

    pub fn down_h(&self) -> usize {
        return self.rbox.h_down;
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
