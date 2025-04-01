#!/bin/sh

openssl req -x509 -newkey rsa:2048 -keyout localhost_private.pem -nodes -out localhost_cert.pem -subj "/CN=localhost" -extensions v3_req -config <(printf "[req]\ndistinguished_name=req\n[v3_req]\nbasicConstraints=critical,CA:FALSE\nkeyUsage=nonRepudiation,digitalSignature,keyEncipherment\nextendedKeyUsage=serverAuth\nsubjectAltName=DNS:localhost")
