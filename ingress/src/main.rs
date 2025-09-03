use reversi::Host;
use reversi::Reversi;

const HOSTS: &[Host] = &[
    Host {
        domain: "furver.se",
        redirects: &["www.furver.se"],
        upstream: "localhost:49211",
    },
    Host {
        domain: "rainbow.furver.se",
        redirects: &[],
        upstream: "localhost:45392",
    },
];

pub fn main() -> ! {
    Reversi::from(HOSTS).run()
}
