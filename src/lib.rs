#[macro_export]
macro_rules! router {
    ($request:expr, $(($method:pat) ($($pat:tt)+) => $value:block),*) => {
        {
            let request = $request;

            fn splitter(ch: char) -> bool {
                match ch {
                    '.' | '/' | '-' => { true },
                    _ => { false }
                }
            }

            let request_url = match request.uri {
                ::hyper::uri::RequestUri::AbsolutePath(ref s) => {
                    let pos = s.find('?').unwrap_or(s.len());
                    Some(&s[..pos])
                },
                ::hyper::uri::RequestUri::AbsoluteUri(ref url) => Some(url.path()),
                _ => None,
            };

            let mut matched = false;

            if let Some(request_url) = request_url {
                $({
                        if !matched {
                            match request.method {
                                $method => {
                                    matched = router!(__check_pattern request_url $value $($pat)+);
                                },
                                _ => {}
                            }
                        }
                })+
            }
        }
    };

    (__check_pattern $url:ident $value:block /{$p:ident} $($rest:tt)*) => (
        if !$url.starts_with('/') {
            false
        } else {
            let url = &$url[1..];
            let pat_end = url.find(splitter).unwrap_or(url.len());
            let rest_url = &url[pat_end..];

            if let Some($p) = url[0 .. pat_end].parse().ok() {
                router!(__check_pattern rest_url $value $($rest)*)
            } else {
                false
            }
        }
    );

    (__check_pattern $url:ident $value:block /{$p:ident: $t:ty} $($rest:tt)*) => (
        if !$url.starts_with('/') {
            false
        } else {
            let url = &$url[1..];
            let pat_end = url.find(splitter).unwrap_or(url.len());
            let rest_url = &url[pat_end..];

            if let Some($p) = url[0 .. pat_end].parse().ok() {
                let $p: $t = $p;
                router!(__check_pattern rest_url $value $($rest)*)
            } else {
                false
            }
        }
    );

    (__check_pattern $url:ident $value:block /$p:ident $($rest:tt)*) => (
        {
            let required = concat!("/", stringify!($p));
            if $url.starts_with(required) {
                let rest_url = &$url[required.len()..];
                router!(__check_pattern rest_url $value $($rest)*)
            } else {
                false
            }
        }
    );

    (__check_pattern $url:ident $value:block - $($rest:tt)*) => (
        {
            if $url.starts_with('-') {
                let rest_url = &$url[1..];
                router!(__check_pattern rest_url $value $($rest)*)
            } else {
                false
            }
        }
    );

    (__check_pattern $url:ident $value:block . $($rest:tt)*) => (
        {
            if $url.starts_with('.') {
                let rest_url = &$url[1..];
                router!(__check_pattern rest_url $value $($rest)*)
            } else {
                false
            }
        }
    );

    (__check_pattern $url:ident $value:block) => (
        if $url.len() == 0 { $value; true } else { false }
    );

    (__check_pattern $url:ident $value:block /) => (
        if $url == "/" { $value; true } else { false }
    );

    (__check_pattern $url:ident $value:block $p:ident $($rest:tt)*) => (
        {
            let required = stringify!($p);
            if $url.starts_with(required) {
                let rest_url = &$url[required.len()..];
                router!(__check_pattern rest_url $value $($rest)*)
            } else {
                false
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
