use reversi::Host;
use reversi::Reversi;

const HOSTS: &[Host] = &[Host {
    domain: "www.fimfarchive.net",
    redirects: &["fimfarchive.net"],
    upstream: "localhost:34407",
}];

pub fn main() -> ! {
    Reversi::from(HOSTS).run()
}
