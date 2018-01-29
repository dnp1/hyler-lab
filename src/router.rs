use hyper::Method;

struct Route {
    name: String,
    handler: i32,
    method: Method,
}


struct Bounds {
    left: usize,
    right: usize,
}

struct SegmentBuilder {
    handler: Option<i32>,
    name: String,
    children: Vec<SegmentBuilder>,
    param: Option<Box<SegmentBuilder>>,
}

impl SegmentBuilder {
    fn is_param(name: &str) -> bool {
        name == "{}"
    }

    fn new<'a>(v: &'a mut [Route]) -> SegmentBuilder {
        v.sort_by(|x, y| x.name.cmp(&y.name));
        let divisor = "/";
        let mut root = SegmentBuilder {
            handler: None,
            name: "".to_owned(),
            children: Vec::new(),
            param: None,
        };

        for route in v {
            let size: usize = route.name.split(divisor).count();
            if size == 0 {
                root.handler = Some(route.handler)
            }

            let mut parent = &mut root;

            for (inx, name) in route.name.split(divisor).enumerate() {
                let handler = if inx + 1 == size {
                    Some(route.handler)
                } else {
                    None
                };

                let segment_pos = parent.children.iter().position(|element| element.name == name);
                parent = match segment_pos {
                    None => {
                        let mut segment = SegmentBuilder {
                            handler,
                            name: name.to_owned(),
                            children: Vec::new(),
                            param: None,
                        };
                        if Self::is_param(&route.name) {
                            parent.param = Some(Box::new(segment))
                        } else {
                            parent.children.push(segment);
                        };
                        &mut segment
                    },
                    Some(inx) => {
                        let segment = parent.children.get_mut(inx).unwrap();
                        segment.handler = handler;
                        segment
                    }
                };

            }
        }
        root
    }


    fn build_tree(segment_builder: &SegmentBuilder, mut segments: Vec<Segment>, mut names: String, mut left: usize, mut right: usize) -> (Vec<Segment>, String, usize, usize) {
        left = right;
        right += segment_builder.children.len();
        // find string, matching, get string, otherwise, push and get
        let left_str = match segment_builder.name.find(&segment_builder.name) {
            Some(inx) => inx,
            None => {
                let inx = names.len();
                names.push_str(&segment_builder.name);
                inx
            }
        };
        let right_str = names.len();

        segments.push(Segment {
            handler: segment_builder.handler,
            children: Bounds {
                left,
                right,
            },
            name: Bounds {
                left: left_str,
                right: right_str,
            },
        });

        for child in &segment_builder.children {
            let ret = Self::build_tree(child, segments, names, left, right);
            segments = ret.0;
            names = ret.1;
            left = ret.2;
            right = ret.3;
        }

        (segments, names, left, right)
    }


    fn build(self) -> (Vec<Segment>, String) {
        let (mut segments, mut names, _, _) = Self::build_tree(&self, Vec::new(), String::new(), 0, 1);
        (segments, names)
    }
}

struct Segment {
    handler: Option<i32>,
    name: Bounds,
    children: Bounds,
//    wildcard_child: Option<usize>,
//    param_child: Option<usize>,
//    is_param: bool,
}

pub struct MethodRouter {}


impl MethodRouter {
    fn new<'a>(v: &'a mut [Route]) -> MethodRouter {
        let builder = SegmentBuilder::new(v);
        MethodRouter {}
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
    pub fn add_route<'a>(&mut self, method: Method, route: &'a str, handler: i32) {
        let vet = match method {
            Method::Options => &mut self.options,
            Method::Get => &mut self.get,
            Method::Post => &mut self.post,
            Method::Put => &mut self.put,
            Method::Delete => &mut self.delete,
            Method::Head => &mut self.head,
            Method::Trace => &mut self.trace,
            Method::Connect => &mut self.connect,
            Method::Patch => &mut self.patch,
            _ => return,
        };
        vet.push(Route { name: route.to_owned(), handler, method });
    }

    pub fn build(mut self) -> Option<Router> {
        let router = Router {
            options: MethodRouter::new(&mut self.options),
            get: MethodRouter::new(&mut self.get),
            post: MethodRouter::new(&mut self.post),
            put: MethodRouter::new(&mut self.put),
            delete: MethodRouter::new(&mut self.delete),
            head: MethodRouter::new(&mut self.head),
            trace: MethodRouter::new(&mut self.trace),
            connect: MethodRouter::new(&mut self.connect),
            patch: MethodRouter::new(&mut self.patch),
        };
        Some(router)
    }
}





