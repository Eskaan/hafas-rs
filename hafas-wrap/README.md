# Hafas-wrap Library
The `hafas-wrap` crate provides an interface for HAFAS. This is done using static request and response objects which can be parsed into json strings, requested and parsed back into objects. 

# Structure and usage
The main functionality comes with the [`Client`], see the documentation for it on how to use this library.

This library also provides some [`hafas_profiles`] for common endpoints and [request and response structs](methods) to simplyfy the working with HAFAS endpoints.
The [`util`] module provides functions to de- and encode Base64-AES Strings using a statically set key and a md5 hashing function needed for requests.