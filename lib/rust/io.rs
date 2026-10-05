use std::fmt::Write;

struct IO4PS {
    in_str: String,
    out_str: String,
}

struct TokenizedInput<'a> {
    buffer: std::str::SplitWhitespace<'a>,
}

#[allow(dead_code)]
impl IO4PS {
    fn new() -> Self {
        Self {
            in_str: String::new(),
            out_str: String::new(),
        }
    }

    fn read_all(&mut self) {
        let _ = std::io::Read::read_to_string(&mut std::io::stdin(), &mut self.in_str);
    }

    fn tokenize(&'_ self) -> TokenizedInput<'_> {
        TokenizedInput {
            buffer: self.in_str.split_whitespace(),
        }
    }

    fn writeln<T>(&mut self, data: T)
    where
        T: std::fmt::Display,
    {
        let _ = writeln!(self.out_str, "{}", data);
    }

    fn write<T>(&mut self, data: T)
    where
        T: std::fmt::Display,
    {
        let _ = write!(self.out_str, "{}", data);
    }

    fn flush(&mut self) {
        print!("{}", self.out_str);
        self.out_str.clear();
    }
}

impl TokenizedInput<'_> {
    fn next<T>(&mut self) -> T
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: core::fmt::Debug,
    {
        self.buffer.next().unwrap().parse::<T>().unwrap()
    }

    fn next_raw(&mut self) -> &str {
        self.buffer.next().unwrap()
    }
}

struct Joiner<'a, 'b, T: std::fmt::Display> {
    sep: &'a str,
    list: &'b [T],
}

impl<'a, 'b, T: std::fmt::Display> Joiner<'a, 'b, T> {
    pub fn new(sep: &'a str, list: &'b [T]) -> Joiner<'a, 'b, T> {
        Joiner { sep, list }
    }
}

impl<'a, 'b, T: std::fmt::Display> std::fmt::Display for Joiner<'a, 'b, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut iter = self.list.iter();
        if let Some(first) = iter.next() {
            write!(f, "{}", first)?;
            for item in iter {
                write!(f, "{}{}", self.sep, item)?;
            }
        }
        Ok(())
    }
}
