use {
    std::net::IpAddr,
    structopt::{clap, StructOpt},
};

#[derive(StructOpt)]
#[structopt(author, about, setting = clap::AppSettings::ArgRequiredElseHelp)]
#[structopt(rename_all = "kebab-case")]
pub enum Args {
    /// Show information of given ip address, like ASNs, RIR, IANA, etc
    Ip {
        /// the ip address
        ip: IpAddr,
    },
}

pub fn get() -> Args {
    Args::from_args()
}
