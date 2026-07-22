pub mod day05{
    pub fn day_description(){
        println!("--------------------------------");
        println!("------ DAY 05 -----");
        println!("----We are going to use complext data types today -----");


        // creation vector - option 0
        vector_creation_0();

        vector_creation_1();

        let v3 = vector_generation_0();
        print!("All v3 elements (coming from the child fn): {:?}\n", v3);
        
        let v3_1 = consume_vector_first_item(&v3);
        print!("First v3 element (coming from the child fn), reference user: {:?}\n", v3_1);
        
        
        let v3_select = selecting_vector_item(&v3, 2);
        print!("Selected v3 element (coming from the child fn), reference user: {:?}\n", v3_select);
        

    }

    fn vector_creation_0(){

        // mutable vector created empty then updated. 
        let mut v1: Vec<i32> = Vec::new();

        v1.push(1);
        v1.push(5);
        v1.push(67);
        // specital notiation for vectors
        print!("All v1 elements: {:?}\n", v1)
    }


    fn vector_creation_1(){

        // immutinble vector
        let v2 = vec![3,45,67];
        // specital notiation for vectors
        print!("All v2 elements: {:?}\n", v2)
    }

    fn vector_generation_0() -> Vec<i32> {
    
        let v3 = vec![6,3,47];

        v3
    }

    fn consume_vector_first_item(v:&Vec<i32>) -> i32 {
        v[0]
    }

    fn selecting_vector_item(v:&Vec<i32>, position: usize) -> (Vec<i32>,i32) {
        ( v.clone(), v[position]  )

    }


}