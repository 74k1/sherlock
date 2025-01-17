use gio::glib;

use super::{util::SherlockError, Loader};

impl Loader {
    pub fn load_resources()-> Result<(), SherlockError>{
        let res_bytes = include_bytes!("../../resources.gresources");
        let resource = gio::Resource::from_data(&glib::Bytes::from_static(res_bytes))
            .map_err(|e| SherlockError {
                name: "Resource Parsing Error".to_string(),
                message: format!("Failed to parse gresources."),
                traceback: e.to_string(),
            })?;
        gio::resources_register(&resource);
        Ok(())
    }
}

