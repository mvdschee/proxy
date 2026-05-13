use crate::{
	Error, Result,
	core::{
		handlers::filesystem::{safe_path, write_file},
		models::{
			certs::{Certificate, CertificateType},
			tasks::TaskInterval,
		},
	},
	info,
};
use rcgen::{CertifiedKey, generate_simple_self_signed};
use std::time::Duration;
use tokio::time;

pub async fn initial_certs(certificates: Vec<Certificate>) -> Result<()> {
	for certificate in &certificates {
		// self signed certificates are good until the year 4096
		// this will be replace every restart so it's safe to keep using the default setting
		if certificate.cert_type == CertificateType::SelfSigned {
			info!("generating self-signed certificate for {}", certificate.host);

			let subject_alt_names = vec![certificate.host.to_string()];
			let pem_filename = format!("{}.pem", certificate.host);
			let key_filename = format!("{}.key", certificate.host);

			let key_path = safe_path(&certificate.cert_dir, &key_filename)?;
			let pem_path = safe_path(&certificate.cert_dir, &pem_filename)?;

			let CertifiedKey {
				cert,
				signing_key,
			} = generate_simple_self_signed(subject_alt_names)
				.map_err(|e| Error::Certificate(e.to_string()))?;

			let pem_serialized = cert.pem();
			let key_serialized = signing_key.serialize_pem();

			write_file(pem_path, pem_serialized.as_bytes())?;
			write_file(key_path, key_serialized.as_bytes())?;
		}
	}

	Ok(())
}

pub async fn background_certs_task(
	certificates: Vec<Certificate>,
	task_interval: TaskInterval,
) -> Result<()> {
	// only acme certificates need to be renewed
	let certificates = certificates
		.into_iter()
		.filter(|cert| cert.cert_type == CertificateType::Acme)
		.collect::<Vec<Certificate>>();

	loop {
		info!("certificates: {}", certificates.len());

		// for certificate in &certificates {
		// }

		time::sleep(Duration::from_secs(*task_interval)).await;
	}

	Ok(())
}
