use rand::Rng;

mod elem_wise_mul;
use crate::elem_wise_mul::elem_wise_mul::elem_wise_multiply;




fn main() {
    let mut rng = rand::thread_rng();
    let mut v1 :Vec<f32> = vec![0.0; 15];
    //The following is equivalent but potentially less efficient or rather slower
    //let mut v1 : Vec<f32> = Vec::with_capacity(15);
    println!("v1 before assignment of random f32 elems == ");
    println!("{:?}",v1);
    let mut v2 : Vec<f32> = vec![0.0; 15];
    println!("v2 before assignment of random f32 elems == ");
    println!("v2 == {:?}",v2); 
    println!();
    println!();
    assert!(v1.len() == 15);
    assert_eq!(v2.len(), 15);
    
    //using an iterator is more efficient in Rust than looping through indices
    let mut v1_iterator= v1.iter_mut();

    
    while let Some(val) = v1_iterator.next(){
        *val = rng.gen::<f32>();
    }
    
    let v2 :Vec<f32> = (&mut v2).into_iter()
        .map(|x| {
            *x = rng.gen::<f32>();
            *x
        }
        ).collect();

    println!("End of v1 and v2 assignment of random f32 elems");
    println!("v1 is now equal to {:?}",v1);
    println!("v2 now equates to {:?}",v2);
  
    println!();
    println!();
    //Another way to generate random vectors 
    //Functional style using iterators
    let numbers : Vec<i32> = (1..=10) //Range<i32> is an iterator too
        .map(|_|{ //map is another iterator adaptor
            rng.gen_range(1,21)
        })
    .collect(); //collect is a consuming adaptor that consumes and collects 
    //the "iterated" values into a vector above.
    
    println!("numbers == {:?}",numbers);
    

    //And now using the imperative style:->
    let mut nums : Vec<i64> = Vec::with_capacity(25);
    for _ in 1..25{
        nums.push(rng.gen_range(99,999));
    }
    println!();
    println!("nums == {:?}",nums);
    
    //Let's multiply the 2 vectors v1 and v2 element-wise
    //The function elem_wise_multiply() will take a reference to the vectors
    match elem_wise_multiply(&v1,&v2){
        Some(out) => {
            println!("result of vector multiplication == ");
            println!("{:?}",out);
        }
        None => {panic!("Panicking!!! The 2 vectors MUST have equal lengths.");}
    }

}

