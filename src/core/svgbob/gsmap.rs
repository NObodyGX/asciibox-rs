use super::node::{GArrow, GDirect, GNode};
use super::parse::{parse_arrow, parse_node, valid_arrow_check, valid_node_check};
use nom::IResult;
use std::cmp::max;

#[derive(Debug, Clone)]
pub struct GBoard {
    // 记录所有 node 信息
    pub nodes: Vec<GNode>,
    pub board: Vec<Vec<u16>>,
    pub w: u16,
    pub h: u16,
    idx: u16,
}

impl GBoard {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            board: Vec::new(),
            w: 0,
            h: 0,
            idx: 1,
        }
    }

    pub fn add_node(&mut self, node: &GNode) -> bool {
        for x in self.nodes.iter() {
            if x.id.eq(&node.id) {
                return false;
            }
        }
        let mut a_node = node.clone();
        a_node.idx = self.idx;
        self.idx += 1;
        self.h = max(a_node.x + 1, self.h);
        self.w = max(a_node.y + 1, self.w);
        self.nodes.push(a_node);
        true
    }

    pub fn get_node(&mut self, id: &String) -> Option<&GNode> {
        if self.nodes.is_empty() {
            return None;
        }
        for node in self.nodes.iter() {
            if node.id.eq(id) {
                return Some(node);
            }
        }
        return None;
    }

    fn get_node_by_id(&self, idx: &u16) -> Option<&GNode> {
        if self.nodes.is_empty() {
            return None;
        }
        for node in self.nodes.iter() {
            if node.idx.eq(idx) {
                return Some(node);
            }
        }
        return None;
    }

    fn rebuild_borad(&mut self) {
        self.board = Vec::new();
        let h = self.h + 9;
        let w = self.w + 9;
        for _ih in 0..h {
            let mut a: Vec<u16> = Vec::new();
            for _ in 0..w {
                a.push(0);
            }
            self.board.push(a);
        }
    }

    fn add_nodes_into_board(&mut self) {
        for node in self.nodes.iter() {
            let x = node.x;
            let y = node.y;
            match self.board.get_mut(x as usize) {
                Some(v) => {
                    v[y as usize] = node.idx;
                }
                None => {}
            }
        }
    }

    fn link_arrow_to_node(&mut self, src:&String, dst:&String, arrow: &GArrow) {
        for node in self.nodes.iter_mut() {
            if node.id.eq(src) {
                node.add_arrow(arrow, true);
                continue;
            }
            if node.id.eq(dst) {
                node.add_arrow(arrow, false);
            }
        }
    }

    fn relocate_right(&mut self, id:&String, x:u16, y:u16) {
        for node in self.nodes.iter_mut() {
            if node.id.eq(id) {
                node.x = x;
                node.y = y;
                continue;
            }
            if node.x == x && node.y >=y {
                node.y += 1;
            }
        }
    }

    pub fn load_arrows(&mut self, arrows: &Vec<GArrow>) -> Option<&str> {
        self.rebuild_borad();
        for arrow in arrows {
            let src = &arrow.src;
            let dst = &arrow.dst;
            if src.eq(dst) {
                continue;
            }
            let lnode = self.get_node(src)?;
            let x = lnode.x;
            let y = lnode.y;
            match arrow.direct {
                GDirect::Left => {
                    self.relocate_right(&dst, x, max(1, y) - 1);
                    self.link_arrow_to_node(src, dst, arrow);
                }
                GDirect::Right => {
                    self.relocate_right(&dst, x,  y + 1);
                    self.link_arrow_to_node(src, dst, arrow);
                }
                _ => {}
            }
        }
        self.add_nodes_into_board();
        Some("")
    }

    pub fn show(&self) -> String {
        let mut w_val: Vec<usize> = Vec::new(); // 每行 cell 的宽度
        let mut h_val: Vec<usize> = Vec::new(); // 每行的高度
        for _ in 0..max(self.w + 9, self.h + 9) {
            w_val.push(0);
            h_val.push(0);
        }
        // 先计算显示的长宽
        for node in self.nodes.iter() {
            w_val[node.y as usize] = max(w_val[node.y as usize], node.total_w());
            h_val[node.x as usize] = max(h_val[node.x as usize], node.total_h());
        }
        // 逐行打印
        let mut content = String::new();

        for (x, items) in self.board.iter().enumerate() {
            let mut linestr: String = String::new();
            if x >= h_val.len() {
                break;
            }
            for h in 0..h_val[x as usize] {
                for (y, idx) in items.iter().enumerate() {
                    if y >= w_val.len() {
                        break;
                    }
                    if idx.eq(&0) {
                        linestr.push_str(" ".repeat(w_val[y]).as_str());
                        continue;
                    }
                    match self.get_node_by_id(idx) {
                        Some(node) => {
                            linestr.push_str(
                                node.render(h as u16, h_val[x as usize], w_val[y as usize])
                                    .as_str(),
                            );
                        }
                        None => {
                            linestr.push_str(" ".repeat(w_val[y]).as_str());
                        }
                    }
                }
                linestr.push('\n');
            }
            content.push_str(linestr.trim_end());
            // trim_end 会清除最后的换行
            content.push('\n');
        }
        content
    }
}

#[derive(Debug)]
pub struct GSMap {
    board: GBoard,
    arrows: Vec<GArrow>,
}

impl GSMap {
    pub fn new() -> Self {
        Self {
            board: GBoard::new(),
            arrows: Vec::new(),
        }
    }

    pub fn load_content(&mut self, content: &str) -> String {
        // let mut lines = content.lines();
        let mut lines: Vec<&str> = content.split('\n').filter(|&s| !s.is_empty()).collect();
        let mut linenum: u16 = 0;
        for line in lines.iter_mut() {
            match self.parse_line(line, linenum) {
                Ok(_) => {
                    linenum += 1;
                }
                Err(e) => {
                    println!("{:?}", e);
                    continue;
                }
            }
        }
        println!("load content done.");
        self.board.load_arrows(&self.arrows);
        let content = self.board.show();
        content
    }

    // 逐行解析出现的节点，如果有多个节点，这几个节点默认是一排的
    // 后续依据节点之间的联系会重排节点位置
    fn parse_line<'a>(&'a mut self, line: &'a str, linenum: u16) -> IResult<&str, &str> {
        let mut text: &str;
        let mut vtext: &str;
        let mut direct: GDirect;
        let mut lid: String;
        let mut rid: String;
        let mut node: GNode;
        let mut w: u16 = 0;

        // 第一个 node
        (text, vtext) = valid_node_check(line)?;
        let (id, name) = parse_node(vtext)?;
        node = GNode::new(id.to_string(), name.to_string(), linenum, w);
        lid = node.id.clone();
        self.board.add_node(&node);
        loop {
            w += 1;
            if text.len() < 3 {
                break;
            }
            // 再接着 arrow
            (text, vtext) = valid_arrow_check(text)?;
            direct = parse_arrow(vtext);
            if text.len() <= 0 {
                break;
            }
            w += 1;
            // 再接着 node
            (text, vtext) = valid_node_check(text)?;
            let (id, name) = parse_node(vtext)?;
            node = GNode::new(id.to_string(), name.to_string(), linenum, w);
            rid = node.id.clone();
            self.board.add_node(&node);
            self.arrows.push(GArrow::new(direct, lid, rid.clone()));
            lid = rid;
        }
        Ok(("", ""))
    }
}