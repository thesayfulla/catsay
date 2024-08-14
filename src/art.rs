use std::collections::HashMap;

pub fn get_art(option: &str) -> Option<&'static str> {
    let mut cats = HashMap::new();

    cats.insert("whiskers", r#"
 /\_/\  
( o.o ) 
 > ^ <
"#);

    cats.insert("mittens", r#"
 /\_/\  
( @.@ ) 
 > ^ <
"#);

    cats.insert("shadow", r#"
 /\_/\  
( -.- ) 
  >~<
"#);

    cats.insert("luna", r#"
 /\_/\  
( ' ' ) 
 >_^_<
"#);

    cats.insert("simba", r#"
 /\_/\  
( o_o ) 
 >- -<
"#);

    cats.insert("nala", r#"
 /\_/\  
( ^.^ ) 
 > ^ <
"#);

    cats.insert("oreo", r#"
 /\_/\  
( >.< ) 
  > <
"#);


    cats.insert("cleo", r#"
 /\_/\  
( -.- ) 
 >_^_<
"#);

    cats.get(option).copied()
}

pub fn get_options() -> Vec<&'static str> {
    let mut options = Vec::new();

    options.push("whiskers");
    options.push("mittens");
    options.push("shadow");
    options.push("luna");
    options.push("simba");
    options.push("nala");
    options.push("oreo");
    options.push("cleo");

    options
}
