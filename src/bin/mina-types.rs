use std::{cmp::Ordering, fs::File, io::{BufReader, BufRead}};

use anyhow::{format_err, Result};
use bin_prot_rs::{
    shape::{Expression, Visitor},
    xref::NamedShape,
};
use clap::{ArgEnum, Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[clap(value_parser)]
    file: String,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Filter {
        #[clap(short, long)]
        depth: Option<usize>,
        #[clap(arg_enum, short, long)]
        enable: Vec<ExprKind>,
    },
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum ExprKind {
    Record,
    Variant,
    Tuple,
    Base,
}

impl ExprKind {
    fn matches(&self, expr: &Expression) -> bool {
        matches!(
            (self, expr),
            (Self::Record, Expression::Record(..))
                | (Self::Variant, Expression::Variant(..))
                | (Self::Tuple, Expression::Tuple(..))
                | (Self::Base, Expression::Base(..))
        )
    }
}

// fn format_tokens(ts: TokenStream) -> String {
//     RustFmt::default().format_tokens(ts.into()).unwrap()
// }

fn main() -> Result<()> {
    let cli = Cli::parse();
    let file = cli.file;
    let mut read =
        BufReader::new(File::open(&file).map_err(|err| format_err!("cannot open {file}: {err}"))?);
    match cli.command {
        Commands::Filter { enable, depth } => {
            struct Filter<'a> {
                matches: bool,
                curr_depth: usize,
                depth: usize,
                enable: &'a Vec<ExprKind>,
            }
            impl<'a> Visitor<'a> for Filter<'a> {
                fn apply(&mut self, expr: &'a Expression) {
                    match self.curr_depth.cmp(&self.depth) {
                        Ordering::Less => {
                            self.curr_depth += 1;
                            expr.visit(self)
                        }
                        Ordering::Equal => {
                            self.matches = self.enable.iter().any(|f| f.matches(expr))
                        }
                        Ordering::Greater => {}
                    }
                }
            }

            let mut buf = String::new();
            while read.read_line(&mut buf)? != 0 {
                let ty = Type::from_mina_shape(&buf)?;
                let mut filter = Filter {
                    matches: false,
                    curr_depth: 0,
                    depth: depth.unwrap_or(1),
                    enable: &enable,
                };
                ty.shape.visit(&mut filter);
                if filter.matches {
                    print!("{buf}");
                }
                buf = String::new();
            }
        }
    }

    //    let mut gen = Generator::new(&types)?;

    //    let ts = gen.generate("Transaction_snark.Pending_coinbase_stack_state.Stable.V1.t");
    //    println!("{}", format_tokens(ts));
    //

    Ok(())
}

struct Type {
    name: String,
    shape: Expression,
}

impl NamedShape for Type {
    fn name(&self) -> &str {
        &self.name
    }

    fn shape(&self) -> &Expression {
        &self.shape
    }
}

impl Type {
    fn from_mina_shape(line: &str) -> Result<Self, anyhow::Error> {
        let mut parts = line.splitn(2, ", ");
        let ty = parts
            .next()
            .ok_or_else(|| format_err!("missing type description"))?;
        let (_, name) = ty
            .split_once(':')
            .ok_or_else(|| format_err!("invalid descriptor format"))?;
        let shape = parts
            .next()
            .ok_or_else(|| format_err!("missing shape for {name}"))?;
        let shape = shape
            .parse()
            .map_err(|e| format_err!("error while reading {name}: {e}"))?;
        Ok(Self {
            name: name.to_string(),
            shape,
        })
    }
}
