server:
	cargo r --bin server localhost:4443 certs/localhost_cert.pem certs/localhost_private.pem

client:
	cargo r --bin client localhost 4443 certs/localhost_cert.pem
