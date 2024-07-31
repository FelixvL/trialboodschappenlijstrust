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

    alleboodschappen:SimpleListModel::<MyPoint>,
//    alleboodschappen:SimpleListModel<MyPoint>, // both work

    compute_greetings: qt_method!(
        fn compute_greetings(&self, verb: String) -> QString {
            format!("{verb} {}", self.name).into()
        }
    ),
    onzeFunctie: qt_method!(
        fn onzeFunctie(&mut self, productNaam: String){
            println!("{}", productNaam);
            println!("{productNaam}");
            let point:MyPoint= MyPoint {
                a: 1,
                b: 1,
            } ;
            let point2:MyPoint= MyPoint {
                a: 4,
                b: 5,
            } ;           
            println!("{}", point.a);
            println!("{}", point.b);
//            let mut r = SimpleListModel::<MyPoint>::default();
            self.alleboodschappen = SimpleListModel::<MyPoint>::default();
            self.alleboodschappen.push(point);
            //r.push(point2);
            println!("{}", self.alleboodschappen[0].a);
            println!("{}", self.alleboodschappen.row_count());
            //let r = SimpleListModel::<MyPoint>(); 
            //let mut r2 = SimpleListModel::<MyPoint>(); 
            let mut r3: SimpleListModel::<MyPoint>; 


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

#[derive(QObject, Default)]
struct Boodschap{
    base: qt_base_class!(trait QObject),
    productNaam: String,
    prijs: u32,
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
