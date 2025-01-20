macro_rules! function {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        name.strip_suffix("::f").unwrap()
    }}
}
pub(crate) use function;

#[derive(Default)]
pub struct Position {

    pub line: u32,
    pub column: u32

}

impl std::fmt::Display for Position {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }

}

macro_rules! file_position {
    () => {
       crate::error::Position {
            line: std::line!(),
            column: std::column!()
       }
    };
}
pub(crate) use file_position;

pub struct Error {
    pub place:   Vec<(&'static str, Position)>,
    pub name:    String,
    pub message: String
}


macro_rules! error {
    {$( $it:ident : $value:expr) ,*} => {
        Err(crate::error::Error {$( $it: $value.into() ),*, 
            place: vec![(crate::error::function!(), crate::error::file_position!())], 
        })
    };
}
pub(crate) use error;

macro_rules! error_forward {
    {$err:expr} => {
       $err.push(crate::error::function!(), crate::error::file_position!())
    };
}
pub(crate) use error_forward;

macro_rules! function_message {
    ($func:expr, $msg:expr) => {
        format!("{} returned the following message: {}", $func, $msg)
    };
}
pub(crate) use function_message;

impl Error {

    pub fn push(mut self, func: &'static str, position: Position) -> Self {

        self.place.push((func, position));
        self

    }

}

impl std::fmt::Display for Error {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut accumulate = String::new();

        let zero = Position::default();

        accumulate.push_str(
            format!("From `{}` at {}\n", 
                if self.place.first().is_none() {
                    "unknown"
                } else {
                    let (func, _) = self.place.first().unwrap();
                    func
                },
                if self.place.first().is_none() {
                    &zero
                } else {
                    let (_, pos) = self.place.first().unwrap();
                    pos
                },
            ).as_str()
        );

        for (func, pos) in self.place.iter().skip(1) {
            accumulate.push_str(format!("Called by `{}` at {}\n", func, pos).as_str());
        }

        accumulate.push_str(format!("{}!\n", self.name).as_str());

        accumulate.push_str(format!("{}.\n", self.message).as_str());

        write!(f, "{}", accumulate)

    }

}