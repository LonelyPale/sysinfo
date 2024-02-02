use colored::{Color, Colorize};
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use std::fmt;

//智能指针版本
pub struct Table {
    pub columns: Vec<Rc<RefCell<Column>>>,
    pub data: Vec<HashMap<String, String>>,
    columns_cache: RefCell<HashMap<String, Rc<RefCell<Column>>>>,

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
    // #3: 使用Rc<RefCell<T>>智能指针来跳过编译器的所有权和生命周期检查，但会有运行时性能损失和运行时Panic。
}

impl Table {
    pub fn new(columns: Vec<Rc<RefCell<Column>>>, data: Vec<HashMap<String, String>>) -> Self {
        let table = Self {
            columns,
            data,
            columns_cache: RefCell::new(Default::default()),
        };
        table.refresh();
        table
    }

    fn refresh(&self) {
        let mut columns_cache = self.columns_cache.borrow_mut();

        for column in &self.columns {
            let mut col = column.borrow_mut();
            let len = col.title.len();
            if col.width < len {
                col.width = len;
            }
            columns_cache.insert(col.key.clone(), column.clone());
        }

        for row in &self.data {
            for (key, value) in row {
                if let Some(column) = columns_cache.get(key) {
                    let mut col = column.borrow_mut();
                    let len = value.len();
                    if col.width < len {
                        col.width = len;
                    }
                }
            }
        }
    }

    fn fmt_header(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let last = self.columns.len() - 1;
        for (index, column) in self.columns.iter().enumerate() {
            if index == 0 {
                write!(f, "{}", column.borrow())?;
            } else if index == last {
                write!(f, " {}\n", column.borrow())?;
            } else {
                write!(f, " {}", column.borrow())?;
            }
        }
        Ok(())
    }

    fn fmt_row(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let last = self.columns.len() - 1;
        for row in &self.data {
            for (index, column) in self.columns.iter().enumerate() {
                let col = column.borrow();
                let mut text = "";
                if let Some(value) = row.get(&col.key) {
                    text = value;
                }

                if index == 0 {
                    write!(f, "{}", col.format(text))?;
                } else if index == last {
                    write!(f, " {}\n", col.format(text))?;
                } else {
                    write!(f, " {}", col.format(text))?;
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
            columns_cache: RefCell::new(HashMap::new()),
        }
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.refresh();
        self.fmt_header(f)?;
        self.fmt_row(f)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct Column {
    pub key: String,
    pub title: String,
    pub width: usize,
    pub hidden: bool,
    pub right_align: bool,
    pub color: Option<Color>,
}

impl Column {
    fn format<S: AsRef<str>>(&self, value: S) -> String {
        let value: &str = value.as_ref();
        if self.hidden {
            String::new()
        } else {
            //处理颜色
            let output = match self.color {
                Some(item) => value.color(item),
                None => value.normal(),
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
        }
    }
}

impl fmt::Display for Column {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.format(&self.title))
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
    let column = Column { ..Column::default() };
    println!("{}", column.format("aaa"));

    let column_ref = &Column { ..Column::default() };
    println!("{}", column_ref.format(&"bbb".to_string()));
}

#[test]
fn test_table() {
    let table = Table {
        columns: vec![
            Rc::new(RefCell::new(Column {
                title: "AAA".to_string(),
                key: "aaa".to_string(),
                right_align: true,
                width: 10,
                color: Some(Color::Cyan),
                ..Column::default()
            })),
            Rc::new(RefCell::new(Column {
                title: "BBB".to_string(),
                key: "bbb".to_string(),
                ..Column::default()
            })),
            Rc::new(RefCell::new(Column {
                title: "CCC".to_string(),
                key: "ccc".to_string(),
                ..Column::default()
            })),
        ],
        data: vec![
            HashMap::from([
                ("aaa".to_string(), "1-1".to_string()),
                ("bbb".to_string(), "1-2-222".to_string()),
                ("ccc".to_string(), "1-3-333-333".to_string()),
            ]),
            HashMap::from([
                ("aaa".to_string(), "-1".to_string()),
            ]),
            HashMap::from([
                ("bbb".to_string(), "-2".to_string()),
            ]),
            HashMap::from([
                ("ccc".to_string(), "-3".to_string()),
            ]),
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

    println!("{}", table);
}

#[test]
fn test4() {
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
    fn r(&mut self) { self.name = 111 }
}

#[test]
fn test123() {
    let mut t = Tab{ name: 1 };
    t.r();
    println!("{:?}", t);
}
