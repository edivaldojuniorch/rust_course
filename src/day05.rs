pub mod day05{
    use std::vec;

    pub fn day_description(){
        println!("--------------------------------");
        println!("------ DAY 05 -----");
        println!("----We are going to use complext data types today -----");


        // creation vector - option 0
        vector_creation_0();

        vector_creation_1();

        let v3: Vec<i32> = vector_generation_0();
        print!("All v3 elements (coming from the child fn): {:?}\n", v3);
        
        let v3_1 = consume_vector_first_item(&v3);
        print!("First v3 element (coming from the child fn), reference user: {:?}\n", v3_1);
        
        
        let v3_select = selecting_vector_item(&v3, 1);
        print!("Selected v3 element (coming from the child fn), reference user: {:?}\n", &v3_select);
        
        vector_iteraction(&v3);

        vector_iteraction_full(&v3);

        
        vector_drop_value(&v3, 0)

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

        let selected: Option<&i32> = v.get(position);

        match selected{
            Some(item) => ( v.clone(), *item),
            None =>{
                println!("There is no third element.");
                (v.clone(), -1)
            } 
        }

    }

    fn vector_iteraction(v: &Vec<i32>) {
        for item in v{
            println!("Item: {}", item);

        }

    }
    
    fn vector_iteraction_full(v: &Vec<i32>) {
        print!("printing from the 'vector_iteraction_full'\n");
        for item in v{
            println!("Item: {}", item);

        }

    }
        
    fn vector_drop_value(v: &Vec<i32>, drop_position: usize) {

        let v0 = &v.clone();

        print!("printing from the 'vector_drop_value'\n");

        let selected: Option<&i32> = v0.get(drop_position);

        match selected{
            Some(item) => print!(" Item found in the position {}: {}\n",drop_position,  *item),
            None => println!("Element not found in the position {}!", drop_position)
 
        }
    

    }


    }

