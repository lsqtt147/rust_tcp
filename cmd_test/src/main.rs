use structopt::StructOpt;
use std::fmt::{self, Display, Formatter}; //引入包

#[derive(StructOpt)]   //使用StructOpt定义一个app
#[structopt(name = "app")]
pub struct AppArgs {
    #[structopt(subcommand)]
    pub command: Command,  //命令行工具有一个子命令
}

#[derive(StructOpt)]   //使用enum定义子命令，加和乘，老师解释：每个variant都包含一个叫做Elements的data，Elements是一个类型，其中包含了一个u32的vector，这个vector做为存放加法、乘法的操作数列表。
pub enum Command {
    /// add operation
    #[structopt(name = "add")]
    Add(Elements),

    ///times operation
    #[structopt(name = "times")]
    Times(Elements),
}

#[derive(StructOpt)]
pub struct Elements {
    pub elements: Vec<u32>,
}

//给Elements类型实现Display Trait，这里是直接将elements数据按照Debug模式进行输出了。
impl Display for Elements {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{:#?}]", self.elements)
    }
}

//主程序
fn main() {
    let opt = AppArgs::from_args();

    match opt.command {
        Command::Add(e) => {
            let result = e.elements.iter().fold(0, |acc, &x| acc + x);
            println!("Operants: {}, result: {}", e, result);
        },
        Command::Times(e) => {
            let result = e.elements.iter().fold(1, |acc, &x| acc * x);
            println!("Operants: {}, result: {}", e, result);
        },
    }
}