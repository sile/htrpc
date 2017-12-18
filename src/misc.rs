use std::sync::{Once, ONCE_INIT};
use url::Url;

use {Error, Result};

pub fn parse_relative_url(url: &str) -> Result<Url> {
    static mut DUMMY_BASE_URL: Option<&'static Url> = None;
    static START: Once = ONCE_INIT;

    START.call_once(|| {
        let url = Url::parse("http://foo/").expect("Never fails");
        unsafe {
            let url = Box::new(url);
            DUMMY_BASE_URL = Some(&*Box::into_raw(url))
        }
    });
    let url = track!(
        Url::options()
            .base_url(unsafe { DUMMY_BASE_URL })
            .parse(url)
            .map_err(Error::from)
    )?;
    Ok(url)
}
