use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum ArgumentValue {
    /// No value expected
    None,
    /// Number value
    Number(Option<usize>),
    /// Text value
    String(Option<String>),
    /// Boolean value
    Boolean(Option<bool>),
}

impl Display for ArgumentValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Stored data after being parsed
#[derive(Clone, Debug)]
struct ParsedArgument {
    index: (char, String),
    value: ArgumentValue,
}

/// Contains info necessary for parsing
#[derive(Debug, Clone)]
struct DefinedArgument {
    index: (char, String),
    description: String,
    value: ArgumentValue,
}

/// Parser struct used for indexing, parsing and adding arguments
#[derive(Default, Clone, Debug)]
pub struct Parser {
    defined_arguments: Vec<DefinedArgument>,
    parsed_arguments: Vec<ParsedArgument>,
}

impl Parser {
    /// Attempt to parse a string iterator into arguments
    /// ```
    /// let parser = Parser::default()
    ///     .add_arg('h', "help", "Helptext", None)
    ///     .parse();
    /// ```
    pub fn parse<T>(&mut self, data: T) -> Result<(), crate::Error>
    where
        T: IntoIterator<Item = String>,
    {
        let mut data_stream = data.into_iter();
        while let Some(value) = data_stream.next() {
            // attempt to find given argument in the list of predefined arguments
            let found_argument = self.defined_arguments.iter().find(|arg| {
                arg.index.1.as_str() == value.trim_start_matches("--")
                    || arg.index.0.to_string() == value.trim_start_matches("-")
            });

            match found_argument {
                Some(argument) => self.parsed_arguments.push(ParsedArgument {
                    index: argument.index.clone(),
                    // attempt to parse requested value into parsed vec
                    value: match argument.value {
                        ArgumentValue::None => ArgumentValue::None,
                        _ => Self::parse_optional(&argument.value, data_stream.next()).map_err(
                            |(kind, value)| {
                                crate::Error::InvalidArgument(
                                    t!(
                                        "ARGUMENTS.ARGUMENT.INVALID_VALUE",
                                        expected = kind,
                                        received = value
                                    )
                                    .to_string(),
                                )
                            },
                        )?,
                    },
                }),
                None => {}
            }
        }
        Ok(())
    }

    fn parse_optional(
        kind: &ArgumentValue,
        value: Option<String>,
    ) -> Result<ArgumentValue, (&ArgumentValue, String)> {
        Ok(match value {
            Some(value) => match kind {
                ArgumentValue::None => ArgumentValue::None,
                ArgumentValue::Number(_) => {
                    ArgumentValue::Number(Some(value.parse().map_err(|_| (kind, value))?))
                }
                ArgumentValue::String(_) => {
                    ArgumentValue::String(Some(value.parse().map_err(|_| (kind, value))?))
                }
                ArgumentValue::Boolean(_) => {
                    ArgumentValue::Boolean(Some(value.parse().map_err(|_| (kind, value))?))
                }
            },
            None => ArgumentValue::None,
        })
    }

    /// Add an argument to the list of parsable arguments
    /// ```
    /// let parser = Parser::default()
    ///     .add_arg('h', 'help', 'Helptext', ArgumentValue::None);
    /// ```
    pub fn add_arg<T, S>(
        &mut self,
        short: char,
        long: T,
        description: S,
        value: ArgumentValue,
    ) -> Self
    where
        T: Into<String>,
        S: Display,
    {
        self.defined_arguments.push(DefinedArgument {
            index: (short, long.into()),
            description: description.to_string(),
            value,
        });
        self.clone()
    }

    /// Returns none if argument was not provided, or some with the expected valuetype in it
    /// ```
    /// let parser = Parser::default()
    ///     .add_arg('h', "help", "Helptext", None)
    ///     .parse();
    /// assert!(parser.get_argument((Some('0'), None)).is_some());
    /// ```
    pub fn get_argument<T>(&self, index: (Option<char>, Option<T>)) -> Option<ArgumentValue>
    where
        T: Into<String> + Clone,
    {
        self.parsed_arguments
            .iter()
            .find(|argument| {
                Some(argument.index.0) == index.0
                    || Some(argument.index.1.clone())
                        == index.1.clone().and_then(|long| Some(long.into()))
            })
            .map(|argument| argument.value.clone())
    }
}
