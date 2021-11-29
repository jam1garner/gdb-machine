use crate::mi_output::*;

pub type ParseError = peg::error::ParseError<peg::str::LineCol>;

impl Output {
    /// Parse the output of GDB MI from a string
    pub fn parse(text: &str) -> Result<Self, ParseError> {
        gdb_machine_interface::output(text)
    }
}

peg::parser! {
    grammar gdb_machine_interface() for str {
        rule token() -> usize
            = n:$(['0'..='9']+) { n.parse().unwrap() }

        rule nl() = quiet!{ "\r" "\n"? }
            / expected!("newline")

        rule hex_digit() = quiet! {['0'..='9' | 'a'..='f' | 'A'..='F']}
            / expected!("hex digit")

        rule string_character() -> char
            = character:$(
                r#"\\"# /
                r#"\'"# /
                r#"\""# /
                r#"\?"# /
                r#"\a"# /
                r#"\b"# /
                r#"\f"# /
                r#"\n"# /
                r#"\r"# /
                r#"\t"# /
                r#"\v"# /
                (r#"\x"#  hex_digit() hex_digit()) /
                [^ '\\' | '"']
            )
        {
            match character {
                r#"\\"# => '\\',
                r#"\'"# => '\'',
                r#"\""# => '"',
                r#"\?"# => '?',
                r#"\a"# => '\x07',
                r#"\b"# => '\x08',
                r#"\f"# => '\x0c',
                r#"\n"# => '\n',
                r#"\r"# => '\r',
                r#"\t"# => '\t',
                r#"\v"# => '\x0b',
                c if c.starts_with(r#"\x"#) => u8::from_str_radix(&c[2..], 16).unwrap() as char,
                c if c.len() == 1 => c.chars().next().unwrap(),
                c => panic!("Failed to parse C character: {:?}", c),
            }
        }

        rule identifier() -> String
            = ident:$(['a'..='z' | 'A'..='Z' | '_' ]+) { ident.into() }

        rule constant() -> Value
            = string:cstring() { Value::Const(string) }

        rule result() -> (String, Value)
            = ident:identifier() "=" val:value() { (ident, val) }

        rule tuple() -> Value
            = "{" values:result() ** "," "}" { Value::Tuple(values.into_iter().collect()) }

        rule list_map() -> Value
            = "[" values:result() ** "," "]" { Value::ListMap(values.into_iter().collect()) }

        rule list() -> Value
            = "[" values:value() ** "," "]" { Value::List(values.into_iter().collect()) }

        rule value() -> Value
            = tuple()
            / list()
            / list_map()
            / constant()

        rule cstring() -> String
            = "\"" string:string_character()* "\"" { string.into_iter().collect() }

        rule stream_record() -> StreamRecord
            = log_stream_output()
            / target_stream_output()
            / console_stream_output()

        rule log_stream_output() -> StreamRecord
            = "&" string:cstring() nl() { StreamRecord::Log(string) }

        rule target_stream_output() -> StreamRecord
            = "@" string:cstring() nl() { StreamRecord::Target(string) }

        rule console_stream_output() -> StreamRecord
            = "~" string:cstring() nl() { StreamRecord::Console(string) }

        rule result_class() -> ResultClass
            = result:$("done" / "running" / "connected" / "error" / "exit")
        {
            use ResultClass::*;
            match result {
                "done" => Done,
                "running" => Running,
                "connected" => Connected,
                "error" => Error,
                "exit" => Exit,
                _ => unreachable!()
            }
        }

        rule async_class() -> AsyncClass
            = "thread-group-added" { AsyncClass::ThreadGroupAdded }
            / "thread-created" { AsyncClass::ThreadCreated }
            / "thread-selected" { AsyncClass::ThreadSelected }
            / "thread-exited" { AsyncClass::ThreadExited }
            / "record-started" { AsyncClass::RecordStarted }
            / "record-stopped" { AsyncClass::RecordStopped }
            / "thread-group-started" { AsyncClass::ThreadGroupStarted }
            / "thread-group-exited" { AsyncClass::ThreadGroupExited }
            / "thread-group-removed" { AsyncClass::ThreadGroupRemoved }
            / "traceframe-changed" { AsyncClass::TraceframeChanged }
            / "tsv-created" { AsyncClass::TsvCreated }
            / "tsv-modified" { AsyncClass::TsvModified }
            / "tsv-deleted" { AsyncClass::TsvDeleted }
            / "breakpoint-created" { AsyncClass::BreakpointDeleted }
            / "breakpoint-modified" { AsyncClass::BreakpointModified }
            / "breakpoint-deleted" { AsyncClass::BreakpointDeleted }
            / "library-loaded" { AsyncClass::LibraryLoaded }
            / "library-unloaded" { AsyncClass::LibraryUnloaded }
            / "cmd-param-changed" { AsyncClass::CmdParamChanged }
            / "memory-changed" { AsyncClass::MemoryChanged }
            / "stopped" { AsyncClass::Stopped }

        rule record_fields() -> HashMap<String, Value>
            = "," fields:result() ** ","
        {
            fields.into_iter().collect()
        }

        rule result_record() -> ResultRecord
            = token:token()? "^" result_class:result_class() results:record_fields()? nl() {
                ResultRecord {
                    result_class,
                    results: results.unwrap_or_default(),
                }
            }

        rule async_record() -> AsyncRecord
            = token:token()? kind:(
                "*" { AsyncRecordKind::Exec } /
                "+" { AsyncRecordKind::Status } /
                "=" { AsyncRecordKind::Notify }
            ) output:async_output() nl()
        {
            AsyncRecord { kind, output, token }
        }

        rule async_output() -> AsyncOutput
            = class:async_class() results:record_fields()? {
                AsyncOutput {
                    class,
                    results: results.unwrap_or_default(),
                }
            }

        rule out_of_band_record() -> OutOfBandRecord
            = async_rec:async_record() { OutOfBandRecord::Async(async_rec) }
            / stream:stream_record() { OutOfBandRecord::Stream(stream) }

        pub(crate) rule output() -> Output
            = out_of_band:out_of_band_record()* result:result_record()? /*nl()*/ "(gdb)" nl() {
                Output { out_of_band, result }
            }

    }
}
