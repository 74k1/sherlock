use super::util::SherlockSearch;
use gtk4::ListBoxRow;

use super::util::{get_builder, insert_attrs};
use super::Tile;

impl Tile {
    pub fn simple_text_tile(
        index: i32,
        lines: &Vec<String>,
        method: &str,
        keyword: &str,
    ) -> (i32, Vec<ListBoxRow>) {
        let mut results: Vec<ListBoxRow> = Default::default();
        let mut index_ref = index;

        for item in lines {
            if item.fuzzy_match(keyword) {
                let builder = get_builder(
                    "/dev/skxxtz/sherlock/ui/simple_text_tile.ui",
                    index_ref,
                    true,
                );

                builder.title.set_text(item);

                let attrs: Vec<(&str, &str)> = vec![("method", method), ("keyword", keyword)];

                insert_attrs(&builder.attrs, attrs);
                index_ref += 1;
                results.push(builder.object);
            }
        }

        return (index_ref, results);
    }
}
