# Generate 2048-bit RSA private key:

openssl genrsa -out key.pem 2048

# Generate a Certificate Signing Request:

openssl req -new -sha256 -key key.pem -out csr.csr

# Generate a self-signed x509 certificate suitable for use on web servers.

openssl req -x509 -sha256 -days 365 -key key.pem -in csr.csr -out certificate.pem

# Create SSL identity file in PKCS12 as mentioned here

openssl pkcs12 -export -out client-identity.p12 -inkey key.pem -in certificate.pem
