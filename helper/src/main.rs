use std::fmt::format;
//This is a much better way to create the classes needed. Do as the author
use std::fs::File;
use std::io::Result;
use std::io::Write;
struct AstSpecs {
    output: String,
    name: String,
    groupNames: Vec<String>,
    groups: Vec<Vec<(String, String)>>,
}
fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("cargo run <Output Directory>");
    let Exr = AstSpecs {
        output: path,
        name: String::from("Exr"),
        groupNames: vec![
            String::from("Binary"),
            String::from("Grouping"),
            String::from("Literal"),
            String::from("Unary"),
        ],
        groups: vec![
            vec![
                (String::from("Expr"), String::from("left")),
                (String::from("Token"), String::from("operator")),
                (String::from("Expr"), String::from("right")),
            ],
            vec![(String::from("Expr"), String::from("expression"))],
            vec![(String::from("String"), String::from("Value"))],
            vec![
                (String::from("Token"), String::from("operator")),
                (String::from("Expr"), String::from("right")),
            ],
        ],
    };
    ast_file_gen(Exr);
}

fn ast_file_gen(config: AstSpecs) -> Result<()> {
    let output_dir = String::from("../interepv1/src/dev/");
    let mut file = File::create(format!("{}{}.rs", output_dir, config.name))?;
    //So we need to first scan the original string, identify the class names, save them in a vector
    //Also for each class, we need to take these expressions and break them down, comma to comma
    let mut class_data: String = String::from(format!(
        "
use crate::token_type::{{Token, Types}};
struct {}
{{
}}
",
        config.name
    ));
    for (name, data) in config.groupNames.iter().zip(config.groups.iter()) {
        let mut class = String::from("struct ");
        class = format!("{} {} \n {{\n", class, name);
        for tupes in data.iter() {
            let (x, y) = tupes;
            class = format!("{} {}:{},\n", class, y, x);
        }
        class = format!("{} }}\n", class);
        class_data = format!("{}{}", class_data, class);
    }
    file.write_all(class_data.as_bytes())?;
    Ok(())
}
