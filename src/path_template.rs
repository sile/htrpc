#[derive(Debug, Clone)]
pub struct PathTemplate {
    segments: &'static [PathSegment],
}
impl PathTemplate {
    pub fn new(segments: &'static [PathSegment]) -> Self {
        PathTemplate { segments }
    }
    pub fn var_count(&self) -> usize {
        self.segments
            .iter()
            .filter(|s| s == &&PathSegment::Var)
            .count()
    }
    pub fn len(&self) -> usize {
        self.segments.len()
    }
    pub fn get_val(&self, i: usize) -> Option<&str> {
        if let PathSegment::Val(s) = self.segments[i] {
            Some(s)
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum PathSegment {
    Val(&'static str),
    Var,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        use self::PathSegment::*;
        static SEGMENTS: &[PathSegment] = &[Val("foo"), Var, Val("baz")];
        let _path = PathTemplate::new(SEGMENTS);
    }
}
