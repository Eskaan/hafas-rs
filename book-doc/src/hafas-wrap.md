# The hafas_wrap library

The main functionality comes with the `Client` object, as all requests are done with it.

This library also provides some [`hafas_profiles`](#hafas-profiles) for common endpoints and [request and response structs](#methods) to simplify the working with HAFAS endpoints.
The [`util`](#utils) module provides functions to de- and encode Base64-AES Strings using a statically set key and a md5 hashing function needed for requests.

## The request function

It provides a simple, through customisable system to access the HAFAS[^hafas].
There are three common request parameters:
```rust,ignore
{{#include ../../hafas-wrap/src/lib.rs:76:80}}
```
- A reference to it`self`, to access the [reqwest] request client used to make web requests

- A reference to an instance of the `HafasProfile` type
  
  A `HafasProfile` specifies the endpoint, possible encryption and additional config data that have to be passed on with the request

- An array containing instances of the `RawHafasRequest` type

  The `RawHafasRequest` type (and it's request generic) have to implement `serde::Serialize + Send`. 
  These implementations are needed for the type to be serialized to a JSON string, used in the POST request to HAFAS[^hafas].

The `request` function also tries to parse the resulting request bodies into the specified generic type `O`. 
This type has to implement `DeserializeOwned` so it can be deserialized from a JSON object. 

## Request procedure
There are two methods of requesting data: Either `request` or `request_raw`, 
where `request` is mainly a wrapper around the `request_raw` method to make parsing of the results easier.

Here is a summary of what the functions do:

Both:

1. Clone the `HafasProfile` into a JSON object,
  
   Serialize the requests into a JSON object and set the `svcReqL` value of the request to it. 
  
   Serialize the JSON object into a String
   ```rust,ignore
{{#include ../../hafas-wrap/src/lib.rs:147:149}}
   ```

2. Optionally create a checksum for the GET parameters of the request.
   ```rust,ignore
{{#include ../../hafas-wrap/src/lib.rs:151:154}}
   ```
   
3. Create, send and wait for the response text of a web request to HAFAS[^hafas].
   ```rust,ignore
{{#include ../../hafas-wrap/src/lib.rs:156:164}}
   ```

`request_raw` at this point returns the response String, whilst `request` continues with parsing the response:

Only `request`:

5. Checks if HAFAS[^hafas] returned an error message.
   ```rust,ignore
{{#include ../../hafas-wrap/src/lib.rs:85:87}}
   ```

6. Deserializes the request to a JSON object and then tries to get the response body segment as an array.
   ```rust,ignore
{{#include ../../hafas-wrap/src/lib.rs:101:106}}
   ```

7. Deserializes the array of JSON objects into the specified generic type `O`
   ```rust,ignore
{{#include ../../hafas-wrap/src/lib.rs:107:117}}
   ```

## HAFAS Profiles
In the `hafas_profiles` module, you can find multiple config presets for different HAFAS[^hafas] endpoints. 

These were collected from different other open source projects.

## Methods
In the `methods` module, you can find some Request and Response struct presets. You do not have to use these, but they have been tested and seem to work.

Current available methods:
- HimSearch
- JourneyMatch
- JourneyDetail

## Utils
Some miscellaneous functions like ones to AES-Encrypt with a set key or to hash a String using the `md5` digest algorithm. 

[reqwest]: https://crates.io/crates/reqwest
[^hafas]: [Hacon Fahrplan Auskunfts System](https://de.wikipedia.org/wiki/HAFAS)
