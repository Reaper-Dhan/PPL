use std::collections::HashMap; 

fn main() 
{ 
	let mut my_map = HashMap::new(); 
	my_map.insert("CARS", 3); 
	my_map.insert("BIKES", 2); 
	my_map.insert("CYCLES", 5); 
	match my_map.get("CYCLES") 
	{ 
		Some(&count) => println!("I have {} CYCLES", count), 
		None => println!("I don't have any CYCLES"), 
	} 
	for (vehicle, count) in &my_map 
	{ 
		println!("I have {} {}", count, vehicle); 
	} 
} 
