use serde::Serialize;
use url::Url;

use Result;

#[derive(Debug, Clone)]
pub struct PathTemplate {
    segments: &'static [PathSegment],
}
impl PathTemplate {
    pub fn new(segments: &'static [PathSegment]) -> Self {
        PathTemplate { segments }
    }

    // TODO
    pub fn fill_path_segments<T>(&self, url: &mut Url, params: T) -> Result<()>
        where T: Serialize
    {
        let mut serializer = ::path_template_ser::Serializer::new();
        track_try!(params.serialize(&mut serializer));
        let _url = url;
        panic!()
    }
}

#[derive(Debug)]
pub enum PathSegment {
    Val(&'static str),
    Var(&'static str),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        use self::PathSegment::*;
        static SEGMENTS: &[PathSegment] = &[Val("foo"), Var("Var"), Val("baz")];
        let _path = PathTemplate::new(SEGMENTS);
    }
}
