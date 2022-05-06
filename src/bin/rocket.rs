#[macro_use]
extern crate rocket;

#[macro_use(context)]
extern crate quizmeet_rs;

use percent_encoding::percent_decode;
use quizmeet_rs::{entries::Entry, quiz_sum::*, stats::*};
use regex::Regex;
use rocket_dyn_templates::Template;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/summary")]
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

#[get("/parse")]
fn parse() -> String {
    let (team_sums, quizzer_sums) = open_json(Some(String::from("json/*.json")), None).unwrap();

    format!(
        "team_sums: {:#?}\nquizzer_sums: {:#?}",
        team_sums, quizzer_sums
    )
}

fn table_template(regex: Option<Regex>) -> Template {
    let (team_sums, quizzer_sums) = get_lists(Some(String::from("json/*.json")), regex).unwrap();
    let team_avgs: Vec<String> = team_sums
        .iter()
        .map(|v| format!("{:.2}", v.avg()))
        .collect();
    let quizzer_avgs: Vec<String> = quizzer_sums
        .iter()
        .map(|v| format!("{:.2}", v.avg()))
        .collect();

    Template::render(
        "table-view",
        context! {
            team_sums,
            team_avgs,
            quizzer_sums,
            quizzer_avgs,
        },
    )
}

#[get("/table")]
pub fn table() -> Template {
    table_template(None)
}

#[get("/table/div/<div>")]
pub fn table_div(div: &str) -> Template {
    let mut r = String::from("D");
    r += div;
    r += r"Q(?P<q>(\d|\w)+).json$";
    table_template(Some(Regex::new(r.as_str()).unwrap()))
}

#[get("/table/<regex>")]
pub fn table_regex(regex: &str) -> Template {
    let iter = percent_decode(regex.as_bytes());
    let decoded = iter.decode_utf8_lossy().into_owned();
    table_template(Some(Regex::new(&decoded).unwrap()))
}

#[get("/tera")]
pub fn tera() -> Template {
    let name = String::from("Tommy");
    Template::render(
        "index",
        context! {
            title: "Hello",
            name: Some(name),
            items: vec!["One", "Two", "Three"],
        },
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, summary, parse, table, table_div, table_regex, tera])
        .attach(Template::fairing())
}
