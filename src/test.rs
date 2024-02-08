use crate::utils::{get_entries, get_essays};

#[test]
fn check() {
    let ep = get_entries("res", "md");
    // logging::log!("{:?}", ep);
    
    for i in ep {
        let x = get_essays(&i).unwrap().amb;
        println!("{}", serde_yaml::to_string(&x).unwrap());
    }

}