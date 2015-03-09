#libhttpd
A convenient library for embedding http servers into Rust projects.

####Current Features - 03/04/15
* keep-alive
* 'get' response
* gzip content encoding
* deflate content encoding
* chunked transfer encoding


Currently coded against Rust nightlies. 

Test the provided example server by running '**cargo test**' and then adding files you want to serve to the '**target/debug**' directory.  Server is currently bound to 127.0.0.1 ( localhost ) port 8080.



####Update - 03/09/15
* Deflate content encoding is now available.  Encoders now have weights associated with them.  The default is 100u8, Heavier weights take preference over lower weights.  Set two encoders with equal weight to use them both at once.
* Switched internal tcp streams over to Rust's new IO api.