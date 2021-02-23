use sequoia_openpgp as openpgp;
use openpgp::cert::prelude::*;
use openpgp::crypto::Password;
use openpgp::serialize::stream::*;
use openpgp::policy::StandardPolicy as policy;
use openpgp::parse::{Parse, stream::*};
use anyhow;

use std::io::{Read, Write};

//References:
//https://docs.sequoia-pgp.org/sequoia_guide/chapter_01/index.html
//https://gitlab.com/sequoia-pgp/sequoia/blob/cb001fdaec7e6fa91109f7649ab170e534ec7227/openpgp/examples/generate-sign-verify.rs

pub fn create_key(username: &str, password: Option<Password>) -> openpgp::Result<Cert> {
	let (cert, _) = CertBuilder::new()
	.add_userid(username)
	.set_password(password)
	.generate()?;
	
	Ok(cert)
}

/*
TODO

I can split these signatures into ones that return the vec and ones that write to a direct buffer

*/

pub fn sign_data(plain_data: &[u8], cert: &Cert) -> openpgp::Result<Vec<u8>> {
	let keypair = cert.primary_key().key().clone()
	.parts_into_secret()?.into_keypair()?;
	
	//Create a pipe for the data to go through
	let mut signed_data = Vec::new();

	let message = Message::new(&mut signed_data);
	let message = Signer::new(message, keypair).build()?;
	let mut message = LiteralWriter::new(message).build()?;
	message.write_all(plain_data)?;
	message.finalize()?;

	Ok(signed_data)
}

struct Helper<'a> {
	cert: &'a Cert
}

pub fn verify(signed_data: &[u8], cert: &Cert) -> openpgp::Result<Vec<u8>> {
	let mut plaintext = Vec::<u8>::new();

	let policy = policy::new();
	let mut verifier = VerifierBuilder::from_bytes(signed_data)?
		.with_policy(&policy, None, Helper {cert: cert})?;
	
	verifier.read_to_end(&mut plaintext);

	Ok(plaintext)
}

impl<'a> VerificationHelper for Helper<'a> {
	fn get_certs(&mut self, _ids: &[openpgp::KeyHandle]) -> openpgp::Result<Vec<Cert>> {
		Ok(vec![self.cert.clone()])
	}


	///I completely stole this from the example pages
	fn check(&mut self, structure: MessageStructure) -> openpgp::Result<()> {
		        // In this function, we implement our signature verification
        // policy.
 
        let mut good = false;
        for (i, layer) in structure.into_iter().enumerate() {
            match (i, layer) {
                // First, we are interested in signatures over the
                // data, i.e. level 0 signatures.
                (0, MessageLayer::SignatureGroup { results }) => {
                    // Finally, given a VerificationResult, which only says
                    // whether the signature checks out mathematically, we apply
                    // our policy.
                    match results.into_iter().next() {
                        Some(Ok(_)) =>
                            good = true,
                        Some(Err(e)) =>
                            return Err(openpgp::Error::from(e).into()),
                        None =>
                            return Err(anyhow::anyhow!("No signature")),
                    }
                },
                _ => return Err(anyhow::anyhow!(
                    "Unexpected message structure")),
            }
        }
 
        if good {
            Ok(()) // Good signature.
        } else {
            Err(anyhow::anyhow!("Signature verification failed"))
        }
	}
}
