## mini-hyper-router

A port of tomaka's router macro from [rouille](https://github.com/tomaka/rouille) for use with [hyper](https://github.com/hyperium/hyper) (stable).

See [examples/hello_world.rs](/examples/hello_world.rs) for usage.

Note that path matches that consume `hyper::server::Response` must explicitly `return` to prevent `use after move` errors.

Match blocks do not return any useful value. If no paths match, processing will continue at the end of the macro, so you can use multiple `router` calls or handle "route not found" at that point.