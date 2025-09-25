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
        write!(
            f,
            "{}",
            match self {
                ArgumentValue::None => "None",
                ArgumentValue::Number(_) => "Number",
                ArgumentValue::String(_) => "String",
                ArgumentValue::Boolean(_) => "Boolean",
            }
        )
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

impl Display for Parser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // -short --long = command
        // text = description
        // [type] = argumenttype
        let lines_first: Vec<(String, ArgumentValue, String)> = self
            .defined_arguments
            .iter()
            .map(|argument| {
                (
                    format!("-{} --{}", argument.index.0, argument.index.1),
                    argument.value.clone(),
                    argument.description.clone(),
                )
            })
            .collect();

        // calculate maximum command length to pad command properly to insert value
        let mut command_max = lines_first
            .iter()
            .map(|(command, _, _)| command)
            .max_by_key(|command| Some(command.len()))
            .cloned()
            .unwrap_or_default()
            .len();

        let lines_second: Vec<(String, String)> = lines_first
            .iter()
            .map(|(command, value, description)| {
                (
                    if matches!(value, ArgumentValue::None) {
                        format!("{:<command_max$}", command, command_max = command_max)
                    } else {
                        format!(
                            "{:<command_max$}\t<{}>",
                            command,
                            value,
                            command_max = command_max
                        )
                    },
                    description.clone(),
                )
            })
            .collect();

        command_max = lines_second
            .iter()
            .map(|(command, _)| command)
            .max_by_key(|command| Some(command.len()))
            .cloned()
            .unwrap_or_default()
            .len();

        // merge description and prettified commands
        let lines_final: Vec<String> = lines_second
            .iter()
            .map(|(command, description)| {
                format!(
                    "\t{:<command_max$}\t{}",
                    command,
                    description,
                    command_max = command_max
                )
            })
            .collect();

        write!(
            f,
            "{} v{} ({})\nrunning\n{} v{} ({})\n\n{}",
            std::env::var("MAIN_PKG_NAME").unwrap_or_default(),
            std::env::var("MAIN_PKG_VERSION").unwrap_or_default(),
            std::env::var("MAIN_PKG_DESCRIPTION").unwrap_or_default(),
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
            env!("CARGO_PKG_DESCRIPTION"),
            lines_final.join("\n")
        )
    }
}
