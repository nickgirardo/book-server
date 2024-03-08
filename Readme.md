# Book Server

As a bit of a refresher on Rust, I went along with the simple http server example in the book. I kind of got carried away and ended up adding more and more features to it. It's still not at all practical, but I'm happy with it and I've learned a bit about Rust and a bit about HTTP through working on it

Major missing features:

- Parsing request headers and bodies. Currently in parsing of requests only the first line is considered; the rest of the request is simply discarded.
- Handling of POST etc. requests. Currently the server only supports setting GET handlers and responding to GET, HEAD, and OPTION requests. HEAD requests are just done by discarding the body of a GET request and OPTIONS always state that OPTION GET and HEAD requests are allowed (which is true).
- Query parameter support. Currently there is no support in place for query parameters.
- Non-utf8 responses. Currently responses bodies are represented as `String`s and only converted to bytes as they're being sent. This is unnecessarily constraining and disallows sending many types of responses, such as images. Only a minor change would be needed to resolve this.
- Improved ergonomics for route params. Currently route params are simply stored in a `HashMap` which leads to a bit of awkwardness when accessing them. The type system makes no guarantees of which values are in the HashMap. This could lead to issues if, for instance, you change the name of the parameter in the route but forget to update the name of the key in the `HashMap::get` call.

Nice-to-haves:

- The `file_response` utility is already very useful, but it could be cool to set the response's `Content-Type` header based on the type of the file it sends.
- It could be nice to automatically compress responses based on the request's `Accept-Encoding`.

Obviously there's a lot more than these simple items which separates this from a proper HTTP server (no thought has been given to security), but these are some simple examples of areas where this program is lacking.
