fn print_elements(elements: &[String]) {
    // for element in elements {
    //     println!("{}", element)
    // }

    elements
        .iter()
        .map(|el| format!("{} {}", el, el))
        .for_each(|el| println!("{}", el));
}
fn shorten_strings(elements: &mut [String]) {
    elements.iter_mut().for_each(|el| el.truncate(1));
}

fn to_uppercase(elements: &[String]) -> Vec<String> {
    elements.iter().map(|el| el.to_uppercase()).collect()
}

fn move_elements(vec_a: Vec<String>, vec_b: &mut Vec<String>) {
    vec_a.into_iter().for_each(|el| vec_b.push(el));
}

fn explode(list: &[String]) -> Vec<Vec<String>> {
    list.iter()
        .map(|el| el.chars().map(|c| c.to_string()).collect())
        .collect()
}

fn find_color_or(elements: &[String], search: &str) -> String {
    elements
        .iter()
        .find(|el| el.contains(search))
        .map_or(String::from("not found"), |el| String::from(el))
}

fn main() {
    let mut colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];
    //print_elements(&colors[0..3])

    // shorten_strings(&mut colors);

    // let exploded = explode(&colors);
    // println!("{:#?}", exploded)

    let found = find_color_or(&colors, "re");

    println!("{:#?}", found)
}
