use hyper::Method;

struct Route {
    name: String,
    handler: i32,
    method: Method,
}

// Let think about algorithm

struct ParamBuilder {
    handler: Option<i32>,
    children: Vec<SegmentBuilder>
}


struct Param {
    handler: Option<i32>,
    children: Option<Bounds>
}

struct Bounds {
    left: usize,
    right: usize,
}

struct SegmentBuilder {
    handler: Option<i32>,
    name: String,
    children: Vec<SegmentBuilder>,
    param: Option<Param>,
}

struct Segment {
    handler: Option<i32>,
    name: Bounds,
    children: Bounds,
    param: Option<Param>,
}
pub struct MethodRouter {}


impl MethodRouter {
    fn new<'a>(v: &'a mut Vec<Route>) -> MethodRouter {
        let divisor = "/";
        let level_vet = Vec::new();
        let names = String::new();


        for route in v {
            let mut level_count = 0;
            let mut level = Vec::new();
            let split = route.name.split(divisor);

            for name in  {
                level.push(SegmentBuilder {
                    handler:None,
                    name: Bounds{left:0, right: 0},
                    children: Bounds{left: 0; right 0}

                })
                level_vet.push(level);
                levent_count += 1;
            }


        }
        MethodRouter{}
    }
}

pub struct Router {
    options: MethodRouter,
    get: MethodRouter,
    post: MethodRouter,
    put: MethodRouter,
    delete: MethodRouter,
    head: MethodRouter,
    trace: MethodRouter,
    connect: MethodRouter,
    patch: MethodRouter,
}

pub struct RouterBuilder {
    options: Vec<Route>,
    get: Vec<Route>,
    post: Vec<Route>,
    put: Vec<Route>,
    delete: Vec<Route>,
    head: Vec<Route>,
    trace: Vec<Route>,
    connect: Vec<Route>,
    patch: Vec<Route>,
}

impl RouterBuilder {
    fn get_vet<'a>(self, method: Method) ->  Option<&'a mut Vec<Route>> {
        let mut vet = match method {
            Method::Options => &mut self.options,
            Method::Get => &mut self.get,
            Method::Post => &mut self.post,
            Method::Put => &mut self.put,
            Method::Delete => &mut self.delete,
            Method::Head => &mut self.head,
            Method::Trace => &mut self.trace,
            Method::Connect => &mut self.connect,
            Method::Patch => &mut self.patch,
            _ => return None,
        };
        return Some(vet)
    }
    pub fn add_route<'a>(&mut self, method: Method, route: &'a str, handler: i32) {
        let mut vet = match self.get_vet(method) {
            Some(v) => v,
            None => return,
        };
        vet.push(Route { name: route.to_owned(), handler, method });
    }

    pub fn build(self) -> Option<Router> {
        let mut router = Router {
            options: MethodRouter::new(self.get_vet(Method::Options)?),
            get: MethodRouter::new(self.get_vet(Method::Get)?),
            post: MethodRouter::new(self.get_vet(Method::Post)?),
            put: MethodRouter::new(self.get_vet(Method::Put)?),
            delete: MethodRouter::new(self.get_vet(Method::Delete)?),
            head: MethodRouter::new(self.get_vet(Method::Head)?),
            trace: MethodRouter::new(self.get_vet(Method::Trace)?),
            connect: MethodRouter::new(self.get_vet(Method::Connect)?),
            patch: MethodRouter::new(self.get_vet(Method::Patch)?),
        };
        Some(router)
    }
}





