use std::collections::BTreeMap;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;

use async_trait::async_trait;
use http::header;
use http::uri::Authority;
use http::uri::PathAndQuery;
use rustls::crypto::aws_lc_rs::sign::any_supported_type;

use pingora::Error;
use pingora::ErrorType;
use pingora::Result;
use pingora::http::ResponseHeader;
use pingora::listeners::tls::TlsSettings;
use pingora::prelude::HttpPeer;
use pingora::proxy::ProxyHttp;
use pingora::proxy::Session;
use pingora::proxy::http_proxy_service;
use pingora::server::Server;
use pingora::tls::cert_resolvers::CertifiedKey;
use pingora::tls::cert_resolvers::ResolvesServerCertUsingSni;
use pingora::tls::load_certs_and_key_files;

type Peer = Box<HttpPeer>;

#[derive(Clone)]
struct Spec {
    domain: &'static str,
    redirects: &'static [&'static str],
    upstream: &'static str,
}

const SPECS: &[Spec] = &[Spec {
    domain: "www.fimfarchive.net",
    redirects: &["fimfarchive.net"],
    upstream: "localhost:34407",
}];

enum Target {
    Invalid,
    Redirect(String),
    Upstream(Peer),
}

struct Mapper {
    redirects: BTreeMap<&'static str, &'static str>,
    upstreams: BTreeMap<&'static str, Peer>,
}

impl Mapper {
    fn certpath(domain: &str, file: &str) -> String {
        let dir = PathBuf::from(format!("/certs/live/{domain}"));
        let path = dir.join(file).canonicalize().unwrap();
        path.to_str().unwrap().into()
    }

    fn resolve(&self, session: &mut Session) -> Target {
        let lookup = session
            .get_header(header::HOST)
            .and_then(|header| header.to_str().ok())
            .or(session.req_header().uri.host())
            .and_then(|host| Authority::from_str(host).ok());

        let Some(authority) = lookup else {
            return Target::Invalid;
        };

        if let Some(peer) = self.upstreams.get(authority.host()) {
            return Target::Upstream(peer.clone());
        }

        if let Some(target) = self.redirects.get(authority.host()) {
            let path = session.req_header().uri.path_and_query();
            let path = path.map(PathAndQuery::as_str).unwrap_or("/");
            return Target::Redirect(format!("https://{target}{path}"));
        }

        Target::Invalid
    }
}

impl From<&[Spec]> for Mapper {
    fn from(value: &[Spec]) -> Self {
        let mut redirects = BTreeMap::new();
        let mut upstreams = BTreeMap::new();

        for spec in value {
            let domain = spec.domain;
            let peer = HttpPeer::new(spec.upstream, false, domain.into());
            upstreams.insert(domain, Box::new(peer));

            for redirect in spec.redirects {
                redirects.insert(*redirect, domain);
            }
        }

        Self {
            redirects,
            upstreams,
        }
    }
}

impl From<&Mapper> for TlsSettings {
    fn from(val: &Mapper) -> Self {
        let mut certs = ResolvesServerCertUsingSni::new();
        let redirects = val.redirects.keys();
        let upstreams = val.upstreams.keys();

        for domain in upstreams.chain(redirects) {
            let cert = Mapper::certpath(domain, "fullchain.pem");
            let key = Mapper::certpath(domain, "privkey.pem");
            let (cert, key) = load_certs_and_key_files(&cert, &key).unwrap().unwrap();
            let key = any_supported_type(&key).unwrap();
            let ck = CertifiedKey::new(cert, key);

            certs.add(domain, ck).unwrap();
        }

        TlsSettings::resolver(Arc::new(certs)).unwrap()
    }
}

#[async_trait]
impl ProxyHttp for Mapper {
    type CTX = Option<Target>;

    fn new_ctx(&self) -> Self::CTX {
        None
    }

    async fn request_filter(&self, session: &mut Session, context: &mut Self::CTX) -> Result<bool> {
        use http::StatusCode as Code;

        let mut response = Box::new(match context.get_or_insert(self.resolve(session)) {
            Target::Invalid => ResponseHeader::build(Code::SERVICE_UNAVAILABLE, Some(0))?,
            Target::Upstream(_) => return Ok(false),
            Target::Redirect(target) => {
                let mut response = ResponseHeader::build(Code::SEE_OTHER, Some(0))?;
                response.insert_header(header::LOCATION, target.as_str())?;
                response
            }
        });

        response.insert_header(header::CONTENT_LENGTH, "0")?;
        session.write_response_header(response, true).await?;
        session.write_response_body(None, true).await?;

        Ok(true)
    }

    async fn upstream_peer(&self, session: &mut Session, context: &mut Self::CTX) -> Result<Peer> {
        match context.take().unwrap_or(self.resolve(session)) {
            Target::Upstream(peer) => Ok(peer),
            _ => Err(Error::new(ErrorType::InternalError)),
        }
    }
}

pub fn main() {
    let mapper = Mapper::from(SPECS);
    let tls = TlsSettings::from(&mapper);
    let mut server = Server::new(None).unwrap();
    let mut service = http_proxy_service(&server.configuration, mapper);

    service.add_tcp("[::]:8080");
    service.add_tls_with_settings("[::]:8443", None, tls);

    server.bootstrap();
    server.add_service(service);
    server.run_forever();
}
