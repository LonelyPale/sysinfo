use crate::table::Args::{AsColoredString, AsStr, AsString};
use colored::{Color, ColoredString, Colorize, Style, Styles};
use std::collections::HashMap;
use std::fmt;

//标准版本：已完成
pub struct Table {
    columns: Vec<Column>,
    data: Vec<HashMap<String, String>>,
    columns_cache: HashMap<String, usize>,
    // columns_cache: HashMap<String, &'a mut Column>,
    // 1. error: lifetime may not live long enough: self.columns_cache.insert(column.key.clone(), column); argument requires that `'1` must outlive `'a`
    // 2. error[E0502]: cannot borrow `table` as immutable because it is also borrowed as mutable
    // error: 此处不能使用 &'a mut Column 来引用数据，
    // 因为'a虽然解决了self.columns_cache.insert(column.key.clone(), column); 报错argument requires that `'1` must outlive `'a`的生命周期问题，
    // 因为这样会使 pub fn refresh(&mut self) 必须改为 pub fn refresh(&'a mut self)，
    // refresh 增加 'a 后，调用refresh会使self变为&mut self一致持续到Self的整个生命周期，且无法通过{}代码块来释放所有权，无法切换到&self，println!也无法使用。
    // 解决方案：
    // #1: columns_cache: HashMap<String, Column>, 相当于存了2份Column，增加内存使用。
    // #2: columns_cache: HashMap<String, i32>, String是key，i32是columns: Vec<Column>,的索引，相当于自己通过index来查找和修改原始的Column，使用麻烦。
    // #3: 使用Rc<RefCell<T>>智能指针来跳过编译阶段的所有权和生命周期检查(运行时检查)，但会有运行时性能损失和运行时Panic。
}

impl Table {
    pub fn new(columns: Vec<Column>, data: Vec<HashMap<String, String>>) -> Self {
        let mut table = Self {
            columns,
            data,
            ..Self::default()
        };
        table.refresh_cache();
        table
    }

    fn refresh_cache(&mut self) {
        for (index, column) in self.columns.iter_mut().enumerate() {
            let len = column.title.len();
            if column.width < len {
                column.width = len;
            }
            self.columns_cache.insert(column.key.clone(), index);
        }

        for row in &self.data {
            for (key, value) in row {
                if let Some(index) = self.columns_cache.get(key) {
                    let column = &mut self.columns[*index];
                    let len = value.len();
                    if column.width < len {
                        column.width = len;
                    }
                }
            }
        }
    }

    fn fmt_header(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let last = self.columns.len() - 1;
        for (index, column) in self.columns.iter().enumerate() {
            if index == 0 {
                write!(f, "{}", column)?;
            } else if index == last {
                write!(f, "  {}\n", column)?;
            } else {
                write!(f, "  {}", column)?;
            }
        }
        Ok(())
    }

    fn fmt_row(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let last = self.columns.len() - 1;
        for row in &self.data {
            for (index, column) in self.columns.iter().enumerate() {
                let mut text = "";
                if let Some(value) = row.get(&column.key) {
                    text = value;
                }

                if index == 0 {
                    write!(f, "{}", column.format(AsStr(text)))?;
                } else if index == last {
                    write!(f, "  {}\n", column.format(AsStr(text)))?;
                } else {
                    write!(f, "  {}", column.format(AsStr(text)))?;
                }
            }
        }
        Ok(())
    }
}

