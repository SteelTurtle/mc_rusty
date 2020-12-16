# McRusty - a basic order simulator for a restaurant

> STILL HEALTHIER THAN MC DONALDS

[![Rust Version][rust-image]][rust-url]
[![Actix Version][actix-image]][actix-url]

McRusty simulates a multi-threading environment where several "waiters" are concurrently trying to create,
read and delete orders for the tables of a virtual restaurant. Given its purely demonstrative purpose, the
domain of the simulation is extremely simple. The core of the simulation is an `HashMap<usize, Vec<Dish>>` "
hidden" behind a reference-counted mutex. This makes possible to queue all the read/write operations attempted
on the map by multiple threads.

## How to run McRusty

For either **Linux**, **Windows** or **OS X**, the process to run the program is identical. Assuming you
already have the most recent Rust development environment distribution
_([available here][rust-lang-distro])_ you can run the server component with the following command:

```sh
cargo run --bin mc_rusty_server
```

In addition, you can run a simple load test and populate the restaurant domain by running the test client, _
after you have started the server_:

```sh
cargo run --bin test_client
```

Although very basic, the crate comes with a small test suite that can be launched with the following command:

```sh
cargo test
```  

## Usage example

The examples below refer to the use of the command line tool `HTTPie` (https://httpie.io/)
to interact with the server API. Of course any CLI utility like `curl`
will work just fine. Once the server is started, its minimal REST API can be used to execute the following
operations:
> HTTP GET requests to obtain the list of current ordered dishes for a specific table number:

```sh
http GET 127.0.0.1:9090/api/orders/{table_number}
```

The output should be similar to the one below:

```sh
HTTP/1.1 200 OK
content-length: 154
content-type: application/json
date: Mon, 23 Nov 2020 11:46:55 GMT

{
    "Ok": [
        {
            "name": "steak",
            "preparation": 7
        },
        {
            "name": "mixed salad",
            "preparation": 11
        }
    ]
}
```

> HTTP POST commands to save a new order for a given table:

```sh
echo '{"name": "お好み焼き"}' | http POST 127.0.0.1:9090/api/orders/{table_number}
```

You should receive an HTTP response code `201` as below:

```sh
HTTP/1.1 201 Created
content-length: 0
date: Mon, 23 Nov 2020 11:54:53 GMT
```

> HTTP DELETE command to delete a given dish from the order associated to a table. For example,
> let's delete the dish we just ordered for table 14:

```sh
http DELETE 127.0.0.1:9090/api/orders/14/お好み焼き
```

The response should be an HTTP code `204`:

```sh
HTTP/1.1 204 No Content
date: Mon, 23 Nov 2020 12:03:00 GMT
```

## Release History

* 0.0.1
    * Work in progress

## List of potential improvements

So many I do not even know where to start :)

- vastly improve the current toy-level API error handling
- better handlers logic to identify edge cases for the user input
- better documentation for the code by using Rust in-line docs
- a lot more testing
- a lot more features
- a lot more of everything else

## Contributing, because why not?

1. Fork it (<https://github.com/SteelTurtle/McRusty/fork>)
2. Create your feature branch (`git checkout -b feature/fooBar`)
3. Commit your changes (`git commit -am 'Add some fooBar'`)
4. Push to the branch (`git push origin feature/fooBar`)
5. Create a new Pull Request

<!-- Various links -->

[rust-image]: https://img.shields.io/badge/Rust-1.48-orange?style=flat-square

[rust-url]: https://rust-lang.org

[actix-image]: https://img.shields.io/badge/Actix-3-blue?style=flat-square

[actix-url]: https://actix.rs

[rust-lang-distro]: https://www.rust-lang.org/learn/get-started
