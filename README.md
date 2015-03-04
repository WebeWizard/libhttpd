#libhttpd
A convenient library for embedding http servers into Rust projects.

####Current Features - 03/04/15
* keep-alive
* 'get' response
* gzip content encoding
* deflate content encoding
* chunked transfer encoding


Currently coded against Rust nightlies. 

Test the provided example server by running '**cargo test**' and then adding files you want to serve to the '**target**' directory.



####Update - 03/04/15
Completely rewrote the library.  Had to remove contexts for now, but this version now has multi-threaded encoding for http message bodies.  Currently only gzip and chunked are available, deflate will be available soon.
