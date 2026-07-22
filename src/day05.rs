pub mod day05{
    pub fn day_description(){
        println!("--------------------------------");
        println!("------ DAY 05 -----");
        println!("----We are going to use complext data types today -----");


        // creation vector
        vector_creation();

    }

    fn vector_creation(){

        // mutable vector created empty then updated. 
        let mut v1: Vec<i32> = Vec::new();

        v1.push(1);
        v1.push(5);
        v1.push(67);
    }
}