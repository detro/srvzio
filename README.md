[![Crates.io](http://meritbadge.herokuapp.com/srvzio)](https://crates.io/crates/srvzio)
[![Build Status](https://travis-ci.org/detro/srvzio.svg?branch=master)](https://travis-ci.org/detro/srvzio)
[![License](https://img.shields.io/badge/License-BSD%203--Clause-blue.svg)](https://opensource.org/licenses/BSD-3-Clause)
[![Doc.rs](https://docs.rs/srvzio/badge.svg)](https://docs.rs/crate/srvzio)

# srvzio - an helpful crate to implement services

## Services? What services?

This crate is inspired by my admiration and attachment to Google's [Guava](https://github.com/google/guava) library
for Java. Especially the [Services](https://github.com/google/guava/wiki/ServiceExplained).

A `Service` is then something that I would define as
_an entity that, when started it does work, and when stopped it does not_. Awfully vague, isn't it? OK, here is the
definition from the Guava wiki page on the topic:

<blockquote>
The Service represents an object with an operational state, with methods to start and stop.
For example, webservers, RPC servers, and timers can [be] Services[s].

Managing the state of services like these, which require proper startup and shutdown management, can be nontrivial,
especially if multiple threads or scheduling is involved.
</blockquote>

**srvzio** aims to provide a _rustic_ version of this. But, because of the enormous differences between Java and Rust,
we will start small, with few and simple abstractions, and then will hopefully grow the crate over time (maybe with
your contributions?).

## Building blocks

* `Service`: a Trait representing an object that can be started and can be stopped
* `ServiceStatusFlag`: a type designed to represent the internal state of a `Service` implementation
* `ServiceManager`: a [composite](https://en.wikipedia.org/wiki/Composite_pattern) of concrete `Service`s

## License

[BSD 3-Clause License](./LICENSE) 

