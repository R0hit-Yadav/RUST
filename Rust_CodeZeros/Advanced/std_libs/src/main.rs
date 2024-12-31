//standard library 

//std::men

// //std::mem::replace
// use std::mem;
// fn main()
// {
//     let mut x=5;
//     let old_value=mem::replace(&mut x,10);
//     println!("old:{},New :{}",old_value,x);
// }   



// //std::mem::swap
// fn main()
// {
//     let mut a=1;
//     let mut b=2;
//     std::mem::swap(&mut a,&mut b);
//     println!("a:{},b:{}",a,b);

// }


// //std::mem::take
// fn main()
// {
//     let mut vec=vec![1,2,3];
//     let old=std::mem::take(&mut vec);
//     println!("old:{:?},New:{:?}",old,vec);
// }

// //std::mem::forget
// fn main()
// {
//     let vec=vec![1,2,3];
//     std::mem::forget(vec);
// }



// //std::fs 
// use std::fs;
// fn main()->std::io::Result<()>
// {
//     //write to file 
//     fs::write("new_ex.txt","hello coders!! my name is rohit")?;


//     //read from file 
//     let content=fs::read_to_string("new_ex.txt")?;
//     println!("File content:{}",content);
//     Ok(())

// }



// //create and delete files 
// fn main()->std::io::Result<()>
// {
//     //create a new file 
//     let mut file=std::fs::File::create("./src/new_ex.txt")?;

//     //remove a file 
//     std::fs::remove_file("./src/ex.txt")?;
//     Ok(())
// }




// //directory oprations
// fn main()-> std::io::Result<()>
// {
//     //create a directory 
//     std::fs::create_dir("./src/my_dir")?;

//     //remove a directory
//     std::fs::remove_dir("./src/my_dir")?;
//     Ok(())
// }




//time management
// std::time::Duration 


// use std::time::Duration;
// fn main()
// {
//     let duration=Duration::new(5,0);
//     println!("Duration:{} seconds",duration.as_secs());
// }




// //std::time::Instant
// use std::time::Instant;
// fn main()
// {
//     let start=Instant::now();
//     for _ in 0..10{}
//     let duration=start.elapsed();
//     println!("Time Elapsed:{:?}",duration);

// }


// //self,$self,$mut_self
// struct Counter
// {
//     count:i32,
// }

// impl Counter
// {
//     //consumes the instance
//     fn consume(self)
//     {
//         println!("Consumed:{}",self.count);
//     }

//     //immutable borrow
//     fn view(&self)
//     {
//         println!("View:{}",self.count);
//     }

//     //mutable borrow
//     fn increment(&mut self)
//     {
//         self.count+=1;
//     }
// }

// fn main()
// {
//     let mut counter=Counter{count:10};
//     counter.view();         //view:10
//     counter.increment(); //mutates 'count'
//     counter.consume();  //consumes the instance
// }



// //build.rs
// fn main()
// {
//     println!("cargo:rustc-link-lib=dylib=my_library");

// }



//functional methods 

// //result 
// fn divide(a:i32,b:i32)->Result<i32,String>
// {
//     if b==0
//     {
//         Err("Division by zero".to_string())
//     }
//     else 
//     {
//         Ok(a/b)
//     }
// }

// fn main()
// {
//     match divide(10,2)
//     {
//         Ok(result)=>println!("Result:{}",result),
//         Err(e)=>println!("Error:{}",e),
//     }

// }




// //option
// fn find_number(nums:Vec<i32>,target:i32)-> Option<usize>
// {
//     nums.iter().position(|&x|x==target)
// }
// fn main()
// {
//     if let Some(index)=find_number(vec![1,2,3],2)
//     {
//         println!("Found at index:{}",index);
//     }
//     else
//     {
//         println!("Number not found");
//     }
// }



// //and_then 
// fn double(val:Option<i32>)->Option<i32>
// {
//     val.and_then(|x| Some(x*2))
// }

// fn main()
// {
//     println!("Result:{:?}",double(Some(4)));
// }



// //ok_or_else
// fn get_value(option:Option<i32>)-> Result<i32,String>
// {
//     option.ok_or_else(|| "Value not found".to_string())
// }

// fn main()
// {
//     println!("Result:{:?}",get_value(Some(5)));
// }


// //map
// fn main()
// {
//     let nums=vec![1,2,3];
//     let squared:Vec<i32>=nums.iter().map(|x|x*x).collect();
//     println!("Result:{:?}",squared);
// }



// //mao_err
// fn divide(a:i32,b:i32)->Result<i32,String>
// {
//     if b==0
//     {
//         Err("Cannot divide by zero".to_string())
//     }
//     else 
//     {
//         Ok(a/b)
//     }
// }

// fn main()
// {
//     let result=divide(10,5).map_err(|e| format!("Error:{}",e));
//     println!("Result:{:?}",result);
// }



// //iter
// fn main()
// {
//     let nums=vec![1,2,3];
//     for num in nums.iter()
//     {
//         println!("{}",num);
//     }
// }



// //iter_mut
// fn main()
// {
//     let mut nums=vec![1,2,3];
//     for num in nums.iter_mut()
//     {
//         *num *=2;
//     }
//     println!("Result:{:?}",nums);

// }


// //into_iter
// fn main()
// {
//     let nums=vec![1,2,3];
//     for num in nums.into_iter()
//     {
//         println!("{}",num);
//     }
// }