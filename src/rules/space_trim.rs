//! This rule is triming spaces of the whole string.
//!
//! Options
//! - trim_space: bool

use crate::{config::Config, cursor::Cursor};

pub fn rule(cursor: &mut Cursor, config: &Config) {
    // skip if there is no options
    if !config.no_space_inside_hyper_mark {
        return;
    }

    // skip non-after-token situations
    if let Some(after) = cursor.after() {
        // skip non-mark situations
        if !cursor.current().is_wrapper() && !after.is_wrapper() {
            return;
        }

        // 1. left x left
        // 2. right x right
        // 3. left x non-mark
        // 4. non-mark x right
        if (cursor.current().is_start_wrapper() && after.is_start_wrapper())
            || (cursor.current().is_end_wrapper() && after.is_end_wrapper())
            || (cursor.current().is_start_wrapper() && !after.is_wrapper())
            || (!cursor.current().is_wrapper() && after.is_end_wrapper())
        {
            cursor.current_mut().remove_space_after();
        }
    }
}
