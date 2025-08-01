mod imp {
    use gtk4::subclass::prelude::*;
    use gtk4::CompositeTemplate;
    use gtk4::{glib, Spinner};
    use gtk4::{Box as GtkBox, Image, Label};

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/dev/skxxtz/sherlock/ui/weather_tile.ui")]
    pub struct WeatherTile {
        #[template_child(id = "temperature")]
        pub temperature: TemplateChild<Label>,

        #[template_child(id = "location")]
        pub location: TemplateChild<Label>,

        #[template_child(id = "icon-name")]
        pub icon: TemplateChild<Image>,

        #[template_child(id = "spinner")]
        pub spinner: TemplateChild<Spinner>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for WeatherTile {
        const NAME: &'static str = "WeatherTile";
        type Type = super::WeatherTile;
        type ParentType = GtkBox;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for WeatherTile {}
    impl WidgetImpl for WeatherTile {}
    impl BoxImpl for WeatherTile {}
}

use gtk4::glib;

glib::wrapper! {
    pub struct WeatherTile(ObjectSubclass<imp::WeatherTile>)
        @extends gtk4::Widget, gtk4::Box,
        @implements gtk4::Buildable;
}

impl WeatherTile {
    pub fn new() -> Self {
        glib::Object::new::<Self>()
    }
}
