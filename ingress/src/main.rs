use reversi::Host;
use reversi::Reversi;

const HOSTS: &[Host] = &[
    Host {
        domain: "www.fimfarchive.net",
        redirects: &["fimfarchive.net"],
        upstream: "localhost:34407",
    },
    Host {
        domain: "furver.se",
        redirects: &["www.furver.se"],
        upstream: "localhost:49211",
    },
];

pub fn main() -> ! {
    Reversi::from(HOSTS).run()
}
