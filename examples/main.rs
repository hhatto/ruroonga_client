extern crate ruroonga_client as groonga;
extern crate json_flex;

use groonga::builtin::command_query::CommandQuery;

const GROONGA_SERVER: &'static str = "localhost";

fn create_table() {
    let mut request = groonga::HTTPRequest::new();
    let mut command = CommandQuery::new("table_create");
    let uri_base = groonga::URIBase::new()
        .base_uri(GROONGA_SERVER.to_string()).build();
    command.set_argument(vec![("name", "Sites"),
                              ("flags","TABLE_HASH_KEY"),("key_type","ShortText")]);
    let url = groonga::RequestURI::new(uri_base, command.encode()).url();
    println!("load url: {}", url);
    let res = request.get(url);
    let result = request.receive(&mut res.unwrap()).unwrap();
    println!("result: {}", result);
}

fn create_column() {
    let mut request = groonga::HTTPRequest::new();
    let mut command = CommandQuery::new("column_create");
    let uri_base = groonga::URIBase::new()
        .base_uri(GROONGA_SERVER.to_string()).build();
    command.set_argument(vec![("table", "Sites"),
                              ("name","title"),("type","ShortText")]);
    let url = groonga::RequestURI::new(uri_base, command.encode()).url();
    println!("load url: {}", url);
    let res = request.get(url);
    let result = request.receive(&mut res.unwrap()).unwrap();
    println!("result: {}", result);
}

fn load() {
    let data = r#"
[
{"_key":"http://example.org/","title":"This is test record 1!"},
{"_key":"http://example.net/","title":"test record 2."},
{"_key":"http://example.com/","title":"test test record three."},
{"_key":"http://example.net/afr","title":"test record four."},
{"_key":"http://example.org/aba","title":"test test test record five."},
{"_key":"http://example.com/rab","title":"test test test test record six."},
{"_key":"http://example.net/atv","title":"test test test record seven."},
{"_key":"http://example.org/gat","title":"test test record eight."},
{"_key":"http://example.com/vdw","title":"test test record nine."},
]"#;
    let mut request = groonga::HTTPRequest::new();
    let mut load_command = CommandQuery::new("load");
    let uri_base = groonga::URIBase::new()
        .base_uri(GROONGA_SERVER.to_string()).build();
    load_command.set_argument(vec![("table", "Sites")]);
    let load_url = groonga::RequestURI::new(uri_base, load_command.encode()).url();
    println!("load url: {}", load_url);
    println!("load data: {}", data);
    let load_res = request.load(load_url, data.to_string());
    let load_result = request.receive(&mut load_res.unwrap()).unwrap();
    println!("result: {}", load_result);
}

#[derive(Clone, Debug)]
struct Result {
    id: i64,
    key: String,
    title: String
}

fn main() {
    create_table();
    create_column();
    load();
    let mut request = groonga::HTTPRequest::new();
    let mut command = CommandQuery::new("select");
    let uri_base = groonga::URIBase::new()
        .base_uri(GROONGA_SERVER.to_string()).build();
    command.set_argument(vec![("table", "Sites")]);
    let url = groonga::RequestURI::new(uri_base, command.encode()).url();
    println!("url: {}", url);
    let res = request.get(url);
    let result = request.receive(&mut res.unwrap()).unwrap();
    println!("result: {}", result);
    let mut decode = groonga::ResultParser::new(result);
    println!("status: {:?}", decode.status().unwrap());
    println!("start: {:?}", decode.start_time().unwrap());
    println!("elapsed: {:?}", decode.elapsed_time().unwrap());
    if decode.status().unwrap().clone() == 0 {
        println!("matched columns: {:?}", decode.matched_columns().unwrap());
        let decoded_vec = decode.into_row().columns().unwrap();
        println!("index access: {:?}", decoded_vec[2]);
        // Read got response
        let mut result_vec: Vec<Result> = Vec::new();
        println!("-- Display raw decoded json values --");
        // Skip reading result header
        for v in decoded_vec.iter().skip(2) {
            println!("{:?}", v);
            let raw = v.unwrap_vec();
            let elem = Result { id: raw[0].unwrap_i64().clone(),
                                key: raw[1].unwrap_string().clone(),
                                title: raw[2].unwrap_string().clone() };
            result_vec.push(elem.clone());
        }
        println!("-- Mapped to user-defined struct -- ");
        println!("{:?}", result_vec);
    } else {
        println!("Couldn't get success response.")
    }
}
