use crate::mi_output::Output;

macro_rules! assert_parse_succeed {
    ($($test_name:ident = $input:literal),* $(,)?) => {
        $(
            #[test]
            fn $test_name() {
                const INPUT: &str = $input;
                match Output::parse(INPUT) {
                    Ok(parsed_output) => {
                        dbg!(parsed_output);
                    }
                    Err(err) =>{
                        let line_num = err.location.line;
                        let col = err.location.column;

                        let line = INPUT.split('\n').nth(line_num - 1).unwrap();
                        let expected: Vec<_> = err.expected.tokens().collect();

                        let line = &line[col.saturating_sub(0x21)..(col + 0x3f).min(line.len())];
                        let err_col = col.saturating_sub(col.saturating_sub(0x21) + 1);

                        println!("Error in parsing test {} on line {}:", stringify!($test_name), line_num);
                        println!("{}", line);
                        println!("{}^ ---- expected {}", " ".repeat(err_col), match &expected[..] {
                            [] => String::from("nothing"),
                            &[only] => only.into(),
                            [all_but_last @ .. , last] => format!("one of {} or {}", all_but_last.join(", "), last)
                        });

                        panic!("Failed to parse");
                    }
                }
            }
        )*
    };
}

assert_parse_succeed!(
    output = "^done,completion=\"\",matches=[\"!\",\"+\",\"-\",\"<\",\">\",\"actions\",\"add-auto-load-safe-path\",\"add-auto-load-scripts-directory\",\"add-inferior\",\"add-symbol-file\",\"add-symbol-file-from-memory\",\"adi\",\"advance\",\"agent-printf\",\"alias\",\"append\",\"apropos\",\"attach\",\"awatch\",\"backtrace\",\"bookmark\",\"break\",\"break-range\",\"bt\",\"call\",\"catch\",\"cd\",\"checkpoint\",\"clear\",\"clone-inferior\",\"collect\",\"commands\",\"compare-sections\",\"compile\",\"complete\",\"condition\",\"continue\",\"core-file\",\"define\",\"define-prefix\",\"delete\",\"demangle\",\"detach\",\"directory\",\"disable\",\"disassemble\",\"disconnect\",\"display\",\"document\",\"dont-repeat\",\"down\",\"down-silently\",\"dprintf\",\"dump\",\"echo\",\"edit\",\"enable\",\"end\",\"eval\",\"exec-file\",\"explore\",\"expression\",\"faas\",\"file\",\"find\",\"finish\",\"flash-erase\",\"flushregs\",\"focus\",\"forward-search\",\"frame\",\"fs\",\"ftrace\",\"function\",\"generate-core-file\",\"goto-bookmark\",\"guile\",\"guile-repl\",\"handle\",\"hbreak\",\"help\",\"if\",\"ignore\",\"inferior\",\"info\",\"init-if-undefined\",\"interpreter-exec\",\"interrupt\",\"jit-reader-load\",\"jit-reader-unload\",\"jump\",\"kill\",\"layout\",\"list\",\"load\",\"macro\",\"maintenance\",\"make\",\"mem\",\"monitor\",\"my_bt\",\"new-ui\",\"next\",\"nexti\",\"ni\",\"nosharedlibrary\",\"output\",\"overlay\",\"passcount\",\"path\",\"pipe\",\"print\",\"print-object\",\"printf\",\"ptype\",\"pwd\",\"python\",\"python-interactive\",\"queue-signal\",\"quit\",\"rbreak\",\"rc\",\"record\",\"refresh\",\"remote\",\"remove-inferiors\",\"remove-symbol-file\",\"restart\",\"restore\",\"return\",\"reverse-continue\",\"reverse-finish\",\"reverse-next\",\"reverse-nexti\",\"reverse-search\",\"reverse-step\",\"reverse-stepi\",\"rni\",\"rsi\",\"run\",\"rwatch\",\"save\",\"search\",\"section\",\"select-frame\",\"set\",\"sharedlibrary\",\"shell\",\"show\",\"si\",\"signal\",\"skip\",\"source\",\"start\",\"starti\",\"step\",\"stepi\",\"stepping\",\"stop\",\"strace\",\"symbol-file\",\"taas\",\"target\",\"task\",\"tbreak\",\"tcatch\",\"tdump\",\"teval\",\"tfaas\",\"tfind\",\"thbreak\",\"thread\",\"tp\",\"trace\",\"tsave\",\"tstart\",\"tstatus\",\"tstop\",\"tty\",\"tui\",\"tvariable\",\"undisplay\",\"unset\",\"until\",\"up\",\"up-silently\",\"update\",\"watch\",\"wh\",\"whatis\",\"where\",\"while\",\"while-stepping\",\"winheight\",\"with\",\"ws\",\"x\",\"|\"],max_completions_reached=\"0\"\r\n(gdb)\r\n",
);
