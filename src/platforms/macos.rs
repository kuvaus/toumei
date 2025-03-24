use objc2::rc::Retained;
use objc2::runtime::AnyObject;
use objc2_foundation::{ns_string, NSUserDefaults};
use objc2::msg_send;

use crate::{Error, Mode};

pub fn detect_tray_transparency() -> Result<Mode, Error> {
    unsafe {
        let defaults = NSUserDefaults::standardUserDefaults();
        let ua_domain = defaults
            .persistentDomainForName(ns_string!("com.apple.universalaccess"))
            .ok_or(Error::PersistentDomainFailed)?;

        let reduce_transparency = ua_domain
            .objectForKey(ns_string!("reduceTransparency"))
            .map(|value: Retained<AnyObject>| {
                let value: &AnyObject = &*value;
                let reduced: bool = msg_send![value, boolValue];
                reduced
            })
            .unwrap_or(false);

        Ok((!reduce_transparency).into())
    }
}