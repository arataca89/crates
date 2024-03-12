pub fn soma(x: i32, y: i32) -> i32 {
    x + y
}

#[test]
fn soma_test(){
	assert_eq!(soma(2,3), 5);
}
//#[cfg(test)]
// mod tests {
    // use super::*;

    // #[test]
    // fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    // }
// }
