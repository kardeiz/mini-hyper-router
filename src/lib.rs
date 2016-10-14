// Much of the body of this macro is taken from
// https://github.com/tomaka/rouille/blob/master/src/router.rs
// The original copyright notice is included below

// Copyright (c) 2016 The Rouille developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

#[macro_export]
macro_rules! routing {
    ($request:expr, $([$method:pat] ($($path:tt)+) => $value:block),*) => {
        {
            let request = $request;

            fn capture_split(ch: char) -> bool {
                match ch {
                    '.' | '/' => { true },
                    _ => { false }
                }
            }

            fn path_split(ch: char) -> bool {
                match ch {
                    '?' | '#' => { true },
                    _ => { false }
                }
            } 

            let path = match request.uri {
                ::hyper::uri::RequestUri::AbsolutePath(ref s) => {
                    let pos = s.find(path_split).unwrap_or(s.len());
                    &s[..pos]
                },
                ::hyper::uri::RequestUri::AbsoluteUri(ref url) => url.path(),
                _ => panic!("Unexpected request URI")
            };

            $({
                match request.method {
                    $method => {
                        routing!(__check__ path $value $($path)+);
                    },
                    _ => {}
                }
            })+
        }
    };

    (__check__ $path:ident $value:block /<$p:ident> $($rest:tt)*) => (
        if $path.starts_with('/') {
            let url = &$path[1..];
            let pat_end = url.find(capture_split).unwrap_or(url.len());
            let rest = &url[pat_end..];

            if let Some($p) = url[0 .. pat_end].parse().ok() {
                routing!(__check__ rest $value $($rest)*)
            }
        }
    );

    (__check__ $path:ident $value:block /<<$p:ident>> $($rest:tt)*) => (
        if $path.starts_with('/') {
            let path = &$path[1..];
            let end = path.len();
            if let Some($p) = path[0 .. end].parse().ok() {
                $value
            }
        }
    );

    (__check__ $path:ident $value:block /$p:ident $($rest:tt)*) => (
        {
            let required = concat!("/", stringify!($p));
            if $path.starts_with(required) {
                let rest = &$path[required.len()..];
                routing!(__check__ rest $value $($rest)*)
            }
        }
    );

    (__check__ $path:ident $value:block . $($rest:tt)*) => (
        {
            if $path.starts_with('.') {
                let rest = &$path[1..];
                routing!(__check__ rest $value $($rest)*)
            }
        }
    );

    (__check__ $path:ident $value:block) => (
        if $path.len() == 0 { $value }
    );

    (__check__ $path:ident $value:block /) => (
        if $path == "/" { $value } 
    );

    (__check__ $path:ident $value:block $p:ident $($rest:tt)*) => (
        {
            let required = stringify!($p);
            if $path.starts_with(required) {
                let rest = &$path[required.len()..];
                routing!(__check__ rest $value $($rest)*)
            } 
        }
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
