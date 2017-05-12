#[macro_export]
macro_rules! htrpc_entry_point {
    ($($segment:tt),*) => {
        {
            static SEGMENTS: &[$crate::path_template::PathSegment] =
                &[$(htrpc_expand_segment!($segment)),*];
            $crate::path_template::PathTemplate::new(SEGMENTS)
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! htrpc_expand_segment {
    (_) => {
        $crate::path_template::PathSegment::Var
    };
    ($s:expr) => {
        $crate::path_template::PathSegment::Val($s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    pub fn is_var_remaning(&self, i: usize) -> bool {
        self.segments[i..]
            .iter()
            .find(|s| **s == PathSegment::Var)
            .is_some()
    }
    pub fn get_val(&self, i: usize) -> Option<&'static str> {
        if let PathSegment::Val(s) = self.segments[i] {
            Some(s)
        } else {
            None
        }
    }
    pub fn segments(&self) -> &'static [PathSegment] {
        self.segments
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
        let path0 = PathTemplate::new(SEGMENTS);
        let path1 = htrpc_entry_point!["foo", _, "baz"];
        assert_eq!(path0, path1);
    }
}
