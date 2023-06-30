//! TLS key and certificate parsing.

use {
    crate::config::TlsConfig,
    anyhow::{ensure, Context, Result},
    rustls::ServerConfig,
    std::{fs::File, io::BufReader},
};

/// Builds the TLS config as required by actix
/// if the required components are provided in the config.
pub fn build_server_config(tls_config: &TlsConfig) -> Result<ServerConfig> {
    let reader = |path| -> Result<_> {
        let reader = File::open(path).with_context(|| format!("error opening {:?}", path))?;
        Ok(BufReader::new(reader))
    };

    let client_cert_verifier = {
        let root_cert_store =
            rustls_pemfile::certs(&mut reader(&tls_config.root_certificates_pem_path)?)
                .context("error parsing root certificates pem file")?
                .into_iter()
                .map(rustls::Certificate)
                .try_fold(
                    rustls::RootCertStore::empty(),
                    |mut store, cert| -> Result<_> {
                        store
                            .add(&cert)
                            .context("couldn't add certificate to root store")?;
                        Ok(store)
                    },
                )?;

        ensure!(!root_cert_store.is_empty(), "no root certificates found");

        rustls::server::AllowAnyAnonymousOrAuthenticatedClient::new(root_cert_store)
    };

    let cert_chain = rustls_pemfile::certs(&mut reader(&tls_config.cert_path)?)
        .context("couldn't parse cert")?
        .into_iter()
        .map(rustls::Certificate)
        .collect();

    let key = {
        let mut keys = rustls_pemfile::pkcs8_private_keys(&mut reader(&tls_config.cert_key_path)?)
            .context("couldn't parse cert key")?;

        ensure!(!keys.is_empty(), "no cert key found");
        // get the private key
        keys.swap_remove(0)
    };

    rustls::ServerConfig::builder()
        .with_safe_defaults()
        .with_client_cert_verifier(client_cert_verifier)
        .with_single_cert(cert_chain, rustls::PrivateKey(key))
        .context("couldn't set up certificate chain")
}
