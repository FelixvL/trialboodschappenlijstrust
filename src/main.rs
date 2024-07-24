/*
 * Copyright (C) 2024  esaf
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; version 3.
 *
 * rtb is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

#[macro_use]
extern crate cstr;
extern crate cpp;
#[macro_use]
extern crate qmetaobject;

use std::env;
use std::path::PathBuf;

use gettextrs::{bindtextdomain, textdomain};
use qmetaobject::*;

//use std::collections::LinkedList;
//use std::cell::RefCell;
mod qrc;

use qmetaobject::QMetaType;

#[derive(QObject, Default)]
struct BoodschappenLijst {
    base: qt_base_class!(trait QObject),
    name: qt_property!(QString; NOTIFY name_changed),
    name_changed: qt_signal!(),
    boodschap: Boodschap,
    nummer: u32,
//    list: LinkedList<u32> = LinkedList::new(), 
    //list: qt_property!(Vec<i32>; CONST),
    //QVector<int> integerVector;
    compute_greetings: qt_method!(
        fn compute_greetings(&self, verb: String) -> QString {
            format!("{verb} {}", self.name).into()
        }
    ),
    onzeFunctie: qt_method!(
        fn onzeFunctie(&self, productNaam: String){
            println!("{}", productNaam);
            println!("{productNaam}");
            //vec<MyPoint>
            //ed:ExtraDing::new();
            //let a:MyPoint = MyPoint();
            // println!("{}", a.0);
            //println!("{}", a.1);
            let point:MyPoint= MyPoint {
                a: 1,
                b: 1,
            } ;
            
            println!("{}", point.a);
            println!("{}", point.b);
            
            

        }
    )
}
// use qmetaobject::QMetaType;
#[derive(SimpleListItem, Clone)]
struct MyPoint{
    pub a:u32,
    pub b:u32,

    // fn new(c:u32, d:u32)->Self{
    //     a = c;
    //     b = d;
    // }
}

//impl QMetaType for MyPoint {};

// impl QMetaType for MyPoint {
//     // fn b()->u32{
//         //     26
//         // }
//     }
//#[derive(SimpleListModel)] 
//struct OnzeLijst{
//} 
//    Ratchanan says:" The public member needs to implement the QMetaType trait" 
//    Ratchanan says:But "impl QMetaType for i32" exists...
//pub struct SimpleListModel<MyPoint: SimpleListItem + 'static> { /* private fields */ }
// trait OnzeLijst{
// }
// #[derive(SimpleListModel, MyPoint)]
// impl SimpleListModel for OnzeLijst{

// }

#[derive(Default, Clone)]
struct ExtraDing{
    nummer: u32,
}
impl QMetaType for ExtraDing{
    // fn new(nummer:u32)->Self{
    //     Self{ nummer }
    // }
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
