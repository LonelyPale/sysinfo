use std::fmt;
use std::collections::HashMap;

use colored::{Color, ColoredString, Colorize, Style, Styles};

use crate::table::CombineString::{AsStr, AsString, AsColoredString};

//标准版本(函数指针render)：已完成
pub struct Table {
    columns: Vec<Column>,
    columns_cache: HashMap<String, usize>,
    data: Vec<HashMap<String, String>>,
    custom: HashMap<String, String>,

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
    pub fn new(columns: Vec<Column>, data: Vec<HashMap<String, String>>, custom: HashMap<String, String>) -> Self {
        let mut table = Self {
            columns,
            data,
            custom,
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
        let column_last = self.columns.len() - 1;
        let record_last = self.data.len() - 1;
        for (record_index, record) in self.data.iter().enumerate() {
            for (column_index, column) in self.columns.iter().enumerate() {
                let mut text = "";
                if let Some(value) = record.get(&column.key) {
                    text = value;
                }

                let args = RenderArgs {
                    value: AsStr(text),
                    key: &column.key,
                    column_index,
                    column,
                    columns: &self.columns,
                    record_index,
                    record,
                    data: &self.data,
                    custom: &self.custom,
                };

                if column_index == 0 {
                    write!(f, "{}", column.format(AsStr(text), Some(args)))?;
                } else if column_index == column_last && record_index != record_last {
                    write!(f, "  {}\n", column.format(AsStr(text), Some(args)))?;
                } else {
                    write!(f, "  {}", column.format(AsStr(text), Some(args)))?;
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
            columns_cache: HashMap::new(),
            data: Vec::new(),
            custom: HashMap::new(),
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

pub struct Column {
    pub key: String,
    pub title: String,
    pub width: usize,
    pub hidden: bool,
    pub right_align: bool,
    pub color: Option<Color>,
    pub style: Style,
    pub render: Option<Render>,
    // pub render: Option<fn(args: RenderArgs) -> CombineString<'a>>, //使用别名类型Render可以不用定义生命周期<'a>
    // fn render(args: RenderArgs) -> CombineString {} //使用函数变量fn不能捕获外部变量，但闭包可以。
    // 函数变量是具体的类型，但闭包不是，每个闭包都是一个单独的类型，即时一模一样的两个闭包也是两个完全不同的类型，所以闭包只能用特征约束。
    // 但此处如果使用闭包，会有泛型约束循环依赖和生命周期的问题，Column依赖Render，Render依赖RenderArgs，RenderArgs依赖Column。
}

//warning: bounds on generic parameters are not enforced in type aliases
//type Render<S: AsRef<str>> = fn(args: RenderArgs<S>) -> CombineString<S>;
type Render = fn(args: RenderArgs) -> CombineString;

pub struct RenderArgs<'a> {
    pub value: CombineString<'a>,
    pub key: &'a str,
    pub column_index: usize,
    pub column: &'a Column,
    pub columns: &'a Vec<Column>,
    pub record_index: usize,
    pub record: &'a HashMap<String, String>,
    pub data: &'a Vec<HashMap<String, String>>,
    pub custom: &'a HashMap<String, String>,
}

/// 使用泛型<S: AsRef<str>>，最终会导致泛型参数循环依赖，直到编译器报错 Column -> Render -> RenderArgs和CombineString -> Column
#[derive(Debug)]
pub enum CombineString<'a> {
    AsStr(&'a str),
    AsString(String),
    AsColoredString(ColoredString),
}

impl Column {
    fn format(&self, value: CombineString, args: Option<RenderArgs>) -> String {
        // fn format<S: AsRef<str>>(&self, value: S) -> String {}
        // let value: &str = value.as_ref();

        if self.hidden {
            String::new()
        } else {
            let value = match self.render {
                None => value,
                Some(render) => match args {
                    None => value,
                    Some(args) => render(args),
                },
            };

            let output = match value {
                AsStr(val) => {
                    let mut val = val.normal();
                    //处理颜色
                    if let Some(c) = self.color {
                        val.fgcolor = Some(c);
                    }
                    //处理样式
                    val.style = self.style;
                    val
                }
                AsString(val) => {
                    let mut val = val.normal();
                    //处理颜色
                    if let Some(c) = self.color {
                        val.fgcolor = Some(c);
                    }
                    //处理样式
                    val.style = self.style;
                    val
                }
                AsColoredString(val) => val,
            };

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
        if self.title.len() == 0 {
            return write!(f, "{}", self.format(AsStr(&self.title), None));
        }

        let mut text = self.title.normal();
        text.fgcolor = self.color;
        text.style = Styles::Bold | Styles::Underline;
        write!(f, "{}", self.format(AsColoredString(text), None))
        // write!(f, "{}", self.format(&self.title))
    }
}

#[test]
fn test() {
    // std::fmt::format();
    // std::fmt::Display::fmt();

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
fn test_combine_string() {
    let s = "111";
    let s1 = AsStr(s);
    let s2 = AsStr(&s);

    let mut sm = "222";
    let s3 = AsStr(sm);

    sm = "333";
    let s4 = AsStr(sm);
    let s5 = AsStr(&sm);

    println!("{:?} {:?} {:?} {:?} {:?}", s1, s2, s3, s4, s5);
}

#[test]
fn test_column() {
    let column = Column {
        ..Column::default()
    };
    println!("{}", column.format(AsStr("aaa"), None));

    let column_ref = &Column {
        ..Column::default()
    };
    println!("{}", column_ref.format(AsStr("bbb"), None));
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

#[test]
fn test5() {
    let mut a = 111;
    let b = &mut a;
    *b = 222;
    println!("{}", b);
    println!("{}", a);
}
