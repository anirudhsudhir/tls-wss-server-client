server:
	cargo r --bin tls_server localhost:4443 certs/localhost_cert.pem certs/localhost_private.pem

client:
	cargo r --bin tls_client localhost 4443 certs/localhost_cert.pem
