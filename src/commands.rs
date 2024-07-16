

pub enum cmd {
    List {
        #[clap(long,short)]
        report: String,
        all: bool
    }
}