impl Default for Table {
    fn default() -> Self {
        Self {
            columns: Vec::new(),
            data: Vec::new(),
            columns_cache: HashMap::new(),
        }
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_header(f)?;
        self.fmt_row(f)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Column {
    pub key: String,
    pub title: String,
    pub width: usize,
    pub hidden: bool,
    pub right_align: bool,
    pub color: Option<Color>,
    pub style: Style,
    pub render: Option<fn() -> String>,
}

pub enum Args<'a> {
    AsStr(&'a str),
    AsString(String),
    AsColoredString(ColoredString),
}

impl Column {
    fn format(&self, value: Args) -> String {
        // fn format<S: AsRef<str>>(&self, value: S) -> String {}
        // let value: &str = value.as_ref();

        if self.hidden {
            String::new()
        } else if let Some(render) = self.render {
            render()
        } else {
            let mut value = match value {
                AsStr(arg) => arg.normal(),
                AsString(arg) => arg.normal(),
                AsColoredString(arg) => arg,
            };

            //处理颜色
            let mut output = match self.color {
                Some(item) => {
                    value.fgcolor = Some(item);
                    value
                }
                None => value,
            };

            //处理样式
            output.style = output.style | self.style;

            //render
            //function(text, record, index) {}
            //生成复杂数据的渲染函数，参数分别为当前行的值，当前行数据，行索引
            // if value == "Device" || value == "Type" || value == "MountPoint" {
            //     output.style = self.style;
            // }
            // let Some(a) = match value {
            //
            //     &_ => {}
            // };
            // rust中如何让泛型函数参数是特征A或特征B
            // fn format<S: A>(&self, value: S) -> String {}

            //处理对齐
            let width = self.width;
            if self.right_align {
                format!("{output:>width$}")
            } else {
                format!("{output:<width$}")
            }
        }
    }
}

impl Default for Column {
    fn default() -> Self {
        Self {
            key: "".to_string(),
            title: "".to_string(),
            width: 0,
            hidden: false,
            right_align: false,
            color: None,
            style: Style::default(),
            render: None,
        }
    }
}

impl fmt::Display for Column {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut text = self.title.normal();
        text.style = Styles::Bold | Styles::Underline;
        write!(f, "{}", self.format(AsColoredString(text)))
        // write!(f, "{}", self.format(&self.title))
    }
}

#[test]
fn test() {
    // std::fmt::Display::fmt();
    // std::fmt::format();

    let s1 = "abc";
    let s2 = s1;
    println!("{:p}", s1);
    println!("{:p}", s2);

    println!("({})", "111");
    println!("({})", "222".hidden());
    println!("({})", "333".normal());
    println!("({})", "444".color(Color::Blue));

    assert_eq!(format!("Hello {:<5}!", "x"), "Hello x    !");
    assert_eq!(format!("Hello {:-<5}!", "x"), "Hello x----!");
    assert_eq!(format!("Hello {:^5}!", "x"), "Hello   x  !");
    assert_eq!(format!("Hello {:>5}!", "x"), "Hello     x!");
}

#[test]
fn test_hash_map() {
    let map: HashMap<String, String> = HashMap::new();
    let item = &map[""];
    println!("{}", item);
}

#[test]
fn test_column() {
    let column = Column {
        ..Column::default()
    };
    println!("{}", column.format(AsStr("aaa")));

    let column_ref = &Column {
        ..Column::default()
    };
    println!("{}", column_ref.format(AsString("bbb".to_string())));
}

#[test]
fn test_table() {
    let mut table = Table {
        columns: vec![
            Column {
                title: "AAA".to_string(),
                key: "aaa".to_string(),
                right_align: true,
                width: 10,
                color: Some(Color::Cyan),
                ..Column::default()
            },
            Column {
                title: "BBB".to_string(),
                key: "bbb".to_string(),
                ..Column::default()
            },
            Column {
                title: "CCC".to_string(),
                key: "ccc".to_string(),
                ..Column::default()
            },
        ],
        data: vec![
            HashMap::from([
                ("aaa".to_string(), "1-1".to_string()),
                ("bbb".to_string(), "1-2-222".to_string()),
                ("ccc".to_string(), "1-3-333-333".to_string()),
            ]),
            HashMap::from([("aaa".to_string(), "-1".to_string())]),
            HashMap::from([("bbb".to_string(), "-2".to_string())]),
            HashMap::from([("ccc".to_string(), "-3".to_string())]),
            HashMap::from([
                ("aaa".to_string(), "2-1".to_string()),
                ("bbb".to_string(), "2-2".to_string()),
                ("ccc".to_string(), "2-3".to_string()),
            ]),
            HashMap::from([
                ("aaa".to_string(), "3-1".to_string()),
                ("bbb".to_string(), "3-2".to_string()),
                ("ccc".to_string(), "3-3".to_string()),
            ]),
        ],
        ..Table::default()
    };

    table.refresh_cache();
    println!("{}", table);
}

#[test]
fn test4() {
    use std::cell::RefCell;
    use std::rc::Rc;

    let shared_data: Rc<RefCell<String>> = Rc::new(RefCell::new("123".to_string()));
    let shared_data_clone = Rc::clone(&shared_data);
    println!("{:?}", shared_data);

    // let a = shared_data_clone.borrow();
    // println!("{:?}", a);

    let x = shared_data_clone.clone();
    {
        let mut b = x.borrow_mut();
        *b = "asdf".to_string();
        println!("{:?}", b);
    }

    println!("{:?}", shared_data.borrow());
}

#[derive(Debug)]
struct Tab {
    name: i32,
}

impl Tab {
    fn new() -> Self {
        let mut t = Self { name: 0 };
        t.r();
        t
    }
    fn r(&mut self) {
        self.name = 111
    }
}

#[test]
fn test123() {
    let mut t = Tab { name: 1 };
    t.r();
    println!("{:?}", t);

    let t = Tab::new();
    println!("{:?}", t);
}

#[test]
fn test5() {
    let mut a = 111;
    let b = &mut a;
    *b = 222;
    println!("{}", b);
    println!("{}", a);
}
