use colored::{Color, Colorize};
use std::borrow::Cow;
use std::fmt;

pub struct Table<'a> {
    pub header: Header<'a>,
    pub rows: Vec<Row<'a>>,
}

impl<'a> fmt::Display for Table<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.header)?;
        for row in &self.rows {
            write!(f, "{}", row)?;
        }
        Ok(())
    }
}

pub struct Header<'a> {
    columns: Vec<Column<'a>>,
}

impl<'a> fmt::Display for Header<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

pub struct Row<'a> {
    columns: Vec<Column<'a>>,
}

impl<'a> fmt::Display for Row<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (index, value) in self.columns.iter().enumerate() {
            if index == 0 {
                write!(f, "{}", value)?;
            } else {
                write!(f, " {}", value)?;
            }
        }
        Ok(())
    }
}

pub struct Column<'a> {
    key: &'a str,
    value: String,
    style: Cow<'a, Style>,
}

impl<'a> fmt::Display for Column<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.style.format(&self.value))
    }
}

// 派生词: Copyable Cloneable Styleable
// pub trait Styleable {
//     fn format(&self, value: &String) -> String;
// }

#[derive(Clone)]
pub struct Style {
    pub hidden: bool,
    pub right_align: bool,
    pub width: usize,
    pub color: Option<Color>,
}

impl Style {
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

            //处理对齐方式
            let width = self.width;
            if self.right_align {
                format!("{output:>width$}")
            } else {
                format!("{output:<width$}")
            }
        }
    }
}

impl Default for Style {
    fn default() -> Self {
        Self {
            hidden: false,
            right_align: false,
            width: 0,
            color: None,
        }
    }
}

pub struct Filter {}

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
fn test_style() {
    let style = Style { ..Style::default() };
    println!("{}", style.format("111"));

    let style_ref = &Style { ..Style::default() };
    println!("{}", style_ref.format(&"222".to_string()));
}

#[test]
fn test_column() {
    let column = Column {
        key: "111",
        value: "aaa".to_string(),
        style: Cow::Owned(Style::default()),
    };
    println!("{}", column);

    let style = Style {
        right_align: true,
        width: 10,
        color: Some(Color::Cyan),
        ..Style::default()
    };
    let column_ref = Column {
        key: "222",
        value: "bbb".to_string(),
        style: Cow::Borrowed(&style),
    };
    println!("{}", column_ref);
}

#[test]
fn test_row() {
    let column = Column {
        key: "111",
        value: "aaa".to_string(),
        style: Cow::Owned(Style::default()),
    };

    let style = Style {
        right_align: true,
        width: 10,
        color: Some(Color::Cyan),
        ..Style::default()
    };
    let column_ref = Column {
        key: "222",
        value: "bbb".to_string(),
        style: Cow::Borrowed(&style),
    };

    let column3 = Column {
        key: "333",
        value: "ccc".to_string(),
        style: Cow::Owned(Style::default()),
    };

    let row = Row {
        columns: vec![column, column_ref, column3],
    };
    println!("{}", row);
}
