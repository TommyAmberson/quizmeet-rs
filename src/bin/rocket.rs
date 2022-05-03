#[macro_use] extern crate rocket;

use quizmeet_rs::quiz_sum::*;

use rocket_dyn_templates::Template;
use rocket::serde;

#[macro_export]
macro_rules! context {
    ($($key:ident $(: $value:expr)?),*$(,)?) => {{
        use $crate::serde::ser::{Serialize, Serializer, SerializeMap};
        use ::std::fmt::{Debug, Formatter};

        #[allow(non_camel_case_types)]
        struct ContextMacroCtxObject<$($key: Serialize),*> {
            $($key: $key),*
        }

        #[allow(non_camel_case_types)]
        impl<$($key: Serialize),*> Serialize for ContextMacroCtxObject<$($key),*> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where S: Serializer,
            {
                let mut map = serializer.serialize_map(None)?;
                $(map.serialize_entry(stringify!($key), &self.$key)?;)*
                map.end()
            }
        }

        #[allow(non_camel_case_types)]
        impl<$($key: Debug + Serialize),*> Debug for ContextMacroCtxObject<$($key),*> {
            fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result {
                f.debug_struct("context!")
                    $(.field(stringify!($key), &self.$key))*
                    .finish()
            }
        }

        ContextMacroCtxObject {
            $($key $(: $value)?),*
        }
    }};
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/")]
fn summary() -> String {
    // println!("{}", quiz_sum::hello());
    let mut sum = Summary::new();
    sum.open_ods().unwrap();
    // dbg!(&sum);
    // dbg!(sum.get_team_prelims(1));
    let mut result = String::from("");
    let t = sum.get_team_order(|q| q.div == 1 && matches!(q.quiz, QuizType::Preliminary(_)));
    dbg!(&t);
    result.push_str(&format!("{:?}", &t));
    let q = sum.get_quizzer_order(|q| q.div == 1);
    dbg!(&q);
    result.push_str(&format!("{:?}", q));

    result
}

#[get("/")]
pub fn tera() -> Template {
    let name = String::from("Tommy");
    Template::render("index", context! {
        title: "Hello",
        name: Some(name),
        items: vec!["One", "Two", "Three"],
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/summary", routes![summary])
        .mount("/tera", routes![tera])
        .attach(Template::fairing())
}

