use std::collections::HashMap;

use crate::response::Response;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Route(Vec<RoutePart>);

#[derive(Debug, Clone, PartialEq, Eq)]
enum RoutePart {
    Static(String),
    Param(String),
    Wildcard,
}

pub struct RouteHandler(pub Route, pub fn(RouteParams) -> Response);

// TODO better error type?
fn into_parts(route: &str) -> Result<Vec<RoutePart>, &str> {
    if route.len() == 0 {
        return Err("Empty route");
    }

    let route = if route.chars().nth(0).unwrap() == '/' {
        &route[1..]
    } else {
        return Err("Route must begin with \"/\"");
    };

    if route.len() == 0 {
        return Ok(Vec::new());
    };

    let parts: Vec<_> = route.split("/").collect();

    parts
        .into_iter()
        .map(|part| {
            if part == "" {
                // Should empty route parts be allowed?
                Err("Empty route part")
            } else if part.chars().nth(0).unwrap() == '*' {
                if part.len() > 1 {
                    Err("Named wildcard")
                } else {
                    Ok(RoutePart::Wildcard)
                }
            } else if part.chars().nth(0).unwrap() == ':' {
                let tail = &part[1..];
                if tail == "" {
                    Err("Unnamed match")
                } else {
                    Ok(RoutePart::Param(String::from(tail)))
                }
            } else {
                Ok(RoutePart::Static(String::from(part)))
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn building_routes() {
        assert!(Route::new("/").is_ok());
        assert!(Route::new("/test/route").is_ok());
        assert!(Route::new("/test/:param").is_ok());
        assert!(Route::new("/test/:param/:second").is_ok());
        assert!(Route::new("/test/:param/after").is_ok());
        assert!(Route::new("/test/*").is_ok());
        assert!(Route::new("/test/*/*").is_ok());
        assert!(Route::new("/test/*/*/after").is_ok());

        assert_eq!(Route::new("/test/*hello"), Err("Named wildcard"));
        assert_eq!(Route::new("/test/:/"), Err("Unnamed match"));
        assert_eq!(Route::new("//"), Err("Empty route part"));
    }
}

impl Route {
    pub fn new(route: &str) -> Result<Self, &str> {
        into_parts(route).map(|parts| Route(parts))
    }

    // Matches rotues
    // Returns a tuple of self and the params for use with `find_map`
    pub fn match_route<'a>(
        &'a self,
        route: &str,
        handler: &'a RouteHandler,
    ) -> Option<(&RouteHandler, RouteParams)> {
        let route = match route.chars().nth(0) {
            Some('/') => &route[1..],
            _ => return None,
        };

        let mut params = HashMap::<String, String>::new();

        // Empty route case
        if route.len() == 0 {
            return if self.0.len() == 0 {
                // NOTE on this empty match the params hash map is empty
                Some((handler, RouteParams::new(params)))
            } else {
                None
            };
        }

        let match_parts: Vec<_> = route.split("/").collect();

        // For now forcing the route and the potential match to have the same number of parts
        // If we have a more powerful wildcard e.g. `**` this might no longer hold
        if self.0.len() != match_parts.len() {
            return None;
        }

        for (route_part, candidate) in (&self.0).into_iter().zip(match_parts) {
            match route_part {
                RoutePart::Wildcard => continue,
                RoutePart::Param(key) => {
                    params.insert(key.clone(), String::from(candidate));
                    continue;
                }
                RoutePart::Static(str) => {
                    if candidate != str {
                        return None;
                    }
                }
            }
        }

        Some((handler, RouteParams::new(params)))
    }
}

impl RouteHandler {
    pub fn new(route: Route, handler: fn(RouteParams) -> Response) -> Self {
        RouteHandler(route, handler)
    }
}

pub struct RouteParams(pub HashMap<String, String>);

impl RouteParams {
    pub fn new(hash_map: HashMap<String, String>) -> Self {
        RouteParams(hash_map)
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.0.get(key)
    }
}
