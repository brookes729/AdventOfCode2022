// pub fn get_user_input(request: &str) -> String {
//     let mut line = String::new();
//     println!("{request}");
//     std::io::stdin().read_line(&mut line).unwrap();
//     trim_newline(&mut line);
//     line
// }

// fn trim_newline(s: &mut String) {
//     if s.ends_with('\n') {
//         s.pop();
//         if s.ends_with('\r') {
//             s.pop();
//         }
//     }
// }
