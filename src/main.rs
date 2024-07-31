#[macro_use]
extern crate cstr;
extern crate cpp;
#[macro_use]
extern crate qmetaobject;

use std::env;
use std::path::PathBuf;

use gettextrs::{bindtextdomain, textdomain};
use qmetaobject::*;

mod qrc;

#[derive(QObject, Default)]
struct BoodschappenLijst {
    base: qt_base_class!(trait QObject),
    name: qt_property!(QString; NOTIFY name_changed),
    name_changed: qt_signal!(),
    alleboodschappen:SimpleListModel::<Boodschap>,

    compute_greetings: qt_method!(
        fn compute_greetings(&self, verb: String) -> QString {
            format!("{verb} {}", self.name).into()
        }
    ),
    trial_init: qt_method!(
        fn trial_init(&mut self){
            self.alleboodschappen = SimpleListModel::<Boodschap>::default();
        }
    ),
    onze_functie: qt_method!(
        fn onze_functie(&mut self, product_naam_p: String){
            let point:Boodschap= Boodschap {
                product_naam: product_naam_p,
                prijs: 1,
            } ;
            self.alleboodschappen.push(point);
            for a in self.alleboodschappen.iter(){
                println!("{}", a.product_naam);
            }
        }
    )
}

#[derive(SimpleListItem, Default)]
struct Boodschap{
    pub product_naam: String,
    pub prijs: u32,
}

fn main() {
    init_gettext();
    unsafe {
        cpp! { {
            #include <QtCore/QCoreApplication>
            #include <QtCore/QString>
        }}
        cpp! {[]{
            QCoreApplication::setApplicationName(QStringLiteral("rtb.esaf"));
        }}
    }
    QQuickStyle::set_style("Suru");
    qrc::load();
    qml_register_type::<BoodschappenLijst>(cstr!("BoodschappenLijst"), 1, 0, cstr!("BoodschappenLijst"));

    let mut engine = QmlEngine::new();
    engine.load_file("qrc:/qml/Main.qml".into());
    engine.exec();
}

fn init_gettext() {
    let domain = "rtb.esaf";
    textdomain(domain).expect("Failed to set gettext domain");

    let mut app_dir_path = env::current_dir().expect("Failed to get the app working directory");
    if !app_dir_path.is_absolute() {
        app_dir_path = PathBuf::from("/usr");
    }

    let path = app_dir_path.join("share/locale");

    bindtextdomain(domain, path.to_str().unwrap()).expect("Failed to bind gettext domain");
}
