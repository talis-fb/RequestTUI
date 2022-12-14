use std::path::Path;

use crate::{utils::config::AppFile, view::help::DocView};

use self::handler::DocReaderHandler;

pub mod handler;


pub struct DocsFactory;
impl DocsFactory {
    fn get_file_content(file: &str) -> String {
        let path_file = Path::new(".").join("doc").join(file);
        std::fs::read_to_string(&path_file).map_err(|e| e.to_string()).unwrap()
    }

    pub fn help_reader() -> DocReaderHandler {
        let content = DocsFactory::get_file_content("help.json");
        DocReaderHandler::init(DocView::from_string(content))
    }
}
