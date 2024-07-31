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

use qmetaobject::QMetaType;

#[derive(QObject, Default)]
struct BoodschappenLijst {
    base: qt_base_class!(trait QObject),
    name: qt_property!(QString; NOTIFY name_changed),
    name_changed: qt_signal!(),
    boodschap: Boodschap,
    nummer: u32,
    alleboodschappen:SimpleListModel::<Boodschap>,

    compute_greetings: qt_method!(
        fn compute_greetings(&self, verb: String) -> QString {
            format!("{verb} {}", self.name).into()
        }
    ),
    trialInit: qt_method!(
        fn trialInit(&mut self){
            self.alleboodschappen = SimpleListModel::<Boodschap>::default();
        }
    ),
    onzeFunctie: qt_method!(
        fn onzeFunctie(&mut self, productNaamx: String){
            println!("{}", productNaamx);
            let point:Boodschap= Boodschap {
                productNaam: productNaamx,
                prijs: 1,
            } ;
            self.alleboodschappen.push(point);
            println!("---{}", self.alleboodschappen[0].productNaam);
            println!("{}", self.alleboodschappen.row_count());
            println!("-----------------");
            for a in self.alleboodschappen.iter(){
                println!("{}", a.productNaam);
            }
        }
    )
}
#[derive(SimpleListItem, Default)]
struct MyPoint{
    pub a:u32,
    pub b:u32,
}

#[derive(Default, Clone)]
struct ExtraDing{
    nummer: u32,
}
impl QMetaType for ExtraDing{
}

#[derive(SimpleListItem, Default)]
struct Boodschap{
    pub productNaam: String,
    pub prijs: u32,
}
impl Boodschap{

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
