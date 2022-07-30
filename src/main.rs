extern crate statust;

use std::cell::RefCell;

use easy_repl::{command, CommandStatus, Repl};

fn main() {
    // let s = "Hello, world!";
    // println!("{:?}", s);
    // println!("{:?}", statust::predict_type(s));

    // let args: Ve&c<String> = env::args().collect();
    // let fname = &args[1];

    // let df = statust::DataFrame::read(fname).unwrap();
    // df.print();
    // df.print_describe();

    // let writer = statust::FileWriter {
    //     fname: "output.txt".to_string(),
    //     data: statust::ResultType::DescribeMany(df.describe()),
    // };
    // writer.write_file();
    let mut df = RefCell::new(statust::DataFrame::new());
    let mut ref1 = &df;
    let ref2 = &df;

    let mut repl = Repl::builder()
        .description("Basic Statust REPL")
        .prompt("=> ")
        .add(
            "setdata",
            command! {
                "Read data from a file",
                (fname: String) => |fname: String| {
                    *ref1.borrow_mut() = statust::DataFrame::read(fname.as_str()).unwrap();
                    println!("Dataframe read from {}", fname);
                    Ok(CommandStatus::Done)
                }
            },
        )
        .add(
            "print",
            command! {
                "Print the dataframe",
                () => || {
                    ref2.borrow().print();
                    Ok(CommandStatus::Done)
                }
            },
        )
        .build()
        .expect("Failed to create repl");

    repl.run().expect("Critical REPL error");
}
