# Result

```bash
$ curl -vso /dev/null https://127.0.0.1:3000 2>&1 | grep ALPN
* ALPN: curl offers h2,http/1.1


$ curl -vso /dev/null https://127.0.0.1:3000
*   Trying 127.0.0.1:3000...
* Connected to 127.0.0.1 (127.0.0.1) port 3000
* ALPN: curl offers h2,http/1.1
* (304) (OUT), TLS handshake, Client hello (1):
} [296 bytes data]
*  CAfile: /etc/ssl/cert.pem
*  CApath: none
* Send failure: Broken pipe
* LibreSSL/3.3.6: error:02FFF020:system library:func(4095):Broken pipe
* Closing connection
* Send failure: Broken pipe

```
