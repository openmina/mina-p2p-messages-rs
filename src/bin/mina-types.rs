use std::{
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{bail, format_err, Result};
use bin_prot_rs::{
    doc::Doc,
    shape::{Expression, self},
    visit::{Visiting, Visitor},
    xref::{NamedShape, XRef},
};
use clap::{ArgEnum, Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[clap(value_parser)]
    file: String,

    #[clap(short, long)]
    no_eval: bool,

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
    Doc {
        #[clap(short, long)]
        _type: Vec<String>,
        #[clap(short, long)]
        all: bool,
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
    let mut r =
        BufReader::new(File::open(&file).map_err(|err| format_err!("cannot open {file}: {err}"))?);
    let shapes = read(&mut r, !cli.no_eval)?;
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

            for Type { shape, source, .. } in shapes {
                let mut filter = Filter {
                    matches: false,
                    curr_depth: 0,
                    depth: depth.unwrap_or(1),
                    enable: &enable,
                };
                shape.visit(&mut filter);
                if filter.matches {
                    print!("{source}");
                }
            }
            Ok(())
        }
        Commands::Doc { _type, all } => doc(shapes, _type, all),
    }

    //    let mut gen = Generator::new(&types)?;

    //    let ts = gen.generate("Transaction_snark.Pending_coinbase_stack_state.Stable.V1.t");
    //    println!("{}", format_tokens(ts));
    //
}

fn read(read: &mut impl BufRead, eval: bool) -> Result<Vec<Type>> {
    let mut mina_types = Vec::new();
    let mut buf = String::new();
    while read.read_line(&mut buf)? != 0 {
        let source = std::mem::take(&mut buf);
        let ty = Type::from_mina_shape(source, eval)?;
        mina_types.push(ty);
    }
    Ok(mina_types)
}

fn doc(shapes: Vec<Type>, _types: Vec<String>, all: bool) -> Result<()> {
    if _types.is_empty() != all {
        bail!("should be either --type or --all");
    }

    let xref = XRef::new(&shapes)?;
    let git_base =
        "https://github.com/MinaProtocol/mina/blob/b14f0da9ebae87acd8764388ab4681ca10f07c89/";
    let mut stdout = std::io::stdout();
    if all {
        let mut doc = Doc::new(&xref, git_base.to_string(), &mut stdout);
        doc.generate_all()?;
    } else {
        for _type in &_types {
            let mut doc = Doc::new(&xref, git_base.to_string(), &mut stdout);
            doc.generate(_type)?;
        }
    }

    Ok(())
}

struct Type {
    name: String,
    shape: Expression,
    source: String,
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
    fn from_mina_shape(source: String, eval: bool) -> Result<Self, anyhow::Error> {
        let mut parts = source.splitn(2, ", ");
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
            shape: if eval { shape::eval(&shape)? } else { shape },
            source,
        })
    }
}
